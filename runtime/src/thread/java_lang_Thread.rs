//! Utilities for interacting with `java.lang.Thread` instances

use crate::objects::instance::Instance;
use crate::objects::reference::Reference;

use common::traits::PtrType;
use instructions::Operand;

/// Value for the `java.lang.Thread$FieldHolder#status` field
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ThreadStatus {
	New = 0,
	Runnable = 1,
	Sleeping = 2,
	InObjectWait = 3,
	InObjectWaitTimed = 4,
	Parked = 5,
	ParkedTimed = 6,
	BlockedOnMonitorEnter = 7,
	Terminated = 8,
}
pub(super) fn set_eetop(obj: Reference, eetop: jni::sys::jlong) {
	let offset = crate::globals::fields::java_lang_Thread::eetop_field_offset();

	let instance = obj.extract_class();
	instance
		.get_mut()
		.put_field_value0(offset, Operand::Long(eetop));
}

/// java.lang.Thread$FieldHolder accessors
pub mod holder {
	use crate::objects::instance::Instance;
	use crate::objects::reference::Reference;
	use crate::thread::java_lang_Thread::ThreadStatus;

	use common::int_types::s4;
	use common::traits::PtrType;
	use instructions::Operand;
	use jni::sys::jlong;
	fn get_field_holder_field(obj: &Reference, offset: usize) -> Operand<Reference> {
		let class_instance = obj.extract_class();

		let field_holder_offset = crate::globals::fields::java_lang_Thread::holder_field_offset();
		let field_holder_ref = &class_instance
			.get_mut()
			.get_field_value0(field_holder_offset);

		let field_holder_instance = field_holder_ref.expect_reference().extract_class();
		field_holder_instance.get_mut().get_field_value0(offset)
	}

	pub fn stack_size(obj: &Reference) -> jlong {
		let offset = crate::globals::fields::java_lang_Thread::holder::stackSize_field_offset();
		get_field_holder_field(obj, offset).expect_long()
	}

	fn set_field_holder_field(obj: Reference, offset: usize, value: Operand<Reference>) {
		let class_instance = obj.extract_class();

		let field_holder_offset = crate::globals::fields::java_lang_Thread::holder_field_offset();
		let field_holder_ref = &class_instance
			.get_mut()
			.get_field_value0(field_holder_offset);

		let field_holder_instance = field_holder_ref.expect_reference().extract_class();
		field_holder_instance
			.get_mut()
			.put_field_value0(offset, value);
	}

	pub fn set_stack_size(obj: Reference, stack_size: jlong) {
		let offset = crate::globals::fields::java_lang_Thread::holder::stackSize_field_offset();
		set_field_holder_field(obj, offset, Operand::Long(stack_size));
	}

	pub fn set_priority(obj: Reference, priority: s4) {
		let offset = crate::globals::fields::java_lang_Thread::holder::priority_field_offset();
		set_field_holder_field(obj, offset, Operand::Int(priority));
	}

	pub(in crate::thread) fn set_daemon(_obj: Reference, _daemon: bool) {
		todo!()
	}

	pub fn set_thread_status(obj: Reference, thread_status: ThreadStatus) {
		let offset = crate::globals::fields::java_lang_Thread::holder::threadStatus_field_offset();
		set_field_holder_field(obj, offset, Operand::Int(thread_status as s4));
	}
}
