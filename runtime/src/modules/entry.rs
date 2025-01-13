use super::package::Package;
use crate::classpath::classloader::ClassLoader;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;
use crate::string_interner::StringInterner;
use crate::thread::exceptions::{throw, Throws};

use std::cell::SyncUnsafeCell;
use std::fmt::{Debug, Formatter};

use symbols::{sym, Symbol};

// NOTE: The fields are `UnsafeCell`s due to the bootstrapping process.
//       Mutation does not occur outside of `java.base`.

/// Representation of a `java.lang.Module` object
pub struct Module {
	pub(super) obj: SyncUnsafeCell<Reference>,
	pub(super) open: bool,
	pub(super) name: Option<Symbol>,
	pub(super) version: SyncUnsafeCell<Option<Symbol>>,
	pub(super) location: SyncUnsafeCell<Option<Symbol>>,
}

impl Debug for Module {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut s = f.debug_struct("Module");

		s.field("open", &self.open)
			.field("name", &self.name.map_or("unnamed", |s| s.as_str()));

		if let Some(version) = self.version() {
			s.field("version", &version.as_str());
		}

		if let Some(location) = self.location() {
			s.field("location", &location.as_str());
		}

		s.finish()
	}
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
	pub fn unnamed(obj: Reference) -> Throws<Self> {
		verify_obj(obj.clone())?;

		let name = obj
			.get_field_value0(crate::globals::fields::java_lang_Module::name_field_offset())
			.expect_reference();
		if !name.is_null() {
			throw!(@DEFER IllegalArgumentException);
		}

		Throws::Ok(Self {
			obj: SyncUnsafeCell::new(obj),
			open: true,
			name: None,
			version: SyncUnsafeCell::new(None),
			location: SyncUnsafeCell::new(None),
		})
	}

	pub fn named(
		obj: Reference,
		is_open: bool,
		version: Option<Symbol>,
		location: Option<Symbol>,
		package_names: Vec<String>,
	) -> Throws<Option<Self>> {
		verify_obj(obj.clone())?;

		let name_obj = obj
			.get_field_value0(crate::globals::fields::java_lang_Module::name_field_offset())
			.expect_reference();
		if name_obj.is_null() {
			throw!(@DEFER IllegalArgumentException, "Module name cannot be null");
		}

		let name = StringInterner::symbol_from_java_string(name_obj.extract_class());
		let loader = obj
			.get_field_value0(crate::globals::fields::java_lang_Module::loader_field_offset())
			.expect_reference();

		if name == sym!(java_base) {
			init_java_base(obj, is_open, version, location, package_names, loader);
			return Throws::Ok(None);
		}

		todo!("Named modules other than java.base")
	}
}

fn verify_obj(obj: Reference) -> Throws<()> {
	if obj.is_null() {
		throw!(@DEFER NullPointerException, "Null module object");
	}

	if !obj.is_instance_of(crate::globals::classes::java_lang_Module()) {
		throw!(
			@DEFER
			IllegalArgumentException,
			"module is not an instance of type java.lang.Module"
		);
	}

	Throws::Ok(())
}

fn init_java_base(
	obj: Reference,
	is_open: bool,
	version: Option<Symbol>,
	location: Option<Symbol>,
	package_names: Vec<String>,
	loader: Reference,
) -> Throws<()> {
	assert!(!is_open, "java.base module cannot be open");
	if !loader.is_null() {
		throw!(
			@DEFER
			IllegalArgumentException,
			"Class loader must be the boot class loader"
		);
	}

	let java_base = ClassLoader::bootstrap().java_base();
	if java_base.has_obj() {
		throw!(@DEFER InternalError, "Module java.base is already defined");
	}

	let mut bad_package_name = None;
	super::with_module_lock(|guard| {
		for package in package_names {
			if !Package::verify_name(&package) {
				bad_package_name = Some(package);
				return;
			}

			let package = Package::new(Symbol::intern_owned(package), java_base);
			ClassLoader::bootstrap().insert_package_if_absent(&guard, package);
		}

		guard.fixup_java_base(java_base, obj, version, location)
	});

	if let Some(bad_package_name) = bad_package_name {
		throw!(
			@DEFER
			IllegalArgumentException,
			"Invalid package name: {} for module: java.base",
			bad_package_name,
		);
	}

	Throws::Ok(())
}

impl Module {
	/// Get the name of this module
	///
	/// This will only return `None` for modules created with [`Module::unnamed()`]
	pub fn name(&self) -> Option<Symbol> {
		self.name
	}

	pub fn version(&self) -> Option<Symbol> {
		unsafe { *self.version.get() }
	}

	pub fn location(&self) -> Option<Symbol> {
		unsafe { *self.location.get() }
	}

	/// Get the associated `java.lang.Module` instance
	pub fn obj(&self) -> Reference {
		let obj_ptr = self.obj.get();
		unsafe { &*obj_ptr }.clone()
	}

	/// Check whether this entry has an associated `java.lang.Module` object
	///
	/// This is only needed for `java.base` early in VM initialization. It is always `true` for other
	/// entries.
	pub fn has_obj(&self) -> bool {
		let obj_ptr = self.obj.get();
		let obj_ref = unsafe { &*obj_ptr };
		!obj_ref.is_null()
	}
}
