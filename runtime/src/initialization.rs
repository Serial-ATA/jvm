use crate::class::Class;
use crate::classpath::classloader::ClassLoader;
use crate::native::jni::invocation_api::main_java_vm;
use crate::thread::JavaThread;

use crate::class_instance::ClassInstance;
use crate::java_call;
use crate::reference::{ClassRef, Reference};
use crate::string_interner::StringInterner;
use classfile::accessflags::MethodAccessFlags;
use classfile::FieldType;
use common::int_types::s4;
use instructions::Operand;
use jni::java_vm::JavaVm;
use jni::sys::JavaVMInitArgs;
use symbols::sym;

pub fn create_java_vm(args: Option<&JavaVMInitArgs>) -> JavaVm {
	let thread = JavaThread::new();
	unsafe {
		JavaThread::set_current_thread(thread);
	}

	initialize_thread(JavaThread::current_mut());
	unsafe { main_java_vm() }
}

fn initialize_thread(thread: &mut JavaThread) {
	// Load some important classes first
	load_global_classes();

	// Grab the java.lang.String field offsets
	{
		let string_class = crate::globals::classes::java_lang_String();
		let string_value_field = string_class
			.fields()
			.find(|field| {
				!field.is_static()
					&& field.name == b"value"
					&& matches!(field.descriptor, FieldType::Array(ref val) if **val == FieldType::Byte)
			})
			.expect("java.lang.String should have a value field");

		let string_coder_field = string_class
			.fields()
			.find(|field| {
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

	JavaThread::set_field_holder_offsets();
	create_thread_object(thread);

	// https://github.com/openjdk/jdk/blob/04591595374e84cfbfe38d92bff4409105b28009/src/hotspot/share/runtime/threads.cpp#L408

	init_phase_1(thread);

	// TODO: ...

	init_phase_2(thread);

	// TODO: ...

	init_phase_3(thread);

	// TODO: https://github.com/openjdk/jdk/blob/04591595374e84cfbfe38d92bff4409105b28009/src/java.base/share/native/libjli/java.c#L1805
}

fn load_global_classes() {
	macro_rules! load {
		($($name:ident),+ $(,)?) => {{
			paste::paste! {
				$(
				let class = ClassLoader::Bootstrap.load(sym!($name)).unwrap();
				unsafe { $crate::globals::classes::[<set_ $name>](class); }
				)+
			}
		}};
	}

	load!(
		java_lang_Object,
		java_lang_Class,
		java_lang_String,
		java_lang_ClassLoader,
	);

	// Fixup mirrors, as we have classes that were loaded before java.lang.Class
	ClassLoader::fixup_mirrors();

	load!(
		java_lang_Thread,
		java_lang_Thread_FieldHolder,
		java_lang_ThreadGroup,
		java_lang_Throwable,
		java_lang_Cloneable,
	);

	// Primitive arrays
	load!(
		bool_array,
		byte_array,
		char_array,
		double_array,
		float_array,
		int_array,
		long_array,
		short_array,
	)
}

fn create_thread_object(thread: &mut JavaThread) {
	let thread_group_class = crate::globals::classes::java_lang_ThreadGroup();
	let system_thread_group_instance =
		Reference::class(ClassInstance::new(ClassRef::clone(&thread_group_class)));

	let init_method = thread_group_class
		.vtable()
		.find(
			sym!(object_initializer_name),
			sym!(void_method_signature),
			MethodAccessFlags::NONE,
		)
		.expect("java.lang.ThreadGroup should have an initializer");

	let name = StringInterner::intern_str("main");
	java_call!(
		thread,
		init_method,
		Operand::Reference(Reference::clone(&system_thread_group_instance))
	);

	unsafe {
		crate::globals::threads::set_main_thread_group(Reference::clone(
			&system_thread_group_instance,
		));
	}

	thread.init_obj(system_thread_group_instance);
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
fn init_phase_1(thread: &mut JavaThread) {
	let system_class = ClassLoader::Bootstrap.load(sym!(java_lang_System)).unwrap();
	let init_phase_1 = Class::resolve_method_step_two(
		system_class,
		sym!(initPhase1_name),
		sym!(void_method_signature),
	)
	.unwrap();

	java_call!(thread, init_phase_1);
}

/// Call `java.lang.System#initPhase2`
///
/// This is responsible for initializing the module system. Prior to this point, the only module
/// available to us is `java.base`.
fn init_phase_2(thread: &mut JavaThread) {
	let system_class = ClassLoader::Bootstrap.load(sym!(java_lang_System)).unwrap();

	// TODO: Actually set these arguments accordingly
	let display_vm_output_to_stderr = false;
	let print_stacktrace_on_exception = true;

	// TODO: Need some way to check failure
	let init_phase_2 = Class::resolve_method_step_two(
		system_class,
		sym!(initPhase2_name),
		sym!(bool_bool_int_signature),
	)
	.unwrap();

	java_call!(
		thread,
		init_phase_2,
		display_vm_output_to_stderr as s4,
		print_stacktrace_on_exception as s4
	);

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
fn init_phase_3(thread: &mut JavaThread) {
	let system_class = ClassLoader::Bootstrap.load(sym!(java_lang_System)).unwrap();

	let init_phase_3 = Class::resolve_method_step_two(
		system_class,
		sym!(initPhase3_name),
		sym!(void_method_signature),
	)
	.unwrap();

	java_call!(thread, init_phase_3);
}
