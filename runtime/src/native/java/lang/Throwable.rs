use crate::objects::instance::array::PrimitiveArrayInstance;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::symbols::sym;
use crate::thread::JavaThread;
use crate::thread::frame::stack::VisibleStackFrame;
use crate::{classes, globals};

use std::slice;
use std::sync::Once;

use ::jni::env::JniEnv;
use ::jni::sys::{jint, jlong};
use common::int_types::s4;

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
/// ```text
/// struct BackTrace {
///     method: &Method as jlong,
///     pc: jlong,
/// }
/// ```
///
/// Flattened into an `long[]`:
///
/// ```text
/// ["java/lang/Foo#foo", 2, "java/lang/Foo#bar", 5]
/// ```
pub struct BackTrace {
	inner: Vec<jlong>,
}

impl BackTrace {
	const NUMBER_OF_FIELDS: usize = 2;

	pub fn from_encoded(backtrace: &[jlong]) -> BackTraceIter<'_> {
		BackTraceIter {
			inner: backtrace.iter(),
		}
	}

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
			VisibleStackFrame::Regular(frame) => frame.stashed_pc(),
			_ => -1,
		};

		self.inner.push(std::ptr::from_ref(method) as jlong);
		self.inner.push(pc as jlong);
	}

	fn into_obj(self) -> Reference {
		let content = self.inner.into_boxed_slice();

		let array = PrimitiveArrayInstance::new::<jlong>(content);
		Reference::array(array)
	}
}

pub struct BackTraceElement {
	pub method: &'static Method,
	pub pc: jlong,
}

pub struct BackTraceIter<'a> {
	inner: slice::Iter<'a, jlong>,
}

impl Iterator for BackTraceIter<'_> {
	type Item = BackTraceElement;

	fn next(&mut self) -> Option<Self::Item> {
		match self.inner.next_chunk::<2>() {
			Ok([method, pc]) => Some(BackTraceElement {
				method: unsafe { &*(*method as *const Method) },
				pc: *pc,
			}),
			Err(_) => None,
		}
	}
}

// Initialize the java.lang.Throwable field offsets
unsafe fn initialize() {
	static ONCE: Once = Once::new();
	ONCE.call_once(|| unsafe {
		classes::java::lang::Throwable::init_offsets();
	});
}

pub fn fillInStackTrace(
	env: JniEnv,
	this: Reference, // java.lang.Throwable
	_dummy: s4,
) -> Reference /* java.lang.Throwable */
{
	unsafe { initialize() };

	// Reset the current fields
	classes::java::lang::Throwable::set_backtrace(this, Reference::null());
	classes::java::lang::Throwable::set_stackTrace(this, Reference::null());

	let current_thread = unsafe { &*JavaThread::for_env(env.raw().cast_const()) };

	let stack_depth = current_thread.frame_stack().visible_depth();

	let mut frames_to_skip = 0;
	for frame in current_thread.frame_stack().iter() {
		if frame
			.method()
			.class()
			.is_subclass_of(globals::classes::java_lang_Throwable())
		{
			if frame.method().name == sym!(fillInStackTrace_name)
				|| frame.method().name == sym!(object_initializer_name)
			{
				frames_to_skip += 1;
			}

			continue;
		}

		break;
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

	classes::java::lang::Throwable::set_backtrace(this, backtrace.into_obj());
	classes::java::lang::Throwable::set_depth(this, backtrace_depth as jint);

	this
}
