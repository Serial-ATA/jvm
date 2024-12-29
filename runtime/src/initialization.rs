use crate::classpath::classloader::ClassLoader;
use crate::java_call;
use crate::native::jni::invocation_api::main_java_vm;
use crate::objects::class_instance::ClassInstance;
use crate::objects::reference::Reference;
use crate::string_interner::StringInterner;
use crate::thread::{java_lang_Thread, JavaThread, JavaThreadBuilder};

use classfile::accessflags::MethodAccessFlags;
use classfile::FieldType;
use common::int_types::s4;
use instructions::Operand;
use jni::java_vm::JavaVm;
use jni::sys::JavaVMInitArgs;
use symbols::sym;

pub fn create_java_vm(args: Option<&JavaVMInitArgs>) -> JavaVm {
	let thread = JavaThreadBuilder::new().finish();
	unsafe {
		JavaThread::set_current_thread(thread);
	}

	initialize_thread(JavaThread::current());
	unsafe { main_java_vm() }
}

fn initialize_thread(thread: &JavaThread) {
	// Load some important classes first
	load_global_classes();

	init_field_offsets();

	// Init some important classes
	initialize_global_classes(thread);

	create_thread_object(thread);

	// TODO: Set jdk/internal/misc/UnsafeConstants

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
		java_lang_Module,
		java_lang_Thread,
		java_lang_Thread_FieldHolder,
		java_lang_ThreadGroup,
		java_lang_Throwable,
		java_lang_Cloneable,
		java_lang_ref_Reference,
		java_lang_ref_Finalizer,
		jdk_internal_reflect_MethodAccessorImpl,
	);

	// Primitive types
	load!(
		java_lang_Boolean,
		java_lang_Byte,
		java_lang_Character,
		java_lang_Double,
		java_lang_Float,
		java_lang_Integer,
		java_lang_Long,
		java_lang_Short,
		java_lang_Void,
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

fn init_field_offsets() {
	// java.lang.Class
	{
		let class_class = crate::globals::classes::java_lang_Class();
		let class_name_field = class_class
			.fields()
			.find(|field| {
				!field.is_static()
					&& field.name == sym!(name)
					&& matches!(field.descriptor, FieldType::Object(ref val) if **val == *b"java/lang/String")
			})
			.expect("java.lang.Class should have a name field");

		unsafe {
			crate::globals::field_offsets::java_lang_Class::set_name_field_offset(
				class_name_field.idx,
			);
		}
	}

	// java.lang.String
	{
		let string_class = crate::globals::classes::java_lang_String();
		let string_value_field = string_class
			.fields()
			.find(|field| {
				!field.is_static()
					&& field.name == sym!(value)
					&& matches!(field.descriptor, FieldType::Array(ref val) if **val == FieldType::Byte)
			})
			.expect("java.lang.String should have a value field");

		let string_coder_field = string_class
			.fields()
			.find(|field| {
				field.is_final()
					&& field.name == sym!(coder)
					&& matches!(field.descriptor, FieldType::Byte)
			})
			.expect("java.lang.String should have a value field");

		unsafe {
			crate::globals::field_offsets::java_lang_String::set_field_offsets(
				string_value_field.idx,
				string_coder_field.idx,
			);
		}
	}

	// java.lang.Module
	{
		let module_class = crate::globals::classes::java_lang_Module();
		let module_name_field = module_class
			.fields()
			.find(|field| {
				!field.is_static()
					&& field.name == sym!(name)
					&& matches!(field.descriptor, FieldType::Object(ref val) if **val == *b"java/lang/String")
			})
			.expect("java.lang.Module should have a name field");

		let module_loader_field = module_class
			.fields()
			.find(|field| {
				!field.is_static()
					&& field.name == sym!(loader)
					&& matches!(field.descriptor, FieldType::Object(ref val) if **val == *b"java/lang/ClassLoader")
			})
			.expect("java.lang.Module should have a loader field");

		unsafe {
			crate::globals::field_offsets::java_lang_Module::set_name_field_offset(
				module_name_field.idx,
			);
			crate::globals::field_offsets::java_lang_Module::set_loader_field_offset(
				module_loader_field.idx,
			);
		}
	}

	// java.lang.ref.Reference
	{
		let reference_class = crate::globals::classes::java_lang_ref_Reference();
		let reference_referent_field = reference_class
			.fields()
			.find(|field| {
				!field.is_static()
					&& field.name == sym!(referent)
					&& matches!(field.descriptor, FieldType::Object(_))
			})
			.expect("java.lang.ref.Reference should have a referent field");

		unsafe {
			crate::globals::field_offsets::java_lang_ref_Reference::set_referent_field_offset(
				reference_referent_field.idx,
			);
		}
	}

	// java.lang.Thread
	java_lang_Thread::set_field_offsets();
}

fn initialize_global_classes(thread: &JavaThread) {
	crate::globals::classes::java_lang_Object().initialize(thread);
	crate::globals::classes::java_lang_Class().initialize(thread);
	crate::globals::classes::java_lang_String().initialize(thread);

	crate::globals::classes::java_lang_Character().initialize(thread);

	crate::globals::classes::java_lang_Thread().initialize(thread);
	crate::globals::classes::java_lang_ThreadGroup().initialize(thread);
	crate::globals::classes::java_lang_ref_Finalizer().initialize(thread);
}

fn create_thread_object(thread: &JavaThread) {
	let thread_group_class = crate::globals::classes::java_lang_ThreadGroup();
	let system_thread_group_instance = Reference::class(ClassInstance::new(thread_group_class));

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
fn init_phase_1(thread: &JavaThread) {
	let system_class = ClassLoader::Bootstrap.load(sym!(java_lang_System)).unwrap();
	let init_phase_1 = system_class
		.resolve_method_step_two(sym!(initPhase1_name), sym!(void_method_signature))
		.unwrap();

	java_call!(thread, init_phase_1);
}

/// Call `java.lang.System#initPhase2`
///
/// This is responsible for initializing the module system. Prior to this point, the only module
/// available to us is `java.base`.
fn init_phase_2(thread: &JavaThread) {
	let system_class = ClassLoader::Bootstrap.load(sym!(java_lang_System)).unwrap();

	// TODO: Actually set these arguments accordingly
	let display_vm_output_to_stderr = false;
	let print_stacktrace_on_exception = true;

	// TODO: Need some way to check failure
	let init_phase_2 = system_class
		.resolve_method_step_two(sym!(initPhase2_name), sym!(bool_bool_int_signature))
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
fn init_phase_3(thread: &JavaThread) {
	let system_class = ClassLoader::Bootstrap.load(sym!(java_lang_System)).unwrap();

	let init_phase_3 = system_class
		.resolve_method_step_two(sym!(initPhase3_name), sym!(void_method_signature))
		.unwrap();

	java_call!(thread, init_phase_3);
}
