use crate::error::{Result, RuntimeError};
use crate::field::Field;
use crate::heap::class::Class;
use crate::heap::reference::ClassRef;

use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex};

use classfile::{ClassFile, FieldType};
use common::int_types::u1;
use once_cell::sync::Lazy;

const SUPPORTED_MAJOR_LOWER_BOUND: u1 = 45;
const SUPPORTED_MAJOR_UPPER_BOUND: u1 = 64;
const SUPPORTED_MAJOR_VERSION_RANGE: RangeInclusive<u1> =
	SUPPORTED_MAJOR_LOWER_BOUND..=SUPPORTED_MAJOR_UPPER_BOUND;

static BOOTSTRAP_LOADED_CLASSES: Lazy<Mutex<HashMap<Vec<u1>, ClassRef>>> =
	Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClassLoader {
	Bootstrap,
	UserDefined,
}

impl ClassLoader {
	pub(crate) fn lookup_class(name: &[u1]) -> Option<ClassRef> {
		let loaded_classes = BOOTSTRAP_LOADED_CLASSES.lock().unwrap();

		let classref = loaded_classes.get(name);
		classref.map(Arc::clone)
	}

	fn insert_bootstrapped_class(name: Vec<u1>, classref: ClassRef) {
		let mut loaded_classes = BOOTSTRAP_LOADED_CLASSES.lock().unwrap();
		loaded_classes.insert(name, classref);
	}
}

impl ClassLoader {
	pub fn load(&self, name: &[u1]) -> Option<ClassRef> {
		match self {
			ClassLoader::Bootstrap => Self::load_bootstrap(name),
			ClassLoader::UserDefined => unimplemented!("User defined loader"),
		}
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.3.1
	fn load_bootstrap(name: &[u1]) -> Option<ClassRef> {
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
	fn load_class_by_name(self, name: &[u1]) -> ClassRef {
		if let Some(class) = Self::lookup_class(name) {
			return class;
		}

		if name.first() == Some(&b'[') {
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
			super_class = Some(self.resolve_super_class(super_class_name));
		}

		// TODO:
		// 4. If C has any direct superinterfaces, the symbolic references from C to its direct
		//    superinterfaces are resolved using the algorithm of §5.4.3.1.

		// If no exception is thrown in steps 1-4, then derivation of the class or interface C succeeds.
		// The Java Virtual Machine marks C to have L as its defining loader, records that L is an initiating
		// loader of C (§5.3.4), and creates C in the method area (§2.5.4).

		let class = Class::new(classfile, super_class, self);

		// Finally, prepare the class (§5.4.2)
		// "Preparation may occur at any time following creation but must be completed prior to initialization."
		Self::prepare(Arc::clone(&class));

		// Set the mirror
		if let Some(mirror_class) = Self::lookup_class(b"java/lang/Class") {
			Class::set_mirror(mirror_class, Arc::clone(&class));
		}

		Self::insert_bootstrapped_class(name.to_vec(), Arc::clone(&class));

		class
	}

	fn resolve_super_class(self, super_class_name: &[u1]) -> ClassRef {
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
	fn create_array_class(self, descriptor: &[u1]) -> ClassRef {
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
		let component = FieldType::parse(&mut &descriptor[..]).unwrap(); // TODO: Error handling

		if let FieldType::Object(ref obj) = component {
			self.load(obj);
		}

		//     The Java Virtual Machine creates a new array class with the indicated component type and number of dimensions.
		let array_class = Class::new_array(descriptor, component, self);

		//     If the component type is a reference type, the Java Virtual Machine marks C to have the defining loader of the component type as its defining loader.
		//     Otherwise, the Java Virtual Machine marks C to have the bootstrap class loader as its defining loader.

		// (Already handled)

		//     In any case, the Java Virtual Machine then records that L is an initiating loader for C (§5.3.4).

		// (Already handled)

		// TODO:
		//     If the component type is a reference type, the accessibility of the array class is determined by the accessibility of its component type (§5.4.4).
		//     Otherwise, the array class is accessible to all classes and interfaces.

		// Set the mirror
		if let Some(mirror_class) = Self::lookup_class(b"java/lang/Class") {
			Class::set_mirror(mirror_class, Arc::clone(&array_class));
		}

		Self::insert_bootstrapped_class(descriptor.to_vec(), Arc::clone(&array_class));
		array_class
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.4.2
	fn prepare(class: ClassRef) {
		// Preparation involves creating the static fields for a class or interface and initializing such fields
		// to their default values (§2.3, §2.4). This does not require the execution of any Java Virtual Machine code;
		// explicit initializers for static fields are executed as part of initialization (§5.5), not preparation.
		let class_instance = class.unwrap_class_instance_mut();
		for (idx, field) in class_instance
			.fields
			.iter()
			.filter(|field| field.is_static())
			.enumerate()
		{
			class_instance.static_field_slots[idx] = Field::default_value_for_ty(&field.descriptor);
		}

		// TODO:
		// During preparation of a class or interface C, the Java Virtual Machine also imposes loading constraints (§5.3.4):
	}

	/// Recreate mirrors for all loaded classes
	pub fn fixup_mirrors() {
		let java_lang_class =
			Self::lookup_class(b"java/lang/Class").expect("java.lang.Class should be loaded");
		for (_, class) in BOOTSTRAP_LOADED_CLASSES.lock().unwrap().iter() {
			Class::set_mirror(Arc::clone(&java_lang_class), Arc::clone(class));
		}
	}

	/// Get the package name from a fully qualified class name
	pub fn package_name_for_class(name: &[u1]) -> Result<Option<&[u1]>> {
		if name.is_empty() {
			return Err(RuntimeError::BadClassName);
		}

		let Some(end) = name.iter().rposition(|c| *c == b'/') else {
			return Ok(None);
		};

		let mut start_index = 0;

		// Skip over '['
		if name.starts_with(b"[") {
			start_index = name.iter().skip_while(|c| **c == b'[').count();

			// A fully qualified class name should not contain a 'L'
			if start_index >= name.len() || name[start_index] == b'L' {
				return Err(RuntimeError::BadClassName);
			}
		}

		if start_index >= name.len() {
			return Err(RuntimeError::BadClassName);
		}

		return Ok(Some(&name[start_index..end]));
	}
}
