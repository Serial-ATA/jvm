use std::sync::Arc;

use common::int_types::u1;

pub type ModuleRef = Arc<Module>;
pub type PackageRef = Arc<Package>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Module {
	name: Box<[u1]>,

	readable: Vec<ModuleRef>,
	version: Box<[u1]>,
	location: Box<[u1]>,

	open: bool,
}

impl Module {
	/// Whether the provided module is readable by this module
	pub fn can_read(&self, other: ModuleRef) -> bool {
		self.readable.contains(&other)
	}

	/// Whether all packages in the module are unqualifiedly exported
	///
	/// See [PackageExportType]
	pub fn is_open(&self) -> bool {
		self.open
	}
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum PackageExportType {
	/// Package is not exported
	None,
	/// Package is unqualifiedly exported
	Unqualified,
	/// Package is qualifiedly exported
	AllUnnamed,
	/// Package is exported to all unnamed modules
	UnqualifiedOrAllUnnamed,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Package {
	name: Box<[u1]>,
	module: ModuleRef,

	export_type: PackageExportType,
}
