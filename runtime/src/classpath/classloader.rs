use crate::error::{Result, RuntimeError};
use crate::objects::class::Class;

use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::sync::{LazyLock, Mutex};

use classfile::{ClassFile, FieldType};
use common::int_types::u1;
use symbols::{sym, Symbol};

const SUPPORTED_MAJOR_LOWER_BOUND: u1 = 45;
const SUPPORTED_MAJOR_UPPER_BOUND: u1 = 67;
const SUPPORTED_MAJOR_VERSION_RANGE: RangeInclusive<u1> =
	SUPPORTED_MAJOR_LOWER_BOUND..=SUPPORTED_MAJOR_UPPER_BOUND;

static BOOTSTRAP_LOADED_CLASSES: LazyLock<Mutex<HashMap<Symbol, &'static Class>>> =
	LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClassLoader {
	Bootstrap,
	UserDefined,
}

impl ClassLoader {
	pub(crate) fn lookup_class(name: Symbol) -> Option<&'static Class> {
		let loaded_classes = BOOTSTRAP_LOADED_CLASSES.lock().unwrap();

		loaded_classes.get(&name).map(|&class| class)
	}

	fn insert_bootstrapped_class(name: Symbol, classref: &'static Class) {
		let mut loaded_classes = BOOTSTRAP_LOADED_CLASSES.lock().unwrap();
		loaded_classes.insert(name, classref);
	}
}

impl ClassLoader {
	pub fn load(&self, name: Symbol) -> Option<&'static Class> {
		match self {
			ClassLoader::Bootstrap => Self::load_bootstrap(name),
			ClassLoader::UserDefined => unimplemented!("User defined loader"),
		}
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.3.1
	fn load_bootstrap(name: Symbol) -> Option<&'static Class> {
		// First, the Java Virtual Machine determines whether the bootstrap class loader has
		// already been recorded as an initiating loader of a class or interface denoted by N.
		// If so, this class or interface is C, and no class loading or creation is necessary.
		if let ret @ Some(_) = Self::lookup_class(name) {
			return ret;
		}

		// Otherwise, the Java Virtual Machine passes the argument N to an invocation of a method on
		// the bootstrap class loader [...] and then [...] create C, via the algorithm of §5.3.5.
		let classref = ClassLoader::Bootstrap.load_class_by_name(name);

		// TODO:
		// If no purported representation of C is found, the bootstrap class loader throws a ClassNotFoundException.
		// The process of loading and creating C then fails with a NoClassDefFoundError whose cause is the ClassNotFoundException.

		// If a purported representation of C is found, but deriving C from the purported representation fails,
		// then the process of loading and creating C fails for the same reason.

		// Otherwise, the process of loading and creating C succeeds.
		Some(classref)
	}

	// Deriving a Class from a class File Representation
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.3.5
	fn load_class_by_name(self, name: Symbol) -> &'static Class {
		if let Some(class) = Self::lookup_class(name) {
			return class;
		}

		let name_str = name.as_str();
		if name_str.starts_with('[') {
			return self.create_array_class(name);
		}

		// TODO:
		// 1. First, the Java Virtual Machine determines whether L has already been recorded
		//    as an initiating loader of a class or interface denoted by N. If so, this derivation
		//    attempt is invalid and derivation throws a LinkageError.

		// 2. Otherwise, the Java Virtual Machine attempts to parse the purported representation.
		let classfile_bytes = super::find_classpath_entry(name);
		let classfile = ClassFile::read_from(&mut &classfile_bytes[..]).unwrap(); // TODO: handle errors

		//    The purported representation may not in fact be a valid representation of C, so
		//    derivation must detect the following problems:

		// TODO:
		//  2.1. If the purported representation is not a ClassFile structure (§4.1, §4.8), derivation
		//       throws a ClassFormatError.

		//  2.2. Otherwise, if the purported representation is not of a supported major or
		//       minor version (§4.1), derivation throws an UnsupportedClassVersionError.
		assert!(
			SUPPORTED_MAJOR_VERSION_RANGE.contains(&(classfile.major_version as u1)),
			"UnsupportedClassVersionError"
		);

		// TODO:
		//  2.3. Otherwise, if the purported representation does not actually represent a class or
		//       interface named N, derivation throws a NoClassDefFoundError. This occurs when the
		//       purported representation has either a this_class item which specifies a name other
		//       than N, or an access_flags item which has the ACC_MODULE flag set.

		//  3. If C has a direct superclass, the symbolic reference from C to its direct
		//     superclass is resolved using the algorithm of §5.4.3.1. Note that if C is an interface
		//     it must have Object as its direct superclass, which must already have been loaded.
		//     Only Object has no direct superclass.
		let mut super_class = None;

		if let Some(super_class_name) = classfile.get_super_class() {
			super_class = Some(self.resolve_super_class(Symbol::intern_bytes(super_class_name)));
		}

		// TODO:
		// 4. If C has any direct superinterfaces, the symbolic references from C to its direct
		//    superinterfaces are resolved using the algorithm of §5.4.3.1.

		// If no exception is thrown in steps 1-4, then derivation of the class or interface C succeeds.
		// The Java Virtual Machine marks C to have L as its defining loader, records that L is an initiating
		// loader of C (§5.3.4), and creates C in the method area (§2.5.4).

		let class = unsafe { Class::new(classfile, super_class, self) };

		// Finally, prepare the class (§5.4.2)
		// "Preparation may occur at any time following creation but must be completed prior to initialization."
		class.prepare();

		// Set the mirror
		if let Some(mirror_class) = Self::lookup_class(sym!(java_lang_Class)) {
			// SAFETY: The only condition of `set_mirror` is that the class isn't in use yet.
			unsafe {
				Class::set_mirror(mirror_class, class);
			}
		}

		Self::insert_bootstrapped_class(name, class);

		class
	}

	fn resolve_super_class(self, super_class_name: Symbol) -> &'static Class {
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

		self.load_class_by_name(super_class_name)
	}

	// Creating array classes
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.3.3
	fn create_array_class(self, descriptor: Symbol) -> &'static Class {
		// The following steps are used to create the array class C denoted by the name N in association with the class loader L.
		// L may be either the bootstrap class loader or a user-defined class loader.

		// First, the Java Virtual Machine determines whether L has already been recorded as an initiating loader of an array class with
		// the same component type as N. If so, this class is C, and no array class creation is necessary.
		if let Some(ret) = Self::lookup_class(descriptor) {
			return ret;
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
				self.load(Symbol::intern_bytes(&obj));
				break;
			}

			if let FieldType::Array(array) = *component {
				// Just strip '[' until we finally reach the component type.
				descriptor_str = &descriptor_str[1..];
				self.load(Symbol::intern(descriptor_str));
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

		// Set the mirror
		if let Some(mirror_class) = Self::lookup_class(sym!(java_lang_Class)) {
			// SAFETY: The only condition of `set_mirror` is that the class isn't in use yet.
			unsafe {
				array_class.set_mirror(mirror_class);
			}
		}

		Self::insert_bootstrapped_class(descriptor, array_class);
		array_class
	}

	/// Recreate mirrors for all loaded classes
	pub fn fixup_mirrors() {
		let java_lang_class = crate::globals::classes::java_lang_Class();
		for (_, class) in BOOTSTRAP_LOADED_CLASSES.lock().unwrap().iter() {
			// SAFETY: The only condition of `set_mirror` is that the class isn't in use yet.
			unsafe {
				class.set_mirror(java_lang_class);
			}
		}
	}

	/// Get the package name from a fully qualified class name
	pub fn package_name_for_class(name: Symbol) -> Result<Option<&'static str>> {
		let name_str = name.as_str();

		if name_str.is_empty() {
			return Err(RuntimeError::BadClassName);
		}

		let Some(end) = name_str.as_bytes().iter().rposition(|c| *c == b'/') else {
			return Ok(None);
		};

		let mut start_index = 0;

		// Skip over '['
		if name_str.starts_with('[') {
			start_index = name_str.chars().skip_while(|c| *c == '[').count();

			// A fully qualified class name should not contain a 'L'
			if start_index >= name_str.len() || name_str.as_bytes()[start_index] == b'L' {
				return Err(RuntimeError::BadClassName);
			}
		}

		if start_index >= name_str.len() {
			return Err(RuntimeError::BadClassName);
		}

		return Ok(Some(&name_str[start_index..end]));
	}
}
