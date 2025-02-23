use crate::classpath::loader::ClassLoader;
use crate::objects::class::Class;
use crate::objects::instance::{CloneableInstance, Header};
use crate::objects::monitor::Monitor;
use crate::objects::reference::{PrimitiveArrayInstanceRef, Reference};
use crate::symbols::{sym, Symbol};
use crate::thread::exceptions::{throw, Throws};

use std::alloc::{alloc_zeroed, Layout};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::{fmt, ptr, slice};

use common::int_types::{s4, u1};
use common::traits::PtrType;
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

	fn array_signature(self) -> Symbol {
		match self {
			TypeCode::Boolean => sym!(bool_array),
			TypeCode::Char => sym!(char_array),
			TypeCode::Float => sym!(float_array),
			TypeCode::Double => sym!(double_array),
			TypeCode::Byte => sym!(byte_array),
			TypeCode::Short => sym!(short_array),
			TypeCode::Int => sym!(int_array),
			TypeCode::Long => sym!(long_array),
		}
	}

	fn array_class(self) -> &'static Class {
		match self {
			TypeCode::Boolean => crate::globals::classes::bool_array(),
			TypeCode::Byte => crate::globals::classes::byte_array(),
			TypeCode::Char => crate::globals::classes::char_array(),
			TypeCode::Double => crate::globals::classes::double_array(),
			TypeCode::Float => crate::globals::classes::float_array(),
			TypeCode::Int => crate::globals::classes::int_array(),
			TypeCode::Long => crate::globals::classes::long_array(),
			TypeCode::Short => crate::globals::classes::short_array(),
		}
	}

	fn alloc_zeroed(self, count: usize) -> *mut u8 {
		match self {
			TypeCode::Boolean => alloc_zeroed_array::<jboolean>(count),
			TypeCode::Char => alloc_zeroed_array::<jchar>(count),
			TypeCode::Float => alloc_zeroed_array::<jfloat>(count),
			TypeCode::Double => alloc_zeroed_array::<jdouble>(count),
			TypeCode::Byte => alloc_zeroed_array::<jbyte>(count),
			TypeCode::Short => alloc_zeroed_array::<jshort>(count),
			TypeCode::Int => alloc_zeroed_array::<jint>(count),
			TypeCode::Long => alloc_zeroed_array::<jlong>(count),
		}
	}
}

fn alloc_zeroed_array<T>(count: usize) -> *mut u8 {
	let layout = Layout::array::<T>(count).expect("should always be valid");
	unsafe { alloc_zeroed(layout) }
}

/// An instance of a primitive array
#[derive(Debug)]
pub struct PrimitiveArrayInstance {
	header: Header,
	monitor: Arc<Monitor>,
	pub class: &'static Class,
	length: u32,
	ty: TypeCode,
	base: *mut u8,
}

impl CloneableInstance for PrimitiveArrayInstance {
	type ReferenceTy = PrimitiveArrayInstanceRef;

	unsafe fn clone(&self) -> Self::ReferenceTy {
		let new_array = self.ty.alloc_zeroed(self.length as usize);
		match self.ty {
			TypeCode::Boolean => {
				let new_array_slice = slice::from_raw_parts_mut::<jboolean>(
					new_array as *mut _,
					self.length as usize,
				);
				let previous_slice =
					slice::from_raw_parts::<jboolean>(self.base as *const _, self.length as usize);
				new_array_slice.copy_from_slice(previous_slice);
			},
			TypeCode::Char => {
				let new_array_slice =
					slice::from_raw_parts_mut::<jchar>(new_array as *mut _, self.length as usize);
				let previous_slice =
					slice::from_raw_parts::<jchar>(self.base as *const _, self.length as usize);
				new_array_slice.copy_from_slice(previous_slice);
			},
			TypeCode::Float => {
				let new_array_slice =
					slice::from_raw_parts_mut::<jfloat>(new_array as *mut _, self.length as usize);
				let previous_slice =
					slice::from_raw_parts::<jfloat>(self.base as *const _, self.length as usize);
				new_array_slice.copy_from_slice(previous_slice);
			},
			TypeCode::Double => {
				let new_array_slice =
					slice::from_raw_parts_mut::<jdouble>(new_array as *mut _, self.length as usize);
				let previous_slice =
					slice::from_raw_parts::<jdouble>(self.base as *const _, self.length as usize);
				new_array_slice.copy_from_slice(previous_slice);
			},
			TypeCode::Byte => {
				let new_array_slice =
					slice::from_raw_parts_mut::<jbyte>(new_array as *mut _, self.length as usize);
				let previous_slice =
					slice::from_raw_parts::<jbyte>(self.base as *const _, self.length as usize);
				new_array_slice.copy_from_slice(previous_slice);
			},
			TypeCode::Short => {
				let new_array_slice =
					slice::from_raw_parts_mut::<jshort>(new_array as *mut _, self.length as usize);
				let previous_slice =
					slice::from_raw_parts::<jshort>(self.base as *const _, self.length as usize);
				new_array_slice.copy_from_slice(previous_slice);
			},
			TypeCode::Int => {
				let new_array_slice =
					slice::from_raw_parts_mut::<jint>(new_array as *mut _, self.length as usize);
				let previous_slice =
					slice::from_raw_parts::<jint>(self.base as *const _, self.length as usize);
				new_array_slice.copy_from_slice(previous_slice);
			},
			TypeCode::Long => {
				let new_array_slice =
					slice::from_raw_parts_mut::<jlong>(new_array as *mut _, self.length as usize);
				let previous_slice =
					slice::from_raw_parts::<jlong>(self.base as *const _, self.length as usize);
				new_array_slice.copy_from_slice(previous_slice);
			},
		}

		PrimitiveArrayInstancePtr::new(Self {
			header: self.header.clone(),
			monitor: Arc::new(Monitor::new()),
			class: self.class,
			length: self.length,
			ty: self.ty,
			base: new_array,
		})
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
	pub unsafe fn new<T>(elements: Box<[T]>) -> PrimitiveArrayInstanceRef
	where
		T: PrimitiveType,
	{
		let length = elements.len();
		assert!(length <= s4::MAX as usize);

		let ty = T::TYPE_CODE;

		let base = alloc_zeroed_array::<T>(length);
		let new_array_slice = slice::from_raw_parts_mut::<T>(base as *mut T, length);
		new_array_slice.copy_from_slice(&*elements);

		PrimitiveArrayInstancePtr::new(Self {
			header: Header::new(),
			monitor: Arc::new(Monitor::new()),
			class: ty.array_class(),
			length: length as u32,
			ty,
			base,
		})
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.newarray
	pub fn new_from_type(type_code: u1, count: s4) -> Throws<PrimitiveArrayInstanceRef> {
		if count.is_negative() {
			throw!(@DEFER NegativeArraySizeException);
		}

		let type_code = TypeCode::from_u8(type_code);
		let array_signature = type_code.array_signature();
		let array = type_code.alloc_zeroed(count as usize);

		let array_class = ClassLoader::bootstrap().load(array_signature)?;

		Throws::Ok(PrimitiveArrayInstancePtr::new(Self {
			header: Header::new(),
			monitor: Arc::new(Monitor::new()),
			class: array_class,
			length: count as u32,
			ty: type_code,
			base: array,
		}))
	}

	/// Get a pointer to the start of this array
	///
	/// This must be used with great care, with respect to the actual underlying type of the array.
	pub unsafe fn base(&self) -> *mut u8 {
		self.base
	}

	pub fn as_slice<T: PrimitiveType>(&self) -> &[T] {
		assert_eq!(T::TYPE_CODE, self.ty);

		// SAFETY: We just verified that the type is correct
		unsafe { slice::from_raw_parts::<T>(self.base as *const _, self.length as usize) }
	}

	// TODO: Make arrays implement `Instance`
	pub fn monitor(&self) -> Arc<Monitor> {
		self.monitor.clone()
	}
}

impl super::Array for PrimitiveArrayInstance {
	type Component = Operand<Reference>;

	const BASE_OFFSET: usize = 0;

	fn header(&self) -> &Header {
		&self.header
	}

	fn len(&self) -> usize {
		self.length as usize
	}

	unsafe fn get_unchecked(&self, index: usize) -> Self::Component {
		match self.ty {
			TypeCode::Boolean => {
				Operand::from(unsafe { *(self.base as *const jboolean).add(index) })
			},
			TypeCode::Char => Operand::from(unsafe { *(self.base as *const jchar).add(index) }),
			TypeCode::Float => Operand::from(unsafe { *(self.base as *const jfloat).add(index) }),
			TypeCode::Double => Operand::from(unsafe { *(self.base as *const jdouble).add(index) }),
			TypeCode::Byte => Operand::from(unsafe { *(self.base as *const jbyte).add(index) }),
			TypeCode::Short => Operand::from(unsafe { *(self.base as *const jshort).add(index) }),
			TypeCode::Int => Operand::from(unsafe { *(self.base as *const jint).add(index) }),
			TypeCode::Long => Operand::from(unsafe { *(self.base as *const jlong).add(index) }),
		}
	}

	unsafe fn store_unchecked(&mut self, index: usize, value: Self::Component) {
		match value {
			Operand::Int(val) => match self.ty {
				TypeCode::Boolean => {
					let base = self.base as *mut jboolean;
					ptr::write::<jboolean>(base, (val & 1) == 1)
				},
				TypeCode::Byte => {
					let base = self.base as *mut jbyte;
					ptr::write::<jbyte>(base.add(index), val as jbyte)
				},
				TypeCode::Short => {
					let base = self.base as *mut jshort;
					ptr::write::<jshort>(base.add(index), val as jshort)
				},
				TypeCode::Char => {
					let base = self.base as *mut jchar;
					ptr::write::<jchar>(base.add(index), val as jchar)
				},
				TypeCode::Int => {
					let base = self.base as *mut jint;
					ptr::write::<jint>(base.add(index), val)
				},
				_ => unreachable!(),
			},
			Operand::Float(val) => {
				let base = self.base as *mut jfloat;
				ptr::write::<jfloat>(base.add(index), val);
			},
			Operand::Double(val) => {
				let base = self.base as *mut jdouble;
				ptr::write::<jdouble>(base.add(index), val);
			},
			Operand::Long(val) => {
				let base = self.base as *mut jlong;
				ptr::write::<jlong>(base.add(index), val);
			},
			_ => unreachable!(),
		}
	}

	unsafe fn copy_into(&self, src_pos: usize, dest: &mut Self, dest_pos: usize, length: usize) {
		// This will only check the type of `self`, the safety conditions of this method require that the caller
		// verify these arrays to be of the same type.
		match self.ty {
			TypeCode::Boolean => unsafe {
				let src_ptr = (self.base as *mut jboolean).add(src_pos);
				let dest_ptr = (dest.base as *mut jboolean).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
			TypeCode::Char => unsafe {
				let src_ptr = (self.base as *mut jchar).add(src_pos);
				let dest_ptr = (dest.base as *mut jchar).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
			TypeCode::Float => unsafe {
				let src_ptr = (self.base as *mut jfloat).add(src_pos);
				let dest_ptr = (dest.base as *mut jfloat).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
			TypeCode::Double => unsafe {
				let src_ptr = (self.base as *mut jdouble).add(src_pos);
				let dest_ptr = (dest.base as *mut jdouble).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
			TypeCode::Byte => unsafe {
				let src_ptr = (self.base as *mut jbyte).add(src_pos);
				let dest_ptr = (dest.base as *mut jbyte).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
			TypeCode::Short => unsafe {
				let src_ptr = (self.base as *mut jshort).add(src_pos);
				let dest_ptr = (dest.base as *mut jshort).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
			TypeCode::Int => unsafe {
				let src_ptr = (self.base as *mut jint).add(src_pos);
				let dest_ptr = (dest.base as *mut jint).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
			TypeCode::Long => unsafe {
				let src_ptr = (self.base as *mut jlong).add(src_pos);
				let dest_ptr = (dest.base as *mut jlong).add(dest_pos);
				src_ptr.copy_to_nonoverlapping(dest_ptr, length);
			},
		}
	}

	unsafe fn copy_within(&mut self, src_pos: usize, dest_pos: usize, length: usize) {
		// This will only check the type of `self`, the safety conditions of this method require that the caller
		// verify these arrays to be of the same type.
		match self.ty {
			TypeCode::Boolean => unsafe {
				let src_ptr = (self.base as *mut jboolean).add(src_pos);
				let dest_ptr = (self.base as *mut jboolean).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
			TypeCode::Char => unsafe {
				let src_ptr = (self.base as *mut jchar).add(src_pos);
				let dest_ptr = (self.base as *mut jchar).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
			TypeCode::Float => unsafe {
				let src_ptr = (self.base as *mut jfloat).add(src_pos);
				let dest_ptr = (self.base as *mut jfloat).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
			TypeCode::Double => unsafe {
				let src_ptr = (self.base as *mut jdouble).add(src_pos);
				let dest_ptr = (self.base as *mut jdouble).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
			TypeCode::Byte => unsafe {
				let src_ptr = (self.base as *mut jbyte).add(src_pos);
				let dest_ptr = (self.base as *mut jbyte).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
			TypeCode::Short => unsafe {
				let src_ptr = (self.base as *mut jshort).add(src_pos);
				let dest_ptr = (self.base as *mut jshort).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
			TypeCode::Int => unsafe {
				let src_ptr = (self.base as *mut jint).add(src_pos);
				let dest_ptr = (self.base as *mut jint).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
			TypeCode::Long => unsafe {
				let src_ptr = (self.base as *mut jlong).add(src_pos);
				let dest_ptr = (self.base as *mut jlong).add(dest_pos);
				src_ptr.copy_to(dest_ptr, length);
			},
		}
	}
}

// A pointer to a PrimitiveArrayInstance
//
// This can *not* be constructed by hand, as dropping it will
// deallocate the instance.
#[derive(PartialEq)]
pub struct PrimitiveArrayInstancePtr(usize);

impl PtrType<PrimitiveArrayInstance, PrimitiveArrayInstanceRef> for PrimitiveArrayInstancePtr {
	fn new(val: PrimitiveArrayInstance) -> PrimitiveArrayInstanceRef {
		let boxed = Box::new(val);
		let ptr = Box::into_raw(boxed);
		PrimitiveArrayInstanceRef::new(Self(ptr as usize))
	}

	#[inline(always)]
	fn as_raw(&self) -> *const PrimitiveArrayInstance {
		self.0 as *const PrimitiveArrayInstance
	}

	#[inline(always)]
	fn as_mut_raw(&self) -> *mut PrimitiveArrayInstance {
		self.0 as *mut PrimitiveArrayInstance
	}

	fn get(&self) -> &PrimitiveArrayInstance {
		unsafe { &(*self.as_raw()) }
	}

	fn get_mut(&self) -> &mut PrimitiveArrayInstance {
		unsafe { &mut (*self.as_mut_raw()) }
	}
}

impl Drop for PrimitiveArrayInstancePtr {
	fn drop(&mut self) {
		let _ = unsafe { Box::from_raw(self.0 as *mut PrimitiveArrayInstance) };
	}
}

impl Debug for PrimitiveArrayInstancePtr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let class = self.get();
		f.write_str(&class.class.name.as_str())
	}
}
