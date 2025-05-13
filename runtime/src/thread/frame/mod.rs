pub mod native;
pub mod stack;

use crate::objects::constant_pool::ConstantPool;
use crate::objects::method::Method;
use crate::stack::local_stack::LocalStack;
use crate::stack::operand_stack::OperandStack;
use crate::thread::JavaThread;

use std::cell::UnsafeCell;
use std::fmt::{Debug, Formatter};
use std::mem;
use std::sync::atomic::{AtomicIsize, Ordering};

use common::int_types::{s1, s2, s4, u1, u2, u4};

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.6
#[rustfmt::skip]
pub struct Frame {
    // Each frame has:

    // its own array of local variables (§2.6.1)
	locals: LocalStack,
    // its own operand stack (§2.6.2)
	stack: OperandStack,
    // and a reference to the run-time constant pool (§2.5.5)
	constant_pool: &'static ConstantPool,
	method: &'static Method,
	thread: UnsafeCell<*const JavaThread>,
	
	// Used to remember the last pc when we return to a frame after a method invocation
	cached_pc: AtomicIsize,
	pub depth: isize,
}

impl Debug for Frame {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Frame")
			.field("locals", &self.locals)
			.field("stack", &self.stack)
			.field("method", &self.method)
			.field("cached_pc", &self.cached_pc.load(Ordering::Acquire))
			.finish()
	}
}

impl Frame {
	/// Create a new `Frame` for a [`Method`] invocation
	pub fn new(
		thread: &JavaThread,
		locals: LocalStack,
		max_stack: u2,
		method: &'static Method,
	) -> Self {
		let constant_pool = method
			.class()
			.constant_pool()
			.expect("Methods do not exist on array classes");
		Self {
			locals,
			stack: OperandStack::new(max_stack as usize),
			constant_pool,
			method,
			thread: UnsafeCell::new(&raw const *thread),
			cached_pc: AtomicIsize::default(),
			depth: 0,
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

	/// Get a reference to the constant pool
	#[inline]
	pub fn constant_pool(&self) -> &'static ConstantPool {
		self.constant_pool
	}

	/// Get a reference to the associated operand stack
	#[inline]
	pub fn stack(&self) -> &OperandStack {
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
	pub fn method(&self) -> &'static Method {
		self.method
	}

	/// Get the stashed [pc] for this frame
	///
	/// This will only be set if the current thread needs to execute a method within this frame.
	///
	/// [pc]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.5.1
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
	/// [pc]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.5.1
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
	/// [pc]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.5.1
	pub fn read_byte(&mut self) -> u1 {
		let pc;
		{
			let thread = self.thread();
			pc = thread.pc.load(Ordering::Relaxed);
		}

		let ret = self.method.code.code[(pc + self.depth) as usize];
		self.depth += 1;

		ret
	}

	/// Read 2 bytes from the associated method's code at the current [pc]
	///
	/// [pc]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.5.1
	pub fn read_byte2(&mut self) -> u2 {
		let b1 = u2::from(self.read_byte());
		let b2 = u2::from(self.read_byte());

		b1 << 8 | b2
	}

	/// Read 4 bytes from the associated method's code at the current [pc]
	///
	/// [pc]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.5.1
	pub fn read_byte4(&mut self) -> u4 {
		let b1 = u4::from(self.read_byte());
		let b2 = u4::from(self.read_byte());
		let b3 = u4::from(self.read_byte());
		let b4 = u4::from(self.read_byte());

		b1 << 24 | b2 << 16 | b3 << 8 | b4
	}

	/// Same as [`read_byte()`](Self::read_byte), casting to `s1`
	pub fn read_byte_signed(&mut self) -> s1 {
		self.read_byte() as s1
	}

	/// Same as [`read_byte2()`](Self::read_byte2), casting to `s2`
	pub fn read_byte2_signed(&mut self) -> s2 {
		self.read_byte2() as s2
	}

	/// Same as [`read_byte4()`](Self::read_byte4), casting to `s4`
	pub fn read_byte4_signed(&mut self) -> s4 {
		self.read_byte4() as s4
	}

	/// Skip padding bytes in an instruction
	///
	/// This is used in the `tableswitch` and `lookupswitch` instructions.
	pub fn skip_padding(&mut self) {
		let current_pc = self.thread().pc.load(Ordering::Relaxed) + self.depth;

		let mut pc = current_pc;
		while pc % 4 != 0 {
			pc += 1;
			self.depth += 1;
		}
	}

	pub fn take_cached_depth(&mut self) -> isize {
		mem::replace(&mut self.depth, 0)
	}

	pub fn commit_pc(&mut self, strategy: PcUpdateStrategy) {
		match strategy {
			PcUpdateStrategy::Offset(off) => {
				let _ = self.thread().pc.fetch_add(off, Ordering::Relaxed);
			},
			PcUpdateStrategy::FromInstruction => {
				let _ = self.thread().pc.fetch_add(self.depth, Ordering::Relaxed);
			},
		}

		self.depth = 0;
	}
}

pub enum PcUpdateStrategy {
	Offset(isize),
	FromInstruction,
}
