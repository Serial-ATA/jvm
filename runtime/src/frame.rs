use crate::method::Method;
use crate::stack::local_stack::LocalStack;
use crate::stack::operand_stack::OperandStack;
use crate::thread::JavaThread;

use std::cell::UnsafeCell;
use std::fmt::{Debug, Formatter};
use std::sync::atomic::{AtomicIsize, Ordering};

use classfile::ConstantPoolRef;
use common::int_types::{s1, s2, s4, u1, u2, u4};

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.6
#[rustfmt::skip]
pub struct Frame {
    // Each frame has:

    // its own array of local variables (§2.6.1)
	locals: LocalStack,
    // its own operand stack (§2.6.2)
	stack: OperandStack,
    // and a reference to the run-time constant pool (§2.5.5)
	constant_pool: ConstantPoolRef,
	method: &'static Method,
	thread: UnsafeCell<*mut JavaThread>,
	
	// Used to remember the last pc when we return to a frame after a method invocation
	cached_pc: AtomicIsize,
}

impl Debug for Frame {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Frame")
			.field("locals", &self.locals)
			.field("stack", &self.stack)
			.field("method", &self.method)
			.field("cached_pc", &self.cached_pc)
			.finish()
	}
}

impl Frame {
	/// Create a new `Frame` for a [`Method`] invocation
	pub fn new(
		thread: &mut JavaThread,
		locals: LocalStack,
		max_stack: u2,
		constant_pool: ConstantPoolRef,
		method: &'static Method,
	) -> Self {
		Self {
			locals,
			stack: OperandStack::new(max_stack as usize),
			constant_pool,
			method,
			thread: UnsafeCell::new(&raw mut *thread),
			cached_pc: AtomicIsize::default(),
		}
	}
}

// Getters
impl Frame {
	/// Get a reference to the associated [`JavaThread`]
	#[inline]
	pub fn thread(&self) -> &JavaThread {
		unsafe { &**self.thread.get() }
	}

	/// Get a mutable reference to the associated [`JavaThread`]
	#[inline]
	pub fn thread_mut(&self) -> &mut JavaThread {
		unsafe { &mut **self.thread.get() }
	}

	/// Get a reference to the associated operand stack
	#[inline]
	pub fn stack(&mut self) -> &OperandStack {
		&self.stack
	}

	/// Get a mutable reference to the associated operand stack
	#[inline]
	pub fn stack_mut(&mut self) -> &mut OperandStack {
		&mut self.stack
	}

	/// Get a reference to the associated local variables
	#[inline]
	pub fn local_stack(&self) -> &LocalStack {
		&self.locals
	}

	/// Get a mutable reference to the associated local variables
	#[inline]
	pub fn local_stack_mut(&mut self) -> &mut LocalStack {
		&mut self.locals
	}

	/// Get the method associated with this frame
	#[inline]
	pub fn method(&self) -> &Method {
		self.method
	}

	/// Get the stashed [pc] for this frame
	///
	/// This will only be set if the current thread needs to execute a method within this frame.
	///
	/// [pc]: https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.5.1
	pub fn stashed_pc(&self) -> isize {
		self.cached_pc.load(Ordering::Relaxed)
	}
}

// Setters
impl Frame {
	/// Stash the current [pc] for later
	///
	/// This is used when a new frame is added to the thread's stack.
	///
	/// [pc]: https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.5.1
	pub fn stash_pc(&self) {
		let current_pc;
		{
			let thread = self.thread();
			current_pc = thread.pc.load(Ordering::Relaxed);
		}

		self.cached_pc.store(current_pc, Ordering::Relaxed);
	}
}

// Reading
impl Frame {
	/// Read a byte from the associated method's code at the current [pc]
	///
	/// [pc]: https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.5.1
	pub fn read_byte(&self) -> u1 {
		let pc;
		{
			let thread = self.thread();
			pc = thread.pc.fetch_add(1, Ordering::Relaxed);
		}

		self.method.code.code[pc as usize]
	}

	/// Read 2 bytes from the associated method's code at the current [pc]
	///
	/// [pc]: https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.5.1
	pub fn read_byte2(&self) -> u2 {
		let b1 = u2::from(self.read_byte());
		let b2 = u2::from(self.read_byte());

		b1 << 8 | b2
	}

	/// Read 4 bytes from the associated method's code at the current [pc]
	///
	/// [pc]: https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.5.1
	pub fn read_byte4(&self) -> u4 {
		let b1 = u4::from(self.read_byte());
		let b2 = u4::from(self.read_byte());
		let b3 = u4::from(self.read_byte());
		let b4 = u4::from(self.read_byte());

		b1 << 24 | b2 << 16 | b3 << 8 | b4
	}

	/// Same as [`read_byte()`](Self::read_byte), casting to `s1`
	pub fn read_byte_signed(&self) -> s1 {
		self.read_byte() as s1
	}

	/// Same as [`read_byte2()`](Self::read_byte2), casting to `s2`
	pub fn read_byte2_signed(&self) -> s2 {
		self.read_byte2() as s2
	}

	/// Same as [`read_byte4()`](Self::read_byte4), casting to `s4`
	pub fn read_byte4_signed(&self) -> s4 {
		self.read_byte4() as s4
	}

	/// Skip padding bytes in an instruction
	///
	/// This is used in the `tableswitch` and `lookupswitch` instructions.
	pub fn skip_padding(&self) {
		let thread = self.thread();

		let mut pc = thread.pc.load(Ordering::Relaxed);
		while pc % 4 != 0 {
			pc += 1;
		}

		thread.pc.store(pc, Ordering::Relaxed);
	}
}
