use super::JavaThread;
use crate::objects::reference::Reference;
use crate::stack::local_stack::LocalStack;

use std::sync::atomic::Ordering;

use classfile::accessflags::MethodAccessFlags;
use common::traits::PtrType;
use instructions::{Operand, StackLike};
use symbols::sym;

/// See [`JavaThread::throw_exception`]
pub(super) fn throw(thread: &JavaThread, object_ref: Reference) {
	// https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-6.html#jvms-6.5.athrow
	// The objectref must be of type reference and must refer to an object that is an instance of class Throwable or of a subclass of Throwable.

	let class_instance = object_ref.extract_class();

	let throwable_class = crate::globals::classes::java_lang_Throwable();
	assert!(
		class_instance.get().class() == throwable_class
			|| class_instance.get().is_subclass_of(&throwable_class)
	);

	// Search each frame for an exception handler
	thread.stash_and_reset_pc();
	while let Some(current_frame) = thread.frame_stack.current() {
		let current_frame_pc = current_frame.stashed_pc();

		// If an exception handler that matches objectref is found, it contains the location of the code intended to handle this exception.
		if let Some(handler_pc) = current_frame
			.method()
			.find_exception_handler(class_instance.get().class(), current_frame_pc)
		{
			// The pc register is reset to that location,the operand stack of the current frame is cleared, objectref
			// is pushed back onto the operand stack, and execution continues.
			thread.pc.store(handler_pc, Ordering::Relaxed);

			let stack = current_frame.stack_mut();
			stack.clear();
			stack.push_reference(object_ref);

			return;
		}

		let _ = thread.frame_stack.pop();
	}

	// No handler found, we have to print the stack trace and exit
	thread.frame_stack.clear();
	print_stack_trace(thread, object_ref);
}

fn print_stack_trace(thread: &JavaThread, object_ref: Reference) {
	let print_stack_trace = object_ref
		.extract_class()
		.get()
		.class()
		.vtable()
		.find(
			sym!(printStackTrace_name),
			sym!(void_method_signature),
			MethodAccessFlags::NONE,
		)
		.expect("java/lang/Throwable#printStackTrace should exist");

	let mut locals = LocalStack::new(1);
	locals[0] = Operand::Reference(object_ref);

	thread.invoke_method_with_local_stack(print_stack_trace, locals);
}
