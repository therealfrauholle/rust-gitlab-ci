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

//! GitLab Report
//!
//! https://gitlab.com/gitlab-org/security-products/security-report-schemas/-/tree/master/src

use super::*;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Report {
	pub version:          String,
	pub scan:             Option<Scan>,
	pub vulnerabilities:  Vec<Vulnerability>,
	#[serde(default)]
	pub remediations:     Vec<Remediation>,
	#[serde(default)]
	pub dependency_files: Vec<DependencyFile>
}

#[derive(Clone, Debug, Serialize)]
pub struct Scan {
	pub start_time: String, // ISO8601
	pub end_time:   String, // ISO8601
	pub r#type:     ScanType,
	pub status:     ScanStatus,
	#[serde(default)]
	pub messages:   Vec<ScanMessage>,
	pub analyzer:   Option<ScanAnalyzer>,
	pub scanner:    ScanScanner
}

#[derive(Copy, Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScanType {
	ClusterImageScanning,
	ContainerScanning,
	CoverageFuzzing,
	Dast,
	ApiFuzzing,
	DependencyScanning,
	Sast,
	SecretDetection
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ScanStatus {
	Success,
	Failure
}

#[derive(Clone, Debug, Serialize)]
pub struct ScanMessage {
	pub level: ScanMessageLevel,
	pub value: String
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ScanMessageLevel {
	Info,
	Warn,
	Fatal
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ScanAnalyzer {
	pub id:      String,
	pub name:    String,
	pub url:     Option<String>,
	pub vendor:  String,
	pub version: String
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ScanScanner {
	pub id:      String,
	pub name:    String,
	pub url:     Option<String>,
	pub vendor:  String,
	pub version: String
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Vulnerability {
	pub id:                      Option<String>,
	pub category:                String,
	pub name:                    Option<String>,
	pub message:                 Option<String>,
	pub description:             Option<String>,
	pub severity:                Option<VulnerabilitySeverity>,
	pub confidence:              Option<VulnerabilityConfidence>,
	pub solution:                Option<String>,
	pub scanner:                 VulnerabilityScanner,
	pub identifiers:             Vec<VulnerabilityIdentifier>,
	pub links:                   Vec<String>,
	pub location:                VulnerabilityLocation,
	pub raw_source_code_extract: Option<String>
}

#[derive(Clone, Debug, Serialize)]
pub enum VulnerabilitySeverity {
	Info,
	Unknown,
	Low,
	Medium,
	High,
	Critical
}

#[derive(Clone, Debug, Serialize)]
pub enum VulnerabilityConfidence {
	Ignore,
	Unknown,
	Experimental,
	Low,
	Medium,
	High,
	Confirmed
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct VulnerabilityScanner {
	pub id:   String,
	pub name: String
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum VulnerabilityLocation {
	ClusterImageScanning {
		dependency:       VulnerabilityLocationDependency,
		operating_system: String,
		image:            String,
		kubernetes_resource: VulnerabilityLocationClusterImageScanningKubernetesResource
	},
	ContainerScanning {
		dependency:       VulnerabilityLocationDependency,
		operating_system: String,
		image:            String
	},
	CoverageFuzzing {
		crash_address:      Option<String>,
		stacktrace_snippet: Option<String>,
		crash_state:        Option<String>,
		crash_type:         Option<String>
	},
	DependencyScanning {
		file:       Option<String>,
		dependency: VulnerabilityLocationDependency
	},
	Sast {
		file:       Option<String>,
		start_line: Option<usize>,
		end_line:   Option<usize>,
		#[serde(rename = "class")]
		module:     Option<String>,
		#[serde(rename = "method")]
		item:       Option<String>
	},
	SecretDetection {
		file:       Option<String>,
		commit:     VulnerabilityLocationSecretDetectionCommit,
		start_line: Option<usize>,
		end_line:   Option<usize>,
		#[serde(rename = "class")]
		module:     Option<String>,
		#[serde(rename = "method")]
		item:       Option<String>
	}
}

impl Default for VulnerabilityLocation {
	fn default() -> Self {
		Self::Sast {
			file:       None,
			start_line: None,
			end_line:   None,
			module:     None,
			item:       None
		}
	}
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct VulnerabilityLocationDependency {
	pub package:         Option<VulnerabilityLocationDependencyPackage>,
	pub version:         Option<String>,
	pub iid:             Option<usize>,
	pub direct:          Option<bool>,
	#[serde(default)]
	pub dependency_path: Vec<VulnerabilityLocationDependencyPath>
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct VulnerabilityLocationDependencyPackage {
	pub name: String
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct VulnerabilityLocationDependencyPath {
	pub iid: usize
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct VulnerabilityLocationClusterImageScanningKubernetesResource {
	pub namespace:      String,
	pub kind:           String,
	pub name:           String,
	pub container_name: String,
	pub agent_id:       Option<String>,
	pub cluster_id:     Option<String>
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct VulnerabilityLocationSecretDetectionCommit {
	pub author:  Option<String>,
	pub date:    Option<String>,
	pub message: Option<String>,
	pub sha:     Option<String>
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct VulnerabilityIdentifier {
	pub r#type: String,
	pub name:   String,
	pub value:  String,
	pub url:    Option<String>
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Remediation {
	pub fixes:   Vec<RemediationFix>,
	pub summary: String,
	pub diff:    String
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RemediationFix {
	pub cve: String
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct DependencyFile {
	pub path:            String,
	pub package_manager: String,
	#[serde(default)]
	pub dependencies:    Vec<VulnerabilityLocationDependency>
}