use super::entry::Module;

use symbols::Symbol;

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

/// A representation of a package in Java
pub struct Package {
	name: Symbol,
	module: &'static Module,

	export_type: PackageExportType,
}

impl Package {
	pub fn name(&self) -> Symbol {
		self.name
	}

	pub fn module(&self) -> &'static Module {
		self.module
	}
}
