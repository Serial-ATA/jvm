use super::instance::{Header, Instance};
use crate::globals::{BASE_TYPES_TO_FIELD_TYPES, PRIMITIVES};
use crate::objects::class::Class;
use crate::objects::field::Field;
use crate::objects::reference::{MirrorInstanceRef, Reference};
use crate::objects::monitor::Monitor;

use std::cell::UnsafeCell;
use std::fmt::{Debug, Formatter};
use std::ptr::NonNull;
use std::sync::Arc;

use classfile::FieldType;
use common::traits::PtrType;
use instructions::Operand;
use jni::sys::{jchar, jint};

#[derive(Debug, Clone, PartialEq)]
enum MirrorTarget {
	Class(&'static Class),
	Primitive(FieldType),
}

// TODO: Make fields private
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
	monitor: Arc<Monitor>,
	class: &'static Class,
	fields: Box<[UnsafeCell<Operand<Reference>>]>,
	target: MirrorTarget,
}

impl Debug for MirrorInstance {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("MirrorInstance")
			.field("class", &self.class.name.as_str())
			.field("fields", &self.fields)
			.field("target", &self.target)
			.finish()
	}
}

impl MirrorInstance {
	pub fn new(target: &'static Class) -> MirrorInstanceRef {
		let mirror_class = crate::globals::classes::java_lang_Class();
		let fields =
			Self::initialize_fields(mirror_class, target, target.access_flags().as_u2(), false);
		MirrorInstancePtr::new(Self {
			header: Header::new(),
			monitor: Arc::new(Monitor::new()),
			class: mirror_class,
			fields,
			target: MirrorTarget::Class(target),
		})
	}

	pub fn new_array(target: &'static Class) -> MirrorInstanceRef {
		let mirror_class = crate::globals::classes::java_lang_Class();
		let fields =
			Self::initialize_fields(mirror_class, target, target.access_flags().as_u2(), false);

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

		let component_type_offset =
			crate::globals::fields::java_lang_Class::componentType_field_offset();
		unsafe {
			*fields[component_type_offset].get() = Operand::Reference(component_type_mirror);
		}

		MirrorInstancePtr::new(Self {
			header: Header::new(),
			monitor: Arc::new(Monitor::new()),
			class: mirror_class,
			fields,
			target: MirrorTarget::Class(target),
		})
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
			!matches!(target, FieldType::Array(_) | FieldType::Object(_)),
			"`Array` and `Object` field types are incompatible with the primitive mirror"
		);

		let mirror_class = crate::globals::classes::java_lang_Class();
		let target_class = Self::target_for_primitive(&target);

		// TODO: Are these modifiers correct?
		let fields = Self::initialize_fields(mirror_class, target_class, 1, true);
		MirrorInstancePtr::new(Self {
			header: Header::new(),
			monitor: Arc::new(Monitor::new()),
			class: mirror_class,
			fields,
			target: MirrorTarget::Primitive(target),
		})
	}

	pub fn is_primitive(&self) -> bool {
		matches!(&self.target, MirrorTarget::Primitive(_))
	}

	pub fn is_array(&self) -> bool {
		matches!(&self.target, MirrorTarget::Class(class) if class.is_array())
	}

	/// The backing class of this mirror
	///
	/// This is always `java.lang.Class`, and is only really useful as a marker.
	///
	/// In the following:
	///
	/// ```java
	/// var c = String.class;
	/// ```
	///
	/// `c` is an instance of `Class<?>`, which this represents.
	///
	/// To get the class that this mirror is targeting (in this case, `java.lang.String`), use [`MirrorInstance::target_class`].
	pub fn class(&self) -> &'static Class {
		self.class
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
	pub fn target_class(&self) -> &'static Class {
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

	pub fn set_module(&self, module: Reference) {
		let module_offset = crate::globals::fields::java_lang_Class::module_field_offset();
		let ptr = self.fields[module_offset].get();

		unsafe {
			assert!(
				(&*ptr).expect_reference().is_null(),
				"Attempt to set a module twice"
			);
		}

		// Early in initialization, even before java.lang.Module is loaded, this will
		// be called with a null reference. Since the mirror is already default initialized with null,
		// there's nothing to do. Return early to preserve the assertion below.
		if module.is_null() {
			return;
		}

		assert!(module.is_instance_of(crate::globals::classes::java_lang_Module()));

		unsafe {
			*ptr = Operand::Reference(module);
		}
	}

	pub fn set_class_data(&self, class_data: Reference) {
		let class_data_offset = crate::globals::fields::java_lang_Class::classData_field_offset();
		let ptr = self.fields[class_data_offset].get();

		unsafe {
			*ptr = Operand::Reference(class_data);
		}
	}

	fn target_for_primitive(primitive: &FieldType) -> &'static Class {
		match primitive {
			FieldType::Byte => crate::globals::classes::java_lang_Byte(),
			FieldType::Char => crate::globals::classes::java_lang_Character(),
			FieldType::Double => crate::globals::classes::java_lang_Double(),
			FieldType::Float => crate::globals::classes::java_lang_Float(),
			FieldType::Int => crate::globals::classes::java_lang_Integer(),
			FieldType::Long => crate::globals::classes::java_lang_Long(),
			FieldType::Short => crate::globals::classes::java_lang_Short(),
			FieldType::Boolean => crate::globals::classes::java_lang_Boolean(),
			FieldType::Void => crate::globals::classes::java_lang_Void(),
			_ => unreachable!("only primitive types should exist within primitive mirrors"),
		}
	}

	fn initialize_fields(
		mirror_class: &'static Class,
		target_class: &'static Class,
		modifiers: jchar,
		is_primitive: bool,
	) -> Box<[UnsafeCell<Operand<Reference>>]> {
		let instance_field_count = mirror_class.instance_field_count();

		// Set the default values for our non-static fields
		let mut fields = Vec::with_capacity(instance_field_count);
		for field in mirror_class.instance_fields() {
			fields.push(UnsafeCell::new(Field::default_value_for_ty(
				&field.descriptor,
			)))
		}

		let class_loader_offset =
			crate::globals::fields::java_lang_Class::classLoader_field_offset();
		let modifiers_offset = crate::globals::fields::java_lang_Class::modifiers_field_offset();
		let primitive_offset = crate::globals::fields::java_lang_Class::primitive_field_offset();
		unsafe {
			*fields[class_loader_offset].get() = Operand::Reference(target_class.loader().obj());
			*fields[modifiers_offset].get() = Operand::Int(modifiers as jint);
			*fields[primitive_offset].get() = Operand::Int(is_primitive as jint);
		}

		fields.into_boxed_slice()
	}
}

impl Instance for MirrorInstance {
	fn header(&self) -> &Header {
		&self.header
	}

	fn monitor(&self) -> Arc<Monitor> {
		self.monitor.clone()
	}

	fn get_field_value(&self, field: &Field) -> Operand<Reference> {
		self.get_field_value0(field.index())
	}

	fn get_field_value0(&self, field_idx: usize) -> Operand<Reference> {
		if field_idx >= self.fields.len() {
			panic!(
				"Failed to resolve field index: {:?}, in class: {:?}",
				field_idx, self.class
			);
		}

		let ptr = self.fields[field_idx].get();
		let value = unsafe { &*ptr };
		value.clone()
	}

	fn put_field_value(&mut self, field: &Field, value: Operand<Reference>) {
		self.put_field_value0(field.index(), value)
	}

	fn put_field_value0(&mut self, field_idx: usize, value: Operand<Reference>) {
		if field_idx >= self.fields.len() {
			panic!(
				"Failed to resolve field index: {:?}, in class: {:?}",
				field_idx, self.class
			);
		}

		let ptr = self.fields[field_idx].get();
		let current = unsafe { &*ptr };
		assert!(
			current.is_compatible_with(&value),
			"Expected type compatible with: {:?}, found: {:?}",
			current,
			value
		);

		unsafe {
			*ptr = value;
		}
	}

	unsafe fn get_field_value_raw(&self, field_idx: usize) -> NonNull<Operand<Reference>> {
		assert!(field_idx < self.fields.len());
		NonNull::new_unchecked(self.fields.as_ptr().offset(field_idx as isize) as _)
	}
}

// A pointer to a MirrorInstance
//
// This can *not* be constructed by hand, as dropping it will
// deallocate the instance.
#[derive(PartialEq)]
pub struct MirrorInstancePtr(usize);

impl PtrType<MirrorInstance, MirrorInstanceRef> for MirrorInstancePtr {
	fn new(val: MirrorInstance) -> MirrorInstanceRef {
		let boxed = Box::new(val);
		let ptr = Box::into_raw(boxed);
		MirrorInstanceRef::new(Self(ptr as usize))
	}

	#[inline(always)]
	fn as_raw(&self) -> *const MirrorInstance {
		self.0 as *const MirrorInstance
	}

	#[inline(always)]
	fn as_mut_raw(&self) -> *mut MirrorInstance {
		self.0 as *mut MirrorInstance
	}

	fn get(&self) -> &MirrorInstance {
		unsafe { &(*self.as_raw()) }
	}

	fn get_mut(&self) -> &mut MirrorInstance {
		unsafe { &mut (*self.as_mut_raw()) }
	}
}

impl Drop for MirrorInstancePtr {
	fn drop(&mut self) {
		let _ = unsafe { Box::from_raw(self.0 as *mut MirrorInstance) };
	}
}

impl Debug for MirrorInstancePtr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let instance = self.get();
		write!(f, "{:?}", instance)
	}
}
