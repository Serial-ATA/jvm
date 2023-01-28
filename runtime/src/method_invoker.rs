use crate::frame::FrameRef;
use crate::method::Method;
use crate::reference::{MethodRef, Reference};
use crate::stack::local_stack::LocalStack;
use crate::thread::ThreadRef;
use crate::Thread;

use common::int_types::{u1, u2};
use instructions::{Operand, StackLike};

macro_rules! trace_method {
	($method:ident) => {{
		#[cfg(debug_assertions)]
		{
			log::trace!("[METHOD CALL] {:?}", $method);
		}
	}};
}

pub struct MethodInvoker;

impl MethodInvoker {
	/// Invoke a method with the provided args
	///
	/// This will not pop anything off of the stack of the current Frame
	pub fn invoke_with_args(thread: ThreadRef, method: MethodRef, args: Vec<Operand<Reference>>) {
		let max_locals = method.code.max_locals;
		let parameter_count = method.parameter_count;
		let is_native_method = method.access_flags & Method::ACC_NATIVE != 0;

		let local_stack =
			Self::construct_local_stack(max_locals, parameter_count, true, is_native_method, args);

		Self::invoke_(thread, method, local_stack)
	}

	/// Invoke a method
	///
	/// This will pop the necessary number of arguments off of the current Frame's stack
	pub fn invoke(frame: FrameRef, method: MethodRef) {
		let max_locals = method.code.max_locals;
		let parameter_count = method.parameter_count;
		let is_static_method = method.access_flags & Method::ACC_STATIC != 0;
		let is_native_method = method.access_flags & Method::ACC_NATIVE != 0;

		// Move the arguments from the previous frame into a new local stack
		let mut args_from_frame = Vec::new();
		if parameter_count > 0 {
			args_from_frame = frame.get_operand_stack_mut().popn(parameter_count as usize);
		}

		let mut local_stack = Self::construct_local_stack(
			max_locals,
			parameter_count,
			is_static_method,
			is_native_method,
			args_from_frame,
		);

		// For non-static methods, the first argument will be `this`.
		// We need to check for null before proceeding.
		if !is_static_method {
			let this = frame.get_operand_stack_mut().pop_reference();
			if this == Reference::Null {
				panic!("NullPointerException")
			}

			local_stack[0] = Operand::Reference(this);
		}

		Self::invoke_(frame.thread(), method, local_stack);
	}

	fn invoke_(thread: ThreadRef, method: MethodRef, local_stack: LocalStack) {
		trace_method!(method);
		Thread::invoke_method_with_local_stack(thread, method, local_stack);
	}

	fn construct_local_stack(
		max_locals: u2,
		parameter_count: u1,
		is_static_method: bool,
		is_native_method: bool,
		existing_args: Vec<Operand<Reference>>,
	) -> LocalStack {
		let mut stack_size = max_locals;
		if is_native_method && max_locals == 0 {
			// A native method will not have a `max_locals`, but we still need to account for
			// the parameters that get passed along.
			stack_size = u2::from(parameter_count);
		}

		if !is_static_method {
			// Add an extra slot to account for `this`
			stack_size += 1;
		}

		let mut local_stack = LocalStack::new(stack_size as usize);

		// The starting position of the arguments depends on the method being static,
		// due to us needing to reserve a spot for the `this` operand at the front of the
		// stack if it is not.
		let mut pos_in_stack = if is_static_method { 0 } else { 1 };

		for arg in existing_args {
			let operand_size = match arg {
				Operand::Double(_) | Operand::Long(_) => 2,
				_ => 1,
			};

			local_stack[pos_in_stack] = arg;
			pos_in_stack += operand_size;
		}

		local_stack
	}
}
