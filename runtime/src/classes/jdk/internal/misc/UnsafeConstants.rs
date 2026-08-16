use crate::objects::instance::Instance;

use classfile::FieldType;
use instructions::Operand;
use jni::sys::jint;

crate::classes::field_module! {
	@CLASS jdk_internal_misc_UnsafeConstants;

	@FIELDSTART
	/// `jdk.internal.misc.UnsafeConstants#ADDRESS_SIZE0` field offset
	///
	/// Expected field type: `jint`
	@FIELD ADDRESS_SIZE0: FieldType::Integer,
	/// `jdk.internal.misc.UnsafeConstants#PAGE_SIZE` field offset
	///
	/// Expected field type: `jint`
	@FIELD PAGE_SIZE: FieldType::Integer,
	/// `jdk.internal.misc.UnsafeConstants#BIG_ENDIAN` field offset
	///
	/// Expected field type: `jboolean`
	@FIELD BIG_ENDIAN: FieldType::Boolean,
	/// `jdk.internal.misc.UnsafeConstants#UNALIGNED_ACCESS` field offset
	///
	/// Expected field type: `jboolean`
	@FIELD UNALIGNED_ACCESS: FieldType::Boolean,
	/// `jdk.internal.misc.UnsafeConstants#DATA_CACHE_LINE_FLUSH_SIZE` field offset
	///
	/// Expected field type: `jint`
	@FIELD DATA_CACHE_LINE_FLUSH_SIZE: FieldType::Integer,
}

/// Initialize the static fields for `jdk.internal.misc.UnsafeConstants`
///
/// # Safety
///
/// This **requires** that:
/// * `jdk.internal.misc.UnsafeConstants` is loaded *and* initialized
/// * all field offsets have been initialized
pub unsafe fn init() {
	let class = crate::globals::classes::jdk_internal_misc_UnsafeConstants();
	let mirror = class.mirror();

	// NOTE: The fields are already default initialized to 0
	unsafe {
		mirror.put_field_value(
			ADDRESS_SIZE0_field(),
			Operand::from(size_of::<usize>() as jint),
		);
		mirror.put_field_value(
			PAGE_SIZE_field(),
			Operand::from(platform::mem::get_page_size() as jint),
		);
		mirror.put_field_value(
			BIG_ENDIAN_field(),
			Operand::from(jint::from(cfg!(target_endian = "big"))),
		);
	}
	// TODO: class.set_static_field(unaligned_access_field_index(), /* ... */);
	// TODO: class.set_static_field(data_cache_line_flush_size_field_index(), /* ... */);
}
