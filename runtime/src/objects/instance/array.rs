mod object;
pub use object::*;
mod primitive;
pub use primitive::*;

use crate::objects::instance::object::Object;
use crate::thread::exceptions::{Throws, throw};

use common::int_types::s4;

pub trait Array: Object {
	type Component;

	const BASE_OFFSET: usize;

	/// The length of the array in elements
	fn len(&self) -> usize;

	/// The alignment of the component type
	fn align(&self) -> usize;

	/// The size of the component type
	fn scale(&self) -> usize;

	#[inline]
	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Read the element `value` at `index`
	///
	/// # Exceptions
	///
	/// This will throw `ArrayIndexOutOfBoundsException` if `index` is negative or out of bounds.
	fn array_get(&self, index: s4) -> Throws<Self::Component> {
		if index.is_negative() || index as usize >= self.len() {
			throw!(@DEFER ArrayIndexOutOfBoundsException);
		}

		// SAFETY: Performed a bounds check already
		Throws::Ok(unsafe { self.get_unchecked(index as usize) })
	}

	/// Read the element at `index`
	///
	/// # Safety
	///
	/// This will **not** do a bounds check. The caller must verify that `index` is not out of bounds.
	unsafe fn get_unchecked(&self, index: usize) -> Self::Component;

	/// Write the element `value` at `index`
	///
	/// # Exceptions
	///
	/// This will throw `ArrayIndexOutOfBoundsException` if `index` is negative or out of bounds.
	fn store(&self, index: s4, value: Self::Component) -> Throws<()> {
		if index.is_negative() || index as usize >= self.len() {
			throw!(@DEFER ArrayIndexOutOfBoundsException);
		}

		// SAFETY: Performed a bounds check already
		unsafe {
			self.store_unchecked(index as usize, value);
		}

		Throws::Ok(())
	}

	/// Write the element `value` at `index`
	///
	/// # Safety
	///
	/// This will **not** do a bounds check. The caller must verify that `index` is not out of bounds.
	unsafe fn store_unchecked(&self, index: usize, value: Self::Component);

	/// Copy the contents of `self[src_pos..length]` into `dest[dest_pos..length]`
	///
	/// # Safety
	///
	/// The caller must verify that:
	///
	/// * `src_pos` + `length` <= `self.len()`
	/// * `dest_pos` + `length` <= `dest.len()`
	/// * `self` and `dest` have the same component type
	unsafe fn copy_into(&self, src_pos: usize, dest: &Self, dest_pos: usize, length: usize);

	/// Copy the contents of `self[src_pos..length]` into `self[dest_pos..length]`
	///
	/// # Safety
	///
	/// The caller must verify that:
	///
	/// * `src_pos` + `length` <= `self.len()`
	unsafe fn copy_within(&self, src_pos: usize, dest_pos: usize, length: usize);
}
