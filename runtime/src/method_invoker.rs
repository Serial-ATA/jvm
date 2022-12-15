use crate::frame::FrameRef;
use crate::method::Method;
use crate::reference::{MethodRef, Reference};
use crate::stack::local_stack::LocalStack;
use crate::stack::operand_stack::Operand;
use crate::thread::ThreadRef;
use crate::Thread;

use instructions::StackLike;

pub struct MethodInvoker;

impl MethodInvoker {
	/// Invoke a method with the provided args
	///
	/// This will not pop anything off of the stack of the current Frame
	pub fn invoke_with_args(thread: ThreadRef, method: MethodRef, args: Vec<Operand>) {
		let mut local_stack = LocalStack::new(method.code.max_locals as usize);
		Self::construct_local_stack(&mut local_stack, args, true);

		Self::invoke_(thread, method, local_stack)
	}

	/// Invoke a method
	///
	/// This will pop the necessary number of arguments off of the current Frame's stack
	pub fn invoke(frame: FrameRef, method: MethodRef) {
		let is_static_method = method.access_flags & Method::ACC_STATIC != 0;
		let parameter_count = method.descriptor.parameters.len();

		let mut local_stack = LocalStack::new(method.code.max_locals as usize);

		// Move the arguments from the previous frame into a new local stack
		if parameter_count > 0 {
			let args_from_frame = frame.get_operand_stack_mut().popn(parameter_count);
			Self::construct_local_stack(&mut local_stack, args_from_frame, is_static_method);
		}

		// For non-static methods, the first argument will be `this`.
		// We need to check for null before proceeding.
		if !is_static_method {
			let this = frame.get_operand_stack_mut().pop_reference();
			match this {
				Reference::Null => panic!("NullPointerException"), // TODO
				_ => {},
			}

			local_stack[0] = Operand::Reference(this);
		}

		Self::invoke_(frame.thread(), method, local_stack);
	}

	fn invoke_(thread: ThreadRef, method: MethodRef, local_stack: LocalStack) {
		Thread::invoke_method_with_local_stack(thread, method, local_stack);
	}

	fn construct_local_stack(
		local_stack: &mut LocalStack,
		existing_args: Vec<Operand>,
		is_static_method: bool,
	) {
		// The starting position of the arguments depends on the method being static,
		// due to us needing to reserve a spot for the `this` operand at the front of the
		// stack if it is not.
		let mut pos_in_stack = match is_static_method {
			true => 0,
			false => 1,
		};

		for arg in existing_args {
			let operand_size = match arg {
				Operand::Double(_) | Operand::Long(_) => 2,
				_ => 1,
			};

			local_stack[pos_in_stack] = arg;
			pos_in_stack += operand_size;
		}
	}
}
