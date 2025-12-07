use crate::{JNI_LIB_PREFIX, JNI_LIB_SUFFIX};

use std::ffi::{CStr, NulError, OsString, c_void};
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
	Open { description: Option<String> },
	Close { description: Option<String> },
	Symbol { description: Option<String> },
	CStr(NulError),
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Error::Open { description } => match description {
				Some(desc) => write!(f, "{desc}"),
				None => write!(f, "Failed to open library for unknown reason"),
			},
			Error::Close { description } => match description {
				Some(desc) => write!(f, "{desc}"),
				None => write!(f, "Failed to close library for unknown reason"),
			},
			Error::Symbol { description } => match description {
				Some(desc) => write!(f, "{desc}"),
				None => write!(f, "Failed to get symbol for unknown reason"),
			},
			Error::CStr(err) => write!(f, "{err}"),
		}
	}
}

impl From<NulError> for Error {
	fn from(err: NulError) -> Self {
		Error::CStr(err)
	}
}

impl core::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;

pub struct Library(super::imp::libs::LibraryImpl);

impl Library {
	pub fn load(name: impl Into<OsString>) -> Result<Self> {
		let imp = unsafe { super::imp::libs::LibraryImpl::load(name)? };
		Ok(Self(imp))
	}

	pub fn load_from_path(base: &Path, name: &str) -> Result<Self> {
		Self::load(base.join(format!("{JNI_LIB_PREFIX}{name}{JNI_LIB_SUFFIX}")))
	}

	/// Get a handle to the current process
	pub fn current() -> Result<Self> {
		let imp = unsafe { super::imp::libs::LibraryImpl::current()? };
		Ok(Self(imp))
	}

	/// Consume and close the library
	///
	/// # Errors
	///
	/// If the library fails to close, a description of the error will be returned. The format of that
	/// error will, of course, be platform-specific.
	pub fn close(self) -> Result<()> {
		unsafe { super::imp::libs::LibraryImpl::close(self.0) }
	}

	/// Attempt to lookup a symbol by `name`
	///
	/// # Errors
	///
	/// If the lookup fails, a description of the error will be returned. The format of that error
	/// will, of course, be platform-specific.
	///
	/// # Safety
	///
	/// * The caller *must* ensure that the specified `T` is the correct type of the symbol
	pub unsafe fn symbol<'library, T: 'library>(
		&'library self,
		name: &CStr,
	) -> Result<Sym<'library, T>> {
		let imp = unsafe { self.0.symbol(name)? };
		Ok(Sym {
			inner: imp,
			phantom: PhantomData,
		})
	}

	/// Create a `Library` from a raw pointer to a library handle
	///
	/// # Safety
	///
	/// * The caller *must* ensure that `raw` is a valid pointer obtained from `dlopen`, for example.
	pub unsafe fn from_raw(raw: *mut c_void) -> Self {
		Self(unsafe { super::imp::libs::LibraryImpl::from_raw(raw) })
	}

	/// Get a raw pointer to the library handle
	pub fn raw(&self) -> *mut c_void {
		self.0.raw()
	}
}

/// A symbol from a loaded [`Library`]
///
/// The lifetime of this symbol is tied to the [`Library`] from which it was loaded.
///
/// See [`Library::symbol()`] for safety notes.
pub struct Sym<'lib, T> {
	inner: super::imp::libs::Sym<T>,
	phantom: PhantomData<&'lib T>,
}

impl<T> Sym<'_, T> {
	pub fn raw(&self) -> *mut c_void {
		self.inner.raw()
	}
}

impl<'a, T: 'a> Deref for Sym<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe { &*(std::ptr::from_ref(&self.inner.raw()).cast::<T>()) }
	}
}
