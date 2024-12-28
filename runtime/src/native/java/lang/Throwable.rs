use crate::classpath::classloader::ClassLoader;
use crate::objects::array::{ArrayContent, ArrayInstance};
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;

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

	static mut StackTraceElement_classLoaderName_FIELD_OFFSET: usize = 0;
	static mut StackTraceElement_moduleName_FIELD_OFFSET: usize = 0;
	static mut StackTraceElement_moduleVersion_FIELD_OFFSET: usize = 0;
	static mut StackTraceElement_declaringClass_FIELD_OFFSET: usize = 0;
	static mut StackTraceElement_methodName_FIELD_OFFSET: usize = 0;
	static mut StackTraceElement_fileName_FIELD_OFFSET: usize = 0;
	static mut StackTraceElement_lineNumber_FIELD_OFFSET: usize = 0;

	unsafe fn initialize(class: &'static Class) {
		static ONCE: AtomicBool = AtomicBool::new(false);
		if ONCE
			.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
			.is_err()
		{
			// Already initialized
			return;
		}

		let mut field_set = 0;
		for field in class.fields() {
			if field.name == sym!(classLoaderName) {
				assert!(field.descriptor.is_class(b"java/lang/String"));
				StackTraceElement_classLoaderName_FIELD_OFFSET = field.idx;
				field_set |= 1;
				continue;
			}

			if field.name == sym!(moduleName) {
				assert!(field.descriptor.is_class(b"java/lang/String"));
				StackTraceElement_moduleName_FIELD_OFFSET = field.idx;
				field_set |= 1 << 1;
				continue;
			}

			if field.name == sym!(moduleVersion) {
				assert!(field.descriptor.is_class(b"java/lang/String"));
				StackTraceElement_moduleVersion_FIELD_OFFSET = field.idx;
				field_set |= 1 << 2;
				continue;
			}

			if field.name == sym!(declaringClass) {
				assert!(field.descriptor.is_class(b"java/lang/String"));
				StackTraceElement_declaringClass_FIELD_OFFSET = field.idx;
				field_set |= 1 << 3;
				continue;
			}

			if field.name == sym!(methodName) {
				assert!(field.descriptor.is_class(b"java/lang/String"));
				StackTraceElement_methodName_FIELD_OFFSET = field.idx;
				field_set |= 1 << 4;
				continue;
			}

			if field.name == sym!(fileName) {
				assert!(field.descriptor.is_class(b"java/lang/String"));
				StackTraceElement_fileName_FIELD_OFFSET = field.idx;
				field_set |= 1 << 5;
				continue;
			}

			if field.name == sym!(lineNumber) {
				assert!(field.descriptor.is_int());
				StackTraceElement_lineNumber_FIELD_OFFSET = field.idx;
				field_set |= 1 << 6;
				continue;
			}
		}

		assert_eq!(
			field_set, 0b1111111,
			"Not all fields were found in java/lang/StackTraceElement"
		);
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
			let declaring_class = StringInterner::intern_symbol(method.class().name);
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
					let file_name = StringInterner::intern_symbol(
						method_class
							.constant_pool
							.get::<cp_types::ConstantUtf8>(idx),
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

			if let VisibleStackFrame::Regular(frame) = frame {
				let pc = frame.thread().pc.load(Ordering::Relaxed) - 1;
				let line_number = method.get_line_number(pc);
				stacktrace_element.get_mut().put_field_value0(
					StackTraceElement_lineNumber_FIELD_OFFSET,
					Operand::Int(line_number),
				);
			}

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
	let this_class = this_class_instance.get().class();
	// TODO: Make global field
	let stacktrace_field = this_class.fields().find(|field| {
		field.name.as_str() == "stackTrace" && matches!(&field.descriptor, FieldType::Array(value) if value.is_class(b"java/lang/StackTraceElement"))
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
