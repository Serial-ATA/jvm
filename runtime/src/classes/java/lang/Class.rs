use crate::classes::AsMirrorInstanceRef;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;

use classfile::FieldType;
use instructions::Operand;

/// `java.lang.Class#name` field
pub fn name<I: AsMirrorInstanceRef>(instance: I) -> Reference {
	instance
		.as_mirror_instance_ref()
		.get_field_value0(name_field_index())
		.expect_reference()
}

pub fn set_name<I: AsMirrorInstanceRef>(instance: I, value: Reference) {
	instance
		.as_mirror_instance_ref()
		.put_field_value0(name_field_index(), Operand::Reference(value))
}

/// `java.lang.Class#module` field
pub fn module<I: AsMirrorInstanceRef>(instance: I) -> Reference {
	instance
		.as_mirror_instance_ref()
		.get_field_value0(module_field_index())
		.expect_reference()
}

pub fn set_module<I: AsMirrorInstanceRef>(instance: I, value: Reference) {
	instance
		.as_mirror_instance_ref()
		.put_field_value0(module_field_index(), Operand::Reference(value))
}

/// `java.lang.Class#classLoader` field
pub fn classLoader<I: AsMirrorInstanceRef>(instance: I) -> Reference {
	instance
		.as_mirror_instance_ref()
		.get_field_value0(classLoader_field_index())
		.expect_reference()
}

pub fn set_classLoader<I: AsMirrorInstanceRef>(instance: I, value: Reference) {
	instance
		.as_mirror_instance_ref()
		.put_field_value0(classLoader_field_index(), Operand::Reference(value))
}

crate::classes::field_module! {
	@CLASS java_lang_Class;

	@FIELDSTART
	/// `java.lang.Class#name` field offset
	///
	/// Expected type: `Reference` to `java.lang.String`
	@FIELD name: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.Class#module` field offset
	///
	/// Expected type: `Reference` to `java.lang.Module`
	@FIELD module: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Module"),
	/// `java.lang.Class#classLoader` field offset
	///
	/// Expected type: `Reference` to `java.lang.ClassLoader`
	@FIELD classLoader: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/ClassLoader"),
	/// `java.lang.Class#classData` field offset
	///
	/// Expected type: Reference to `java.lang.Object`
	@FIELD classData: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Object"),
	/// `java.lang.Class#modifiers` field offset
	///
	/// Expected type: char
	@FIELD modifiers: FieldType::Character,
	/// `java.lang.Class#primitive` field offset
	///
	/// Expected type: boolean
	@FIELD primitive: FieldType::Boolean,
	/// `java.lang.Class#componentType` field offset
	///
	/// Expected type: `Reference` to `java.lang.Class`
	@FIELD componentType: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
}
