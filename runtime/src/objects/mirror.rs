use super::instance::{Header, Instance};
use crate::objects::class::Class;
use crate::objects::field::Field;
use crate::objects::reference::{MirrorInstanceRef, Reference};

use std::cell::UnsafeCell;
use std::fmt::{Debug, Formatter};
use std::ptr::NonNull;

use classfile::FieldType;
use common::traits::PtrType;
use instructions::Operand;

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
		let fields = Self::initialize_fields(mirror_class, target);
		MirrorInstancePtr::new(Self {
			header: Header::new(),
			class: mirror_class,
			fields,
			target: MirrorTarget::Class(target),
		})
	}

	pub fn new_array(target: &'static Class) -> MirrorInstanceRef {
		let mirror_class = crate::globals::classes::java_lang_Class();
		let fields = Self::initialize_fields(mirror_class, target);
		MirrorInstancePtr::new(Self {
			header: Header::new(),
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

		let fields = Self::initialize_fields(mirror_class, target_class);
		MirrorInstancePtr::new(Self {
			header: Header::new(),
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

	pub fn set_module(&self, module: Reference) {
		let module_offset = crate::globals::fields::java_lang_Class::module_field_offset();
		let ptr = self.fields[module_offset].get();
		unsafe {
			assert!(
				(&*ptr).expect_reference().is_null(),
				"Attempt to set a module twice"
			);
			*ptr = Operand::Reference(module);
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
		unsafe {
			*fields[class_loader_offset].get() = Operand::Reference(target_class.loader().obj());
		}

		fields.into_boxed_slice()
	}
}

impl Instance for MirrorInstance {
	fn header(&self) -> &Header {
		&self.header
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
