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

mod cargo;
mod audit;
mod junit;
mod code_climate;
mod sast;

use {
	self::{cargo::*, audit::*, junit::*, code_climate::*, sast::*},
	std::{io, collections::HashMap},
	serde::*
};

const HELP: &str = r#"
Usage: gitlab-report [options]

Description:
Generates GitLab compitable reports from cargo JSON output.

If no input file is specified, the messages will be read from STDIN.
If no output file is specified, the report will be written to STDOUT.
If no output format is specified, it will be inferred.

Options:
    -h, --help                   display this help
    -i, --input-file <path>      input file
    -o, --output-file <path>     output file
    -p, --input-format <format>  input format, one of `test`, `clippy`, `bench` or `audit`
    -f, --output-format <format> output format, one of `junit`, `code-quality`, `openmetrics` or `sast`
    
Examples:
	cargo test --no-fail-fast -- -Z unstable-options --format json | gitlab-report -p test > report.xml
	cargo clippy --message-format=json | gitlab-report -p clippy > gl-code-quality-report.json
	cargo bench -- -Z unstable-options --format json | gitlab-report -p bench > metrics.txt
	cargo audit | gitlab-report -p audit > gl-sast-report.json
"#;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum InputFormat {
	Test,
	Clippy,
	Bench,
	Audit
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum OutputFormat {
	Junit,
	CodeClimate,
	OpenMetrics,
	Sast
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
				v => {
					eprintln!("error: invalid input format: {}", v);
					std::process::exit(1);
				}
			}),
			Some("-f") | Some("--output-format") => format_out = Some(match &*args.next().unwrap() {
				"junit"        => OutputFormat::Junit,
				"code-quality" => OutputFormat::CodeClimate,
				"openmetrics"  => OutputFormat::OpenMetrics,
				"sast"         => OutputFormat::Sast,
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
		(Some(InputFormat::Test),   None | Some(OutputFormat::Junit))       => generate_test_to_junit(reader, writer),
		(Some(InputFormat::Test),   Some(OutputFormat::OpenMetrics))        => generate_test_to_open_metrics(reader, writer),
		(Some(InputFormat::Clippy), None | Some(OutputFormat::CodeClimate)) => generate_clippy_to_code_quality(reader, writer),
		(Some(InputFormat::Clippy), Some(OutputFormat::OpenMetrics))        => generate_clippy_to_open_metrics(reader, writer),
		(Some(InputFormat::Bench),  None | Some(OutputFormat::OpenMetrics)) => generate_bench_to_open_metrics(reader, writer),
		(Some(InputFormat::Audit),  None | Some(OutputFormat::Sast))        => generate_audit_to_sast(reader, writer),
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

fn generate_test_to_junit(
	reader: impl io::BufRead,
	mut writer: impl io::Write
) {
	let mut suites = Vec::new();
	
	for line in reader.lines() {
		let msg = match line.and_then(|line| serde_json::from_str(&line)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)))
		{
			Ok(v) => v,
			Err(e) => {
				eprintln!("error: failed to parse message: {}", e);
				std::process::exit(1);
			}
		};
		
		match msg {
			CargoMessage::Suite(CargoTestReportSuite::Started(v)) => {
				let now = std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos();
				suites.push(JUnitReportTestsuite {
					id:         suites.len(),
					name:       format!("cargo test #{}", suites.len()),
					timestamp:  chrono::NaiveDateTime::from_timestamp(
						(now / 1_000_000_000) as _, (now % 1_000_000_000) as _)
						.format("%Y-%m-%dT%H:%M:%S").to_string(),
					hostname:   "localhost".to_string(),
					tests:      v.test_count,
					testcases:  Some(Vec::new()),
					..Default::default()
				});
			}
			CargoMessage::Suite(CargoTestReportSuite::Ok(v) | CargoTestReportSuite::Failed(v)) => {
				let suite = suites.last_mut().unwrap();
				suite.failures = v.failed;
				suite.errors   = 0;
				suite.skipped  = v.ignored + v.filtered_out;
				suite.time     = v.exec_time;
			}
			CargoMessage::Test(CargoTestReportTest { name, event }) => {
				let now = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs_f64();
				let suite = suites.last_mut().unwrap();
				let testcases = suite.testcases.as_mut().unwrap();
				let (module, name) = name.rsplit_once("::").unwrap_or(("", &name));
				
				match event {
					CargoTestReportTestEvent::Started => suite.testcases.as_mut().unwrap().push(JUnitReportTestsuiteTestcase {
						status:    None,
						name:      name.to_string(),
						classname: module.to_string(),
						time:      now
					}),
					CargoTestReportTestEvent::Ignored => testcases.push(JUnitReportTestsuiteTestcase {
						status:    Some(JUnitReportTestsuiteTestcaseStatus::Skipped),
						name:      name.to_string(),
						classname: module.to_string(),
						time:      0.0
					}),
					event @ CargoTestReportTestEvent::Ok(_) | event @ CargoTestReportTestEvent::Failed(_) => {
						let testcase = testcases.iter_mut()
							.find(|case| case.classname == module && case.name == name)
							.unwrap();
						
						testcase.time = now - testcase.time;
						testcase.status = match event {
							CargoTestReportTestEvent::Ok(_v) => None,
							CargoTestReportTestEvent::Failed(v) => Some(JUnitReportTestsuiteTestcaseStatus::Failure {
								r#type:  "cargo test".to_string(),
								message: v.stdout.unwrap_or_else(String::new)
							}),
							_ => unreachable!()
						};
					}
				}
			}
			CargoMessage::Bench(_) => ()
		}
	}
	
	eprintln!("  \x1b[32;1mGenerating\x1b[0m JUnit report");
	
	if let Err(e) = writeln!(&mut writer, "<?xml version=\"1.0\" encoding=\"utf-8\"?>") {
		eprintln!("error: failed to generate report: {:?}", e);
		std::process::exit(1);
	} else if let Err(e) = quick_xml::se::to_writer(writer, &JUnitReport(suites)) {
		eprintln!("error: failed to generate report: {:?}", e);
		std::process::exit(1);
	}
}

fn generate_test_to_open_metrics(
	reader: impl io::BufRead,
	mut writer: impl io::Write
) {
	let mut i = 0;
	
	for line in reader.lines() {
		let msg = match line.and_then(|line| serde_json::from_str(&line)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)))
		{
			Ok(v) => v,
			Err(e) => {
				eprintln!("error: failed to parse message: {}", e);
				std::process::exit(1);
			}
		};
		
		if let CargoMessage::Suite(CargoTestReportSuite::Ok(v) | CargoTestReportSuite::Failed(v)) = msg {
			if let Err(e) = write!(&mut writer,
r#"passed{{suite={0}}}: {1}
failed{{suite={0}}}: {2}
allowed_fail{{suite={0}}}: {3}
ignored{{suite={0}}}: {4}
measured{{suite={0}}}: {5}
filtered_out{{suite={0}}}: {6}
exec_time{{suite={0}}}: {7}
"#, format!("cargo test #{}", i), v.passed, v.failed, v.allowed_fail, v.ignored, v.measured, v.filtered_out, v.exec_time) {
				eprintln!("error: failed to generate report: {:?}", e);
				std::process::exit(1);
			}
			
			i += 1;
		}
	}
	
	eprintln!("  \x1b[32;1mGenerating\x1b[0m OpenMetrics report");
}

fn generate_clippy_to_code_quality(
	reader: impl io::BufRead,
	mut writer: impl io::Write
) {
	let mut issues = Vec::new();
	
	for line in reader.lines() {
		let msg = match line.and_then(|line| serde_json::from_str(&line)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)))
		{
			Ok(v) => v,
			Err(e) => {
				eprintln!("error: failed to parse message: {}", e);
				std::process::exit(1);
			}
		};
		
		let msg = match msg {
			RustMessage::CompilerMessage(v) if !v.message.spans.is_empty() => v,
			_ => continue
		};
		
		issues.push(CodeQualityReportIssue {
			r#type:             CODE_QUALITY_REPORT_TYPE,
			check_name:         msg.message.code.as_ref().unwrap().code.clone(),
			description:        msg.message.message.clone(),
			content:            Some(format!("```{}```", msg.message.rendered)),
			categories:         vec![CodeQualityReportIssueCategory::Style],
			location:           msg.message.spans[0].clone().into(),
			other_locations:    (msg.message.spans.len() > 1).then(|| msg.message.spans[1..]
				.iter()
				.map(|span| span.clone().into())
				.collect()),
			remediation_points: None,
			severity:           Some(match &*msg.message.level {
				"error"   => CodeQualityReportIssueSeverity::Major,
				"warning" => CodeQualityReportIssueSeverity::Minor,
				_         => CodeQualityReportIssueSeverity::Info
			}),
			fingerprint:        Some(format!("{:x}", xxhash_rust::xxh3::xxh3_128(msg.message.message.as_bytes())))
		});
	}
	
	eprintln!("  \x1b[32;1mGenerating\x1b[0m code quality report");
	
	if let Err(e) = serde_json::to_writer(&mut writer, &issues) {
		eprintln!("error: failed to generate report: {}", e);
		std::process::exit(1);
	}
}

fn generate_clippy_to_open_metrics(
	reader: impl io::BufRead,
	mut writer: impl io::Write
) {
	let mut metrics = HashMap::new();
	
	for line in reader.lines() {
		let msg = match line.and_then(|line| serde_json::from_str(&line)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)))
		{
			Ok(v) => v,
			Err(e) => {
				eprintln!("error: failed to parse message: {}", e);
				std::process::exit(1);
			}
		};
		
		let msg = match msg {
			RustMessage::CompilerMessage(v) if v.message.spans.is_empty() => v,
			_ => continue
		};
		
		*metrics.entry((msg.message.level, msg.message.code.unwrap().code)).or_insert(0) += 1;
	}
	
	eprintln!("  \x1b[32;1mGenerating\x1b[0m OpenMetrics report");
	
	for ((level, code), value) in metrics {
		if let Err(e) = writeln!(&mut writer, "{}{{code={}}}: {}", level, code, value) {
			eprintln!("error: failed to generate report: {}", e);
			std::process::exit(1);
		}
	}
}

fn generate_bench_to_open_metrics(
	reader: impl io::BufRead,
	mut writer: impl io::Write
) {
	for line in reader.lines() {
		let msg = match line.and_then(|line| serde_json::from_str(&line)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)))
		{
			Ok(v) => v,
			Err(e) => {
				eprintln!("error: failed to parse message: {}", e);
				std::process::exit(1);
			}
		};
		
		if let CargoMessage::Bench(v) = msg {
			if let Err(e) = writeln!(&mut writer, "{}: {}", v.name, v.median) {
				eprintln!("error: failed to generate report: {}", e);
				std::process::exit(1);
			}
		}
	}
	
	eprintln!("  \x1b[32;1mGenerating\x1b[0m OpenMetrics report");
}

fn generate_audit_to_sast(
	mut reader: impl io::BufRead,
	writer: impl io::Write
) {
	let mut report = SastReport {
		version: "2.0".to_string(),
		..Default::default()
	};
	let scanner = SastReportVulnerabilityScanner { id: "cargo_audit".to_string(), name: "Cargo Audit".to_string() };
	
	let mut buf = String::new();
	
	if let Err(e) = reader.read_to_string(&mut buf) {
		eprintln!("error: failed to parse report: {}", e);
		std::process::exit(1);
	}
	
	let mut buf = buf.as_str();
	
	while let Some((issue, rem)) = find_next_issue(buf) {
		buf = rem;
		
		let issue: CargoAuditIssue = match std::str::FromStr::from_str(issue) {
			Ok(v) => v,
			Err(()) => {
				eprintln!("error: failed to parse issue");
				std::process::exit(1);
			}
		};
		
		let __issue_url__ = issue.url;
		
		report.vulnerabilities.push(SastReportVulnerability {
			category:    "Dependency Scanning".to_string(),
			severity:    Some(match &issue.warning {
				Some(_) => SastReportVulnerabilitySeverity::Medium,
				None    => SastReportVulnerabilitySeverity::High
			}),
			name:        issue.warning,
			message:     issue.title,
			confidence:  Some(SastReportVulnerabilityConfidence::Confirmed),
			solution:    issue.solution,
			scanner:     scanner.clone(),
			identifiers: issue.id.map(move |id| SastReportVulnerabilityIdentifier {
				r#type: "RUSTSEC Advisory".to_string(),
				name:   id.clone(),
				value:  id,
				url:    __issue_url__
			}).into_iter().collect(),
			..Default::default()
		})
	}
	
	eprintln!("  \x1b[32;1mGenerating\x1b[0m SAST report");
	
	if let Err(e) = serde_json::to_writer(writer, &report) {
		eprintln!("error: failed to generate report: {}", e);
	}
}