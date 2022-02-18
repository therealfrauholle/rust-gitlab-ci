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

use super::*;

pub fn test_to_junit(
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
			cargo::CargoMessage::Suite(cargo::CargoTestReportSuite::Started(v)) => {
				let now = std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos();
				suites.push(junit::Testsuite {
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
			cargo::CargoMessage::Suite(cargo::CargoTestReportSuite::Ok(v) | cargo::CargoTestReportSuite::Failed(v)) => {
				let suite = suites.last_mut().unwrap();
				suite.failures = v.failed;
				suite.errors   = 0;
				suite.skipped  = v.ignored + v.filtered_out;
				suite.time     = v.exec_time;
			}
			cargo::CargoMessage::Test(cargo::CargoTestReportTest { name, event }) => {
				let now = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs_f64();
				let suite = suites.last_mut().unwrap();
				let testcases = suite.testcases.as_mut().unwrap();
				let (module, name) = name.rsplit_once("::").unwrap_or(("", &name));
				
				match event {
					cargo::CargoTestReportTestEvent::Started => suite.testcases.as_mut().unwrap().push(junit::TestsuiteTestcase {
						status:    None,
						name:      name.to_string(),
						classname: module.to_string(),
						time:      now
					}),
					cargo::CargoTestReportTestEvent::Ignored => {
						let testcase = testcases.iter_mut()
							.find(|case| case.classname == module && case.name == name)
							.unwrap();
						
						testcase.time = now - testcase.time;
						testcase.status = Some(junit::TestsuiteTestcaseStatus::Skipped);
					},
					event @ cargo::CargoTestReportTestEvent::Ok(_) | event @ cargo::CargoTestReportTestEvent::Failed(_) => {
						let testcase = testcases.iter_mut()
							.find(|case| case.classname == module && case.name == name)
							.unwrap();
						
						testcase.time = now - testcase.time;
						testcase.status = match event {
							cargo::CargoTestReportTestEvent::Ok(_v) => None,
							cargo::CargoTestReportTestEvent::Failed(v) => Some(junit::TestsuiteTestcaseStatus::Failure {
								r#type:  "cargo test".to_string(),
								message: v.stdout.unwrap_or_else(String::new)
							}),
							_ => unreachable!()
						};
					}
				}
			}
			cargo::CargoMessage::Bench(_) => ()
		}
	}
	
	eprintln!("  \x1b[32;1mGenerating\x1b[0m JUnit report");
	
	if let Err(e) = writeln!(&mut writer, "<?xml version=\"1.0\" encoding=\"utf-8\"?>") {
		eprintln!("error: failed to generate report: {:?}", e);
		std::process::exit(1);
	} else if let Err(e) = quick_xml::se::to_writer(writer, &junit::Report(suites)) {
		eprintln!("error: failed to generate report: {:?}", e);
		std::process::exit(1);
	}
}

pub fn test_to_open_metrics(
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
		
		if let cargo::CargoMessage::Suite(cargo::CargoTestReportSuite::Ok(v) | cargo::CargoTestReportSuite::Failed(v)) = msg {
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

pub fn clippy_to_code_quality(
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
			clippy::Message::CompilerMessage(v) if !v.message.spans.is_empty() => v,
			_ => continue
		};
		
		issues.push(code_climate::CodeQualityReportIssue {
			r#type:             code_climate::CODE_QUALITY_REPORT_TYPE,
			check_name:         msg.message.code.as_ref().unwrap().code.clone(),
			description:        msg.message.message.clone(),
			content:            Some(format!("```{}```", msg.message.rendered)),
			categories:         vec![code_climate::CodeQualityReportIssueCategory::Style],
			location:           msg.message.spans[0].clone().into(),
			other_locations:    (msg.message.spans.len() > 1).then(|| msg.message.spans[1..]
				.iter()
				.map(|span| span.clone().into())
				.collect()),
			remediation_points: None,
			severity:           Some(match &*msg.message.level {
				"error"   => code_climate::CodeQualityReportIssueSeverity::Major,
				"warning" => code_climate::CodeQualityReportIssueSeverity::Minor,
				_         => code_climate::CodeQualityReportIssueSeverity::Info
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

pub fn clippy_to_open_metrics(
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
			clippy::Message::CompilerMessage(v) if v.message.spans.is_empty() => v,
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

pub fn bench_to_open_metrics(
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
		
		if let cargo::CargoMessage::Bench(v) = msg {
			if let Err(e) = writeln!(&mut writer, "{}: {}", v.name, v.median) {
				eprintln!("error: failed to generate report: {}", e);
				std::process::exit(1);
			}
		}
	}
	
	eprintln!("  \x1b[32;1mGenerating\x1b[0m OpenMetrics report");
}

pub fn audit_to_gitlab_security_report(
	format: gitlab_security_report::ScanType,
	reader: impl io::BufRead,
	writer: impl io::Write
) {
	let mut report = gitlab_security_report::Report { version: "2.0".to_string(), ..Default::default() };
	let scanner    = gitlab_security_report::VulnerabilityScanner { id: "cargo_audit".to_string(), name: "Cargo Audit".to_string() };
	let audit      = match serde_json::from_reader::<_, audit::Report>(reader) {
		Ok(v) => v,
		Err(e) => {
			eprintln!("error: failed to parse report: {}", e);
			std::process::exit(1);
		}
	};
	
	eprintln!("  \x1b[32;1mGenerating\x1b[0m SAST report");
	
	for vulnerability in audit.vulnerabilities.list {
		report.vulnerabilities.push(gitlab_security_report::Vulnerability {
			scanner: scanner.clone(),
			..audit_issue_to_gitlab_vuln(vulnerability, format)
		});
	}
	
	for (_, warnings) in audit.warnings {
		for warning in warnings {
			report.vulnerabilities.push(gitlab_security_report::Vulnerability {
				scanner: scanner.clone(),
				..audit_issue_to_gitlab_vuln(warning, format)
			});
		}
	}
	
	if let Err(e) = serde_json::to_writer(writer, &report) {
		eprintln!("error: failed to generate report: {}", e);
	}
}

pub fn geiger_to_gitlab_security_report(
	format: gitlab_security_report::ScanType,
	reader: impl io::BufRead,
	writer: impl io::Write
) {
	let mut report = gitlab_security_report::Report { version: "2.0".to_string(), ..Default::default() };
	let scanner    = gitlab_security_report::VulnerabilityScanner { id: "cargo_geiger".to_string(), name: "Cargo Geiger".to_string() };
	let geiger     = match serde_json::from_reader::<_, geiger::Report>(reader) {
		Ok(v) => v,
		Err(e) => {
			eprintln!("error: failed to parse report: {}", e);
			std::process::exit(1);
		}
	};
	
	for package in geiger.packages {
		let unsafe_ = package.unsafety.used.functions.unsafe_ + package.unsafety.unused.functions.unsafe_
			+ package.unsafety.used.exprs.unsafe_ + package.unsafety.unused.exprs.unsafe_
			+ package.unsafety.used.item_impls.unsafe_ + package.unsafety.unused.item_impls.unsafe_
			+ package.unsafety.used.item_traits.unsafe_ + package.unsafety.unused.item_traits.unsafe_
			+ package.unsafety.used.methods.unsafe_ + package.unsafety.unused.methods.unsafe_;
		
		if package.unsafety.forbids_unsafe && unsafe_ == 0 {
			continue;
		}
		
		report.vulnerabilities.push(gitlab_security_report::Vulnerability {
			scanner: scanner.clone(),
			..geiger_package_to_gitlab_vuln(package, format)
		});
	}
	
	if let Err(e) = serde_json::to_writer(writer, &report) {
		eprintln!("error: failed to generate report: {}", e);
	}
}

fn audit_issue_to_gitlab_vuln(issue: audit::Issue, ty: gitlab_security_report::ScanType) -> gitlab_security_report::Vulnerability {
	gitlab_security_report::Vulnerability {
		category:    "Dependency Scanning".to_string(),
		severity:    Some(match &issue.kind {
			None    => gitlab_security_report::VulnerabilitySeverity::High,
			Some(_) => gitlab_security_report::VulnerabilitySeverity::Medium
		}),
		name:        Some(match &issue.advisory {
			None => format!("{}@{}", issue.package.name.clone(), issue.package.version.clone()),
			Some(advisory) => advisory.id.clone()
		}),
		message:     
		Some(match &issue.advisory {
			None => format!("{}: {}", issue.kind.unwrap_or("unknown".to_string()), issue.package.name.clone()),
			Some(advisory) => advisory.title.clone()
		}),
		description: Some(match &issue.advisory {
			None => "".to_string(),
			Some(advisory) => advisory.description.clone()
		}),
		confidence:  Some(gitlab_security_report::VulnerabilityConfidence::Confirmed),
		identifiers: match &issue.advisory {
    		Some(advisory) => {
    		    vec![gitlab_security_report::VulnerabilityIdentifier {
				    r#type: "RUSTSEC Advisory".to_string(),
				    name:   advisory.id.clone(),
				    value:  advisory.id.clone(),
				    url:    Some(advisory.url.clone())
			    }]
		    }
    		_ => Vec::new(),
		},
		location:   match ty {
			gitlab_security_report::ScanType::DependencyScanning => gitlab_security_report::VulnerabilityLocation::DependencyScanning {
				file:       None,
				dependency: gitlab_security_report::VulnerabilityLocationDependency {
					package:         Some(gitlab_security_report::VulnerabilityLocationDependencyPackage { name: issue.package.name.clone() }),
					version:         Some(issue.package.version.clone()),
					iid:             None,
					direct:          None,
					dependency_path: Vec::new()
				}
			},
			gitlab_security_report::ScanType::Sast => gitlab_security_report::VulnerabilityLocation::Sast {
				file:       None,
				start_line: None,
				end_line:   None,
				module:     None,
				item:       None
			},
			_ => unreachable!()
		},
		..Default::default()
	}
}

fn geiger_package_to_gitlab_vuln(package: geiger::Package, ty: gitlab_security_report::ScanType) -> gitlab_security_report::Vulnerability {
	let unsafe_used = package.unsafety.used.functions.unsafe_
		+ package.unsafety.used.exprs.unsafe_
		+ package.unsafety.used.item_impls.unsafe_
		+ package.unsafety.used.item_traits.unsafe_
		+ package.unsafety.used.methods.unsafe_;
	let unsafe_unused = package.unsafety.unused.functions.unsafe_
		+ package.unsafety.unused.exprs.unsafe_
		+ package.unsafety.unused.item_impls.unsafe_
		+ package.unsafety.unused.item_traits.unsafe_
		+ package.unsafety.unused.methods.unsafe_;
	
	gitlab_security_report::Vulnerability {
		category:    "Dependency Scanning".to_string(),
		severity:    Some(gitlab_security_report::VulnerabilitySeverity::Info),
		name:        Some(format!("Unsafe usage in package `{}`", package.package.id.name)),
		message:     Some(format!("Found {} `unsafe` usages in package `{}` ({} used by the build)", unsafe_used + unsafe_unused, package.package.id.name, unsafe_used)),
		description: Some(format!(r#"Cargo Geiger Report for package `{}`:
Functions: {}/{}
Expressions: {}/{}
Impls: {}/{}
Traits: {}/{}
Methods:  {}/{}
"#, package.package.id.name,
		package.unsafety.used.functions.unsafe_, package.unsafety.unused.functions.unsafe_,
		package.unsafety.used.exprs.unsafe_, package.unsafety.unused.exprs.unsafe_,
		package.unsafety.used.item_impls.unsafe_, package.unsafety.unused.item_impls.unsafe_,
		package.unsafety.used.item_traits.unsafe_, package.unsafety.unused.item_traits.unsafe_,
		package.unsafety.used.methods.unsafe_, package.unsafety.unused.methods.unsafe_)),
		confidence:  Some(gitlab_security_report::VulnerabilityConfidence::Ignore),
		identifiers: Vec::new(),
		location:   match ty {
			gitlab_security_report::ScanType::DependencyScanning => gitlab_security_report::VulnerabilityLocation::DependencyScanning {
				file:       None,
				dependency: gitlab_security_report::VulnerabilityLocationDependency {
					package:         Some(gitlab_security_report::VulnerabilityLocationDependencyPackage { name: package.package.id.name.clone() }),
					version:         Some(package.package.id.version.clone()),
					iid:             None,
					direct:          None,
					dependency_path: Vec::new()
				}
			},
			gitlab_security_report::ScanType::Sast => gitlab_security_report::VulnerabilityLocation::Sast {
				file:       None,
				start_line: None,
				end_line:   None,
				module:     None,
				item:       None
			},
			_ => unreachable!()
		},
		..Default::default()
	}
}