use super::class::Class;
use super::instance::{CloneableInstance, Header};
use super::reference::{ArrayInstanceRef, Reference};
use crate::classpath::classloader::ClassLoader;

use std::fmt::{Debug, Formatter};
use std::ptr::NonNull;

use common::box_slice;
use common::int_types::{s1, s2, s4, s8, u1, u2};
use common::traits::PtrType;
use instructions::Operand;
use symbols::sym;

/// An instance of an array
///
/// This covers all array types, including primitives and objects.
#[derive(Debug, PartialEq)]
pub struct ArrayInstance {
	header: Header,
	pub class: &'static Class,
	pub elements: ArrayContent,
}

impl CloneableInstance for ArrayInstance {
	type ReferenceTy = ArrayInstanceRef;

	unsafe fn clone(&self) -> Self::ReferenceTy {
		ArrayInstance::new(self.class, self.elements.clone())
	}
}

impl ArrayInstance {
	pub fn new(class: &'static Class, elements: ArrayContent) -> ArrayInstanceRef {
		ArrayInstancePtr::new(Self {
			header: Header::new(),
			class,
			elements,
		})
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.newarray
	pub fn new_from_type(type_code: u1, count: s4) -> ArrayInstanceRef {
		if count.is_negative() {
			panic!("NegativeArraySizeException"); // TODO
		}

		let array_signature = match type_code {
			4 => sym!(bool_array),
			5 => sym!(char_array),
			6 => sym!(float_array),
			7 => sym!(double_array),
			8 => sym!(byte_array),
			9 => sym!(short_array),
			10 => sym!(int_array),
			11 => sym!(long_array),
			_ => panic!("Invalid array type code: {}", type_code),
		};

		let array_class = ClassLoader::Bootstrap.load(array_signature).unwrap();
		let elements = ArrayContent::default_initialize(type_code, count);

		ArrayInstancePtr::new(Self {
			header: Header::new(),
			class: array_class,
			elements,
		})
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.anewarray
	pub fn new_reference(count: s4, component_class: &'static Class) -> ArrayInstanceRef {
		if count.is_negative() {
			panic!("NegativeArraySizeException"); // TODO
		}

		let array_class_name = component_class.array_class_name();
		let array_class = ClassLoader::Bootstrap.load(array_class_name).unwrap();

		let elements = box_slice![Reference::null(); count as usize];
		ArrayInstancePtr::new(Self {
			header: Header::new(),
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
			ArrayContent::Boolean(ref mut contents) => {
				contents[index] = (value.expect_int() & 1) == 1
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

	pub fn header(&self) -> &Header {
		&self.header
	}

	pub fn get_content(&self) -> &ArrayContent {
		&self.elements
	}

	pub unsafe fn get_content_mut(&mut self) -> &mut ArrayContent {
		&mut self.elements
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrayContent {
	Byte(Box<[s1]>),
	Boolean(Box<[bool]>),
	Short(Box<[s2]>),
	Char(Box<[u2]>),
	Int(Box<[s4]>),
	Float(Box<[f32]>),
	Double(Box<[f64]>),
	Long(Box<[s8]>),
	Reference(Box<[Reference]>),
}

macro_rules! expect_functions {
	($([$name:ident, $pat:pat, $ty:ty]),+) => {
		$(
		paste::paste! {
			pub fn [<expect_ $name>](&self) -> &[$ty] {
				match self {
					ArrayContent::$pat(bytes) => bytes,
					_ => panic!("Expected an array of type `{}`", stringify!($name)),
				}
			}

			pub fn [<expect_ $name _mut>](&mut self) -> &mut [$ty] {
				match self {
					ArrayContent::$pat(bytes) => bytes,
					_ => panic!("Expected an array of type `{}`", stringify!($name)),
				}
			}
		}
		)+
	}
}

macro_rules! unsafe_getters {
	($([$name:ident, $pat:pat, $ty:ty]),+) => {
		$(
		paste::paste! {
			pub unsafe fn [<get_ $name _raw>](&mut self, field_offset: usize) -> NonNull<$ty> {
				let ptr = self.base_content_ptr() as *mut $ty;
				unsafe { NonNull::new_unchecked(ptr.offset(field_offset as isize)) }
			}
		}
		)+
	}
}

impl ArrayContent {
	fn default_initialize(type_code: u1, count: s4) -> Self {
		match type_code {
			4 => Self::Boolean(box_slice![bool::default(); count as usize]),
			5 => Self::Char(box_slice![0; count as usize]),
			6 => Self::Float(box_slice![0.; count as usize]),
			7 => Self::Double(box_slice![0.; count as usize]),
			8 => Self::Byte(box_slice![0; count as usize]),
			9 => Self::Short(box_slice![0; count as usize]),
			10 => Self::Int(box_slice![0; count as usize]),
			11 => Self::Long(box_slice![0; count as usize]),
			_ => panic!("Invalid array type code: {}", type_code),
		}
	}

	fn get(&self, index: usize) -> Operand<Reference> {
		match self {
			ArrayContent::Boolean(content) => Operand::Int(s4::from(content[index])),
			ArrayContent::Byte(content) => Operand::Int(s4::from(content[index])),
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
			ArrayContent::Boolean(content) => content.len(),
			ArrayContent::Byte(content) => content.len(),
			ArrayContent::Short(content) => content.len(),
			ArrayContent::Char(content) => content.len(),
			ArrayContent::Int(content) => content.len(),
			ArrayContent::Float(content) => content.len(),
			ArrayContent::Double(content) => content.len(),
			ArrayContent::Long(content) => content.len(),
			ArrayContent::Reference(content) => content.len(),
		}
	}

	pub fn copy_into(
		&self,
		start: usize,
		dest: &mut ArrayContent,
		dest_start: usize,
		length: usize,
	) {
		macro_rules! copy {
			($($pat:path, ($ty:ident))|+) => {
				match self {
					$($pat(self_bytes) => {
						paste::paste! {
							let dest_bytes = dest.[<expect_ $ty _mut>]();
							let (_, dest_slice) = dest_bytes.split_at_mut(dest_start);
							let (_, self_slice) = self_bytes.split_at(start);
							dest_slice[..length].copy_from_slice(&self_slice[..length]);
						}
					}),+
					ArrayContent::Reference(self_bytes) => {
						let dest_bytes = dest.expect_reference_mut();
						let (_, dest_slice) = dest_bytes.split_at_mut(dest_start);
						let (_, self_slice) = self_bytes.split_at(start);
						dest_slice[..length].clone_from_slice(&self_slice[..length]);
					}
				}
			}
		}

		copy! {
			ArrayContent::Byte,      (byte)
			| ArrayContent::Boolean, (boolean)
			| ArrayContent::Short,   (short)
			| ArrayContent::Char,    (char)
			| ArrayContent::Int,     (int)
			| ArrayContent::Float,   (float)
			| ArrayContent::Double,  (double)
			| ArrayContent::Long,    (long)
		}
	}

	fn base_content_ptr(&self) -> *mut u8 {
		match self {
			ArrayContent::Byte(val) => val.as_ptr() as _,
			ArrayContent::Boolean(val) => val.as_ptr() as _,
			ArrayContent::Short(val) => val.as_ptr() as _,
			ArrayContent::Char(val) => val.as_ptr() as _,
			ArrayContent::Int(val) => val.as_ptr() as _,
			ArrayContent::Float(val) => val.as_ptr() as _,
			ArrayContent::Double(val) => val.as_ptr() as _,
			ArrayContent::Long(val) => val.as_ptr() as _,
			ArrayContent::Reference(val) => val.as_ptr() as _,
		}
	}

	expect_functions! {
		[byte, Byte, s1],
		[boolean, Boolean, bool],
		[short, Short, s2],
		[char, Char, u2],
		[int, Int, s4],
		[float, Float, f32],
		[double, Double, f64],
		[long, Long, s8],
		[reference, Reference, Reference]
	}

	unsafe_getters! {
		[byte, Byte, s1],
		[boolean, Boolean, bool],
		[short, Short, s2],
		[char, Char, u2],
		[int, Int, s4],
		[float, Float, f32],
		[double, Double, f64],
		[long, Long, s8],
		[reference, Reference, Reference]
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
		f.write_str(&class.class.name.as_str())
	}
}
