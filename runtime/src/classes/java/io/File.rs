use crate::classes;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;

use classfile::FieldType;

crate::classes::field_module! {
	@CLASS java_io_File;

	@FIELDSTART
	/// `java.io.File#path` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD path: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
}

pub fn path(this: Reference) -> String {
	let path_field_offset = path_field_index();
	let f = this.extract_class();
	let value = f.get_field_value0(path_field_offset).expect_reference();

	classes::java::lang::String::extract(value.extract_class())
}
