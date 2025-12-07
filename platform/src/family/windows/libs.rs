use crate::libs::{Error, Result};

use std::ffi::{CStr, OsString, c_void};
use std::marker::PhantomData;

pub struct LibraryImpl {}

impl LibraryImpl {
	pub unsafe fn load(name: impl Into<OsString>) -> Result<Self> {
		todo!("Windows lib loading")
	}

	pub unsafe fn close(self) -> Result<()> {
		todo!("Windows lib close")
	}

	pub unsafe fn symbol<T>(&self, name: &CStr) -> Result<Sym<T>> {
		todo!("Windows lib symbol")
	}
}

pub struct Sym<T> {
	ptr: *mut c_void,
	phantom: PhantomData<T>,
}

impl<T> Sym<T> {
	pub fn raw(&self) -> *mut c_void {
		self.ptr
	}
}
