//! Methods for interacting with `java.lang.Thread` instances

use crate::objects::class_instance::Instance;
use crate::objects::reference::Reference;

use common::traits::PtrType;
use instructions::Operand;
use symbols::sym;

pub fn set_field_offsets() {
	// java.lang.Thread fields
	{
		let class = crate::globals::classes::java_lang_Thread();

		let mut field_set = 0;
		for (index, field) in class.instance_fields().enumerate() {
			if field.name == sym!(holder) {
				unsafe {
					crate::globals::field_offsets::java_lang_Thread::set_holder_field_offset(index);
				}

				field_set |= 1;
				continue;
			}

			if field.name == sym!(eetop) {
				unsafe {
					crate::globals::field_offsets::java_lang_Thread::set_eetop_field_offset(index);
				}

				field_set |= 1 << 1;
				continue;
			}
		}

		assert_eq!(
			field_set, 0b11,
			"Not all fields were found in java/lang/Thread"
		);
	}

	holder::set_field_holder_offsets();
}

pub(super) fn set_eetop(obj: Reference, eetop: jni::sys::jlong) {
	let offset = crate::globals::field_offsets::java_lang_Thread::eetop_field_offset();

	let instance = obj.extract_class();
	instance
		.get_mut()
		.put_field_value0(offset, Operand::Long(eetop));
}

/// java.lang.Thread$FieldHolder accessors
pub mod holder {
	use crate::objects::class_instance::Instance;
	use crate::objects::reference::Reference;
	use crate::thread::ThreadStatus;

	use common::int_types::s4;
	use common::traits::PtrType;
	use instructions::Operand;
	use jni::sys::jlong;

	pub(super) fn set_field_holder_offsets() {
		let class = crate::globals::classes::java_lang_Thread_FieldHolder();

		let mut field_set = 0;
		for (index, field) in class.fields().enumerate() {
			match field.name.as_str() {
				"stackSize" => {
					unsafe {
						crate::globals::field_offsets::java_lang_Thread::holder::set_stack_size_field_offset(
							index,
						);
					}
					field_set |= 1;
				},
				"priority" => {
					unsafe {
						crate::globals::field_offsets::java_lang_Thread::holder::set_priority_field_offset(
							index,
						);
					}
					field_set |= 1 << 1;
				},
				"daemon" => {
					unsafe {
						crate::globals::field_offsets::java_lang_Thread::holder::set_daemon_field_offset(index);
					}
					field_set |= 1 << 2;
				},
				"threadStatus" => {
					unsafe {
						crate::globals::field_offsets::java_lang_Thread::holder::set_thread_status_field_offset(
							index,
						);
					}
					field_set |= 1 << 3;
				},
				_ => {},
			}
		}

		assert_eq!(
			field_set, 0b1111,
			"Not all fields were found in java/lang/Thread$FieldHolder"
		);
	}

	fn get_field_holder_field(obj: &Reference, offset: usize) -> Operand<Reference> {
		let class_instance = obj.extract_class();

		let field_holder_offset =
			crate::globals::field_offsets::java_lang_Thread::holder_field_offset();
		let field_holder_ref = &class_instance
			.get_mut()
			.get_field_value0(field_holder_offset);

		let field_holder_instance = field_holder_ref.expect_reference().extract_class();
		field_holder_instance.get_mut().get_field_value0(offset)
	}

	pub fn stack_size(obj: &Reference) -> jlong {
		let offset =
			crate::globals::field_offsets::java_lang_Thread::holder::stack_size_field_offset();
		get_field_holder_field(obj, offset).expect_long()
	}

	fn set_field_holder_field(obj: Reference, offset: usize, value: Operand<Reference>) {
		let class_instance = obj.extract_class();

		let field_holder_offset =
			crate::globals::field_offsets::java_lang_Thread::holder_field_offset();
		let field_holder_ref = &class_instance
			.get_mut()
			.get_field_value0(field_holder_offset);

		let field_holder_instance = field_holder_ref.expect_reference().extract_class();
		field_holder_instance
			.get_mut()
			.put_field_value0(offset, value);
	}

	pub fn set_stack_size(obj: Reference, stack_size: jlong) {
		let offset =
			crate::globals::field_offsets::java_lang_Thread::holder::stack_size_field_offset();
		set_field_holder_field(obj, offset, Operand::Long(stack_size));
	}

	pub fn set_priority(obj: Reference, priority: s4) {
		let offset =
			crate::globals::field_offsets::java_lang_Thread::holder::priority_field_offset();
		set_field_holder_field(obj, offset, Operand::Int(priority));
	}

	pub(in crate::thread) fn set_daemon(_obj: Reference, _daemon: bool) {
		todo!()
	}

	pub fn set_thread_status(obj: Reference, thread_status: ThreadStatus) {
		let offset =
			crate::globals::field_offsets::java_lang_Thread::holder::thread_status_field_offset();
		set_field_holder_field(obj, offset, Operand::Int(thread_status as s4));
	}
}
