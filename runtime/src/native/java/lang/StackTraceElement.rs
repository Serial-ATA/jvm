use crate::objects::array::ArrayContent;
use crate::objects::class::Class;
use crate::objects::constant_pool::cp_types;
use crate::objects::instance::Instance;
use crate::objects::method::Method;
use crate::objects::reference::{ClassInstanceRef, Reference};
use crate::string_interner::StringInterner;
use crate::thread::exceptions::throw;
use crate::thread::JavaThread;

use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::jint;
use common::int_types::s8;
use common::traits::PtrType;
use instructions::Operand;

include_generated!("native/java/lang/def/StackTraceElement.definitions.rs");

unsafe fn initialize(class: &'static Class) {
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
		crate::globals::fields::java_lang_StackTraceElement::init_offsets();
	}
}

fn fill_in_stack_trace(stacktrace_element: ClassInstanceRef, method: &Method, pc: s8) {
	let method_class = method.class().unwrap_class_instance();

	let declaring_class_object_field_offset =
		crate::globals::fields::java_lang_StackTraceElement::declaringClassObject_field_offset();
	let declaring_class_object = method.class().mirror();
	stacktrace_element.get_mut().put_field_value0(
		declaring_class_object_field_offset,
		Operand::Reference(Reference::mirror(declaring_class_object)),
	);

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
			let file_name_sym = method_class
				.constant_pool
				.get::<cp_types::ConstantUtf8>(idx)
				.expect("file name should always resolve");

			let file_name = StringInterner::intern_symbol(file_name_sym);
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
	let line_number = method.get_line_number(pc as isize);
	stacktrace_element
		.get_mut()
		.put_field_value0(line_number_field_offset, Operand::Int(line_number));
}

pub fn initStackTraceElements(
	env: JniEnv,
	class: &'static Class,
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

	let stacktrace_elements_instance = elements.extract_array();
	let stacktrace_elements = stacktrace_elements_instance.get();
	let ArrayContent::Reference(stack_trace_elements) = stacktrace_elements.get_content() else {
		panic!("Stack trace array should contain `StackTraceElement`s");
	};

	if stack_trace_elements.len() != depth as usize {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw!(thread, IndexOutOfBoundsException);
	}

	// `x` is a reference to our backtrace. See the `BackTrace` struct in `java/lang/Throwable.rs` for
	// the format.
	let backtrace = x;
	let backtrace_array_instance = backtrace.extract_array();

	let backtrace_array = backtrace_array_instance.get();
	let ArrayContent::Long(backtrace_array_contents) = &backtrace_array.elements else {
		panic!("backtrace array has unexpected contents");
	};

	if backtrace_array_contents.len() % 2 != 0 {
		panic!("backtrace array is not an even length");
	}

	for ([method, pc], stacktrace_element) in backtrace_array_contents
		.array_chunks::<2>()
		.copied()
		.zip(stack_trace_elements.iter())
	{
		let method = unsafe { &*(method as *const Method) };
		fill_in_stack_trace(stacktrace_element.extract_class(), method, pc);
	}
}

pub fn initStackTraceElement(
	_: JniEnv,
	class: &'static Class,
	_element: Reference, // java.lang.StackTraceElement
	_sfi: Reference,     // java.lang.StackFrameInfo
) {
	unsafe {
		initialize(class);
	}

	unimplemented!("java.lang.StackTraceElement#initStackTraceElement");
}
