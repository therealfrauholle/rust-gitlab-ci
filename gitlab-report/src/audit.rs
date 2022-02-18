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

use {super::*, std::collections::HashMap};

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Report {
	pub database:        Database,
	pub lockfile:        Lockfile,
	pub settings:        Settings,
	pub vulnerabilities: SettingsVulnerabilities,
	pub warnings:        HashMap<String, Vec<Issue>>
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Database {
	pub advisory_count: usize,
	pub last_commit:    String,
	pub last_updated:   String
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Lockfile {
	pub dependency_count: usize
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Settings {
	pub target_arch:            Option<String>,
	pub target_os:              Option<String>,
	pub severity:               Option<String>,
	pub ignore:                 Vec<String>,
	pub informational_warnings: Vec<String>,
	pub package_scope:          Option<String>
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SettingsVulnerabilities {
	pub found: bool,
	pub count: usize,
	pub list:  Vec<Issue>
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Issue {
	pub kind:     Option<String>,
	pub advisory: Option<IssueAdvisory>,
	pub versions: Option<IssueVersions>,
	pub affected: Option<IssueAffected>,
	pub package:  IssuePackage
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct IssueAdvisory {
	pub id:            String,
	pub package:       String,
	pub title:         String,
	pub description:   String,
	pub date:          String,
	pub aliases:       Vec<String>,
	pub related:       Vec<String>,
	pub collection:    String,
	pub categories:    Vec<String>,
	pub keywords:      Vec<String>,
	pub cvss:          Option<String>,
	pub informational: Option<String>,
	pub url:           String,
	pub references:    Vec<String>,
	pub withdrawn:     Option<String>
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct IssueVersions {
	pub patched:    Vec<String>,
	pub unaffected: Vec<String>
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct IssueAffected {
	pub arch:      Vec<String>,
	pub os:        Vec<String>,
	pub functions: HashMap<String, Vec<String>>
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct IssuePackage {
	pub name:         String,
	pub version:      String,
	pub source:       String,
	pub checksum:     String,
	pub dependencies: Vec<IssuePackageDependency>,
	pub replace:      Option<String>
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct IssuePackageDependency {
	pub name:    String,
	pub version: String,
	pub source:  Option<String>
}