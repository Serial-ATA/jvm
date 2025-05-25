use std::ffi::{CStr, NulError};
use std::marker::PhantomData;
use std::ops::Deref;

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
	pub fn load(name: &str) -> Result<Self> {
		let imp = unsafe { super::imp::libs::LibraryImpl::load(name)? };
		Ok(Self(imp))
	}

	pub unsafe fn close(self) -> Result<()> {
		unsafe { super::imp::libs::LibraryImpl::close(self.0) }
	}

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
}

pub struct Sym<'lib, T: 'lib> {
	inner: super::imp::libs::Sym<T>,
	phantom: PhantomData<&'lib T>,
}

impl<'a, T: 'a> Deref for Sym<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe { &*(&self.inner.raw() as *const *mut _ as *const T) }
	}
}
