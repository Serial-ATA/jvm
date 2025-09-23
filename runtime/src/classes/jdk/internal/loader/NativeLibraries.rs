use classfile::FieldType;

crate::classes::field_module! {
	@CLASS jdk_internal_loader_NativeLibraries;
	@SUBCLASS NativeLibraryImpl;

	@FIELDSTART
}

pub mod NativeLibraryImpl {
	use super::*;
	use crate::objects::instance::Instance;
	use crate::objects::reference::Reference;
	use instructions::Operand;
	use jni::sys::{jint, jlong};

	pub fn set_handle(this: Reference, handle: jlong) {
		this.extract_class()
			.put_field_value0(handle_field_index(), Operand::Long(handle));
	}

	pub fn set_jniVersion(this: Reference, version: jint) {
		this.extract_class()
			.put_field_value0(jniVersion_field_index(), Operand::Int(version));
	}

	crate::classes::field_module! {
		@CLASS jdk_internal_loader_NativeLibraries_NativeLibraryImpl;

		@FIELDSTART
		/// `jdk.internal.loader.NativeLibraries.NativeLibraryImpl#handle` field offset
		///
		/// Expected type: `jlong`
		@FIELD handle: FieldType::Long,
		/// `jdk.internal.loader.NativeLibraries.NativeLibraryImpl#jniVersion` field offset
		///
		/// Expected type: `jint`
		@FIELD jniVersion: FieldType::Integer,
	}
}
