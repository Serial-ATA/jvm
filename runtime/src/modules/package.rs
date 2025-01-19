use super::entry::Module;
use crate::modules::ModuleLockGuard;

use std::cell::SyncUnsafeCell;
use std::collections::HashSet;
use std::fmt::Debug;

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

	// These fields are only ever mutated while the module lock is held
	qualified_exports: SyncUnsafeCell<HashSet<&'static Module>>,
	export_type: SyncUnsafeCell<PackageExportType>,
}

impl Debug for Package {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Package")
			.field("name", &self.name.as_str())
			.field("module", &self.module)
			.field("export_type", &self.export_type)
			.finish()
	}
}

impl Package {
	pub fn new(name: Symbol, module: &'static Module) -> Package {
		Self {
			name,
			module,
			qualified_exports: SyncUnsafeCell::new(HashSet::new()),
			export_type: SyncUnsafeCell::new(PackageExportType::None),
		}
	}

	pub fn name(&self) -> Symbol {
		self.name
	}

	pub fn module(&self) -> &'static Module {
		self.module
	}

	pub fn set_export_type(&self, _guard: &ModuleLockGuard, export_type: PackageExportType) {
		unsafe { *self.export_type.get() = export_type }
	}

	pub fn add_qualified_export(&self, _guard: &ModuleLockGuard, module: &'static Module) {
		unsafe { &mut *self.qualified_exports.get() }.insert(module);
	}
}

impl Package {
	// When we receive a list of packages in `defineModule0`, they will be in form "java.lang".
	// We need to convert them to an internal path form "java/lang".
	pub(crate) fn name_to_internal(name: String) -> String {
		name.replace('.', "/")
	}

	pub(crate) fn verify_name(name: &str) -> bool {
		if name.is_empty() {
			return false;
		}

		// TODO: Verify valid characters
		true
	}
}
