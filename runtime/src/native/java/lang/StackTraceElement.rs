use crate::classes;
use crate::native::java::lang::String::StringInterner;
use crate::objects::class::ClassPtr;
use crate::objects::instance::class::ClassInstanceRef;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::throw;

use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::jint;
use common::int_types::s8;
use jni::sys::jlong;

include_generated!("native/java/lang/def/StackTraceElement.definitions.rs");

unsafe fn initialize(class: ClassPtr) {
	static ONCE: AtomicBool = AtomicBool::new(false);
	if ONCE
		.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
		.is_err()
	{
		// Already initialized
		return;
	}

	unsafe {
		crate::globals::classes::set_java_lang_StackTraceElement(class);
		classes::java::lang::StackTraceElement::init_offsets();
	}
}

fn fill_in_stack_trace(stacktrace_element: ClassInstanceRef, method: &Method, pc: s8) {
	let declaring_class_object = method.class().mirror();
	classes::java::lang::StackTraceElement::set_declaringClassObject(
		stacktrace_element,
		Reference::mirror(declaring_class_object),
	);

	// TODO: classLoaderName
	// TODO: moduleName
	// TODO: moduleVersion
	let declaring_class = StringInterner::intern(method.class().name());
	classes::java::lang::StackTraceElement::set_declaringClass(
		stacktrace_element,
		Reference::class(declaring_class),
	);

	let method_name = StringInterner::intern(method.name);
	classes::java::lang::StackTraceElement::set_methodName(
		stacktrace_element,
		Reference::class(method_name),
	);

	match method.class().source_file_name() {
		Some(name) => {
			let file_name = StringInterner::intern(name);
			classes::java::lang::StackTraceElement::set_fileName(
				stacktrace_element,
				Reference::class(file_name),
			);
		},
		None => {
			classes::java::lang::StackTraceElement::set_fileName(
				stacktrace_element,
				Reference::null(),
			);
		},
	}

	let line_number = method.line_number(pc as isize);
	classes::java::lang::StackTraceElement::set_lineNumber(stacktrace_element, line_number);
}

pub fn initStackTraceElements(
	env: JniEnv,
	class: ClassPtr,
	elements: Reference, // java.lang.StackTraceElement[]
	x: Reference,        // java.lang.Object
	depth: jint,
) {
	unsafe {
		initialize(class);
	}

	if x.is_null() || elements.is_null() {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw!(thread, NullPointerException);
	}

	let stacktrace_elements = elements.extract_object_array();
	let stacktrace_elements = stacktrace_elements.as_slice();

	if stacktrace_elements.len() != depth as usize {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw!(thread, IndexOutOfBoundsException);
	}

	// `x` is a reference to our backtrace. See the `BackTrace` struct in `java/lang/Throwable.rs` for
	// the format.
	let backtrace = x;
	let backtrace_array = backtrace.extract_primitive_array();
	let backtrace_array_contents = backtrace_array.as_slice::<jlong>();

	if backtrace_array_contents.len() % 2 != 0 {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw!(
			thread,
			InternalError,
			"backtrace array is not an even length"
		);
	}

	for ([method, pc], stacktrace_element) in backtrace_array_contents
		.array_chunks::<2>()
		.copied()
		.zip(stacktrace_elements.iter())
	{
		let method = unsafe { &*(method as *const Method) };
		fill_in_stack_trace(stacktrace_element.extract_class(), method, pc);
	}
}

pub fn initStackTraceElement(
	_: JniEnv,
	class: ClassPtr,
	_element: Reference, // java.lang.StackTraceElement
	_sfi: Reference,     // java.lang.StackFrameInfo
) {
	unsafe {
		initialize(class);
	}

	unimplemented!("java.lang.StackTraceElement#initStackTraceElement");
}
