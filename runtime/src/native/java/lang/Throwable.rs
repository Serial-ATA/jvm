use crate::objects::array::PrimitiveArrayInstance;
use crate::objects::instance::Instance;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::symbols::sym;
use crate::thread::frame::stack::VisibleStackFrame;
use crate::thread::JavaThread;
use crate::{classes, globals};

use std::sync::Once;

use ::jni::env::JniEnv;
use ::jni::sys::{jint, jlong};
use common::int_types::s4;
use common::traits::PtrType;
use instructions::Operand;

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
			// TODO: The stashed pc will never be correct, since the pc is immediately incremented on every read.
			//       This needs to be fixed in the interpreter to only progress the pc when the instruction is finished.
			VisibleStackFrame::Regular(frame) => frame.stashed_pc(),
			_ => -1,
		};

		self.inner.push(method as *const Method as jlong);
		self.inner.push(pc as jlong);
	}

	fn into_obj(self) -> Reference {
		let content = self.inner.into_boxed_slice();

		let array = unsafe { PrimitiveArrayInstance::new::<jlong>(content) };
		Reference::array(array)
	}
}

// Initialize the java.lang.Throwable field offsets
unsafe fn initialize() {
	static ONCE: Once = Once::new();
	ONCE.call_once(|| unsafe {
		classes::java_lang_Throwable::init_offsets();
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
	classes::java_lang_Throwable::set_backtrace(this.extract_class().get_mut(), Reference::null());

	let stack_trace_offset = classes::java_lang_Throwable::stackTrace_field_offset();
	this.put_field_value0(stack_trace_offset, Operand::Reference(Reference::null()));

	let current_thread = unsafe { &*JavaThread::for_env(env.raw() as _) };

	let this_class_instance = this.extract_class();

	let stack_depth = current_thread.frame_stack().visible_depth();

	let mut frames_to_skip = 0;
	for frame in current_thread.frame_stack().iter() {
		if frame
			.method()
			.class()
			.is_subclass_of(globals::classes::java_lang_Throwable())
		{
			frames_to_skip += 1;
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

	classes::java_lang_Throwable::set_backtrace(
		this.extract_class().get_mut(),
		backtrace.into_obj(),
	);

	let depth_field_offset = classes::java_lang_Throwable::depth_field_offset();
	this.put_field_value0(depth_field_offset, Operand::Int(backtrace_depth as jint));

	this
}
