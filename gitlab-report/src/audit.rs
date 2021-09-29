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

#[derive(Clone, Debug, Default)]
pub struct CargoAuditIssue {
	pub __crate:         String,
	pub version:         String,
	pub warning:         Option<String>,
	pub title:           Option<String>,
	pub date:            Option<String>,
	pub id:              Option<String>,
	pub url:             Option<String>,
	pub solution:        Option<String>,
	pub dependency_tree: String
}

impl std::str::FromStr for CargoAuditIssue {
	type Err = ();
	
	#[allow(clippy::while_let_on_iterator)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut lines = s.lines();
		let mut self_ = Self::default();
		
		while let Some(line) = lines.next() {
			if line.trim().is_empty() {
				break;
			}
			
			let (key, val) = match line.split_once(':') {
				Some(v) => v,
				None => continue
			};
			
			match key {
				"Crate"           => self_.__crate  = val.trim().to_string(),
				"Version"         => self_.version  = val.trim().to_string(),
				"Warning"         => self_.warning  = Some(val.trim().to_string()),
				"Title"           => self_.title    = Some(val.trim().to_string()),
				"Date"            => self_.date     = Some(val.trim().to_string()),
				"ID"              => self_.id       = Some(val.trim().to_string()),
				"URL"             => self_.url      = Some(val.trim().to_string()),
				"Solution"        => self_.solution = Some(val.trim().to_string()),
				"Dependency tree" => {
					let mut buf = String::new();
					
					while let Some(line) = lines.next() {
						if line.trim().is_empty() {
							break;
						}
						
						buf.push_str(line);
					}
					
					self_.dependency_tree = buf;
				},
				_                  => continue
			}
		}
		
		if self_.__crate.is_empty() || self_.version.is_empty() || self_.dependency_tree.is_empty() {
			Err(())
		} else {
			Ok(self_)
		}
	}
}

pub fn find_next_issue(mut buf: &str) -> Option<(&str, &str)> {
	loop {
		if buf.is_empty() {
			return None;
		} else if buf.starts_with("Crate:") {
			let end = buf.find("\n\n").unwrap_or(buf.len());
			return Some((&buf[..end], &buf[end..]))
		}
		
		buf = buf.split_once('\n').unwrap_or(("", "")).1;
	}
}