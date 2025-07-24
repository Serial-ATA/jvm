use crate::classes::AsClassInstanceRef;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;

use classfile::FieldType;
use instructions::Operand;

crate::classes::field_module! {
	@CLASS java_lang_ref_Reference;

	@FIELDSTART
	/// `java.lang.ref.Reference#referent` field offset
	///
	/// Expected type: `Reference`
	@FIELD referent: FieldType::Object(_),
}

pub fn referent<I: AsClassInstanceRef>(this: I) -> Reference {
	this.as_class_instance_ref()
		.get_field_value0(referent_field_index())
		.expect_reference()
}

pub fn set_referent<I: AsClassInstanceRef>(this: I, referent: Reference) {
	this.as_class_instance_ref()
		.put_field_value0(referent_field_index(), Operand::Reference(referent));
}
