use crate::objects::instance::Instance;
use crate::objects::reference::Reference;

use classfile::FieldType;
use common::traits::PtrType;
use instructions::Operand;

crate::classes::field_module! {
	@CLASS java_lang_ref_Reference;

	@FIELDSTART
	/// `java.lang.ref.Reference#referent` field offset
	///
	/// Expected type: `Reference`
	@FIELD referent: FieldType::Object(_),
}

pub fn referent(this: Reference) -> Reference {
	this.get_field_value0(referent_field_offset())
		.expect_reference()
}

pub fn set_referent(this: Reference, referent: Reference) {
	this.extract_class()
		.get_mut()
		.put_field_value0(referent_field_offset(), Operand::Reference(referent));
}
