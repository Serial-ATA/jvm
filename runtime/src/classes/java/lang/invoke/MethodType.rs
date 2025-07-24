use crate::objects::instance::Instance;
use crate::objects::instance::array::ObjectArrayInstanceRef;
use crate::objects::instance::class::ClassInstanceRef;
use crate::objects::instance::mirror::MirrorInstanceRef;
use crate::objects::reference::Reference;

use classfile::FieldType;
use instructions::Operand;

/// `java.lang.invoke.MethodType#ptypes` field
pub fn ptypes(instance: ClassInstanceRef) -> ObjectArrayInstanceRef {
	instance
		.get_field_value0(ptypes_field_index())
		.expect_reference()
		.extract_object_array()
}

pub fn set_ptypes(instance: ClassInstanceRef, value: ObjectArrayInstanceRef) {
	instance.put_field_value0(
		ptypes_field_index(),
		Operand::Reference(Reference::object_array(value)),
	)
}

/// `java.lang.invoke.MethodType#ptypes` field
pub fn rtype(instance: ClassInstanceRef) -> Reference {
	instance
		.get_field_value0(rtype_field_index())
		.expect_reference()
}

pub fn set_rtype(instance: ClassInstanceRef, value: MirrorInstanceRef) {
	instance.put_field_value0(
		rtype_field_index(),
		Operand::Reference(Reference::mirror(value)),
	)
}

crate::classes::field_module! {
	@CLASS java_lang_invoke_MethodType;

	@FIELDSTART
	/// `java.lang.invoke.MethodType#ptypes` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class[]`
	@FIELD ptypes: FieldType::Array(val) if val.is_class(b"java/lang/Class"),
	/// `java.lang.invoke.MethodType#rtype` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class`
	@FIELD rtype: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
}
