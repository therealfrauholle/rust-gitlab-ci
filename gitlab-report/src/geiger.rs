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
pub struct Report {
	pub packages:                   Vec<Package>,
	//pub packages_without_metrics:   Vec<()>,
	//pub used_but_not_scanned_files: Vec<()>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Package {
	pub package:  PackagePackage,
	pub unsafety: PackageUnsafety
}

#[derive(Clone, Debug, Deserialize)]
pub struct PackagePackage {
	pub id:                 PackagePackageId,
	pub dependencies:       Vec<PackagePackage>,
	pub dev_dependencies:   Vec<PackagePackage>,
	pub build_dependencies: Vec<PackagePackage>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PackagePackageId {
	pub name: String,
	pub version: String,
	//source:
}

#[derive(Clone, Debug, Deserialize)]
pub struct PackageUnsafety {
	pub used:           PackageUnsafetyCounts,
	pub unused:         PackageUnsafetyCounts,
	pub forbids_unsafe: bool
}

#[derive(Clone, Debug, Deserialize)]
pub struct PackageUnsafetyCounts {
	pub functions:   PackageUnsafetyCount,
	pub exprs:       PackageUnsafetyCount,
	pub item_impls:  PackageUnsafetyCount,
	pub item_traits: PackageUnsafetyCount,
	pub methods:     PackageUnsafetyCount
}

#[derive(Clone, Debug, Deserialize)]
pub struct PackageUnsafetyCount {
	pub safe:    usize,
	pub unsafe_: usize
}