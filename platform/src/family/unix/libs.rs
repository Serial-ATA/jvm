use crate::libs::{Error, Result};

use std::ffi::{CStr, CString, OsString, c_void};
use std::marker::PhantomData;

use libc::{c_char, c_int};

pub struct LibraryImpl {
	lib: *mut c_void,
}

impl LibraryImpl {
	pub unsafe fn load(name: impl Into<OsString>) -> Result<Self> {
		// Clear dlerror
		let _ = unsafe { libc::dlerror() };

		let name = CString::new(name.into().into_encoded_bytes())?;
		unsafe { Self::open_name(name.as_ptr(), libc::RTLD_LAZY) }
	}

	pub unsafe fn current() -> Result<Self> {
		#[cfg(target_vendor = "apple")]
		unsafe {
			Self::open_name(std::ptr::null(), libc::RTLD_FIRST)
		}

		#[cfg(not(target_vendor = "apple"))]
		unsafe {
			Self::open_name(std::ptr::null(), libc::RTLD_LAZY)
		}
	}

	unsafe fn open_name(name: *const c_char, flag: c_int) -> Result<Self> {
		let lib = unsafe { libc::dlopen(name, flag) };
		if lib.is_null() {
			return Err(Error::Open {
				description: unsafe { dlerror() },
			});
		}

		Ok(Self { lib })
	}

	pub unsafe fn close(self) -> Result<()> {
		// Clear dlerror
		let _ = unsafe { libc::dlerror() };

		let ret = unsafe { libc::dlclose(self.lib) };
		if ret != 0 {
			return Err(Error::Close {
				description: unsafe { dlerror() },
			});
		}

		Ok(())
	}

	pub unsafe fn from_raw(raw: *mut c_void) -> Self {
		Self { lib: raw }
	}

	pub fn raw(&self) -> *mut c_void {
		self.lib
	}

	pub unsafe fn symbol<T>(&self, name: &CStr) -> Result<Sym<T>> {
		// Clear dlerror
		let _ = unsafe { libc::dlerror() };

		let sym = unsafe { libc::dlsym(self.lib, name.as_ptr()) };
		if sym.is_null() {
			return Err(Error::Symbol {
				description: unsafe { dlerror() },
			});
		}

		Ok(Sym {
			ptr: sym,
			phantom: PhantomData,
		})
	}
}

unsafe fn dlerror() -> Option<String> {
	let err = unsafe { libc::dlerror() };

	if err.is_null() {
		None
	} else {
		let cstr = unsafe { CStr::from_ptr(err) };
		Some(cstr.to_string_lossy().to_string())
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
