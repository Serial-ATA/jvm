#[cfg(test)]
mod tests;

use crate::classpath::loader::ClassLoader;
use crate::objects::class::ClassPtr;
use crate::objects::instance::array::Array;
use crate::objects::instance::object::Object;
use crate::objects::instance::{CloneableInstance, Header};
use crate::objects::reference::Reference;
use crate::symbols::{Symbol, sym};
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw};

use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::{fmt, slice};

use common::int_types::{s4, u1};
use instructions::Operand;
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TypeCode {
	Boolean = 4,
	Char = 5,
	Float = 6,
	Double = 7,
	Byte = 8,
	Short = 9,
	Int = 10,
	Long = 11,
}

impl TypeCode {
	pub fn from_u8(val: u8) -> Self {
		match val {
			4 => Self::Boolean,
			5 => Self::Char,
			6 => Self::Float,
			7 => Self::Double,
			8 => Self::Byte,
			9 => Self::Short,
			10 => Self::Int,
			11 => Self::Long,
			_ => panic!("Invalid array type code: {val}"),
		}
	}

	pub fn size(self) -> usize {
		match self {
			TypeCode::Boolean => size_of::<jboolean>(),
			TypeCode::Char => size_of::<jchar>(),
			TypeCode::Float => size_of::<jfloat>(),
			TypeCode::Double => size_of::<jdouble>(),
			TypeCode::Byte => size_of::<jbyte>(),
			TypeCode::Short => size_of::<jshort>(),
			TypeCode::Int => size_of::<jint>(),
			TypeCode::Long => size_of::<jlong>(),
		}
	}

	pub fn align(self) -> usize {
		match self {
			TypeCode::Boolean => align_of::<jboolean>(),
			TypeCode::Char => align_of::<jchar>(),
			TypeCode::Float => align_of::<jfloat>(),
			TypeCode::Double => align_of::<jdouble>(),
			TypeCode::Byte => align_of::<jbyte>(),
			TypeCode::Short => align_of::<jshort>(),
			TypeCode::Int => align_of::<jint>(),
			TypeCode::Long => align_of::<jlong>(),
		}
	}

	fn array_signature(self) -> Symbol {
		match self {
			TypeCode::Boolean => sym!(boolean_array),
			TypeCode::Char => sym!(character_array),
			TypeCode::Float => sym!(float_array),
			TypeCode::Double => sym!(double_array),
			TypeCode::Byte => sym!(byte_array),
			TypeCode::Short => sym!(short_array),
			TypeCode::Int => sym!(integer_array),
			TypeCode::Long => sym!(long_array),
		}
	}

	fn array_class(self) -> ClassPtr {
		match self {
			TypeCode::Boolean => crate::globals::classes::boolean_array(),
			TypeCode::Byte => crate::globals::classes::byte_array(),
			TypeCode::Char => crate::globals::classes::character_array(),
			TypeCode::Double => crate::globals::classes::double_array(),
			TypeCode::Float => crate::globals::classes::float_array(),
			TypeCode::Int => crate::globals::classes::integer_array(),
			TypeCode::Long => crate::globals::classes::long_array(),
			TypeCode::Short => crate::globals::classes::short_array(),
		}
	}
}

/// Reference to an allocated primitive array
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PrimitiveArrayInstanceRef(*mut PrimitiveArrayInstance);

impl PrimitiveArrayInstanceRef {
	pub fn as_slice<T: PrimitiveType>(&self) -> &[T] {
		assert_eq!(T::TYPE_CODE, self.ty);

		// SAFETY: We just verified that the type is correct
		unsafe {
			slice::from_raw_parts::<T>(
				self.field_base().cast::<T>().cast_const(),
				self.length as usize,
			)
		}
	}

	pub fn as_bytes(&self) -> &[u8] {
		// SAFETY: Every primtive type can be represented as bytes
		unsafe { slice::from_raw_parts(self.field_base().cast_const(), self.length as usize) }
	}

	// TODO: Remove this, not sound
	pub fn as_bytes_mut(&self) -> &mut [u8] {
		// SAFETY: Every primitive type can be represented as bytes
		unsafe { slice::from_raw_parts_mut(self.field_base().cast::<u8>(), self.length as usize) }
	}
}

impl Object for PrimitiveArrayInstanceRef {
	type Descriptor = PrimitiveArrayInstance;

	fn hash(&self, thread: &'static JavaThread) -> jint {
		self.header.generate_hash(thread)
	}

	fn class(&self) -> ClassPtr {
		unsafe { (&*self.0).class }
	}

	fn is_primitive_array(&self) -> bool {
		true
	}

	unsafe fn raw(&self) -> *mut () {
		self.0.cast()
	}

	unsafe fn field_base(&self) -> *mut u8 {
		let base = self.0.cast::<u8>();
		unsafe { base.add(Self::BASE_OFFSET) }
	}
}

impl PartialEq for PrimitiveArrayInstanceRef {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl Deref for PrimitiveArrayInstanceRef {
	type Target = PrimitiveArrayInstance;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0 }
	}
}

impl Debug for PrimitiveArrayInstanceRef {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.deref().fmt(f)
	}
}

/// An instance of a primitive array
#[derive(Debug)]
pub struct PrimitiveArrayInstance {
	header: Header,
	class: ClassPtr,
	length: u32,
	ty: TypeCode,
}

impl CloneableInstance for PrimitiveArrayInstanceRef {
	type ReferenceTy = PrimitiveArrayInstanceRef;

	unsafe fn clone(&self) -> Self::ReferenceTy {
		let cloned_instance =
			PrimitiveArrayInstance::new_from_type(self.ty as _, self.length as s4).expect("oom");

		// SAFETY: All primitive types are `Copy`
		unsafe {
			self.copy_into(0, &cloned_instance, 0, self.length as usize);
		}

		cloned_instance
	}
}

// Marker trait for Java primitive types, indicating they are safe to zero-initialize
pub trait PrimitiveType: Copy + fmt::Debug {
	const TYPE_CODE: TypeCode;
}

impl PrimitiveType for jboolean {
	const TYPE_CODE: TypeCode = TypeCode::Boolean;
}
impl PrimitiveType for jchar {
	const TYPE_CODE: TypeCode = TypeCode::Char;
}
impl PrimitiveType for jfloat {
	const TYPE_CODE: TypeCode = TypeCode::Float;
}
impl PrimitiveType for jdouble {
	const TYPE_CODE: TypeCode = TypeCode::Double;
}
impl PrimitiveType for jbyte {
	const TYPE_CODE: TypeCode = TypeCode::Byte;
}
impl PrimitiveType for jshort {
	const TYPE_CODE: TypeCode = TypeCode::Short;
}
impl PrimitiveType for jint {
	const TYPE_CODE: TypeCode = TypeCode::Int;
}
impl PrimitiveType for jlong {
	const TYPE_CODE: TypeCode = TypeCode::Long;
}

impl PrimitiveArrayInstance {
	// TODO: Should throw on OOM
	/// Convenience constructor for allocating an array and copying `elements` into it
	pub fn new<T>(elements: Box<[T]>) -> PrimitiveArrayInstanceRef
	where
		T: PrimitiveType,
	{
		let array = PrimitiveArrayInstance::new_from_type(T::TYPE_CODE as _, elements.len() as _)
			.expect("valid");

		{
			// SAFETY: Array is guaranteed to be allocated with T's type code
			let new_array_slice = unsafe {
				slice::from_raw_parts_mut::<T>(array.field_base().cast::<T>(), elements.len())
			};
			new_array_slice.copy_from_slice(&*elements);
		}

		array
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.newarray
	pub fn new_from_type(type_code: u1, count: s4) -> Throws<PrimitiveArrayInstanceRef> {
		if count.is_negative() {
			throw!(@DEFER NegativeArraySizeException);
		}

		let type_code = TypeCode::from_u8(type_code);
		let array_signature = type_code.array_signature();
		let array_class = ClassLoader::bootstrap().load(array_signature)?;

		let descriptor = PrimitiveArrayInstance {
			header: Header::new(),
			class: array_class,
			length: count as u32,
			ty: type_code,
		};
		let array_size = type_code.size() * count as usize;

		let array_ptr = unsafe { PrimitiveArrayInstanceRef::allocate(descriptor, array_size) };

		Throws::Ok(PrimitiveArrayInstanceRef(array_ptr))
	}
}

impl Array for PrimitiveArrayInstanceRef {
	type Component = Operand<Reference>;

	const BASE_OFFSET: usize = size_of::<PrimitiveArrayInstance>();

	#[inline]
	fn len(&self) -> usize {
		self.length as usize
	}

	fn align(&self) -> usize {
		self.ty.align()
	}

	#[inline]
	fn scale(&self) -> usize {
		self.ty.size()
	}

	unsafe fn get_unchecked(&self, index: usize) -> Self::Component {
		// Object::get() operates with byte offsets
		let offset = index * self.scale();

		// SAFETY: Caller verified that `index` is within bounds
		unsafe {
			match self.ty {
				TypeCode::Boolean => Operand::from(Object::get::<jboolean>(self, offset)),
				TypeCode::Char => Operand::from(Object::get::<jchar>(self, offset)),
				TypeCode::Float => Operand::from(Object::get::<jfloat>(self, offset)),
				TypeCode::Double => Operand::from(Object::get::<jdouble>(self, offset)),
				TypeCode::Byte => Operand::from(Object::get::<jbyte>(self, offset)),
				TypeCode::Short => Operand::from(Object::get::<jshort>(self, offset)),
				TypeCode::Int => Operand::from(Object::get::<jint>(self, offset)),
				TypeCode::Long => Operand::from(Object::get::<jlong>(self, offset)),
			}
		}
	}

	unsafe fn store_unchecked(&self, index: usize, value: Self::Component) {
		// Object::put() operates with byte offsets
		let offset = index * self.scale();

		// SAFETY: Caller verified that `index` is within bounds
		unsafe {
			match value {
				Operand::Int(val) => match self.ty {
					TypeCode::Boolean => {
						Object::put::<jboolean>(self, val != 0, offset);
					},
					TypeCode::Byte => {
						Object::put::<jbyte>(self, val as jbyte, offset);
					},
					TypeCode::Short => {
						Object::put::<jshort>(self, val as jshort, offset);
					},
					TypeCode::Char => {
						Object::put::<jchar>(self, val as jchar, offset);
					},
					TypeCode::Int => {
						Object::put::<jint>(self, val, offset);
					},
					_ => unreachable!(),
				},
				Operand::Float(val) => {
					Object::put::<jfloat>(self, val, offset);
				},
				Operand::Double(val) => {
					Object::put::<jdouble>(self, val, offset);
				},
				Operand::Long(val) => {
					Object::put::<jlong>(self, val, offset);
				},
				_ => unreachable!(),
			}
		}
	}

	unsafe fn copy_into(&self, src_pos: usize, dest: &Self, dest_pos: usize, length: usize) {
		// This will only check the type of `self`, the safety conditions of this method require that the caller
		// verify these arrays to be of the same type.
		match self.ty {
			TypeCode::Boolean => unsafe {
				let src_ptr = (self.field_base() as *mut jboolean).add(src_pos);
				let dest_ptr = (dest.field_base() as *mut jboolean).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
			TypeCode::Char => unsafe {
				let src_ptr = (self.field_base() as *mut jchar).add(src_pos);
				let dest_ptr = (dest.field_base() as *mut jchar).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
			TypeCode::Float => unsafe {
				let src_ptr = (self.field_base() as *mut jfloat).add(src_pos);
				let dest_ptr = (dest.field_base() as *mut jfloat).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
			TypeCode::Double => unsafe {
				let src_ptr = (self.field_base() as *mut jdouble).add(src_pos);
				let dest_ptr = (dest.field_base() as *mut jdouble).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
			TypeCode::Byte => unsafe {
				let src_ptr = (self.field_base() as *mut jbyte).add(src_pos);
				let dest_ptr = (dest.field_base() as *mut jbyte).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
			TypeCode::Short => unsafe {
				let src_ptr = (self.field_base() as *mut jshort).add(src_pos);
				let dest_ptr = (dest.field_base() as *mut jshort).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
			TypeCode::Int => unsafe {
				let src_ptr = (self.field_base() as *mut jint).add(src_pos);
				let dest_ptr = (dest.field_base() as *mut jint).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
			TypeCode::Long => unsafe {
				let src_ptr = (self.field_base() as *mut jlong).add(src_pos);
				let dest_ptr = (dest.field_base() as *mut jlong).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
		}
	}

	unsafe fn copy_within(&self, src_pos: usize, dest_pos: usize, length: usize) {
		// This will only check the type of `self`, the safety conditions of this method require that the caller
		// verify these arrays to be of the same type.
		match self.ty {
			TypeCode::Boolean => unsafe {
				let src_ptr = (self.field_base() as *mut jboolean).add(src_pos);
				let dest_ptr = (self.field_base() as *mut jboolean).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
			TypeCode::Char => unsafe {
				let src_ptr = (self.field_base() as *mut jchar).add(src_pos);
				let dest_ptr = (self.field_base() as *mut jchar).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
			TypeCode::Float => unsafe {
				let src_ptr = (self.field_base() as *mut jfloat).add(src_pos);
				let dest_ptr = (self.field_base() as *mut jfloat).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
			TypeCode::Double => unsafe {
				let src_ptr = (self.field_base() as *mut jdouble).add(src_pos);
				let dest_ptr = (self.field_base() as *mut jdouble).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
			TypeCode::Byte => unsafe {
				let src_ptr = (self.field_base() as *mut jbyte).add(src_pos);
				let dest_ptr = (self.field_base() as *mut jbyte).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
			TypeCode::Short => unsafe {
				let src_ptr = (self.field_base() as *mut jshort).add(src_pos);
				let dest_ptr = (self.field_base() as *mut jshort).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
			TypeCode::Int => unsafe {
				let src_ptr = (self.field_base() as *mut jint).add(src_pos);
				let dest_ptr = (self.field_base() as *mut jint).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
			TypeCode::Long => unsafe {
				let src_ptr = (self.field_base() as *mut jlong).add(src_pos);
				let dest_ptr = (self.field_base() as *mut jlong).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
		}
	}
}
