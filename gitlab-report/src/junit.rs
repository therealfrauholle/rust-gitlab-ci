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

//! JUnit Reports
//!
//! https://github.com/windyroad/JUnit-Schema/blob/master/JUnit.xsd

use super::*;

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename = "testsuites")]
pub struct Report(pub Vec<Testsuite>);

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename = "testsuite")]
pub struct Testsuite {
	pub id:         usize,
	pub name:       String,
	pub timestamp:  String,
	pub hostname:   String,
	pub tests:      usize,
	pub failures:   usize,
	pub errors:     usize,
	pub skipped:    usize,
	pub time:       f64,
	#[serde(rename = "property")]
	pub properties: Option<Vec<TestsuiteProperty>>,
	#[serde(rename = "testcase")]
	pub testcases:  Option<Vec<TestsuiteTestcase>>
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TestsuiteProperty {
	pub name:  String,
	pub value: String
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename = "testcase")]
pub struct TestsuiteTestcase {
	pub status:    Option<TestsuiteTestcaseStatus>,
	pub name:      String,
	pub classname: String,
	pub time:      f64
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TestsuiteTestcaseStatus {
	Skipped,
	//Error(#[serde(flatten)] TestsuiteTestcaseErrorOrFailure),
	//Failure(#[serde(flatten)] TestsuiteTestcaseErrorOrFailure)
	Error { r#type:  String, message: String },
	Failure { 
        r#type: String,
        #[serde(rename = "$value")]
        system_output: String
    }
}

//#[derive(Clone, Debug, Serialize)]
//pub struct TestsuiteTestcaseErrorOrFailure {
//	pub r#type:  String,
//	pub message: String
//}