use crate::frame::FrameRef;
use crate::heap::class::Class;
use crate::method::Method;
use crate::reference::{MethodRef, Reference};
use crate::stack::local_stack::LocalStack;
use crate::thread::ThreadRef;
use crate::Thread;

use std::sync::Arc;

use common::int_types::{u1, u2};
use common::traits::PtrType;
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

		let local_stack = Self::construct_local_stack(max_locals, parameter_count, true, args);

		Self::invoke0_(thread, method, local_stack)
	}

	/// Invoke a method
	///
	/// This will pop the necessary number of arguments off of the current Frame's stack
	pub fn invoke(frame: FrameRef, method: MethodRef) {
		Self::invoke_(frame, method, false)
	}

	/// Invoke an interface method
	///
	/// This is identical to `MethodInvoker::invoke`, except it will attempt to find
	/// the implementation of the interface method on the `objectref` class.
	pub fn invoke_interface(frame: FrameRef, method: MethodRef) {
		Self::invoke_(frame, method, true)
	}

	/// Invoke an instance method based on class
	///
	/// This is identical to `MethodInvoker::invoke`, except it will attempt to find
	/// the implementation of the method on the `objectref` class.
	pub fn invoke_virtual(frame: FrameRef, method: MethodRef) {
		if method.is_polymorphic() {
			unimplemented!("polymorphic virtual method invocation");
		}

		Self::invoke_(frame, method, true)
	}

	fn invoke_(frame: FrameRef, mut method: MethodRef, reresolve_method: bool) {
		let mut max_locals = method.code.max_locals;
		let parameter_count = method.parameter_count;
		let is_static_method = method.access_flags & Method::ACC_STATIC != 0;

		// Move the arguments from the previous frame into a new local stack
		let mut args_from_frame = Vec::new();
		if parameter_count > 0 {
			args_from_frame = frame.get_operand_stack_mut().popn(parameter_count as usize);
		}

		// For non-static methods, the first argument will be `this`.
		// We need to check for null before proceeding.
		let mut this_argument = None;
		if !is_static_method {
			let this = frame.get_operand_stack_mut().pop_reference();
			if this == Reference::Null {
				panic!("NullPointerException")
			}

			if reresolve_method {
				method = Class::map_interface_method(this.extract_target_class(), method);
				max_locals = method.code.max_locals;
			}

			this_argument = Some(Operand::Reference(this));
		}

		let mut local_stack = Self::construct_local_stack(
			max_locals,
			parameter_count,
			is_static_method,
			args_from_frame,
		);

		if let Some(this) = this_argument {
			local_stack[0] = this;
		}

		Self::invoke0_(frame.thread(), method, local_stack);
	}

	fn invoke0_(thread: ThreadRef, method: MethodRef, local_stack: LocalStack) {
		trace_method!(method);
		Thread::invoke_method_with_local_stack(thread, method, local_stack);
	}

	fn construct_local_stack(
		max_locals: u2,
		parameter_count: u1,
		is_static_method: bool,
		existing_args: Vec<Operand<Reference>>,
	) -> LocalStack {
		let mut stack_size = max_locals;
		if max_locals == 0 {
			// A native/interface method will not have a `max_locals`, but we still need to account for
			// the parameters that get passed along.
			stack_size = u2::from(parameter_count);
		}

		if !is_static_method {
			// Add an extra slot to account for `this`
			stack_size += 1;
		}

		// We need to account for the `Empty` slots occupied by Long and Double arguments
		// before we can create our `LocalStack`
		let mut num_double_occupants = 0;
		for arg in &existing_args {
			if arg.is_long() || arg.is_double() {
				num_double_occupants += 1;
			}
		}

		stack_size += num_double_occupants;

		let mut local_stack = LocalStack::new(stack_size as usize);

		// The starting position of the arguments depends on the method being static,
		// due to us needing to reserve a spot for the `this` operand at the front of the
		// stack if it is not.
		let mut pos_in_stack = if is_static_method { 0 } else { 1 };

		for arg in existing_args
			.into_iter()
			.filter(|arg| !matches!(arg, Operand::Empty))
		{
			let mut operand_size = 1;
			if arg.is_long() || arg.is_double() {
				operand_size = 2;
			}

			local_stack[pos_in_stack] = arg;
			pos_in_stack += operand_size;
		}

		local_stack
	}
}
