use super::native::NativeFrame;
use super::Frame;
use crate::objects::method::Method;

use std::cell::UnsafeCell;
use std::fmt::Debug;

pub enum StackFrame {
	Real(Frame),
	Native(NativeFrame),
	Fake,
}

impl Debug for StackFrame {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			StackFrame::Real(frame) => write!(
				f,
				"(Real) {:?} (code len: {}, cached pc: {})",
				frame.method(),
				frame.method().code.code.len(),
				frame.cached_pc.load(std::sync::atomic::Ordering::Acquire)
			),
			StackFrame::Native(frame) => write!(f, "(Native) {:?}", frame.method()),
			StackFrame::Fake => write!(f, "Fake"),
		}
	}
}

impl StackFrame {
	fn is_fake(&self) -> bool {
		matches!(self, StackFrame::Fake)
	}
}

pub enum VisibleStackFrame<'a> {
	Regular(&'a Frame),
	Native(&'a NativeFrame),
}

impl<'a> VisibleStackFrame<'a> {
	pub fn method(&self) -> &'static Method {
		match self {
			VisibleStackFrame::Regular(frame) => frame.method(),
			VisibleStackFrame::Native(frame) => frame.method(),
		}
	}
}

impl<'a> From<&'a Frame> for VisibleStackFrame<'a> {
	fn from(frame: &'a Frame) -> Self {
		VisibleStackFrame::Regular(frame)
	}
}

impl<'a> From<&'a NativeFrame> for VisibleStackFrame<'a> {
	fn from(frame: &'a NativeFrame) -> Self {
		VisibleStackFrame::Native(frame)
	}
}

pub struct FrameStack {
	inner: UnsafeCell<Vec<StackFrame>>,
}

impl Debug for FrameStack {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.__inner().iter()).finish()
	}
}

impl FrameStack {
	// TODO
	pub fn new() -> Self {
		FrameStack {
			inner: UnsafeCell::new(Vec::with_capacity(1024)),
		}
	}

	pub fn current(&self) -> Option<&mut Frame> {
		let current_frame = self.__inner_mut().last_mut();
		match current_frame {
			Some(StackFrame::Real(r)) => Some(r),
			_ => None,
		}
	}

	/// The total number of frames in the stack *including* fake frames
	pub fn depth(&self) -> usize {
		self.__inner().len()
	}

	/// The number of frames in the stack *excluding* fake frames
	pub fn visible_depth(&self) -> usize {
		self.iter().count()
	}

	pub fn iter(&self) -> impl DoubleEndedIterator<Item = VisibleStackFrame<'_>> {
		self.__inner().iter().filter_map(|frame| match frame {
			StackFrame::Real(frame) => Some(VisibleStackFrame::Regular(frame)),
			StackFrame::Native(frame) => Some(VisibleStackFrame::Native(frame)),
			StackFrame::Fake => None,
		})
	}

	pub fn get(&self, position: usize) -> Option<VisibleStackFrame<'_>> {
		match self.__inner().get(position) {
			Some(StackFrame::Real(frame)) => Some(VisibleStackFrame::from(frame)),
			Some(StackFrame::Native(frame)) => Some(VisibleStackFrame::from(frame)),
			Some(StackFrame::Fake) => self.get(position - 1), // TODO: HACK!!!
			None => None,
		}
	}

	pub fn push(&self, frame: StackFrame) {
		self.__inner_mut().push(frame);
	}

	pub fn pop(&self) -> Option<StackFrame> {
		self.__inner_mut().pop()
	}

	pub fn pop_real(&self) -> Option<Frame> {
		match self.__inner_mut().pop() {
			Some(StackFrame::Real(r)) => Some(r),
			_ => None,
		}
	}

	pub fn pop_native(&self) -> Option<NativeFrame> {
		match self.__inner_mut().pop() {
			Some(StackFrame::Native(r)) => Some(r),
			_ => None,
		}
	}

	pub fn pop_dummy(&self) {
		match self.__inner_mut().pop() {
			Some(StackFrame::Fake) => return,
			_ => panic!("Expected a dummy frame!"),
		}
	}

	pub fn clear(&self) {
		self.__inner_mut().clear();
	}

	fn __inner(&self) -> &mut Vec<StackFrame> {
		unsafe { &mut *self.inner.get() }
	}

	fn __inner_mut(&self) -> &mut Vec<StackFrame> {
		unsafe { &mut *self.inner.get() }
	}
}
