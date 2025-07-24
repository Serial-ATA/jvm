//! Utilities for interacting with `java.lang.Thread` instances

use crate::objects::instance::Instance;
use crate::objects::instance::class::ClassInstanceRef;
use crate::objects::reference::Reference;

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

pub fn eetop(instance: ClassInstanceRef) -> jlong {
	instance.get_field_value0(eetop_field_index()).expect_long()
}

pub fn set_eetop(instance: ClassInstanceRef, value: jlong) {
	instance.put_field_value0(eetop_field_index(), Operand::Long(value))
}

/// `java.lang.Thread#name` field
pub fn name(instance: ClassInstanceRef) -> Reference {
	instance
		.get_field_value0(name_field_index())
		.expect_reference()
}

pub fn set_name(instance: ClassInstanceRef, value: ClassInstanceRef) {
	instance.put_field_value0(
		name_field_index(),
		Operand::Reference(Reference::class(value)),
	)
}

/// `java.lang.Thread#holder` field
pub fn holder(instance: ClassInstanceRef) -> Reference {
	instance
		.get_field_value0(holder_field_index())
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

	pub fn stackSize(instance: ClassInstanceRef) -> jlong {
		instance
			.get_field_value0(stackSize_field_index())
			.expect_long()
	}

	pub fn set_stackSize(instance: ClassInstanceRef, stack_size: jlong) {
		instance.put_field_value0(stackSize_field_index(), Operand::Long(stack_size));
	}

	pub fn priority(instance: ClassInstanceRef) -> jint {
		instance
			.get_field_value0(priority_field_index())
			.expect_int()
	}

	pub fn set_priority(instance: ClassInstanceRef, priority: jint) {
		instance.put_field_value0(priority_field_index(), Operand::Int(priority));
	}

	pub fn daemon(instance: ClassInstanceRef) -> bool {
		instance.get_field_value0(daemon_field_index()).expect_int() != 0
	}

	pub fn set_daemon(instance: ClassInstanceRef, daemon: bool) {
		instance.put_field_value0(daemon_field_index(), Operand::Int(daemon as s4));
	}

	pub fn threadStatus(instance: ClassInstanceRef) -> ThreadStatus {
		let value = instance
			.get_field_value0(threadStatus_field_index())
			.expect_int();

		// TODO: Would be nice to not panic here
		assert!(value <= ThreadStatus::MAX as s4);
		unsafe { std::mem::transmute(value) }
	}

	pub fn set_threadStatus(instance: ClassInstanceRef, thread_status: ThreadStatus) {
		instance.put_field_value0(
			threadStatus_field_index(),
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
