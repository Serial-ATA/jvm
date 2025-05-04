use crate::objects::class_instance::ClassInstance;
use crate::objects::instance::Instance;
use crate::objects::reference::{MirrorInstanceRef, ObjectArrayInstanceRef, Reference};
use classfile::FieldType;
use instructions::Operand;

/// `java.lang.invoke.MethodType#ptypes` field
pub fn ptypes(instance: &ClassInstance) -> ObjectArrayInstanceRef {
	instance
		.get_field_value0(ptypes_field_offset())
		.expect_reference()
		.extract_object_array()
}

pub fn set_ptypes(instance: &mut ClassInstance, value: ObjectArrayInstanceRef) {
	instance.put_field_value0(
		ptypes_field_offset(),
		Operand::Reference(Reference::object_array(value)),
	)
}

/// `java.lang.invoke.MethodType#ptypes` field
pub fn rtype(instance: &ClassInstance) -> Reference {
	instance
		.get_field_value0(rtype_field_offset())
		.expect_reference()
}

pub fn set_rtype(instance: &mut ClassInstance, value: MirrorInstanceRef) {
	instance.put_field_value0(
		rtype_field_offset(),
		Operand::Reference(Reference::mirror(value)),
	)
}

crate::classes::field_module! {
	@CLASS java_lang_invoke_MethodType;

	@FIELDSTART
	/// `java.lang.invoke.MethodType#ptypes` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class[]`
	@FIELD ptypes: FieldType::Array(ref val) if val.is_class(b"java/lang/Class"),
	/// `java.lang.invoke.MethodType#rtype` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class`
	@FIELD rtype: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
}
