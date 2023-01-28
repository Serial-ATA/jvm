use crate::classpath::classloader::ClassLoader;
use crate::field::Field;
use crate::reference::{
	ArrayInstanceRef, ClassInstanceRef, ClassRef, FieldRef, MirrorInstanceRef, Reference,
};

use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use classfile::FieldType;
use common::int_types::{s1, s2, s4, s8, u1, u2};
use common::traits::PtrType;
use instructions::Operand;

pub trait Instance {
	fn get_field_value(&self, field: FieldRef) -> Operand<Reference>;
	fn get_field_value0(&self, field_idx: usize) -> Operand<Reference>;
	fn put_field_value(&mut self, field: FieldRef, value: Operand<Reference>);
	fn put_field_value0(&mut self, field_idx: usize, value: Operand<Reference>);
}

#[derive(Debug, Clone, PartialEq)]
enum MirrorTarget {
	Class(ClassRef),
	Primitive(FieldType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirrorInstance {
	pub class: ClassRef,
	pub fields: Box<[Operand<Reference>]>,
	target: MirrorTarget,
}

impl MirrorInstance {
	pub fn new(mirror_class: ClassRef, target: ClassRef) -> MirrorInstanceRef {
		let fields = Self::initialize_fields(Arc::clone(&mirror_class));
		MirrorInstancePtr::new(Self {
			class: mirror_class,
			fields,
			target: MirrorTarget::Class(target),
		})
	}

	pub fn new_array(mirror_class: ClassRef, target: ClassRef) -> MirrorInstanceRef {
		let fields = Self::initialize_fields(Arc::clone(&mirror_class));
		MirrorInstancePtr::new(Self {
			class: mirror_class,
			fields,
			target: MirrorTarget::Class(target),
		})
	}

	pub fn new_primitive(mirror_class: ClassRef, target: FieldType) -> MirrorInstanceRef {
		let fields = Self::initialize_fields(Arc::clone(&mirror_class));
		MirrorInstancePtr::new(Self {
			class: mirror_class,
			fields,
			target: MirrorTarget::Primitive(target),
		})
	}

	pub fn expect_class(&self) -> ClassRef {
		match &self.target {
			MirrorTarget::Class(class) => Arc::clone(class),
			_ => panic!("Expected mirror instance to point to class!"),
		}
	}

	pub fn expect_primitive(&self) -> FieldType {
		match &self.target {
			MirrorTarget::Primitive(primitive) => primitive.clone(),
			_ => panic!("Expected mirror instance to point to primitive!"),
		}
	}

	fn initialize_fields(mirror_class: ClassRef) -> Box<[Operand<Reference>]> {
		let class_instance = mirror_class.unwrap_class_instance();
		let instance_field_count = class_instance.instance_field_count;

		// Set the default values for our non-static fields
		let mut fields = Vec::with_capacity(instance_field_count as usize);
		for field in class_instance
			.fields
			.iter()
			.filter(|field| !field.is_static())
		{
			fields.push(Field::default_value_for_ty(&field.descriptor))
		}

		fields.into_boxed_slice()
	}
}

impl Instance for MirrorInstance {
	fn get_field_value(&self, field: FieldRef) -> Operand<Reference> {
		self.get_field_value0(field.idx)
	}

	fn get_field_value0(&self, field_idx: usize) -> Operand<Reference> {
		if field_idx >= self.fields.len() {
			panic!(
				"Failed to resolve field index: {:?}, in class: {:?}",
				field_idx, self.class
			);
		}

		return self.fields[field_idx].clone();
	}

	fn put_field_value(&mut self, field: FieldRef, value: Operand<Reference>) {
		self.put_field_value0(field.idx, value)
	}

	fn put_field_value0(&mut self, field_idx: usize, value: Operand<Reference>) {
		if field_idx >= self.fields.len() {
			panic!(
				"Failed to resolve field index: {:?}, in class: {:?}",
				field_idx, self.class
			);
		}

		self.fields[field_idx] = value;
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

#[derive(Debug, Clone, PartialEq)]
pub struct ClassInstance {
	pub super_class: Option<ClassInstanceRef>,
	pub class: ClassRef,
	pub fields: Box<[Operand<Reference>]>,
}

impl ClassInstance {
	pub fn new(class: ClassRef) -> ClassInstanceRef {
		let class_instance = class.unwrap_class_instance();
		let instance_field_count = class_instance.instance_field_count;

		let mut super_class = None;
		if let Some(ref super_class_) = class.get().super_class {
			super_class = Some(Self::new(Arc::clone(super_class_)));
		}

		// Set the default values for our non-static fields
		let mut fields = Vec::with_capacity(instance_field_count as usize);
		for field in class_instance
			.fields
			.iter()
			.filter(|field| !field.is_static())
		{
			fields.push(Field::default_value_for_ty(&field.descriptor))
		}

		// Sanity check
		assert_eq!(
			instance_field_count as usize,
			fields.len(),
			"Created the wrong number of fields!"
		);

		ClassInstancePtr::new(Self {
			super_class,
			class,
			fields: fields.into_boxed_slice(),
		})
	}
}

impl Instance for ClassInstance {
	fn get_field_value(&self, field: FieldRef) -> Operand<Reference> {
		self.get_field_value0(field.idx)
	}

	fn get_field_value0(&self, field_idx: usize) -> Operand<Reference> {
		let mut count = 0;

		let mut current_class = &self.class;
		loop {
			count += current_class.unwrap_class_instance().instance_field_count as usize;
			if count > field_idx {
				return self.fields[field_idx].clone();
			}

			if let Some(ref super_class) = current_class.get().super_class {
				current_class = super_class;
				continue;
			}

			break;
		}

		panic!(
			"Failed to resolve field index: {:?}, in class: {:?}",
			field_idx, self.class
		);
	}

	fn put_field_value(&mut self, field: FieldRef, value: Operand<Reference>) {
		self.put_field_value0(field.idx, value)
	}

	fn put_field_value0(&mut self, field_idx: usize, value: Operand<Reference>) {
		let mut count = 0;

		let mut current_class = &self.class;
		loop {
			count += current_class.unwrap_class_instance().instance_field_count as usize;
			if count > field_idx {
				self.fields[field_idx] = value;
				return;
			}

			if let Some(ref super_class) = current_class.get().super_class {
				current_class = super_class;
				continue;
			}

			break;
		}

		panic!(
			"Failed to resolve field index: {:?}, in class: {:?}",
			field_idx, self.class
		);
	}
}

// A pointer to a ClassInstance
//
// This can *not* be constructed by hand, as dropping it will
// deallocate the instance.
#[derive(PartialEq)]
pub struct ClassInstancePtr(usize);

impl PtrType<ClassInstance, ClassInstanceRef> for ClassInstancePtr {
	fn new(val: ClassInstance) -> ClassInstanceRef {
		let boxed = Box::new(val);
		let ptr = Box::into_raw(boxed);
		ClassInstanceRef::new(Self(ptr as usize))
	}

	#[inline(always)]
	fn as_raw(&self) -> *const ClassInstance {
		self.0 as *const ClassInstance
	}

	#[inline(always)]
	fn as_mut_raw(&self) -> *mut ClassInstance {
		self.0 as *mut ClassInstance
	}

	fn get(&self) -> &ClassInstance {
		unsafe { &(*self.as_raw()) }
	}

	fn get_mut(&self) -> &mut ClassInstance {
		unsafe { &mut (*self.as_mut_raw()) }
	}
}

impl Drop for ClassInstancePtr {
	fn drop(&mut self) {
		let _ = unsafe { Box::from_raw(self.0 as *mut ClassInstance) };
	}
}

impl Debug for ClassInstancePtr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let class = self.get();
		f.write_str(unsafe { std::str::from_utf8_unchecked(&class.class.get().name) })
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayInstance {
	pub class: ClassRef,
	pub elements: ArrayContent,
}

impl ArrayInstance {
	pub fn new(class: ClassRef, elements: ArrayContent) -> ArrayInstanceRef {
		ArrayInstancePtr::new(Self { class, elements })
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.5.newarray
	pub fn new_from_type(type_code: u1, count: s4) -> ArrayInstanceRef {
		if count.is_negative() {
			panic!("NegativeArraySizeException"); // TODO
		}

		let type_character = match type_code {
			4 => b'Z',
			5 => b'C',
			6 => b'F',
			7 => b'D',
			8 => b'B',
			9 => b'S',
			10 => b'I',
			11 => b'J',
			_ => panic!("Invalid array type code: {}", type_code),
		};

		let array_class = ClassLoader::Bootstrap
			.load(&[b'[', type_character])
			.unwrap();
		let elements = ArrayContent::default_initialize(type_code, count);

		ArrayInstancePtr::new(Self {
			class: array_class,
			elements,
		})
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.5.anewarray
	pub fn new_reference(count: s4, array_class: ClassRef) -> ArrayInstanceRef {
		if count.is_negative() {
			panic!("NegativeArraySizeException"); // TODO
		}

		let elements = vec![Reference::Null; count as usize].into_boxed_slice();
		ArrayInstancePtr::new(Self {
			class: array_class,
			elements: ArrayContent::Reference(elements),
		})
	}

	pub fn get(&self, index: s4) -> Operand<Reference> {
		if index.is_negative() || index as usize > self.elements.element_count() {
			panic!("ArrayIndexOutOfBoundsException"); // TODO
		}

		self.elements.get(index as usize)
	}

	pub fn store(&mut self, index: s4, value: Operand<Reference>) {
		if index.is_negative() || index as usize > self.elements.element_count() {
			panic!("ArrayIndexOutOfBoundsException"); // TODO
		}

		let index = index as usize;
		match self.elements {
			ArrayContent::Byte(ref mut contents) => contents[index] = value.expect_int() as s1,
			ArrayContent::Bool(ref mut contents) => {
				contents[index] = (value.expect_int() & 1) as s1
			},
			ArrayContent::Short(ref mut contents) => contents[index] = value.expect_int() as s2,
			ArrayContent::Char(ref mut contents) => contents[index] = value.expect_int() as u2,
			ArrayContent::Int(ref mut contents) => contents[index] = value.expect_int(),
			ArrayContent::Float(ref mut contents) => contents[index] = value.expect_float(),
			ArrayContent::Double(ref mut contents) => contents[index] = value.expect_double(),
			ArrayContent::Long(ref mut contents) => contents[index] = value.expect_long(),
			ArrayContent::Reference(ref mut contents) => contents[index] = value.expect_reference(),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrayContent {
	Byte(Box<[s1]>),
	Bool(Box<[s1]>),
	Short(Box<[s2]>),
	Char(Box<[u2]>),
	Int(Box<[s4]>),
	Float(Box<[f32]>),
	Double(Box<[f64]>),
	Long(Box<[s8]>),
	Reference(Box<[Reference]>),
}

impl ArrayContent {
	fn default_initialize(type_code: u1, count: s4) -> Self {
		match type_code {
			4 => Self::Bool(vec![0; count as usize].into_boxed_slice()),
			5 => Self::Char(vec![0; count as usize].into_boxed_slice()),
			6 => Self::Float(vec![0.; count as usize].into_boxed_slice()),
			7 => Self::Double(vec![0.; count as usize].into_boxed_slice()),
			8 => Self::Byte(vec![0; count as usize].into_boxed_slice()),
			9 => Self::Short(vec![0; count as usize].into_boxed_slice()),
			10 => Self::Int(vec![0; count as usize].into_boxed_slice()),
			11 => Self::Long(vec![0; count as usize].into_boxed_slice()),
			_ => panic!("Invalid array type code: {}", type_code),
		}
	}

	fn get(&self, index: usize) -> Operand<Reference> {
		match self {
			ArrayContent::Byte(content) | ArrayContent::Bool(content) => {
				Operand::Int(s4::from(content[index]))
			},
			ArrayContent::Short(content) => Operand::Int(s4::from(content[index])),
			ArrayContent::Char(content) => Operand::Int(s4::from(content[index])),
			ArrayContent::Int(content) => Operand::Int(content[index]),
			ArrayContent::Float(content) => Operand::Float(content[index]),
			ArrayContent::Double(content) => Operand::Double(content[index]),
			ArrayContent::Long(content) => Operand::Long(content[index]),
			ArrayContent::Reference(content) => Operand::Reference(content[index].clone()),
		}
	}

	pub fn element_count(&self) -> usize {
		match self {
			ArrayContent::Byte(content) | ArrayContent::Bool(content) => content.len(),
			ArrayContent::Short(content) => content.len(),
			ArrayContent::Char(content) => content.len(),
			ArrayContent::Int(content) => content.len(),
			ArrayContent::Float(content) => content.len(),
			ArrayContent::Double(content) => content.len(),
			ArrayContent::Long(content) => content.len(),
			ArrayContent::Reference(content) => content.len(),
		}
	}

	pub fn expect_byte(&self) -> &[i8] {
		match self {
			ArrayContent::Byte(bytes) => bytes,
			_ => panic!("Expected an array of type `byte`"),
		}
	}
}

// A pointer to a ArrayInstance
//
// This can *not* be constructed by hand, as dropping it will
// deallocate the instance.
#[derive(PartialEq)]
pub struct ArrayInstancePtr(usize);

impl PtrType<ArrayInstance, ArrayInstanceRef> for ArrayInstancePtr {
	fn new(val: ArrayInstance) -> ArrayInstanceRef {
		let boxed = Box::new(val);
		let ptr = Box::into_raw(boxed);
		ArrayInstanceRef::new(Self(ptr as usize))
	}

	#[inline(always)]
	fn as_raw(&self) -> *const ArrayInstance {
		self.0 as *const ArrayInstance
	}

	#[inline(always)]
	fn as_mut_raw(&self) -> *mut ArrayInstance {
		self.0 as *mut ArrayInstance
	}

	fn get(&self) -> &ArrayInstance {
		unsafe { &(*self.as_raw()) }
	}

	fn get_mut(&self) -> &mut ArrayInstance {
		unsafe { &mut (*self.as_mut_raw()) }
	}
}

impl Drop for ArrayInstancePtr {
	fn drop(&mut self) {
		let _ = unsafe { Box::from_raw(self.0 as *mut ArrayInstance) };
	}
}

impl Debug for ArrayInstancePtr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let class = self.get();
		f.write_str(unsafe { std::str::from_utf8_unchecked(&class.class.get().name) })
	}
}
