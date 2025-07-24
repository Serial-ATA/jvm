use crate::classes;
use crate::globals::{BASE_TYPES_TO_FIELD_TYPES, PRIMITIVES};
use crate::objects::class::ClassPtr;
use crate::objects::instance::object::Object;
use crate::objects::instance::{Header, Instance};
use crate::objects::reference::Reference;
use crate::thread::JavaThread;

use classfile::FieldType;
use instructions::Operand;
use jni::sys::{jchar, jint};

use std::fmt::{Debug, Formatter};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub enum MirrorTarget {
	Class(ClassPtr),
	Primitive(FieldType),
}

/// Reference to an allocated mirror instance
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct MirrorInstanceRef(*mut MirrorInstance);

// SAFETY: Synchronization handled manually
unsafe impl Send for MirrorInstanceRef {}

unsafe impl Sync for MirrorInstanceRef {}

impl MirrorInstanceRef {
	const FIELD_BASE: usize = size_of::<MirrorInstance>();
}

impl Object for MirrorInstanceRef {
	type Descriptor = MirrorInstance;

	fn hash(&self, thread: &'static JavaThread) -> jint {
		self.header.generate_hash(thread)
	}

	fn class(&self) -> ClassPtr {
		crate::globals::classes::java_lang_Class()
	}

	fn is_mirror(&self) -> bool {
		true
	}

	unsafe fn raw(&self) -> *mut () {
		self.0.cast()
	}

	unsafe fn field_base(&self) -> *mut u8 {
		let base = self.0.cast::<u8>();
		unsafe { base.add(Self::FIELD_BASE) }
	}
}

impl MirrorInstanceRef {
	fn new(
		obj: *mut MirrorInstance,
		target_class: ClassPtr,
		modifiers: jchar,
		is_primitive: bool,
	) -> Self {
		let ret = Self(obj);

		ret.put_field_value0(
			classes::java::lang::Class::classLoader_field_index(),
			Operand::Reference(target_class.loader().obj()),
		);
		ret.put_field_value0(
			classes::java::lang::Class::modifiers_field_index(),
			Operand::Int(modifiers as jint),
		);
		ret.put_field_value0(
			classes::java::lang::Class::primitive_field_index(),
			Operand::Int(is_primitive as jint),
		);

		ret
	}

	pub fn set_module(&self, module: Reference) {
		let ptr =
			unsafe { self.get_raw::<Reference>(classes::java::lang::Class::module_field_offset()) };

		unsafe {
			assert!((&*ptr).is_null(), "Attempt to set a module twice");
		}

		// Early in initialization, even before java.lang.Module is loaded, this will
		// be called with a null reference. Since the mirror is already default initialized with null,
		// there's nothing to do. Return early to preserve the assertion below.
		if module.is_null() {
			return;
		}

		assert!(module.is_instance_of(crate::globals::classes::java_lang_Module()));

		unsafe {
			*ptr = module;
		}
	}

	pub fn set_class_data(&self, class_data: Reference) {
		self.put_field_value0(
			classes::java::lang::Class::classData_field_index(),
			Operand::Reference(class_data),
		)
	}
}

impl PartialEq for MirrorInstanceRef {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl Deref for MirrorInstanceRef {
	type Target = MirrorInstance;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0 }
	}
}

impl Debug for MirrorInstanceRef {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.deref().fmt(f)
	}
}

/// A mirror instance
///
/// A "mirror" is simply an instance of `java.lang.Class` with an associated target [`Class`].
///
/// In the following:
///
/// ```java
/// var c = String.class;
/// ```
///
/// `c` is a mirror instance, with a target of `java.lang.String`.
pub struct MirrorInstance {
	header: Header,
	target: MirrorTarget,
}

impl Debug for MirrorInstance {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("MirrorInstance")
			.field("header", &self.header)
			.field("target", &self.target)
			.finish()
	}
}

impl MirrorInstance {
	pub fn new(target: ClassPtr) -> MirrorInstanceRef {
		let descriptor = MirrorInstance {
			header: Header::new(),
			target: MirrorTarget::Class(target),
		};

		let mirror_class = crate::globals::classes::java_lang_Class();
		let fields_size = mirror_class.size_of_instance_fields();
		let instance_ptr = unsafe { MirrorInstanceRef::allocate(descriptor, fields_size) };

		MirrorInstanceRef::new(instance_ptr, target, target.access_flags().as_u2(), false)
	}

	pub fn new_array(target: ClassPtr) -> MirrorInstanceRef {
		let descriptor = MirrorInstance {
			header: Header::new(),
			target: MirrorTarget::Class(target),
		};

		let mirror_class = crate::globals::classes::java_lang_Class();
		let fields_size = mirror_class.size_of_instance_fields();
		let instance_ptr = unsafe { MirrorInstanceRef::allocate(descriptor, fields_size) };

		let ret =
			MirrorInstanceRef::new(instance_ptr, target, target.access_flags().as_u2(), false);

		let component_type_mirror;

		let component_type = target.array_component_name();
		if PRIMITIVES.contains(&component_type) {
			let component_str = component_type.as_str();
			let (_, field_type) = BASE_TYPES_TO_FIELD_TYPES
				.iter()
				.find(|(ty, _)| *ty == component_str)
				.expect("all primitives are covered");
			component_type_mirror = crate::globals::mirrors::primitive_mirror_for(&field_type);
		} else {
			let component_class = target.loader().load(component_type).unwrap(); // TODO: handle throws
			component_type_mirror = Reference::mirror(component_class.mirror());
		}

		ret.put_field_value0(
			classes::java::lang::Class::componentType_field_index(),
			Operand::Reference(component_type_mirror),
		);
		ret
	}

	/// Create a new mirror instance for a primitive type
	///
	/// This should **never** be used outside of initialization.
	///
	/// All primitive mirrors are available in [`crate::globals::mirrors`]. For example, [`primitive_int_mirror()`].
	///
	/// [`primitive_int_mirror()`]: crate::globals::mirrors::primitive_int_mirror
	pub fn new_primitive(target: FieldType) -> MirrorInstanceRef {
		assert!(
			target.is_primitive(),
			"`Array` and `Object` field types are incompatible with the primitive mirror"
		);

		let target_class = Self::target_for_primitive(&target);
		let descriptor = MirrorInstance {
			header: Header::new(),
			target: MirrorTarget::Primitive(target),
		};

		let mirror_class = crate::globals::classes::java_lang_Class();
		let fields_size = mirror_class.size_of_instance_fields();
		let instance_ptr = unsafe { MirrorInstanceRef::allocate(descriptor, fields_size) };

		// TODO: Are these modifiers correct?
		let ret = MirrorInstanceRef::new(instance_ptr, target_class, 1, true);
		ret
	}

	pub fn is_primitive(&self) -> bool {
		matches!(&self.target, MirrorTarget::Primitive(_))
	}

	pub fn is_array(&self) -> bool {
		matches!(&self.target, MirrorTarget::Class(class) if class.is_array())
	}

	/// The class that this mirror is targeting
	///
	/// In the following:
	///
	/// ```java
	/// var c = String.class;
	/// ```
	///
	/// `String` (`java.lang.String`) is the target class.
	pub fn target_class(&self) -> ClassPtr {
		match &self.target {
			MirrorTarget::Class(class) => *class,
			MirrorTarget::Primitive(field_ty) => Self::target_for_primitive(field_ty),
		}
	}

	/// The primitive type that this mirror is targeting
	pub fn primitive_target(&self) -> &FieldType {
		match &self.target {
			MirrorTarget::Primitive(field_ty) => field_ty,
			_ => unreachable!("only primitive mirrors should exist within primitive mirrors"),
		}
	}

	fn target_for_primitive(primitive: &FieldType) -> ClassPtr {
		match primitive {
			FieldType::Byte => crate::globals::classes::java_lang_Byte(),
			FieldType::Character => crate::globals::classes::java_lang_Character(),
			FieldType::Double => crate::globals::classes::java_lang_Double(),
			FieldType::Float => crate::globals::classes::java_lang_Float(),
			FieldType::Integer => crate::globals::classes::java_lang_Integer(),
			FieldType::Long => crate::globals::classes::java_lang_Long(),
			FieldType::Short => crate::globals::classes::java_lang_Short(),
			FieldType::Boolean => crate::globals::classes::java_lang_Boolean(),
			FieldType::Void => crate::globals::classes::java_lang_Void(),
			_ => unreachable!("only primitive types should exist within primitive mirrors"),
		}
	}
}

impl Instance for MirrorInstanceRef {}
