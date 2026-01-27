pub mod native;
pub mod stack;

use crate::objects::constant_pool::ConstantPool;
use crate::objects::method::Method;
use crate::stack::local_stack::LocalStack;
use crate::stack::operand_stack::OperandStack;
use crate::thread::JavaThread;
use crate::thread::exceptions::{ExceptionKind, Throws};

use std::cell::UnsafeCell;
use std::fmt::{Debug, Formatter};
use std::sync::atomic::{AtomicIsize, Ordering};

use common::int_types::{s1, s2, s4, u1, u2, u4};
use instructions::StackLike;

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

    // Fields outside the spec:

    method: &'static Method,
	thread: UnsafeCell<*const JavaThread>,
	
	// Used to remember the last pc when we return to a frame after a method invocation
	cached_pc: AtomicIsize,

    // TODO: depth should never be > 5, could be packed with flags
    // Extra depth within the current instruction
    //
    // When parsing a bytecode instruction, `pc` stays at the beginning of the instruction. This keeps
    // track of any additional bytes we read *after* that bytecode (e.g. arguments for the instruction).
    //
    // The depth is used at the end of an instruction to calculate the offset to the next instruction.
	depth: u16,
    flags: u8,
}

impl Debug for Frame {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Frame")
			.field("locals", &self.locals)
			.field("stack", &self.stack)
			.field("method", &self.method)
			.field("cached_pc", &self.cached_pc.load(Ordering::Acquire))
			.finish_non_exhaustive()
	}
}

// Flags
impl Frame {
	const IN_TAIL_CALL: u8 = 0b1;

	pub fn in_tail_call(&self) -> bool {
		self.flags & Self::IN_TAIL_CALL != 0
	}
}

impl Frame {
	/// Create a new `Frame` for a [`Method`] invocation
	///
	/// # Exceptions
	///
	/// This may throw [`ExceptionKind::OutOfMemoryError`] when allocating the [`OperandStack`].
	pub fn new(
		thread: &'static JavaThread,
		locals: LocalStack,
		max_stack: u2,
		method: &'static Method,
	) -> Throws<Self> {
		let constant_pool = method
			.class()
			.constant_pool()
			.expect("Methods do not exist on array classes");
		let stack = OperandStack::new(max_stack as usize)?;
		Throws::Ok(Self {
			locals,
			stack,
			constant_pool,
			method,
			thread: UnsafeCell::new(&raw const *thread),
			cached_pc: AtomicIsize::default(),
			depth: 0,
			flags: 0,
		})
	}

	/// Reuse this frame for a tail method call
	///
	/// This will replace the original [`LocalStack`] and return it. It must be retained and used in
	/// a subsequent call to [`Self::reset_from_tail_call()`].
	///
	/// # Safety
	///
	/// The current [`OperandStack`] is retained (including its current position), so the stack
	/// ***must*** be setup correctly for the target `method`.
	pub(in crate::thread) unsafe fn swap_for_tail_call(
		&mut self,
		method: &'static Method,
	) -> LocalStack {
		assert!(method.parameter_count() as usize <= self.locals.total_slots());
		assert!(!self.has_stashed_depth());

		let mut parameter_count = method.parameter_count() as usize;
		if !method.is_static() {
			// receiver
			parameter_count += 1;
		}

		let locals = unsafe {
			LocalStack::new_with_args(
				self.stack_mut().popn(parameter_count),
				method.code.max_locals as usize,
			)
		};

		let old_locals = core::mem::replace(&mut self.locals, locals);
		self.constant_pool = method
			.class()
			.constant_pool()
			.expect("Methods do not exist on array classes");

		self.depth = self.depth << 8;
		self.method = method;
		self.flags |= Self::IN_TAIL_CALL;

		old_locals
	}

	/// Restore this frame to its state prior to a tail call
	///
	/// NOTE: The [`OperandStack`] will be left in whatever state the prior method returned with.
	pub(in crate::thread) fn reset_from_tail_call(
		&mut self,
		old_locals: LocalStack,
		method: &'static Method,
	) {
		self.locals = old_locals;
		self.constant_pool = method
			.class()
			.constant_pool()
			.expect("Methods do not exist on array classes");
		self.depth = self.depth >> 8;
		self.method = method;
		self.flags |= !Self::IN_TAIL_CALL;
	}
}

// Getters
impl Frame {
	/// Get a reference to the associated [`JavaThread`]
	#[inline]
	pub fn thread(&self) -> &'static JavaThread {
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

	fn depth(&self) -> isize {
		(self.depth & 0b1111_1111) as isize
	}

	fn inc_depth(&mut self) {
		assert!(self.depth() <= u8::MAX as isize);
		self.depth = (self.depth & 0xFF00) | ((self.depth & 0x00FF) + 1);
	}

	fn has_stashed_depth(&self) -> bool {
		(self.depth >> 8) > 0
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

		let ret = self.method.code.code[(pc + self.depth()) as usize];
		self.inc_depth();

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
		let current_pc = self.thread().pc.load(Ordering::Relaxed) + self.depth();

		let mut pc = current_pc;
		while pc % 4 != 0 {
			pc += 1;
			self.inc_depth();
		}
	}

	pub fn take_cached_depth(&mut self) -> isize {
		let depth = self.depth();
		self.depth = 0;

		depth
	}

	/// Commit the [pc] to the current [`JavaThread`]
	///
	/// See [`PcUpdateStrategy`].
	///
	/// [pc]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.5.1
	pub fn commit_pc(&mut self, strategy: PcUpdateStrategy) {
		match strategy {
			PcUpdateStrategy::Offset(off) => {
				let _ = self.thread().pc.fetch_add(off, Ordering::Relaxed);
			},
			PcUpdateStrategy::FromInstruction => {
				let _ = self.thread().pc.fetch_add(self.depth(), Ordering::Relaxed);
			},
		}

		self.depth = 0;
	}
}

#[derive(Copy, Clone, Debug)]
pub enum PcUpdateStrategy {
	/// Update the pc by `offset` bytes
	Offset(isize),
	/// Update the pc to point to the beginning of the current instruction
	FromInstruction,
}
