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

//! GitLab SAST Report
//!
//! https://gitlab.com/gitlab-org/security-products/security-report-schemas/-/blob/master/dist/sast-report-format.json

use super::*;

#[derive(Clone, Debug, Default, Serialize)]
pub struct SastReport {
	pub version:         String,
	pub scan:            Option<SastReportScan>,
	pub vulnerabilities: Vec<SastReportVulnerability>,
	#[serde(default)]
	pub remediations:    Vec<SastReportRemediations>
}

#[derive(Clone, Debug, Serialize)]
pub struct SastReportScan {
	pub start_time: String, // ISO8601
	pub end_time:   String, // ISO8601
	pub r#type:     SastReportScanType,
	pub status:     SastReportScanStatus,
	#[serde(default)]
	pub messages:   Vec<SastReportScanMessage>,
	pub analyzer:   Option<SastReportScanAnalyzer>,
	pub scanner:    SastReportScanScanner
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SastReportScanType {
	Sast
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SastReportScanStatus {
	Success,
	Failure
}

#[derive(Clone, Debug, Serialize)]
pub struct SastReportScanMessage {
	pub level: SastReportScanMessageLevel,
	pub value: String
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SastReportScanMessageLevel {
	Info,
	Warn,
	Fatal
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SastReportScanAnalyzer {
	pub id:      String,
	pub name:    String,
	pub url:     Option<String>,
	pub vendor:  String,
	pub version: String
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SastReportScanScanner {
	pub id:      String,
	pub name:    String,
	pub url:     Option<String>,
	pub vendor:  String,
	pub version: String
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SastReportVulnerability {
	pub id:          Option<String>,
	pub category:    String,
	pub name:        Option<String>,
	pub message:     Option<String>,
	pub description: Option<String>,
	pub severity:    Option<SastReportVulnerabilitySeverity>,
	pub confidence:  Option<SastReportVulnerabilityConfidence>,
	pub solution:    Option<String>,
	pub scanner:     SastReportVulnerabilityScanner,
	pub identifiers: Vec<SastReportVulnerabilityIdentifier>,
	pub location:    SastReportVulnerabilityLocation,
	pub raw_source_code_extract: Option<String>
}

#[derive(Clone, Debug, Serialize)]
pub enum SastReportVulnerabilitySeverity {
	Info,
	Unknown,
	Low,
	Medium,
	High,
	Critical
}

#[derive(Clone, Debug, Serialize)]
pub enum SastReportVulnerabilityConfidence {
	Ignore,
	Unknown,
	Experimental,
	Low,
	Medium,
	High,
	Confirmed
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SastReportVulnerabilityScanner {
	pub id:   String,
	pub name: String
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SastReportVulnerabilityLocation {
	pub file:       Option<String>,
	pub start_line: Option<usize>,
	pub end_line:   Option<usize>,
	#[serde(rename = "class")]
	pub module:     Option<String>,
	#[serde(rename = "method")]
	pub item:       Option<String>,
	pub dependency: Option<()>
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SastReportVulnerabilityIdentifier {
	pub r#type: String,
	pub name:   String,
	pub value:  String,
	pub url:    Option<String>
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SastReportRemediations {

}