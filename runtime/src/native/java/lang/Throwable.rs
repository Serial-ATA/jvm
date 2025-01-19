use crate::objects::array::{ArrayContent, ArrayInstance};
use crate::objects::instance::Instance;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::thread::frame::stack::VisibleStackFrame;
use crate::thread::JavaThread;

use std::ptr::NonNull;
use std::sync::atomic::Ordering;
use std::sync::Once;

use ::jni::env::JniEnv;
use ::jni::sys::{jint, jlong};
use common::int_types::s4;
use common::traits::PtrType;
use instructions::Operand;
use symbols::sym;

include_generated!("native/java/lang/def/Throwable.definitions.rs");

/// A wrapper for backtrace (`java.lang.Throwable#backtrace`) creation
///
/// The field in Java is defined as:
///
/// ```java
/// private transient Object backtrace;
/// ```
///
/// So we're given free rein to define the format of our backtrace.
///
/// The format is the following struct:
///
/// ```
/// struct BackTrace {
/// 	method: &Method as jlong,
/// 	pc: jlong,
/// }
/// ```
///
/// Flattened into an `long[]`:
///
/// ```
/// ["java/lang/Foo#foo", 2, "java/lang/Foo#bar", 5]
/// ```
struct BackTrace {
	inner: Vec<jlong>,
}

impl BackTrace {
	const NUMBER_OF_FIELDS: usize = 2;

	/// Create a new `BackTrace`
	///
	/// `depth` is the number of methods in the backtrace
	fn new(mut depth: usize) -> Self {
		depth *= Self::NUMBER_OF_FIELDS;

		BackTrace {
			inner: Vec::with_capacity(depth),
		}
	}

	#[allow(trivial_casts)]
	fn push(&mut self, frame: VisibleStackFrame<'_>) {
		let method = frame.method();
		let pc = match frame {
			VisibleStackFrame::Regular(frame) => frame.thread().pc.load(Ordering::Relaxed) - 1,
			_ => -1,
		};

		self.inner.push(method as *const Method as jlong);
		self.inner.push(pc as jlong);
	}

	fn into_obj(self) -> Reference {
		let content = ArrayContent::Long(self.inner.into_boxed_slice());

		let long_array_class = crate::globals::classes::long_array();
		let array = ArrayInstance::new(long_array_class, content);
		Reference::array(array)
	}
}

// Initialize the java.lang.Throwable field offsets
unsafe fn initialize() {
	static ONCE: Once = Once::new();
	ONCE.call_once(|| unsafe {
		crate::globals::fields::java_lang_Throwable::init_offsets();
	});
}

pub fn fillInStackTrace(
	env: JniEnv,
	mut this: Reference, // java.lang.Throwable
	_dummy: s4,
) -> Reference /* java.lang.Throwable */
{
	unsafe { initialize() };

	// Reset the current fields
	let backtrace_offset = crate::globals::fields::java_lang_Throwable::backtrace_field_offset();
	this.put_field_value0(backtrace_offset, Operand::Reference(Reference::null()));

	let stack_trace_offset = crate::globals::fields::java_lang_Throwable::stackTrace_field_offset();
	this.put_field_value0(stack_trace_offset, Operand::Reference(Reference::null()));

	let current_thread = unsafe { &*JavaThread::for_env(env.raw() as _) };

	let this_class_instance = this.extract_class();
	let this_class = this_class_instance.get().class();

	let stack_depth = current_thread.frame_stack().depth();

	// We need to skip the current frame at the very least
	let mut frames_to_skip = 1;

	let mut current_class = this_class;
	loop {
		// Skip all frames related to the Throwable class and its superclasses
		frames_to_skip += 1;

		if let Some(super_class) = current_class.super_class {
			current_class = super_class;
			continue;
		}

		break;
	}

	// We need to skip the <athrow> method
	let athrow_frame = current_thread
		.frame_stack()
		.get(frames_to_skip)
		.expect("Frame should exist");
	if athrow_frame.method().name == sym!(athrow_name) {
		frames_to_skip += 1;
	}

	assert!(frames_to_skip < stack_depth);

	// Create the backtrace
	let backtrace_depth = stack_depth - frames_to_skip;
	let mut backtrace = BackTrace::new(backtrace_depth);
	for frame in current_thread
		.frame_stack()
		.iter()
		.skip(frames_to_skip)
		.take(backtrace_depth)
	{
		backtrace.push(frame);
	}

	let backtrace_offset = crate::globals::fields::java_lang_Throwable::backtrace_field_offset();
	this.put_field_value0(backtrace_offset, Operand::Reference(backtrace.into_obj()));

	let depth_field_offset = crate::globals::fields::java_lang_Throwable::depth_field_offset();
	this.put_field_value0(depth_field_offset, Operand::Int(backtrace_depth as jint));

	this
}
