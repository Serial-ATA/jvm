mod object;
pub use object::*;
mod primitive;
pub use primitive::*;

use super::instance::Header;
use crate::thread::exceptions::{Throws, throw};

use common::int_types::s4;

pub trait Array {
	type Component;

	const BASE_OFFSET: usize;

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
