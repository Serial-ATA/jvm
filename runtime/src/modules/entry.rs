use super::package::{Package, PackageExportType};
use crate::classes;
use crate::classpath::jimage;
use crate::classpath::loader::{ClassLoader, ClassLoaderSet};
use crate::objects::instance::object::Object;
use crate::objects::reference::Reference;
use crate::symbols::{Symbol, sym};
use crate::thread::exceptions::{Throws, throw};

use std::cell::SyncUnsafeCell;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

use jni::sys::jlong;

struct ModuleFlags {
	can_read_all_unnamed: bool,
}

/// Representation of a `java.lang.Module` object
pub struct Module {
	name: Option<Symbol>,

	// These fields are only ever mutated while the module lock is held
	flags: SyncUnsafeCell<ModuleFlags>,
	reads: SyncUnsafeCell<HashSet<&'static Self>>,
	loader: SyncUnsafeCell<Option<&'static ClassLoader>>, // Set in `ModuleSet::add()`

	open: bool,

	// NOTE: These fields are `UnsafeCell`s due to the bootstrapping process.
	//       Mutation does not occur outside of `java.base`.
	obj: SyncUnsafeCell<Reference>,
	version: SyncUnsafeCell<Option<Symbol>>,
	location: SyncUnsafeCell<Option<Symbol>>,
}

impl PartialEq for Module {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Eq for Module {}

impl Hash for Module {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
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

		s.finish_non_exhaustive()
	}
}

// Bootstrapping methods
impl Module {
	/// Create the partial `java.base` entry for bootstrapping purposes
	///
	/// The [`Module`] produced is **not valid**.
	pub(crate) fn create_java_base(_guard: &super::ModuleLockGuard) {
		// This doesn't use the constructors, since they do validation.
		let java_base = Module {
			flags: SyncUnsafeCell::new(ModuleFlags {
				can_read_all_unnamed: false,
			}),
			reads: SyncUnsafeCell::new(HashSet::new()),
			loader: SyncUnsafeCell::new(None),

			name: Some(sym!(java_base)),
			open: false,
			obj: SyncUnsafeCell::new(Reference::null()),
			version: SyncUnsafeCell::new(None),
			location: SyncUnsafeCell::new(None),
		};

		ClassLoader::bootstrap().set_java_base(java_base);
	}

	/// Make `java.base` into a real module
	///
	/// # Safety
	///
	/// This must be called on the `java.base` [`Module`]
	pub(super) unsafe fn fixup_java_base(
		&'static self,
		_guard: &super::ModuleLockGuard,
		obj: Reference,
		version: Option<Symbol>,
		location: Option<Symbol>,
	) {
		assert!(
			!self.has_obj(),
			"java.base module can only be initialized once"
		);

		assert!(obj.is_class(), "invalid associated object for module");

		unsafe {
			*self.loader.get() = Some(ClassLoader::bootstrap());

			*self.obj.get() = obj;
			*self.version.get() = version;
			*self.location.get() = location;
		}

		// Store the pointer in the module, to make future lookups cheaper
		classes::java::lang::Module::set_injected_module_ptr_for(
			obj,
			std::ptr::from_ref(self) as jlong,
		);

		// All classes we've loaded up to this point need to be added to `java.base`
		ClassLoader::fixup_modules(obj);
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
		verify_obj(obj)?;

		let name = classes::java::lang::Module::name(obj);
		if !name.is_null() {
			throw!(@DEFER IllegalArgumentException);
		}

		Throws::Ok(Self {
			flags: SyncUnsafeCell::new(ModuleFlags {
				can_read_all_unnamed: true,
			}),
			reads: SyncUnsafeCell::new(HashSet::new()),
			loader: SyncUnsafeCell::new(None),

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
	) -> Throws<()> {
		verify_obj(obj)?;

		let name_obj = classes::java::lang::Module::name(obj);
		if name_obj.is_null() {
			throw!(@DEFER IllegalArgumentException, "Module name cannot be null");
		}

		let module_name = classes::java::lang::String::extract(name_obj.extract_class());
		let loader = classes::java::lang::Module::loader(obj);

		if &module_name == "java.base" {
			return init_java_base(obj, is_open, version, location, package_names, loader);
		}

		// Only the bootstrap loader and PlatformClassLoader can load `java/` packages
		let can_load_java_packages = loader.is_null()
			|| loader.extract_target_class().name()
				== sym!(jdk_internal_loader_ClassLoaders_PlatformClassLoader);

		let mut disallowed_package = None;
		let mut package_symbols = Vec::with_capacity(package_names.len());
		for package in package_names {
			if !Package::verify_name(&package) {
				throw!(@DEFER IllegalArgumentException, "Invalid package name: {} for module: {}", package, module_name.as_str());
			}

			if (package.starts_with("java/") || package.starts_with("java\0"))
				&& !can_load_java_packages
			{
				disallowed_package = Some(package);
				break;
			}

			package_symbols.push(Symbol::intern(package));
		}

		let loader = ClassLoaderSet::find_or_add(loader, false);

		if let Some(disallowed_package) = disallowed_package {
			throw!(@DEFER IllegalArgumentException,
				"Class loader (instance of): {} tried to define prohibited package name: {}",
				loader.name_and_id().as_str(),
				disallowed_package.replace('/', ".")
			);
		}

		let module_name_sym = Symbol::intern(module_name);

		let mut module_already_defined = false;
		let mut duplicate_package: Option<&Package> = None;
		super::with_module_lock(|guard| {
			// Check for any duplicates
			for package in package_symbols.iter().copied() {
				if let Some(duplicate) = loader.lookup_package(guard, package) {
					duplicate_package = Some(duplicate);

					// Also check for a duplicate module. That error takes precedence over the duplicate package.
					if loader.lookup_module(guard, module_name_sym).is_some() {
						module_already_defined = true;
					}

					return;
				}
			}

			let module_entry = Self {
				flags: SyncUnsafeCell::new(ModuleFlags {
					can_read_all_unnamed: false,
				}),
				reads: SyncUnsafeCell::new(HashSet::new()),
				loader: SyncUnsafeCell::new(None),

				obj: SyncUnsafeCell::new(obj),
				open: is_open,
				name: Some(module_name_sym),
				version: SyncUnsafeCell::new(version),
				location: SyncUnsafeCell::new(location),
			};

			let module = loader.insert_module(guard, module_entry);

			for package in package_symbols {
				let package = Package::new(package, module);
				loader.insert_package_if_absent(guard, package);
			}
		});

		if module_already_defined {
			throw!(@DEFER IllegalStateException, "Module {} is already defined", module_name_sym.as_str());
		}

		if let Some(duplicate_package) = duplicate_package {
			match duplicate_package.module().name() {
				Some(name) => {
					throw!(@DEFER IllegalStateException,
						"Package {} for module {} is already in another module, {}, defined to the class loader",
						duplicate_package.name().as_str(),
						module_name_sym.as_str(),
						name.as_str(),
					);
				},
				None => {
					throw!(@DEFER IllegalStateException,
						"Package {} for module {} is already in the unnamed module defined to the class loader",
						duplicate_package.name().as_str(),
						module_name_sym.as_str()
					);
				},
			}
		}

		if loader.is_bootstrap() && !jimage::initialized() {
			todo!("Exploded modules for bootstrap loader")
		}

		Throws::Ok(())
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

			let package = Package::new(Symbol::intern(package), java_base);
			ClassLoader::bootstrap().insert_package_if_absent(guard, package);
		}

		unsafe { java_base.fixup_java_base(guard, obj, version, location) }
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

	pub fn is_open(&self) -> bool {
		self.open
	}

	// Called in `ModuleSet::add`
	pub(super) fn set_classloader(&mut self, loader: &'static ClassLoader) {
		*self.loader.get_mut() = Some(loader);
	}

	pub fn classloader(&self) -> &'static ClassLoader {
		unsafe { *self.loader.get() }.expect("loader should always be available")
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
		*unsafe { &*obj_ptr }
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

	/// Whether this module's version should be displayed in error messages and logs
	///
	/// This is `true` if:
	///
	/// * The [`version`] is non-null
	/// * The module is named
	/// * The module is upgradeable, meaning:
	///   * The [`location`] is "jrt:/java." and its loader is boot or platform
	///   * The [`location`] is "jrt:/jdk.", its loader is one of the builtin loaders and its version
	///     is the same as module `java.base`'s version
	///
	/// [version]: Self::version
	/// [location]: Self::location
	pub fn should_show_version(&self) -> bool {
		if self.version().is_none() || self.name().is_none() {
			return false;
		}

		let Some(_location) = self.location() else {
			return true;
		};

		// TODO: upgradeable module check
		true
	}

	pub fn add_reads(&self, other: Option<&'static Self>) -> Throws<()> {
		if self.name.is_none() {
			// Nothing to do
			return Throws::Ok(());
		}

		super::with_module_lock(|_guard| {
			let flags_ptr = self.flags.get();

			let Some(other) = other else {
				unsafe { (&mut *flags_ptr).can_read_all_unnamed = true };
				return;
			};

			let reads = unsafe { &mut *self.reads.get() };
			reads.insert(other);
		});

		Throws::Ok(())
	}

	pub fn add_exports(&self, other: Option<&'static Self>, package_name: String) {
		if self.name().is_none() || self.is_open() {
			// Nothing to do if `from` is unnamed or open. All packages are exported by default.
			return;
		}

		super::with_module_lock(|guard| {
			let Some(package) = self
				.classloader()
				.lookup_package(guard, Symbol::intern(package_name))
			else {
				return;
			};

			match other {
				Some(other) => package.add_qualified_export(guard, other),
				None => package.set_export_type(guard, PackageExportType::Unqualified),
			}
		})
	}
}
