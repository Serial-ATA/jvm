mod object;
pub use object::*;
mod primitive;
pub use primitive::*;

use super::instance::Header;
use crate::thread::exceptions::{throw, Throws};

use common::int_types::s4;

pub trait Array {
	type Component;

	fn header(&self) -> &Header;

	fn len(&self) -> usize;

	#[inline]
	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	fn get(&self, index: s4) -> Throws<Self::Component> {
		if index.is_negative() || index as usize >= self.len() {
			throw!(@DEFER ArrayIndexOutOfBoundsException);
		}

		// SAFETY: Performed a bounds check already
		Throws::Ok(unsafe { self.get_unchecked(index as usize) })
	}

	unsafe fn get_unchecked(&self, index: usize) -> Self::Component;

	fn store(&mut self, index: s4, value: Self::Component) -> Throws<()> {
		if index.is_negative() || index as usize >= self.len() {
			throw!(@DEFER ArrayIndexOutOfBoundsException);
		}

		// SAFETY: Performed a bounds check already
		unsafe {
			self.store_unchecked(index as usize, value);
		}

		Throws::Ok(())
	}

	/// Same as [`self.store`], without the bounds checking
	///
	/// # Safety
	///
	/// It is up to the caller to ensure that `index` is unsigned and within the bounds of the current array.
	unsafe fn store_unchecked(&mut self, index: usize, value: Self::Component);

	/// Copy the contents of `self[src_pos..length]` into `dest[dest_pos..length]`
	///
	/// # Safety
	///
	/// The caller must verify that:
	///
	/// * `src_pos` + `length` <= `self.len()`
	/// * `dest_pos` + `length` <= `dest.len()`
	/// * `self` and `dest` have the same component type
	unsafe fn copy_into(&self, src_pos: usize, dest: &mut Self, dest_pos: usize, length: usize);

	/// Copy the contents of `self[src_pos..length]` into `self[dest_pos..length]`
	///
	/// # Safety
	///
	/// The caller must verify that:
	///
	/// * `src_pos` + `length` <= `self.len()`
	unsafe fn copy_within(&mut self, src_pos: usize, dest_pos: usize, length: usize);
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum ArrayContent {
// 	Byte(Box<[s1]>),
// 	Boolean(Box<[bool]>),
// 	Short(Box<[s2]>),
// 	Char(Box<[u2]>),
// 	Int(Box<[s4]>),
// 	Float(Box<[f32]>),
// 	Double(Box<[f64]>),
// 	Long(Box<[s8]>),
// 	Reference(Box<[Reference]>),
// }
//
// macro_rules! expect_functions {
// 	($([$name:ident, $pat:pat, $ty:ty]),+) => {
// 		$(
// 		paste::paste! {
// 			pub fn [<expect_ $name>](&self) -> &[$ty] {
// 				match self {
// 					ArrayContent::$pat(bytes) => bytes,
// 					_ => panic!("Expected an array of type `{}`", stringify!($name)),
// 				}
// 			}
//
// 			pub fn [<expect_ $name _mut>](&mut self) -> &mut [$ty] {
// 				match self {
// 					ArrayContent::$pat(bytes) => bytes,
// 					_ => panic!("Expected an array of type `{}`", stringify!($name)),
// 				}
// 			}
// 		}
// 		)+
// 	}
// }
//
// macro_rules! unsafe_getters {
// 	($([$name:ident, $pat:pat, $ty:ty]),+) => {
// 		$(
// 		paste::paste! {
// 			pub unsafe fn [<get_ $name _raw>](&mut self, field_offset: usize) -> NonNull<$ty> {
// 				let ptr = self.base_content_ptr() as *mut $ty;
// 				unsafe { NonNull::new_unchecked(ptr.offset(field_offset as isize)) }
// 			}
// 		}
// 		)+
// 	}
// }
//
// impl ArrayContent {
// 	fn default_initialize(type_code: u1, count: s4) -> Self {
// 		match type_code {
// 			4 => Self::Boolean(box_slice![bool::default(); count as usize]),
// 			5 => Self::Char(box_slice![0; count as usize]),
// 			6 => Self::Float(box_slice![0.; count as usize]),
// 			7 => Self::Double(box_slice![0.; count as usize]),
// 			8 => Self::Byte(box_slice![0; count as usize]),
// 			9 => Self::Short(box_slice![0; count as usize]),
// 			10 => Self::Int(box_slice![0; count as usize]),
// 			11 => Self::Long(box_slice![0; count as usize]),
// 			_ => panic!("Invalid array type code: {}", type_code),
// 		}
// 	}
//
// 	fn get(&self, index: usize) -> Operand<Reference> {
// 		match self {
// 			ArrayContent::Boolean(content) => Operand::Int(s4::from(content[index])),
// 			ArrayContent::Byte(content) => Operand::Int(s4::from(content[index])),
// 			ArrayContent::Short(content) => Operand::Int(s4::from(content[index])),
// 			ArrayContent::Char(content) => Operand::Int(s4::from(content[index])),
// 			ArrayContent::Int(content) => Operand::Int(content[index]),
// 			ArrayContent::Float(content) => Operand::Float(content[index]),
// 			ArrayContent::Double(content) => Operand::Double(content[index]),
// 			ArrayContent::Long(content) => Operand::Long(content[index]),
// 			ArrayContent::Reference(content) => Operand::Reference(content[index].clone()),
// 		}
// 	}
//
// 	pub fn element_count(&self) -> usize {
// 		match self {
// 			ArrayContent::Boolean(content) => content.len(),
// 			ArrayContent::Byte(content) => content.len(),
// 			ArrayContent::Short(content) => content.len(),
// 			ArrayContent::Char(content) => content.len(),
// 			ArrayContent::Int(content) => content.len(),
// 			ArrayContent::Float(content) => content.len(),
// 			ArrayContent::Double(content) => content.len(),
// 			ArrayContent::Long(content) => content.len(),
// 			ArrayContent::Reference(content) => content.len(),
// 		}
// 	}
//
// 	pub fn copy_into(
// 		&self,
// 		start: usize,
// 		dest: &mut ArrayContent,
// 		dest_start: usize,
// 		length: usize,
// 	) {
// 		macro_rules! copy {
// 			($($pat:path, ($ty:ident))|+) => {
// 				match self {
// 					$($pat(self_bytes) => {
// 						paste::paste! {
// 							let dest_bytes = dest.[<expect_ $ty _mut>]();
// 							let (_, dest_slice) = dest_bytes.split_at_mut(dest_start);
// 							let (_, self_slice) = self_bytes.split_at(start);
// 							dest_slice[..length].copy_from_slice(&self_slice[..length]);
// 						}
// 					}),+
// 					ArrayContent::Reference(self_bytes) => {
// 						let dest_bytes = dest.expect_reference_mut();
// 						let (_, dest_slice) = dest_bytes.split_at_mut(dest_start);
// 						let (_, self_slice) = self_bytes.split_at(start);
// 						dest_slice[..length].clone_from_slice(&self_slice[..length]);
// 					}
// 				}
// 			}
// 		}
//
// 		copy! {
// 			ArrayContent::Byte,      (byte)
// 			| ArrayContent::Boolean, (boolean)
// 			| ArrayContent::Short,   (short)
// 			| ArrayContent::Char,    (char)
// 			| ArrayContent::Int,     (int)
// 			| ArrayContent::Float,   (float)
// 			| ArrayContent::Double,  (double)
// 			| ArrayContent::Long,    (long)
// 		}
// 	}
//
// 	fn base_content_ptr(&self) -> *mut u8 {
// 		match self {
// 			ArrayContent::Byte(val) => val.as_ptr() as _,
// 			ArrayContent::Boolean(val) => val.as_ptr() as _,
// 			ArrayContent::Short(val) => val.as_ptr() as _,
// 			ArrayContent::Char(val) => val.as_ptr() as _,
// 			ArrayContent::Int(val) => val.as_ptr() as _,
// 			ArrayContent::Float(val) => val.as_ptr() as _,
// 			ArrayContent::Double(val) => val.as_ptr() as _,
// 			ArrayContent::Long(val) => val.as_ptr() as _,
// 			ArrayContent::Reference(val) => val.as_ptr() as _,
// 		}
// 	}
//
// 	expect_functions! {
// 		[byte, Byte, s1],
// 		[boolean, Boolean, bool],
// 		[short, Short, s2],
// 		[char, Char, u2],
// 		[int, Int, s4],
// 		[float, Float, f32],
// 		[double, Double, f64],
// 		[long, Long, s8],
// 		[reference, Reference, Reference]
// 	}
//
// 	unsafe_getters! {
// 		[byte, Byte, s1],
// 		[boolean, Boolean, bool],
// 		[short, Short, s2],
// 		[char, Char, u2],
// 		[int, Int, s4],
// 		[float, Float, f32],
// 		[double, Double, f64],
// 		[long, Long, s8],
// 		[reference, Reference, Reference]
// 	}
// }
