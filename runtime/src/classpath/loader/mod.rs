mod set;
pub use set::*;

use crate::classes;
use crate::modules::{Module, ModuleLockGuard, ModuleSet, Package};
use crate::native::java::lang::String::rust_string_from_java_string;
use crate::objects::class::Class;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;
use crate::symbols::Symbol;
use crate::thread::exceptions::{throw, Throws};

use std::cell::SyncUnsafeCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::RangeInclusive;
use std::sync::{LazyLock, Mutex};

use classfile::constant_pool::types::raw as raw_types;
use classfile::{ClassFile, FieldType};
use common::int_types::u1;
use common::traits::PtrType;

const SUPPORTED_MAJOR_LOWER_BOUND: u1 = 45;
const SUPPORTED_MAJOR_UPPER_BOUND: u1 = 69;
const SUPPORTED_MAJOR_VERSION_RANGE: RangeInclusive<u1> =
	SUPPORTED_MAJOR_LOWER_BOUND..=SUPPORTED_MAJOR_UPPER_BOUND;

pub struct ClassLoader {
	obj: Reference,

	name: Option<Symbol>,
	name_and_id: Symbol,

	unnamed_module: SyncUnsafeCell<Option<&'static Module>>,

	classes: Mutex<HashMap<Symbol, &'static Class>>,

	// TODO: Is there a better way to do this?
	// Keep the java.base module separate from the other modules for bootstrapping. This field is only
	// valid for the bootstrap loader.
	java_base: SyncUnsafeCell<Option<&'static Module>>,

	// Access to these fields is manually synchronized with the global module mutex
	modules: ModuleSet,
	packages: SyncUnsafeCell<HashMap<Symbol, Package>>,
}

impl PartialEq for ClassLoader {
	fn eq(&self, other: &Self) -> bool {
		self.obj == other.obj
	}
}

impl Debug for ClassLoader {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ClassLoader")
			.field("obj", &self.obj)
			.finish()
	}
}

impl ClassLoader {
	fn from_obj(obj: Reference) -> Self {
		assert!(!obj.is_null(), "cannot create ClassLoader from null obj");
		assert!(obj.is_instance_of(crate::globals::classes::java_lang_ClassLoader()));

		let mut name = None;

		let name_obj = obj
			.get_field_value0(classes::java_lang_ClassLoader::name_field_offset())
			.expect_reference();
		if !name_obj.is_null() {
			let name_str = rust_string_from_java_string(name_obj.extract_class());

			if !name_str.is_empty() {
				name = Some(Symbol::intern(name_str));
			}
		}

		let unnamed_module_obj = obj
			.get_field_value0(classes::java_lang_ClassLoader::unnamedModule_field_offset())
			.expect_reference();
		assert!(
			!unnamed_module_obj.is_null()
				&& unnamed_module_obj.is_instance_of(crate::globals::classes::java_lang_Module())
		);

		let unnamed_module =
			Module::unnamed(unnamed_module_obj).expect("unnamed module creation failed");

		let name_and_id_obj = obj
			.get_field_value0(classes::java_lang_ClassLoader::nameAndId_field_offset())
			.expect_reference();

		let name_and_id;
		if name_and_id_obj.is_null() {
			name_and_id = obj.extract_target_class().name.as_str().replace('/', ".");
		} else {
			name_and_id = rust_string_from_java_string(name_and_id_obj.extract_class());
		}

		assert!(!name_and_id.is_empty(), "class loader has no name and id");

		Self {
			obj,
			name,
			name_and_id: Symbol::intern(name_and_id),

			unnamed_module: SyncUnsafeCell::new(Some(Box::leak(Box::new(unnamed_module)))),
			classes: Mutex::new(HashMap::new()),

			// Never initialized for non-bootstrap loaders
			java_base: SyncUnsafeCell::new(None),

			modules: ModuleSet::new(),
			packages: SyncUnsafeCell::new(HashMap::new()),
		}
	}
}

// Module locked methods
impl ClassLoader {
	pub fn insert_package_if_absent(&self, _guard: &ModuleLockGuard, package: Package) {
		let packages = unsafe { &mut *self.packages.get() };
		packages.entry(package.name()).or_insert(package);
	}

	pub fn lookup_package(&self, _guard: &ModuleLockGuard, name: Symbol) -> Option<&Package> {
		let packages = unsafe { &*self.packages.get() };
		packages.get(&name)
	}

	pub fn insert_module(&self, _guard: &ModuleLockGuard, module: Module) -> &'static Module {
		if module.name().is_none() {
			panic!("Attempted to insert an unnamed module using `insert_module`")
		};

		self.modules.add(_guard, module)
	}

	pub fn lookup_module(&self, _guard: &ModuleLockGuard, name: Symbol) -> Option<&'static Module> {
		self.modules.find(_guard, name)
	}
}

// Bootstrap methods
impl ClassLoader {
	pub fn bootstrap() -> &'static Self {
		static BOOTSTRAP_LOADER: LazyLock<SyncUnsafeCell<ClassLoader>> = LazyLock::new(|| {
			let name_sym = Symbol::intern("bootstrap");

			let loader = ClassLoader {
				obj: Reference::null(),

				name: Some(name_sym),
				name_and_id: name_sym,

				unnamed_module: SyncUnsafeCell::new(None),
				classes: Mutex::new(HashMap::new()),

				java_base: SyncUnsafeCell::new(None),
				modules: ModuleSet::new(),
				packages: SyncUnsafeCell::new(HashMap::new()),
			};

			SyncUnsafeCell::new(loader)
		});

		unsafe { &*BOOTSTRAP_LOADER.get() }
	}

	pub fn java_base(&self) -> &'static Module {
		assert!(self.is_bootstrap());
		unsafe { &*self.java_base.get() }.expect("java.base should be set")
	}

	pub fn set_java_base(&self, java_base: Module) {
		let ptr = self.java_base.get();
		assert!(
			unsafe { &*ptr }.is_none(),
			"java.base cannot be set more than once"
		);

		unsafe { *ptr = Some(Box::leak(Box::new(java_base))) }
	}

	pub fn is_bootstrap(&self) -> bool {
		self.obj.is_null()
	}

	/// Stores a copy of the `jdk.internal.loader.BootLoader#UNNAMED_MODULE` field
	///
	/// This is called from `jdk.internal.loader.BootLoader#setBootLoaderUnnamedModule0`, and can
	/// only be set once.
	pub fn set_bootloader_unnamed_module(entry: Module) {
		let bootloader = ClassLoader::bootstrap();

		let ptr = bootloader.unnamed_module.get();
		assert!(
			unsafe { (*ptr).is_none() },
			"Attempt to set unnamed module for bootloader twice"
		);

		unsafe {
			*ptr = Some(Box::leak(Box::new(entry)));
		}
	}
}

impl ClassLoader {
	pub(crate) fn lookup_class(&self, name: Symbol) -> Option<&'static Class> {
		let loaded_classes = self.classes.lock().unwrap();
		loaded_classes.get(&name).map(|&class| class)
	}

	pub fn unnamed_module(&self) -> &'static Module {
		unsafe { &*self.unnamed_module.get() }.expect("unnamed module should be set")
	}
}

impl ClassLoader {
	/// Get the `name` field of the `ClassLoader`, if it has been set
	pub fn name(&self) -> Option<Symbol> {
		self.name
	}

	/// Get the `nameAndId` field of the `ClassLoader`
	///
	/// Unlike [`name`], this field is always available.
	///
	/// The format is:
	/// * The loader has a name defined: `'<loader-name>' @<id>`
	/// * The loader has no name defined: `<qualified-class-name> @<id>`
	/// * The loader is built-in: `@<id>` is omitted, as there is only one instance
	pub fn name_and_id(&self) -> Symbol {
		self.name_and_id
	}

	pub fn obj(&self) -> Reference {
		self.obj.clone()
	}

	/// Whether the ClassLoader is parallel capable
	pub fn is_parallel_capable(&self) -> bool {
		self.is_bootstrap() || classes::java_lang_ClassLoader::parallelCapable(&self.obj())
	}

	pub fn load(&'static self, name: Symbol) -> Throws<&'static Class> {
		if self.is_bootstrap() {
			return self.load_bootstrap(name);
		}

		unimplemented!("User-defined class loaders")
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.3.1
	fn load_bootstrap(&'static self, name: Symbol) -> Throws<&'static Class> {
		// First, the Java Virtual Machine determines whether the bootstrap class loader has
		// already been recorded as an initiating loader of a class or interface denoted by N.
		// If so, this class or interface is C, and no class loading or creation is necessary.
		if let Some(ret) = self.lookup_class(name) {
			return Throws::Ok(ret);
		}

		// Otherwise, the Java Virtual Machine passes the argument N to an invocation of a method on
		// the bootstrap class loader [...] and then [...] create C, via the algorithm of §5.3.5.
		let classref = self.derive_class(name, None)?;

		// TODO:
		// If no purported representation of C is found, the bootstrap class loader throws a ClassNotFoundException.
		// The process of loading and creating C then fails with a NoClassDefFoundError whose cause is the ClassNotFoundException.

		// If a purported representation of C is found, but deriving C from the purported representation fails,
		// then the process of loading and creating C fails for the same reason.

		// Otherwise, the process of loading and creating C succeeds.
		Throws::Ok(classref)
	}

	// Deriving a Class from a class File Representation
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.3.5
	pub fn derive_class(
		&'static self,
		name: Symbol,
		classfile_bytes: Option<&[u1]>,
	) -> Throws<&'static Class> {
		if let Some(class) = self.lookup_class(name) {
			return Throws::Ok(class);
		}

		let name_str = name.as_str();
		if name_str.starts_with('[') {
			return self.create_array_class(name);
		}

		match classfile_bytes {
			Some(classfile_bytes) => self.derive_class_inner(name, name_str, classfile_bytes),
			None => {
				let classfile_bytes = super::find_classpath_entry(name);
				self.derive_class_inner(name, name_str, &classfile_bytes)
			},
		}
	}

	fn derive_class_inner(
		&'static self,
		name: Symbol,
		name_str: &str,
		classfile_bytes: &[u1],
	) -> Throws<&'static Class> {
		// TODO:
		// 1. First, the Java Virtual Machine determines whether L has already been recorded
		//    as an initiating loader of a class or interface denoted by N. If so, this derivation
		//    attempt is invalid and derivation throws a LinkageError.

		// 2. Otherwise, the Java Virtual Machine attempts to parse the purported representation.
		//    The purported representation may not in fact be a valid representation of C, so
		//    derivation must detect the following problems:
		let Ok(classfile) = ClassFile::read_from(&mut &classfile_bytes[..]) else {
			//  2.1. If the purported representation is not a ClassFile structure (§4.1, §4.8), derivation
			//       throws a ClassFormatError.
			throw!(@DEFER ClassFormatError);
		};

		//  2.2. Otherwise, if the purported representation is not of a supported major or
		//       minor version (§4.1), derivation throws an UnsupportedClassVersionError.
		if !SUPPORTED_MAJOR_VERSION_RANGE.contains(&(classfile.major_version as u1)) {
			throw!(@DEFER UnsupportedClassVersionError);
		}

		//  2.3. Otherwise, if the purported representation does not actually represent a class or
		//       interface named N, derivation throws a NoClassDefFoundError. This occurs when the
		//       purported representation has either a this_class item which specifies a name other
		//       than N, or an access_flags item which has the ACC_MODULE flag set.
		let specified_class_name = classfile
			.constant_pool
			.get::<raw_types::RawClassName>(classfile.this_class);
		if name_str.as_bytes() != &*specified_class_name.name || classfile.access_flags.is_module()
		{
			throw!(@DEFER NoClassDefFoundError);
		}

		//  3. If C has a direct superclass, the symbolic reference from C to its direct
		//     superclass is resolved using the algorithm of §5.4.3.1. Note that if C is an interface
		//     it must have Object as its direct superclass, which must already have been loaded.
		//     Only Object has no direct superclass.
		let mut super_class = None;
		if let Some(super_class_name) = classfile.get_super_class() {
			super_class = Some(self.resolve_super_class(Symbol::intern(&*super_class_name))?);
		}

		// TODO:
		// 4. If C has any direct superinterfaces, the symbolic references from C to its direct
		//    superinterfaces are resolved using the algorithm of §5.4.3.1.

		// If no exception is thrown in steps 1-4, then derivation of the class or interface C succeeds.
		// The Java Virtual Machine marks C to have L as its defining loader, records that L is an initiating
		// loader of C (§5.3.4), and creates C in the method area (§2.5.4).

		let class = unsafe { Class::new(classfile, super_class, self)? };
		self.classes.lock().unwrap().insert(name, class);

		// Finally, prepare the class (§5.4.2)
		// "Preparation may occur at any time following creation but must be completed prior to initialization."
		class.prepare();

		init_mirror(class);

		Throws::Ok(class)
	}

	fn resolve_super_class(&'static self, super_class_name: Symbol) -> Throws<&'static Class> {
		// Any exception that can be thrown as a result of failure of class or interface resolution
		// can be thrown as a result of derivation. In addition, derivation must detect the following problems:

		// TODO:
		//     If any of the superclasses of C is C itself, derivation throws a ClassCircularityError.

		// TODO:
		//     Otherwise, if the class or interface named as the direct superclass of C is in fact an interface
		//     or a final class, derivation throws an IncompatibleClassChangeError.

		// TODO:
		//     Otherwise, if the class named as the direct superclass of C has a PermittedSubclasses attribute (§4.7.31)
		//     and any of the following is true, derivation throws an IncompatibleClassChangeError:

		// TODO:
		//         The superclass is in a different run-time module than C (§5.3.6).

		// TODO:
		//         C does not have its ACC_PUBLIC flag set (§4.1) and the superclass is in a different run-time package than C (§5.3).

		// TODO:
		//         No entry in the classes array of the superclass's PermittedSubclasses attribute refers to a class or interface with the name N.

		// TODO:
		//     Otherwise, if C is a class and some instance method declared in C can override (§5.4.5)
		//     a final instance method declared in a superclass of C, derivation throws an IncompatibleClassChangeError.

		self.derive_class(super_class_name, None)
	}

	// Creating array classes
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.3.3
	fn create_array_class(&'static self, descriptor: Symbol) -> Throws<&'static Class> {
		// The following steps are used to create the array class C denoted by the name N in association with the class loader L.
		// L may be either the bootstrap class loader or a user-defined class loader.

		// First, the Java Virtual Machine determines whether L has already been recorded as an initiating loader of an array class with
		// the same component type as N. If so, this class is C, and no array class creation is necessary.
		if let Some(ret) = self.lookup_class(descriptor) {
			return Throws::Ok(ret);
		}

		// Otherwise, the following steps are performed to create C:
		//
		//     If the component type is a reference type, the algorithm of this section (§5.3) is applied recursively using L in order to load and thereby create the component type of C.
		let mut descriptor_str = descriptor.as_str();
		let array = FieldType::parse(&mut descriptor_str.as_bytes()).unwrap(); // TODO: Error handling
		let FieldType::Array(mut component) = array else {
			unreachable!("The descriptor was validated as an array prior");
		};

		loop {
			if let FieldType::Object(obj) = &*component {
				self.load(Symbol::intern(&obj))?;
				break;
			}

			if let FieldType::Array(array) = *component {
				// Just strip '[' until we finally reach the component type.
				descriptor_str = &descriptor_str[1..];
				self.load(Symbol::intern(descriptor_str))?;
				component = array;
				continue;
			}

			break;
		}

		//     The Java Virtual Machine creates a new array class with the indicated component type and number of dimensions.
		let array_class = unsafe { Class::new_array(descriptor, *component, self) };

		//     If the component type is a reference type, the Java Virtual Machine marks C to have the defining loader of the component type as its defining loader.
		//     Otherwise, the Java Virtual Machine marks C to have the bootstrap class loader as its defining loader.

		// (Already handled)

		//     In any case, the Java Virtual Machine then records that L is an initiating loader for C (§5.3.4).

		// (Already handled)

		// TODO:
		//     If the component type is a reference type, the accessibility of the array class is determined by the accessibility of its component type (§5.4.4).
		//     Otherwise, the array class is accessible to all classes and interfaces.

		init_mirror(array_class);

		self.classes.lock().unwrap().insert(descriptor, array_class);
		Throws::Ok(array_class)
	}

	/// Recreate mirrors for all loaded classes
	pub fn fixup_mirrors() {
		let bootstrap_loader = ClassLoader::bootstrap();
		for class in bootstrap_loader.classes.lock().unwrap().values() {
			// SAFETY: The only condition of `set_mirror` is that the class isn't in use yet.
			unsafe {
				class.set_mirror(None);
			}
		}
	}

	/// Sets all currently loaded classes to be members of `java.base`
	pub fn fixup_modules(obj: Reference) {
		let bootstrap_loader = ClassLoader::bootstrap();
		for class in bootstrap_loader.classes.lock().unwrap().values() {
			class.mirror().get().set_module(obj.clone());
		}
	}
}

fn init_mirror(class: &'static Class) {
	// Set the mirror if `java.lang.Class` is loaded
	let class_loaded = crate::globals::classes::java_lang_Class_opt().is_some();
	if !class_loaded {
		// We cannot do anything to this class until a mirror is available.
		return;
	}

	// SAFETY: The only condition of `set_mirror` is that the class isn't in use yet.
	unsafe {
		class.set_mirror(None);
	}
}
