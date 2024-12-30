use crate::classpath::classloader::ClassLoader;
use crate::objects::array::{ArrayContent, ArrayInstance};
use crate::objects::class::Class;
use crate::objects::instance::Instance;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::thread::frame::stack::VisibleStackFrame;
use crate::thread::JavaThread;

use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;

use ::jni::env::JniEnv;
use ::jni::sys::{jint, jlong};
use classfile::FieldType;
use common::box_slice;
use common::int_types::s4;
use common::traits::PtrType;
use instructions::Operand;
use symbols::sym;

#[allow(non_upper_case_globals)]
mod stacktrace_element {
	use crate::classpath::classloader::ClassLoader;
	use crate::objects::class::Class;
	use crate::objects::class_instance::ClassInstance;
	use crate::objects::constant_pool::cp_types;
	use crate::objects::instance::Instance;
	use crate::objects::reference::Reference;
	use crate::string_interner::StringInterner;
	use crate::thread::frame::stack::VisibleStackFrame;

	use std::sync::atomic::{AtomicBool, Ordering};

	use common::traits::PtrType;
	use instructions::Operand;
	use symbols::sym;

	unsafe fn initialize(class: &'static Class) {
		static ONCE: AtomicBool = AtomicBool::new(false);
		if ONCE
			.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
			.is_err()
		{
			// Already initialized
			return;
		}

		let stack_trace_element_class = ClassLoader::Bootstrap
			.load(sym!(java_lang_StackTraceElement))
			.unwrap();

		unsafe {
			crate::globals::classes::set_java_lang_StackTraceElement(stack_trace_element_class);
			crate::globals::fields::java_lang_StackTraceElement::init_offsets();
		}
	}

	pub fn from_stack_frame(
		stacktrace_element_class: &'static Class,
		frame: VisibleStackFrame<'_>,
	) -> Reference {
		unsafe {
			initialize(stacktrace_element_class);
		}

		let method = frame.method();
		let method_class = method.class().unwrap_class_instance();

		unsafe {
			let stacktrace_element = ClassInstance::new(stacktrace_element_class);

			// TODO: classLoaderName
			// TODO: moduleName
			// TODO: moduleVersion
			let declaring_class_field_offset =
				crate::globals::fields::java_lang_StackTraceElement::declaringClass_field_offset();
			let declaring_class = StringInterner::intern_symbol(method.class().name);
			stacktrace_element.get_mut().put_field_value0(
				declaring_class_field_offset,
				Operand::Reference(Reference::class(declaring_class)),
			);

			let method_name_field_offset =
				crate::globals::fields::java_lang_StackTraceElement::methodName_field_offset();
			let method_name = StringInterner::intern_symbol(method.name);
			stacktrace_element.get_mut().put_field_value0(
				method_name_field_offset,
				Operand::Reference(Reference::class(method_name)),
			);

			let file_name_field_offset =
				crate::globals::fields::java_lang_StackTraceElement::fileName_field_offset();
			match method_class.source_file_index {
				Some(idx) => {
					let file_name = StringInterner::intern_symbol(
						method_class
							.constant_pool
							.get::<cp_types::ConstantUtf8>(idx),
					);
					stacktrace_element.get_mut().put_field_value0(
						file_name_field_offset,
						Operand::Reference(Reference::class(file_name)),
					);
				},
				None => {
					stacktrace_element.get_mut().put_field_value0(
						file_name_field_offset,
						Operand::Reference(Reference::null()),
					);
				},
			}

			let line_number_field_offset =
				crate::globals::fields::java_lang_StackTraceElement::lineNumber_field_offset();
			if let VisibleStackFrame::Regular(frame) = frame {
				let pc = frame.thread().pc.load(Ordering::Relaxed) - 1;
				let line_number = method.get_line_number(pc);
				stacktrace_element
					.get_mut()
					.put_field_value0(line_number_field_offset, Operand::Int(line_number));
			}

			Reference::class(stacktrace_element)
		}
	}
}

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
	env: NonNull<JniEnv>,
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

	let current_thread = unsafe { &*JavaThread::for_env(env.as_ptr() as _) };

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
	for (idx, frame) in current_thread
		.frame_stack()
		.iter()
		.skip(frames_to_skip)
		.take(backtrace_depth)
		.enumerate()
	{
		backtrace.push(frame);
	}

	let backtrace_offset = crate::globals::fields::java_lang_Throwable::backtrace_field_offset();
	this.put_field_value0(backtrace_offset, Operand::Reference(backtrace.into_obj()));

	let depth_field_offset = crate::globals::fields::java_lang_Throwable::depth_field_offset();
	this.put_field_value0(depth_field_offset, Operand::Int(backtrace_depth as jint));

	this
}
