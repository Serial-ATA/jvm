//! Utilities for interacting with `java.lang.Thread` instances

use crate::objects::class_instance::ClassInstance;
use crate::objects::instance::Instance;
use crate::objects::reference::{ClassInstanceRef, Reference};
use classfile::FieldType;
use instructions::Operand;
use jni::sys::jlong;

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

impl ThreadStatus {
	const MAX: ThreadStatus = ThreadStatus::Terminated;
}

pub fn eetop(instance: &ClassInstance) -> jlong {
	instance
		.get_field_value0(eetop_field_offset())
		.expect_long()
}

pub fn set_eetop(instance: &mut ClassInstance, value: jlong) {
	instance.put_field_value0(eetop_field_offset(), Operand::Long(value))
}

/// `java.lang.Thread#name` field
pub fn name(instance: &ClassInstance) -> Reference {
	instance
		.get_field_value0(name_field_offset())
		.expect_reference()
}

pub fn set_name(instance: &mut ClassInstance, value: ClassInstanceRef) {
	instance.put_field_value0(
		name_field_offset(),
		Operand::Reference(Reference::class(value)),
	)
}

/// `java.lang.Thread#holder` field
pub fn holder(instance: &ClassInstance) -> Reference {
	instance
		.get_field_value0(holder_field_offset())
		.expect_reference()
}

crate::classes::field_module! {
	@CLASS java_lang_Thread;
	@SUBCLASS holder;

	@FIELDSTART
	/// `java.lang.Thread#eetop` field offset
	///
	/// Expected type: `jlong`
	@FIELD eetop: FieldType::Long,
	/// `java.lang.Thread#name` field offset
	///
	/// Expected type: `Reference` to `java.lang.String`
	@FIELD name: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.Thread#holder` field offset
	///
	/// Expected type: `Reference` to `java.lang.Thread$FieldHolder`
	@FIELD holder: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Thread$FieldHolder"),
}

pub mod holder {
	use super::*;
	use common::int_types::s4;
	use instructions::Operand;
	use jni::sys::{jint, jlong};

	pub fn stackSize(instance: &ClassInstance) -> jlong {
		instance
			.get_field_value0(stackSize_field_offset())
			.expect_long()
	}

	pub fn set_stackSize(instance: &mut ClassInstance, stack_size: jlong) {
		instance.put_field_value0(stackSize_field_offset(), Operand::Long(stack_size));
	}

	pub fn priority(instance: &ClassInstance) -> jint {
		instance
			.get_field_value0(priority_field_offset())
			.expect_int()
	}

	pub fn set_priority(instance: &mut ClassInstance, priority: jint) {
		instance.put_field_value0(priority_field_offset(), Operand::Int(priority));
	}

	pub fn daemon(instance: &ClassInstance) -> bool {
		instance
			.get_field_value0(daemon_field_offset())
			.expect_int()
			!= 0
	}

	pub fn set_daemon(instance: &mut ClassInstance, daemon: bool) {
		instance.put_field_value0(daemon_field_offset(), Operand::Int(daemon as s4));
	}

	pub fn threadStatus(instance: &ClassInstance) -> ThreadStatus {
		let value = instance
			.get_field_value0(threadStatus_field_offset())
			.expect_int();

		// TODO: Would be nice to not panic here
		assert!(value <= ThreadStatus::MAX as s4);
		unsafe { std::mem::transmute(value) }
	}

	pub fn set_threadStatus(instance: &mut ClassInstance, thread_status: ThreadStatus) {
		instance.put_field_value0(
			threadStatus_field_offset(),
			Operand::Int(thread_status as s4),
		);
	}

	crate::classes::field_module! {
		@CLASS java_lang_Thread_FieldHolder;

		@FIELDSTART
		/// `java.lang.Thread$FieldHolder#stackSize` field offset
		///
		/// Expected field type: `jlong`
		@FIELD stackSize: FieldType::Long,
		/// `java.lang.Thread$FieldHolder#priority` field offset
		///
		/// Expected field type: `jint`
		@FIELD priority: FieldType::Integer,
		/// `java.lang.Thread$FieldHolder#daemon` field offset
		///
		/// Expected field type: `jboolean`
		@FIELD daemon: FieldType::Boolean,
		/// `java.lang.Thread$FieldHolder#threadStatus` field offset
		///
		/// Expected field type: `jint`
		@FIELD threadStatus: FieldType::Integer,
	}
}
