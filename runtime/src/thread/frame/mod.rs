pub mod native;
pub mod stack;

use crate::objects::constant_pool::ConstantPool;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::thread::exceptions::{Throws, throw};
use crate::thread::{JavaThread, JavaThreadState};

use std::fmt::{Debug, Formatter};
use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};

use common::int_types::{s1, s2, s4, s8, u1, u2, u4};
use instructions::{Operand, StackLike};

#[repr(C)]
struct Stash {
	/// The current pc
	pc: AtomicIsize,
	/// The current stack pointer
	sp: AtomicUsize,
}

impl Debug for Stash {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Stash")
			.field("pc", &self.pc.load(Ordering::Acquire))
			.field("sp", &self.sp.load(Ordering::Acquire))
			.finish()
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.6
#[rustfmt::skip]
pub struct Frame {
    // Each frame has:

    // its own array of local variables (§2.6.1)
	locals_base: usize,
    // its own operand stack (§2.6.2)
	stack_base: usize,
    // and a reference to the run-time constant pool (§2.5.5)
	constant_pool: &'static ConstantPool,

    // Fields outside the spec:

    method: &'static Method,
	thread: *const JavaThread,
	
	stash: Stash,

    // Extra depth within the current instruction
    //
    // When parsing a bytecode instruction, `pc` stays at the beginning of the instruction. This keeps
    // track of any additional bytes we read *after* that bytecode (e.g. arguments for the instruction).
    //
    // The depth is used at the end of an instruction to calculate the offset to the next instruction.
	depth: u16,
}

impl Drop for Frame {
	fn drop(&mut self) {
		let thread = self.thread();
		if thread.state() != JavaThreadState::Unwinding {
			// Drop the stack pointer back to the caller's stack
			self.thread().stack().set_stack_pointer(self.locals_base);
		}
	}
}

impl Debug for Frame {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut top_of_stack = self.stash.sp.load(Ordering::Acquire);
		if top_of_stack == 0 {
			// Printing the current frame, so we can use the current sp
			top_of_stack = self.thread().stack().len();
		}

		f.debug_struct("Frame")
			.field("method", &self.method)
			.field("stash", &self.stash)
			.field(
				"locals",
				&self
					.thread()
					.stack()
					.slice(self.locals_base, self.method.code.max_locals as usize),
			)
			.field(
				"stack",
				&self
					.thread()
					.stack()
					.slice(self.stack_base, top_of_stack - self.stack_base),
			)
			.finish_non_exhaustive()
	}
}

impl Frame {
	/// Create a new `Frame` for a [`Method`] invocation
	///
	/// # Exceptions
	///
	/// This may throw [`ExceptionKind::OutOfMemoryError`]
	pub fn new(thread: &'static JavaThread, method: &'static Method) -> Throws<Self> {
		let constant_pool = method
			.class()
			.constant_pool()
			.expect("Methods do not exist on array classes");

		if thread.stack().remaining() < method.code.max_stack as usize {
			throw!(@DEFER StackOverflowError);
		}

		// The parameters are already on the stack
		// NOTE: `parameter_stack_size()` includes the receiver
		let locals_base = thread.stack().len() - method.parameter_stack_size();
		for _ in 0..method.code.max_locals as usize - method.parameter_stack_size() {
			thread.stack().push_op(Operand::Empty);
		}

		let stack_base = thread.stack().len();

		Throws::Ok(Self {
			locals_base,
			stack_base,
			constant_pool,
			method,
			thread: &raw const *thread,
			stash: Stash {
				pc: AtomicIsize::default(),
				sp: AtomicUsize::default(),
			},
			depth: 0,
		})
	}

	pub fn tail(&mut self, method: &'static Method) {
		let parameter_slots = method.parameter_stack_size();

		let thread = self.thread();

		let args_base = thread.stack().len() - parameter_slots;

		for i in 0..parameter_slots {
			let arg = thread.stack().absolute(args_base + i);
			thread.stack().set_absolute(self.locals_base + i, arg);
		}

		// Truncate the stack pointer to right after the shifted arguments
		thread
			.stack()
			.set_stack_pointer(self.locals_base + parameter_slots);
		for _ in 0..(method.code.max_locals as usize - parameter_slots) {
			thread.stack().push_op(Operand::Empty);
		}

		thread.pc.store(0, Ordering::Relaxed);
		self.depth = 0;
		self.stack_base = self.locals_base + method.code.max_locals as usize;
		self.method = method;
		self.constant_pool = method
			.class()
			.constant_pool()
			.expect("Methods do not exist on array classes");
	}
}

impl Frame {
	#[cfg(debug_assertions)]
	fn verify_stack_push(&self, size: usize) {
		// - 1 since we're comparing against the *length* of the stack
		let slots = size - 1;
		debug_assert!(
			(self.thread().stack().len() + slots)
				<= self.stack_base + self.method.code.max_stack as usize,
			"stack overflow",
		);
	}

	#[cfg(not(debug_assertions))]
	fn verify_stack_push(&self, _size: usize) {}

	#[cfg(debug_assertions)]
	fn verify_stack_pop(&self, size: usize) {
		// - 1 since we're comparing against the *length* of the stack
		let slots = size - 1;
		debug_assert!(
			self.thread().stack().len() >= self.stack_base + slots,
			"stack underflow"
		);
	}

	#[cfg(not(debug_assertions))]
	fn verify_stack_pop(&self, _size: usize) {}

	#[cfg(debug_assertions)]
	fn verify_operand_stack_index(&self, index: u2) {
		// + 1 for the base slot
		let occupied_frame_stack_slots = (self.thread().stack().len() + 1) - self.stack_base;
		debug_assert!(
			occupied_frame_stack_slots >= index as usize,
			"index out of bounds"
		);
	}

	#[cfg(not(debug_assertions))]
	fn verify_operand_stack_index(&self, _index: u2) {}

	#[cfg(debug_assertions)]
	fn verify_local_index(&self, index: u2) {
		debug_assert!(
			index < self.method.code.max_locals,
			"index out of bounds {:?}",
			self.thread().frame_stack()
		);
	}

	#[cfg(not(debug_assertions))]
	fn verify_local_index(&self, _index: u2) {}
}

impl StackLike<Reference> for Frame {
	fn clear(&mut self) {
		self.thread().stack().set_stack_pointer(self.stack_base);
	}

	fn push_op(&mut self, op: Operand<Reference>) {
		self.verify_stack_push(if matches!(op, Operand::Long(_) | Operand::Double(_)) {
			2
		} else {
			1
		});
		self.thread().stack().push_op(op)
	}

	fn push_int(&mut self, int: s4) {
		self.verify_stack_push(1);
		self.thread().stack().push_int(int)
	}

	fn push_float(&mut self, float: f32) {
		self.verify_stack_push(1);
		self.thread().stack().push_float(float)
	}

	fn push_double(&mut self, double: f64) {
		self.verify_stack_push(1);
		self.thread().stack().push_double(double)
	}

	fn push_long(&mut self, long: s8) {
		self.verify_stack_push(1);
		self.thread().stack().push_long(long)
	}

	fn push_reference(&mut self, reference: Reference) {
		self.verify_stack_push(1);
		self.thread().stack().push_reference(reference)
	}

	fn pop(&mut self) -> Operand<Reference> {
		self.verify_stack_pop(1);
		self.thread().stack().pop()
	}

	fn pop2(&mut self) {
		self.verify_stack_pop(2);
		self.thread().stack().pop2()
	}

	fn popn(&mut self, count: usize) -> Vec<Operand<Reference>> {
		self.verify_stack_pop(count);
		self.thread().stack().popn(count)
	}

	fn pop_int(&mut self) -> s4 {
		self.verify_stack_pop(1);
		self.thread().stack().pop_int()
	}

	fn pop_float(&mut self) -> f32 {
		self.verify_stack_pop(1);
		self.thread().stack().pop_float()
	}

	fn pop_double(&mut self) -> f64 {
		self.verify_stack_pop(2);
		self.thread().stack().pop_double()
	}

	fn pop_long(&mut self) -> s8 {
		self.verify_stack_pop(2);
		self.thread().stack().pop_long()
	}

	fn pop_reference(&mut self) -> Reference {
		self.verify_stack_pop(1);
		self.thread().stack().pop_reference()
	}

	fn dup(&mut self) {
		self.thread().stack().dup()
	}

	fn dup_x1(&mut self) {
		self.thread().stack().dup_x1()
	}

	fn dup_x2(&mut self) {
		self.thread().stack().dup_x2()
	}

	fn dup2(&mut self) {
		self.verify_stack_push(2);
		self.thread().stack().dup2()
	}

	fn dup2_x1(&mut self) {
		self.verify_stack_push(2);
		self.thread().stack().dup2_x1()
	}

	fn dup2_x2(&mut self) {
		self.verify_stack_push(2);
		self.thread().stack().dup2_x2()
	}

	fn swap(&mut self) {
		self.thread().stack().swap()
	}
}

// Getters
impl Frame {
	/// Get a reference to the associated [`JavaThread`]
	pub fn thread(&self) -> &'static JavaThread {
		unsafe { &*self.thread }
	}

	/// Get a reference to the constant pool
	pub fn constant_pool(&self) -> &'static ConstantPool {
		self.constant_pool
	}

	/// Get the operand at `index`
	pub fn at(&self, index: u2) -> Operand<Reference> {
		self.verify_operand_stack_index(index);
		self.thread().stack().at(index as isize)
	}

	/// Get the operand at `index`
	pub fn local(&self, index: u1) -> Operand<Reference> {
		self.verify_local_index(index as u2);
		let offset = self.locals_base + index as usize;
		self.thread().stack().absolute(offset)
	}

	/// Set the operand at `index` to `op`
	pub fn set_local(&self, index: u1, op: Operand<Reference>) {
		self.verify_local_index(index as u2);
		let offset = self.locals_base + index as usize;
		self.thread().stack().set_absolute(offset, op)
	}

	/// Get the method associated with this frame
	pub fn method(&self) -> &'static Method {
		self.method
	}

	/// Get the stashed [pc] for this frame
	///
	/// This will only be set if the current thread needs to execute a method within this frame.
	///
	/// [pc]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.5.1
	pub fn stashed_pc(&self) -> isize {
		self.stash.pc.load(Ordering::Relaxed)
	}

	fn depth(&self) -> isize {
		self.depth as isize
	}

	fn inc_depth(&mut self) {
		assert!(self.depth() < u16::MAX as isize);
		self.depth += 1;
	}

	fn has_stashed_depth(&self) -> bool {
		self.depth > 0
	}
}

// Setters
impl Frame {
	/// Stash the current [pc] and `sp` for later
	///
	/// This is used when a new frame is added to the thread's stack.
	///
	/// [pc]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.5.1
	pub fn stash(&self) {
		let current_pc;
		{
			let thread = self.thread();
			current_pc = thread.pc.load(Ordering::Relaxed);
		}
		let current_sp = self.thread().stack().len();

		self.stash.pc.store(current_pc, Ordering::Relaxed);
		self.stash.sp.store(current_sp, Ordering::Relaxed);
	}

	/// Apply the stashed [pc] and `sp`
	///
	/// # Safety
	///
	/// This **must** only be called after [`Self::stash()`].
	///
	/// [pc]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.5.1
	pub unsafe fn apply_stash(&mut self) {
		let pc = self.stash.pc.swap(0, Ordering::Relaxed);
		// This is only used *after* method calls, so it's safe to take the depth now
		self.thread()
			.pc
			.store(pc + self.take_cached_depth(), Ordering::Relaxed);

		// The `sp` isn't moved on the thread here, since the `Drop` impl handles it.
		// This is purely cosmetic for the `Debug` impl
		let _sp = self.stash.sp.swap(0, Ordering::Relaxed);
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
		self.depth += (current_pc.next_multiple_of(4) - current_pc) as u16;
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
