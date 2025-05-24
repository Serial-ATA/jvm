use super::FileDescriptor;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;

use classfile::FieldType;
use jni::sys::jint;

crate::classes::field_module! {
	@CLASS java_io_FileInputStream;

	@FIELDSTART
	/// `java.io.FileInputStream#fd` field offset
	///
	/// Expected field type: `Reference` to `java.io.FileDescriptor`
	@FIELD fd: ty @ FieldType::Object(_) if ty.is_class(b"java/io/FileDescriptor"),
}

fn fd_field(this: Reference) -> Reference {
	// `fd` is a reference to a `java.io.FileDescriptor`
	let fd_field_offset = fd_field_offset();
	this.get_field_value0(fd_field_offset).expect_reference()
}

pub fn fd(this: Reference) -> jint {
	let file_descriptor_ref = fd_field(this);
	FileDescriptor::fd(file_descriptor_ref)
}

pub fn set_fd(this: Reference, fd: jint) {
	let file_descriptor_ref = fd_field(this);
	FileDescriptor::set_fd(file_descriptor_ref, fd);
}
