use crate::class::Class;
use crate::classpath::classloader::ClassLoader;
use crate::thread::{Thread, ThreadRef};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use classfile::FieldType;
use common::int_types::s4;
use instructions::Operand;
use symbols::sym;

static JVM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn jvm_initialized() -> bool {
	JVM_INITIALIZED.load(Ordering::Relaxed)
}

pub(crate) fn initialize(thread: ThreadRef) {
	if jvm_initialized() {
		// We've already initialized!
		return;
	}

	// Load some important classes first
	ClassLoader::Bootstrap.load(sym!(java_lang_Object)).unwrap();
	ClassLoader::Bootstrap.load(sym!(java_lang_Class)).unwrap();
	let string_class = ClassLoader::Bootstrap.load(sym!(java_lang_String)).unwrap();
	{
		let string_value_field = string_class
			.unwrap_class_instance()
			.find_field(|field| {
				!field.is_static()
					&& field.name == b"value"
					&& matches!(field.descriptor, FieldType::Array(ref val) if **val == FieldType::Byte)
			})
			.expect("java.lang.String should have a value field");

		let string_coder_field = string_class
			.unwrap_class_instance()
			.find_field(|field| {
				field.is_final()
					&& field.name == b"coder"
					&& matches!(field.descriptor, FieldType::Byte)
			})
			.expect("java.lang.String should have a value field");

		unsafe {
			crate::globals::field_offsets::set_string_field_offsets(
				string_value_field.idx,
				string_coder_field.idx,
			);
		}
	}

	// Fixup mirrors, as we have classes that were loaded before java.lang.Class
	ClassLoader::fixup_mirrors();

	// https://github.com/openjdk/jdk/blob/04591595374e84cfbfe38d92bff4409105b28009/src/hotspot/share/runtime/threads.cpp#L408

	init_phase_1(Arc::clone(&thread));

	// TODO: ...

	init_phase_2(Arc::clone(&thread));

	// TODO: ...

	init_phase_3(Arc::clone(&thread));

	// TODO: https://github.com/openjdk/jdk/blob/04591595374e84cfbfe38d92bff4409105b28009/src/java.base/share/native/libjli/java.c#L1805

	JVM_INITIALIZED.store(true, Ordering::SeqCst);
}

/// Call `java.lang.System#initPhase1`
///
/// This is responsible for initializing the java.lang.System class with the following:
///
/// * System properties
/// * Std{in, out, err}
/// * Signal handlers
/// * OS-specific system settings
/// * Thread group of the main thread
fn init_phase_1(thread: ThreadRef) {
	let system_class = ClassLoader::Bootstrap.load(sym!(java_lang_System)).unwrap();

	let init_phase_1 = Class::resolve_method_step_two(
		system_class,
		sym!(initPhase1_name),
		sym!(void_method_signature),
	)
	.unwrap();
	Thread::pre_main_invoke_method(Arc::clone(&thread), init_phase_1, None);
}

/// Call `java.lang.System#initPhase2`
///
/// This is responsible for initializing the module system. Prior to this point, the only module
/// available to us is `java.base`.
fn init_phase_2(thread: ThreadRef) {
	let system_class = ClassLoader::Bootstrap.load(sym!(java_lang_System)).unwrap();

	// TODO: Actually set these arguments accordingly
	let display_vm_output_to_stderr = false;
	let print_stacktrace_on_exception = true;
	let init_phase_2_args = vec![
		Operand::Int(display_vm_output_to_stderr as s4),
		Operand::Int(print_stacktrace_on_exception as s4),
	];

	// TODO: Need some way to check failure
	let init_phase_2 = Class::resolve_method_step_two(
		system_class,
		sym!(initPhase2_name),
		sym!(bool_bool_int_signature),
	)
	.unwrap();
	Thread::pre_main_invoke_method(thread, init_phase_2, Some(init_phase_2_args));

	unsafe {
		crate::globals::modules::set_module_system_initialized();
	}
}

/// Call `java.lang.System#initPhase3`
///
/// This is responsible for the following:
///
/// * Initialization of and setting the security manager
/// * Setting the system class loader
/// * Setting the thread context class loader
fn init_phase_3(thread: ThreadRef) {
	let system_class = ClassLoader::Bootstrap.load(sym!(java_lang_System)).unwrap();

	let init_phase_3 = Class::resolve_method_step_two(
		system_class,
		sym!(initPhase3_name),
		sym!(void_method_signature),
	)
	.unwrap();
	Thread::pre_main_invoke_method(thread, init_phase_3, None);
}
