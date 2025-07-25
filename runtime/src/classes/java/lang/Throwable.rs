use crate::classes::AsClassInstanceRef;
use crate::java_call;
use crate::native::java::lang::Throwable::BackTrace;
use crate::objects::instance::Instance;
use crate::objects::instance::class::ClassInstanceRef;
use crate::objects::instance::object::Object;
use crate::objects::reference::Reference;
use crate::symbols::sym;
use crate::thread::JavaThread;

use classfile::FieldType;
use classfile::accessflags::MethodAccessFlags;
use instructions::Operand;
use jni::sys::{jint, jlong};

/// `java.lang.Throwable#stackTrace` field
pub fn stackTrace<I: AsClassInstanceRef>(instance: I) -> Reference {
	instance
		.as_class_instance_ref()
		.get_field_value0(stackTrace_field_index())
		.expect_reference()
}

pub fn set_stackTrace<I: AsClassInstanceRef>(instance: I, value: Reference) {
	instance
		.as_class_instance_ref()
		.put_field_value0(stackTrace_field_index(), Operand::Reference(value))
}

/// `java.lang.Throwable#backtrace` field
pub fn backtrace<I: AsClassInstanceRef>(instance: I) -> Reference {
	instance
		.as_class_instance_ref()
		.get_field_value0(backtrace_field_index())
		.expect_reference()
}

pub fn set_backtrace<I: AsClassInstanceRef>(instance: I, value: Reference) {
	instance
		.as_class_instance_ref()
		.put_field_value0(backtrace_field_index(), Operand::Reference(value))
}

/// `java.lang.Throwable#depth` field
pub fn depth<I: AsClassInstanceRef>(instance: I) -> jint {
	instance
		.as_class_instance_ref()
		.get_field_value0(depth_field_index())
		.expect_int()
}

pub fn set_depth<I: AsClassInstanceRef>(instance: I, value: jint) {
	instance
		.as_class_instance_ref()
		.put_field_value0(depth_field_index(), Operand::Int(value))
}

pub fn detail_message<I: AsClassInstanceRef>(instance: I) -> Reference {
	instance
		.as_class_instance_ref()
		.get_field_value0(detailMessage_field_index())
		.expect_reference()
}

pub fn print(this: ClassInstanceRef) {
	eprint!("{}", this.class().external_name());
	let detail_message = detail_message(this);
	if detail_message.is_null() {
		return;
	}

	let detail_message_string = detail_message.extract_class();
	let detail_message = super::String::extract(detail_message_string);

	eprint!(": {detail_message}");
}

pub fn print_stack_trace(this: Reference, thread: &'static JavaThread) {
	assert!(this.is_instance_of(crate::globals::classes::java_lang_Throwable()));

	let exception_class = this.extract_instance_class();
	let print_stack_trace = exception_class
		.vtable()
		.find(
			sym!(printStackTrace_name),
			sym!(void_method_signature),
			MethodAccessFlags::NONE,
		)
		.expect("java/lang/Throwable#printStackTrace should exist");

	java_call!(thread, print_stack_trace, Operand::Reference(this));
}

/// Manual implementation of `java.lang.Throwable#printStackTrace` that doesn't rely on `java.lang.System`
///
/// This is used for exceptions that occur early in VM initialization, which may happen prior to the
/// print streams being set.
pub fn print_stack_trace_without_java_system(this: Reference, thread: &'static JavaThread) {
	assert!(this.is_instance_of(crate::globals::classes::java_lang_Throwable()));

	let instance = this.extract_class();
	print(instance);
	println!();

	let backtrace = backtrace(instance);
	if backtrace.is_null() {
		eprintln!("\t<<no stack trace available>>");
		return;
	}

	let backtrace_array = backtrace.extract_primitive_array();

	let backtrace_array = backtrace_array.as_slice::<jlong>();
	for elem in BackTrace::from_encoded(backtrace_array) {
		let class = elem.method.class();
		eprint!("\tat {}.{}(", class.external_name(), elem.method.name);

		if let Some(module) = class.module().name() {
			match class.module().version() {
				Some(version) => eprint!("{module}@{version}/"),
				None => eprint!("{module}/"),
			}
		}

		let line_number = elem.method.line_number(elem.pc as isize);
		if line_number == -2 {
			eprint!("Native Method)")
		} else {
			if let Some(source_file_name) = elem.method.class().source_file_name() {
				if line_number == -1 {
					eprint!("{source_file_name})")
				} else {
					eprint!("{source_file_name}:{line_number})")
				}
			} else {
				eprint!("Unknown Source)");
			}
		}

		eprintln!();
	}

	let get_cause = instance
		.class()
		.vtable()
		.find(
			sym!(getCause),
			sym!(Throwable_signature),
			MethodAccessFlags::NONE,
		)
		.expect("java.lang.Throwable#getCause should exist");

	let Some(cause) = java_call!(thread, get_cause, Operand::Reference(this)) else {
		// We ignore any exceptions while handling exceptions
		thread.discard_pending_exception();
		return;
	};

	let cause = cause.expect_reference();
	if cause.is_null() {
		return;
	}

	eprint!("Caused by: ");
	print(cause.extract_class());
	eprintln!();
}

crate::classes::field_module! {
	@CLASS java_lang_Throwable;

	@FIELDSTART
	/// `java.lang.Throwable#stackTrace` field offset
	///
	/// Expected field type: `Reference` to `StackTraceElement[]`
	@FIELD stackTrace: FieldType::Array(val) if val.is_class(b"java/lang/StackTraceElement"),
	/// `java.lang.Throwable#backtrace` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Object`
	@FIELD backtrace: FieldType::Object(_),
	/// `java.lang.Throwable#detailMessage` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD detailMessage: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.Throwable#depth` field offset
	///
	/// Expected field type: `jint`
	@FIELD depth: FieldType::Integer,
}
