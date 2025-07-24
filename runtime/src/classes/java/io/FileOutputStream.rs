use super::FileDescriptor;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;
use classfile::FieldType;
use jni::sys::jint;

crate::classes::field_module! {
	@CLASS java_io_FileOutputStream;

	@FIELDSTART
	/// `java.io.FileOutputStream#fd` field offset
	///
	/// Expected field type: `Reference` to `java.io.FileDescriptor`
	@FIELD fd: ty @ FieldType::Object(_) if ty.is_class(b"java/io/FileDescriptor"),
}

pub fn fd(this: &Reference) -> jint {
	// `fd` is a reference to a `java.io.FileDescriptor`
	let fd_field_offset = fd_field_index();
	let file_descriptor_ref = this.get_field_value0(fd_field_offset).expect_reference();

	FileDescriptor::fd(file_descriptor_ref)
}
