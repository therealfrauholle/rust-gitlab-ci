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

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "reason", rename_all = "kebab-case")]
pub enum Message {
	CompilerMessage(CompilerMessage),
	CompilerArtifact,
	BuildFinished,
	BuildScriptExecuted,
	#[serde(other)]
	Other
}

#[derive(Clone, Debug, Deserialize)]
pub struct CompilerMessage {
	pub message: CompilerMessageMessage
}

#[derive(Clone, Debug, Deserialize)]
pub struct CompilerMessageMessage {
	pub rendered: String,
	pub code:     Option<CompilerMessageMessageCode>,
	pub level:    String,
	pub message:  String,
	pub spans:    Vec<CompilerMessageMessageSpan>
}

#[derive(Clone, Debug, Deserialize)]
pub struct CompilerMessageMessageCode {
	pub code:        String,
	pub explanation: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
pub struct CompilerMessageMessageSpan {
	pub file_name:    String,
	pub line_start:   usize,
	pub line_end:     usize,
	pub column_start: usize,
	pub column_end:   usize,
}

impl Into<code_climate::CodeQualityReportLocation> for CompilerMessageMessageSpan {
	fn into(self) -> code_climate::CodeQualityReportLocation {
		code_climate::CodeQualityReportLocation {
			path:      self.file_name,
			lines:     None,
			positions: Some(code_climate::CodeQualityReportPositions {
				begin: code_climate::CodeQualityReportPosition::Coordinates {
					line:   self.line_start,
					column: self.column_start
				},
				end:   code_climate::CodeQualityReportPosition::Coordinates {
					line:   self.line_end,
					column: self.column_end
				}
			})
		}
	}
}