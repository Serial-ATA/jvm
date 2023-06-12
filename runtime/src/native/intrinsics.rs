use crate::reference::MethodRef;

use std::sync::Mutex;

use once_cell::sync::Lazy;

static REGISTERED_INTRINSICS: Lazy<Mutex<Vec<IntrinsicEntry>>> =
	Lazy::new(|| Mutex::new(Vec::new()));

pub fn find_intrinsic(_method: MethodRef, _is_virtual: bool) -> Option<IntrinsicEntry> {
	todo!()
}

// The automatically generated intrinsic candidates
include!("intrinsics_generated.rs");

impl IntrinsicId {
	pub fn does_virtual_dispatch(self) -> bool {
		matches!(self, Self::Object_hashCode | Self::Object_clone)
	}

	/// Whether the intrinsic is available, according to the platform and JVM flags
	pub fn is_enabled(self) -> bool {
		true // TODO
	}
}

/// An intrinsic definition
pub struct IntrinsicEntry {
	is_virtual: bool,
	does_virtual_dispatch: bool,
	intrinsic_id: IntrinsicId,
	method: MethodRef,
	flags: IntrinsicFlags,
}

impl IntrinsicEntry {
	pub fn new(
		method: MethodRef,
		is_virtual: bool,
		does_virtual_dispatch: bool,
		intrinsic_id: IntrinsicId,
	) -> Self {
		Self {
			is_virtual,
			does_virtual_dispatch,
			intrinsic_id,
			method,
			flags: IntrinsicFlags::Native,
		}
	}

	pub fn is_virtual(&self) -> bool {
		self.is_virtual
	}

	pub fn does_virtual_dispatch(&self) -> bool {
		self.does_virtual_dispatch
	}

	pub fn intrinsic_id(&self) -> IntrinsicId {
		self.intrinsic_id
	}
}

/// Access flag combinations relevant to intrinsic methods
#[repr(u8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum IntrinsicFlags {
	/// Not static, native, or synchronized
	Regular,
	/// Is: Static
	/// Is not: Native, synchronized
	Static,
	/// Is: Synchronized
	/// Is not: Static, native
	Synchronized,
	/// Is: Native
	/// Is not: Static, synchronized
	Native,
	/// Is: Static, native
	/// Is not: Synchronized
	StaticNative,
}

impl IntrinsicFlags {
	/// Whether the intrinsic flags contain the static access flag
	pub fn is_static(self) -> bool {
		match self {
			Self::Static | Self::StaticNative => true,
			_ => false,
		}
	}

	/// Whether the intrinsic flags contain the native access flag
	pub fn is_native(self) -> bool {
		match self {
			Self::Native | Self::StaticNative => true,
			_ => false,
		}
	}
}
