use common::int_types::u1;

/// An intrinsic definition
pub struct IntrinsicEntry {
	pub class: &'static [u1],
	pub name: &'static [u1],
	pub descriptor: &'static [u1],
	pub flags: IntrinsicFlags,
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
