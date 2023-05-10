use crate::class_instance::{ArrayContent, ArrayInstance, Instance};
use crate::classpath::classloader::ClassLoader;
use crate::frame::FrameRef;
use crate::native::{JNIEnv, NativeReturn};
use crate::reference::Reference;
use crate::stack::local_stack::LocalStack;

use std::sync::Arc;

use classfile::FieldType;
use common::box_slice;
use common::traits::PtrType;
use instructions::Operand;

#[allow(non_upper_case_globals)]
mod stacktrace_element {
	use crate::class_instance::{ClassInstance, Instance};
	use crate::frame::FrameRef;
	use crate::reference::{ClassRef, Reference};
	use crate::string_interner::StringInterner;

	use std::sync::atomic::Ordering;
	use std::sync::{Arc, Mutex};

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

	unsafe fn initialize(class: ClassRef) {
		let mut initialized = INITIALIZED.lock().unwrap();
		if *initialized {
			return;
		}

		let class_descriptor = class.unwrap_class_instance();

		StackTraceElement_classLoaderName_FIELD_OFFSET = class_descriptor
			.find_field(|field| {
				field.descriptor.is_class(b"java/lang/String") && field.name == b"classLoaderName"
			})
			.expect("classLoaderName field should exist")
			.idx;

		StackTraceElement_moduleName_FIELD_OFFSET = class_descriptor
			.find_field(|field| {
				field.descriptor.is_class(b"java/lang/String") && field.name == b"moduleName"
			})
			.expect("moduleName field should exist")
			.idx;

		StackTraceElement_moduleVersion_FIELD_OFFSET = class_descriptor
			.find_field(|field| {
				field.descriptor.is_class(b"java/lang/String") && field.name == b"moduleVersion"
			})
			.expect("moduleVersion field should exist")
			.idx;

		StackTraceElement_declaringClass_FIELD_OFFSET = class_descriptor
			.find_field(|field| {
				field.descriptor.is_class(b"java/lang/String") && field.name == b"declaringClass"
			})
			.expect("declaringClass field should exist")
			.idx;

		StackTraceElement_methodName_FIELD_OFFSET = class_descriptor
			.find_field(|field| {
				field.descriptor.is_class(b"java/lang/String") && field.name == b"methodName"
			})
			.expect("methodName field should exist")
			.idx;

		StackTraceElement_fileName_FIELD_OFFSET = class_descriptor
			.find_field(|field| {
				field.descriptor.is_class(b"java/lang/String") && field.name == b"fileName"
			})
			.expect("fileName field should exist")
			.idx;

		StackTraceElement_lineNumber_FIELD_OFFSET = class_descriptor
			.find_field(|field| field.descriptor.is_int() && field.name == b"lineNumber")
			.expect("lineNumber field should exist")
			.idx;

		*initialized = true;
	}

	pub fn from_stack_frame(stacktrace_element_class: ClassRef, frame: FrameRef) -> Reference {
		unsafe {
			initialize(Arc::clone(&stacktrace_element_class));
		}

		let method = frame.method();
		let method_class = method.class.unwrap_class_instance();

		unsafe {
			let stacktrace_element = ClassInstance::new(stacktrace_element_class);

			// TODO: classLoaderName
			// TODO: moduleName
			// TODO: moduleVersion
			let declaring_class = StringInterner::intern_string(&method.class.get().name);
			stacktrace_element.get_mut().put_field_value0(
				StackTraceElement_declaringClass_FIELD_OFFSET,
				Operand::Reference(Reference::Class(declaring_class)),
			);

			let method_name = StringInterner::intern_string(&method.name);
			stacktrace_element.get_mut().put_field_value0(
				StackTraceElement_methodName_FIELD_OFFSET,
				Operand::Reference(Reference::Class(method_name)),
			);

			match method_class.source_file_index {
				Some(idx) => {
					let file_name = StringInterner::intern_string(
						method_class.constant_pool.get_constant_utf8(idx),
					);
					stacktrace_element.get_mut().put_field_value0(
						StackTraceElement_fileName_FIELD_OFFSET,
						Operand::Reference(Reference::Class(file_name)),
					);
				},
				None => {
					stacktrace_element.get_mut().put_field_value0(
						StackTraceElement_fileName_FIELD_OFFSET,
						Operand::Reference(Reference::Null),
					);
				},
			}

			let pc = frame.thread().get().pc.load(Ordering::Relaxed) - 1;
			let line_number = method.get_line_number(pc);
			stacktrace_element.get_mut().put_field_value0(
				StackTraceElement_lineNumber_FIELD_OFFSET,
				Operand::Int(line_number),
			);

			Reference::Class(stacktrace_element)
		}
	}
}

pub fn fillInStackTrace(env: JNIEnv, locals: LocalStack) -> NativeReturn {
	let mut this = locals[0].expect_reference();

	let this_class_instance = this.extract_class();
	let this_class = &this_class_instance.get().class;
	let stacktrace_field = this_class.unwrap_class_instance().find_field(|field| field.name == b"stackTrace" && matches!(&field.descriptor, FieldType::Array(value) if value.is_class(b"java/lang/StackTraceElement"))).expect("Throwable should have a stackTrace field");

	let current_thread = env.current_thread.get();
	let stack_depth = current_thread.frame_stack.len();

	// We need to skip the current frame at the very least
	let mut frames_to_skip = 1;

	let mut current_class = this_class.get();
	loop {
		// Skip all frames related to the Throwable class and its superclasses
		frames_to_skip += 1;

		if let Some(ref super_class) = current_class.super_class {
			current_class = super_class.get();
			continue;
		}

		break;
	}

	// We need to skip the <athrow> method
	if current_thread.frame_stack[frames_to_skip].method().name == b"<athrow>" {
		frames_to_skip += 1;
	}

	assert!(frames_to_skip < stack_depth);

	let stacktrace_element_class = ClassLoader::Bootstrap
		.load(b"java/lang/StackTraceElement")
		.expect("StackTraceElement should be available");

	let stacktrace_element_array_class = ClassLoader::Bootstrap
		.load(b"[Ljava/lang/StackTraceElement;")
		.expect("[Ljava/lang/StackTraceElement; should be available");

	// Create the StackTraceElement array
	let mut stacktrace_elements = box_slice![Reference::Null; stack_depth - frames_to_skip];
	for (idx, frame) in current_thread.frame_stack[..stack_depth - frames_to_skip]
		.iter()
		.enumerate()
	{
		stacktrace_elements[idx] = stacktrace_element::from_stack_frame(
			Arc::clone(&stacktrace_element_class),
			FrameRef::clone(frame),
		);
	}

	let array = ArrayInstance::new(
		stacktrace_element_array_class,
		ArrayContent::Reference(stacktrace_elements),
	);
	this.put_field_value0(
		stacktrace_field.idx,
		Operand::Reference(Reference::Array(array)),
	);

	Some(Operand::Reference(this))
}
