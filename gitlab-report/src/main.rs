// MIT License
//
// Copyright (c) 2021 Tobias Pfeiffer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![allow(dead_code, clippy::from_over_into)]

mod generate;
mod cargo;
mod clippy;
mod audit;
mod geiger;
mod fmt;
mod junit;
mod code_climate;
mod gitlab_security_report;

use {std::{io, collections::HashMap}, serde::*};

const HELP: &str = r#"
USAGE:
	gitlab-report [options]

DESCRIPTION:
	Generates GitLab compitable reports from cargo JSON output.
	
	If no input file is specified, the messages will be read from STDIN.
	If no output file is specified, the report will be written to STDOUT.
	If no output format is specified, it will be inferred.

OPTIONS:
    -h, --help                   display this help
    -i, --input-file <path>      input file
    -o, --output-file <path>     output file
    -p, --input-format <format>  input format, one of `test`, `clippy`, `bench`, `audit`, `geiger` or `fmt`
    -f, --output-format <format> output format, one of `junit`, `code-quality`, `openmetrics`, `gl-sast` or `gl-dep-scan`
    
EXAMPLES:
	cargo test --no-fail-fast -- -Z unstable-options --format json | gitlab-report -p test > report.xml
	cargo clippy --message-format=json | gitlab-report -p clippy > gl-code-quality-report.json
	cargo bench -- -Z unstable-options --format json | gitlab-report -p bench > metrics.txt
	cargo audit --json | gitlab-report -p audit -f gl-sast > gl-sast-report.json
	cargo audit --output-format Json | gitlab-report -p geiger -f gl-sast > gl-sast-report.json
"#;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum InputFormat {
	Test,
	Clippy,
	Bench,
	Audit,
	Geiger,
	Fmt
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum OutputFormat {
	Junit,
	CodeClimate,
	OpenMetrics,
	GlSast,
	GlDepScan
}

fn main() {
	let mut args = std::env::args();
	let _name = args.next().unwrap();
	let mut file_in    = None;
	let mut file_out   = None;
	let mut format_in  = None;
	let mut format_out = None;
	
	loop {
		match args.next().as_deref() {
			Some("-h") | Some("--help")          => {
				eprintln!("{}", HELP);
				return;
			}
			Some("-i") | Some("--input")         => file_in    = Some(args.next().unwrap()),
			Some("-o") | Some("--output")        => file_out   = Some(args.next().unwrap()),
			Some("-p") | Some("--input-format")  => format_in  = Some(match &*args.next().unwrap() {
				"test"   => InputFormat::Test,
				"clippy" => InputFormat::Clippy,
				"bench"  => InputFormat::Bench,
				"audit"  => InputFormat::Audit,
				"geiger" => InputFormat::Geiger,
				"fmt"    => InputFormat::Fmt,
				v => {
					eprintln!("error: invalid input format: {}", v);
					std::process::exit(1);
				}
			}),
			Some("-f") | Some("--output-format") => format_out = Some(match &*args.next().unwrap() {
				"junit"        => OutputFormat::Junit,
				"code-quality" => OutputFormat::CodeClimate,
				"openmetrics"  => OutputFormat::OpenMetrics,
				"gl-sast"      => OutputFormat::GlSast,
				"gl-dep-scan"  => OutputFormat::GlDepScan,
				v => {
					eprintln!("error: invalid output format: {}", v);
					std::process::exit(1);
				}
			}),
			Some(v)                              => eprintln!("warning: unknown argument: {}", v),
			None                                 => break,
		}
	}
	
	if format_in.is_none() && format_out.is_none() {
		eprintln!("{}", HELP);
		return;
	}
	
	let reader: Box<dyn io::Read> = match file_in {
		Some(file) => Box::new(match std::fs::File::open(file) {
			 Ok(v) => v,
			 Err(e) => {
				 eprintln!("error: failed to open input file: {}", e);
				 std::process::exit(1);
			 }
		 }),
		None => Box::new(io::stdin())
	};
	
	let writer: Box<dyn io::Write> = match file_out {
		Some(file) => Box::new(match std::fs::OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.open(file)
		{
			 Ok(v) => v,
			 Err(e) => {
				 eprintln!("error: failed to open output file: {}", e);
				 std::process::exit(1);
			 }
		 }),
		None => Box::new(io::stdout())
	};
	
	let reader = io::BufReader::new(reader);
	let writer = io::BufWriter::new(writer);
	
	match (format_in, format_out) {
		(Some(InputFormat::Test),   None | Some(OutputFormat::Junit))       => generate::test_to_junit(reader, writer),
		(Some(InputFormat::Test),   Some(OutputFormat::OpenMetrics))        => generate::test_to_open_metrics(reader, writer),
		(Some(InputFormat::Clippy), None | Some(OutputFormat::CodeClimate)) => generate::clippy_to_code_quality(reader, writer),
		(Some(InputFormat::Clippy), Some(OutputFormat::OpenMetrics))        => generate::clippy_to_open_metrics(reader, writer),
		(Some(InputFormat::Bench),  None | Some(OutputFormat::OpenMetrics)) => generate::bench_to_open_metrics(reader, writer),
		(Some(InputFormat::Audit),  None | Some(OutputFormat::GlSast))      => generate::audit_to_gitlab_security_report(gitlab_security_report::ScanType::Sast, reader, writer),
		(Some(InputFormat::Audit),  Some(OutputFormat::GlDepScan))          => generate::audit_to_gitlab_security_report(gitlab_security_report::ScanType::DependencyScanning, reader, writer),
		(Some(InputFormat::Geiger), None | Some(OutputFormat::GlSast))      => generate::geiger_to_gitlab_security_report(gitlab_security_report::ScanType::Sast, reader, writer),
		(Some(InputFormat::Geiger), Some(OutputFormat::GlDepScan))          => generate::geiger_to_gitlab_security_report(gitlab_security_report::ScanType::DependencyScanning, reader, writer),
		_ => {
			eprintln!(
				"error: invalid input and output format combination: {} -> {}",
				format_in.map_or_else(|| "?".to_string(), |v| format!("{:?}", v)),
				format_out.map_or_else(|| "?".to_string(), |v| format!("{:?}", v)),
			);
			std::process::exit(1);
		}
	}
}
