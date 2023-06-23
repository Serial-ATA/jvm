use crate::reference::MethodRef;

use std::sync::Mutex;

use classfile::accessflags::MethodAccessFlags;
use once_cell::sync::Lazy;

static REGISTERED_INTRINSICS: Lazy<Mutex<Vec<IntrinsicEntry>>> =
	Lazy::new(|| Mutex::new(Vec::new()));

pub fn find_intrinsic(_method: MethodRef, _is_virtual: bool) -> Option<IntrinsicEntry> {
	todo!()
}

fn register_intrinsic(_method: MethodRef, _is_virtual: bool) {
	todo!()
}

// The automatically generated intrinsic candidates
include!("../../../generated/native/intrinsics_generated.rs");

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
		assert_ne!(
			intrinsic_id,
			IntrinsicId::None,
			"Attempted to register an intrinsic entry for non-intrinsic method: {:?}",
			method
		);

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

impl From<MethodAccessFlags> for IntrinsicFlags {
	fn from(value: MethodAccessFlags) -> Self {
		let is_static = value.is_static();
		let is_native = value.is_native();
		let is_synchronized = value.is_synchronized();

		if !is_static && !is_native && !is_synchronized {
			return Self::Regular;
		}

		if is_static {
			assert!(
				(!is_native && !is_synchronized) || is_native && !is_synchronized,
				"Invalid intrinsic flags: {:?} (Must be either Static and Native OR Static)",
				value
			);

			if !is_native && !is_synchronized {
				return Self::Static;
			}

			if is_native && !is_synchronized {
				return Self::StaticNative;
			}
		}

		if is_native {
			assert!(
				!is_synchronized,
				"Invalid intrinsic flags: {:?} (Must be Native AND !Synchronized)",
				value
			);

			return Self::Native;
		}

		assert!(
			is_synchronized,
			"Invalid intrisic flags: {:?} (Must be Synchronized)",
			value
		);

		Self::Synchronized
	}
}
