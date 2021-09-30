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

//! Code Climate Quality Report
//!
//! https://github.com/codeclimate/platform/blob/master/spec/analyzers/SPEC.md

use super::*;

pub const CODE_QUALITY_REPORT_TYPE: &str = "issue";

#[derive(Clone, Debug, Serialize)]
pub struct CodeQualityReport(Vec<CodeQualityReportIssue>);

#[derive(Clone, Debug, Serialize)]
pub struct CodeQualityReportIssue {
	pub r#type:             &'static str,
	pub check_name:         String,
	pub description:        String,
	pub content:            Option<String>,
	pub categories:         Vec<CodeQualityReportIssueCategory>,
	pub location:           CodeQualityReportLocation,
	pub other_locations:    Option<Vec<CodeQualityReportLocation>>,
	pub remediation_points: Option<usize>,
	pub severity:           Option<CodeQualityReportIssueSeverity>,
	pub fingerprint:        Option<String>
}

#[derive(Clone, Debug, Serialize)]
pub enum CodeQualityReportIssueCategory {
	#[serde(rename = "Bug Risk")]
	BugRisk,
	Clarity,
	Compatibility,
	Complexity,
	Duplication,
	Performance,
	Security,
	Style
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CodeQualityReportIssueSeverity {
	Info,
	Minor,
	Major,
	Critical,
	Blocker
}

#[derive(Clone, Debug, Serialize)]
pub struct CodeQualityReportLocation {
	pub path:      String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines:     Option<CodeQualityReportLines>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub positions: Option<CodeQualityReportPositions>
}

#[derive(Clone, Debug, Serialize)]
pub struct CodeQualityReportLines {
	pub begin: usize,
	pub end:   usize
}

#[derive(Clone, Debug, Serialize)]
pub struct CodeQualityReportPositions {
	pub begin: CodeQualityReportPosition,
	pub end:   CodeQualityReportPosition
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum CodeQualityReportPosition {
	Coordinates { line: usize, column: usize },
	Offset { offset: usize }
}