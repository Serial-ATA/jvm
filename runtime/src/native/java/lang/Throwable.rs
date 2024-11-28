use crate::class_instance::{ArrayContent, ArrayInstance, Instance};
use crate::classpath::classloader::ClassLoader;
use crate::reference::Reference;
use crate::JavaThread;

use std::ptr::NonNull;

use ::jni::env::JniEnv;
use classfile::FieldType;
use common::box_slice;
use common::int_types::s4;
use common::traits::PtrType;
use instructions::Operand;
use symbols::sym;

#[allow(non_upper_case_globals)]
mod stacktrace_element {
	use crate::class::Class;
	use crate::class_instance::{ClassInstance, Instance};
	use crate::frame::Frame;
	use crate::reference::Reference;
	use crate::string_interner::StringInterner;

	use std::sync::atomic::Ordering;
	use std::sync::Mutex;

	use common::traits::PtrType;
	use instructions::Operand;

	static mut INITIALIZED: Mutex<bool> = Mutex::new(false);

	static mut StackTraceElement_classLoaderName_FIELD_OFFSET: usize = 0;
	static mut StackTraceElement_moduleName_FIELD_OFFSET: usize = 0;
	static mut StackTraceElement_moduleVersion_FIELD_OFFSET: usize = 0;
	static mut StackTraceElement_declaringClass_FIELD_OFFSET: usize = 0;
	static mut StackTraceElement_methodName_FIELD_OFFSET: usize = 0;
	static mut StackTraceElement_fileName_FIELD_OFFSET: usize = 0;
	static mut StackTraceElement_lineNumber_FIELD_OFFSET: usize = 0;

	unsafe fn initialize(class: &'static Class) {
		let mut initialized = INITIALIZED.lock().unwrap();
		if *initialized {
			return;
		}

		StackTraceElement_classLoaderName_FIELD_OFFSET = class
			.fields()
			.find(|field| {
				field.descriptor.is_class(b"java/lang/String") && field.name == b"classLoaderName"
			})
			.expect("classLoaderName field should exist")
			.idx;

		StackTraceElement_moduleName_FIELD_OFFSET = class
			.fields()
			.find(|field| {
				field.descriptor.is_class(b"java/lang/String") && field.name == b"moduleName"
			})
			.expect("moduleName field should exist")
			.idx;

		StackTraceElement_moduleVersion_FIELD_OFFSET = class
			.fields()
			.find(|field| {
				field.descriptor.is_class(b"java/lang/String") && field.name == b"moduleVersion"
			})
			.expect("moduleVersion field should exist")
			.idx;

		StackTraceElement_declaringClass_FIELD_OFFSET = class
			.fields()
			.find(|field| {
				field.descriptor.is_class(b"java/lang/String") && field.name == b"declaringClass"
			})
			.expect("declaringClass field should exist")
			.idx;

		StackTraceElement_methodName_FIELD_OFFSET = class
			.fields()
			.find(|field| {
				field.descriptor.is_class(b"java/lang/String") && field.name == b"methodName"
			})
			.expect("methodName field should exist")
			.idx;

		StackTraceElement_fileName_FIELD_OFFSET = class
			.fields()
			.find(|field| {
				field.descriptor.is_class(b"java/lang/String") && field.name == b"fileName"
			})
			.expect("fileName field should exist")
			.idx;

		StackTraceElement_lineNumber_FIELD_OFFSET = class
			.fields()
			.find(|field| field.descriptor.is_int() && field.name == b"lineNumber")
			.expect("lineNumber field should exist")
			.idx;

		*initialized = true;
	}

	pub fn from_stack_frame(stacktrace_element_class: &'static Class, frame: &Frame) -> Reference {
		unsafe {
			initialize(stacktrace_element_class);
		}

		let method = frame.method();
		let method_class = method.class.unwrap_class_instance();

		unsafe {
			let stacktrace_element = ClassInstance::new(stacktrace_element_class);

			// TODO: classLoaderName
			// TODO: moduleName
			// TODO: moduleVersion
			let declaring_class = StringInterner::intern_symbol(method.class.name);
			stacktrace_element.get_mut().put_field_value0(
				StackTraceElement_declaringClass_FIELD_OFFSET,
				Operand::Reference(Reference::class(declaring_class)),
			);

			let method_name = StringInterner::intern_symbol(method.name);
			stacktrace_element.get_mut().put_field_value0(
				StackTraceElement_methodName_FIELD_OFFSET,
				Operand::Reference(Reference::class(method_name)),
			);

			match method_class.source_file_index {
				Some(idx) => {
					let file_name = StringInterner::intern_bytes(
						method_class.constant_pool.get_constant_utf8(idx),
					);
					stacktrace_element.get_mut().put_field_value0(
						StackTraceElement_fileName_FIELD_OFFSET,
						Operand::Reference(Reference::class(file_name)),
					);
				},
				None => {
					stacktrace_element.get_mut().put_field_value0(
						StackTraceElement_fileName_FIELD_OFFSET,
						Operand::Reference(Reference::null()),
					);
				},
			}

			let pc = frame.thread().pc.load(Ordering::Relaxed) - 1;
			let line_number = method.get_line_number(pc);
			stacktrace_element.get_mut().put_field_value0(
				StackTraceElement_lineNumber_FIELD_OFFSET,
				Operand::Int(line_number),
			);

			Reference::class(stacktrace_element)
		}
	}
}

include_generated!("native/java/lang/def/Throwable.definitions.rs");

pub fn fillInStackTrace(
	env: NonNull<JniEnv>,
	mut this: Reference, // java.lang.Throwable
	_dummy: s4,
) -> Reference /* java.lang.Throwable */
{
	let current_thread = unsafe { &*JavaThread::for_env(env.as_ptr() as _) };

	let this_class_instance = this.extract_class();
	let this_class = this_class_instance.get().class;
	// TODO: Make global field
	let stacktrace_field = this_class.fields().find(|field| {
		field.name == b"stackTrace" && matches!(&field.descriptor, FieldType::Array(value) if value.is_class(b"java/lang/StackTraceElement"))
	}).expect("Throwable should have a stackTrace field");

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

	let stacktrace_element_class = ClassLoader::Bootstrap
		.load(sym!(java_lang_StackTraceElement))
		.expect("StackTraceElement should be available");

	let stacktrace_element_array_class = ClassLoader::Bootstrap
		.load(sym!(StackTraceElement_array))
		.expect("[Ljava/lang/StackTraceElement; should be available");

	// Create the StackTraceElement array
	let mut stacktrace_elements = box_slice![Reference::null(); stack_depth - frames_to_skip];
	for (idx, frame) in current_thread
		.frame_stack()
		.iter()
		.take(stack_depth - frames_to_skip)
		.enumerate()
	{
		stacktrace_elements[idx] =
			stacktrace_element::from_stack_frame(stacktrace_element_class, frame);
	}

	let array = ArrayInstance::new(
		stacktrace_element_array_class,
		ArrayContent::Reference(stacktrace_elements),
	);
	this.put_field_value0(
		stacktrace_field.idx,
		Operand::Reference(Reference::array(array)),
	);

	this
}
