use crate::objects::reference::Reference;

use symbols::Symbol;

pub struct Module {
	pub(super) obj: Reference,
	pub(super) open: bool,
	pub(super) name: Option<Symbol>,
	pub(super) version: Option<Symbol>,
	pub(super) location: Option<Symbol>,
}

impl Module {
	/// Create an unnamed `Module`
	///
	/// Every `java.lang.ClassLoader` has an `UNNAMED_MODULE` field, which holds a `java.lang.Module`
	/// with no name. This module contains any types loaded by the `ClassLoader` that do not belong
	/// to any module.
	///
	/// There are special rules for unnamed modules, designed to maximize their interoperation with
	/// other run-time modules, as follows:
	///
	/// * A class loader's unnamed module is distinct from all other run-time modules bound to the same class loader.
	/// * A class loader's unnamed module is distinct from all run-time modules (including unnamed modules) bound to other class loaders.
	/// * Every unnamed module reads every run-time module.
	/// * Every unnamed module exports, to every run-time module, every run-time package associated with itself.
	pub fn unnamed(obj: Reference) -> Self {
		assert!(!obj.is_null());

		Self {
			obj,
			open: true,
			name: None,
			version: None,
			location: None,
		}
	}

	pub fn named(
		name: Symbol,
		obj: Reference,
		version: Option<Symbol>,
		location: Option<Symbol>,
	) -> Self {
		// TODO: Assert is instance of java.lang.Module?
		assert!(!obj.is_null());

		Self {
			obj,
			open: false,
			name: Some(name),
			version,
			location,
		}
	}
}

impl Module {
	/// Get the name of this module
	///
	/// This will only return `None` for modules created with [`Module::unnamed()`]
	pub fn name(&self) -> Option<Symbol> {
		self.name
	}

	/// Get the associated `java.lang.Module` instance
	pub fn obj(&self) -> Reference {
		self.obj.clone()
	}

	/// Check whether this entry has an associated `java.lang.Module` object
	///
	/// This is only needed for `java.base` early in VM initialization. It is always `true` for other
	/// entries.
	pub fn has_obj(&self) -> bool {
		!self.obj.is_null()
	}
}
