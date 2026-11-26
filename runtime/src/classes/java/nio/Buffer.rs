use crate::objects::instance::Instance;
use crate::objects::instance::class::ClassInstanceRef;

use classfile::FieldType;
use jni::sys::{jint, jlong};

/// `java.nio.Buffer#capacity` field
pub fn capacity(instance: ClassInstanceRef) -> jint {
	instance
		.get_field_value0(capacity_field_index())
		.expect_int()
}

/// `java.nio.Buffer#address` field
pub fn address(instance: ClassInstanceRef) -> jlong {
	instance
		.get_field_value0(address_field_index())
		.expect_long()
}

crate::classes::field_module! {
	@CLASS java_nio_Buffer;

	@FIELDSTART
	/// `java.nio.Buffer#capacity` field offset
	///
	/// Expected type: `int`
	@FIELD capacity: ty @ FieldType::Integer,
	/// `java.nio.Buffer#address` field offset
	///
	/// Expected type: `long`
	@FIELD address: ty @ FieldType::Long,
}
