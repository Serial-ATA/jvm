use crate::classes::jdk::internal::misc;
use crate::classpath::loader::ClassLoader;
use crate::modules::Module;
use crate::native::java::lang::String::StringInterner;
use crate::native::jni::invocation_api::main_java_vm;
use crate::objects::class_instance::ClassInstance;
use crate::objects::reference::Reference;
use crate::options::JvmOptions;
use crate::symbols::sym;
use crate::thread::exceptions::Throws;
use crate::thread::{JavaThread, JavaThreadBuilder};
use crate::{classes, java_call};

use classfile::accessflags::MethodAccessFlags;
use common::int_types::s4;
use instructions::Operand;
use jni::error::JniError;
use jni::java_vm::JavaVm;
use jni::sys::{JNI_OK, JavaVMInitArgs};

/// Creates and initializes the Java VM
///
/// # Errors
///
/// Errors come in the form ([`JniError`], Option<[`Reference`]>), where the [`Reference`] is the exception thrown.
///
/// There are two error cases:
///
/// * **Before** the creation of the main thread, in which case the exception will be `None` since the
///   VM will not be able to throw any exception.
/// * **After** the creation of the main thread, where the exception *should* be `Some`. In this case,
///   the caller should print the contents of the exception.
pub fn create_java_vm(
	args: Option<&JavaVMInitArgs>,
) -> Result<JavaVm, (JniError, Option<Reference>)> {
	let _span = tracing::debug_span!("initialization").entered();
	tracing::debug!("Creating Java VM");

	if let Some(_vm_options) = crate::classpath::jimage::lookup_vm_options() {
		// TODO: Actually parse the options, for now this is just here to load the JImage
		// https://github.com/openjdk/jdk/blob/03a9a88efbb68537e24b7de28c5b81d6cd8fdb04/src/hotspot/share/runtime/arguments.cpp#L3322
	}

	let options = match args {
		Some(args) => {
			unsafe { JvmOptions::load(args) }.map_err(|_| (JniError::InvalidArguments, None))?
		},
		None => JvmOptions::default(),
	};

	let thread = JavaThreadBuilder::new().finish();
	unsafe {
		JavaThread::set_current_thread(thread);
	}

	if let Err((e, thread_available)) = initialize_thread(JavaThread::current()) {
		if !thread_available {
			return Err((e, None));
		}

		let Some(exception) = JavaThread::current().take_pending_exception() else {
			tracing::warn!("Exception thrown but not set?");
			return Err((e, None));
		};

		return Err((e, Some(exception)));
	}

	Ok(unsafe { main_java_vm() })
}

/// The entire initialization stage of the VM
///
/// The bulk of initialization is handled in the `java.lang.System#initPhase{1,2,3}` methods, but there
/// is some work we need to do before that point.
///
/// The order of operations is very important:
///
/// 1. Create a dummy `java.base` module
///     * This is needed for class loading. The module is fixed up later when `java.lang.Module` is
///       initialized. For now, it will be in an invalid state. See [`create_java_base()`].
/// 2. Load important classes & store their field offsets.
/// 3. *Initialize* some of the classes that were loaded.
/// 4. Create the initial `java.lang.Thread` for the current thread.
///
/// [`create_java_base()`]: crate::modules::ModuleLockGuard::create_java_base()
fn initialize_thread(thread: &'static JavaThread) -> Result<(), (JniError, bool)> {
	crate::modules::with_module_lock(|guard| Module::create_java_base(guard));

	// Load some important classes
	if let Throws::Exception(_) = load_global_classes() {
		return Err((JniError::ExceptionThrown, false));
	}

	init_field_offsets();

	// Init some important classes
	if let Throws::Exception(_) = initialize_global_classes(thread) {
		// An exception was thrown while initializing classes, no thread exists to handle it.
		return Err((JniError::ExceptionThrown, false));
	}

	if !create_thread_object(thread) {
		// An exception was thrown, this thread is NOT safe to use.
		return Err((JniError::ExceptionThrown, false));
	}

	// SAFETY: Preconditions filled in `init_field_offsets` & `initialize_global_classes`
	unsafe {
		misc::UnsafeConstants::init();
	}

	// Create native entrypoints for `java.lang.invoke.MethodHandle#link*` methods
	classes::java::lang::invoke::MethodHandle::init_entry_points();

	init_phase_1(thread).map_err(|e| (e, true))?;
	init_phase_2(thread).map_err(|e| (e, true))?;
	init_phase_3(thread).map_err(|e| (e, true))?;

	Ok(())
}

fn load_global_classes() -> Throws<()> {
	macro_rules! load {
		($($name:ident),+ $(,)?) => {{
			paste::paste! {
				$(
				let class = ClassLoader::bootstrap().load(sym!($name))?;
				unsafe { $crate::globals::classes::[<set_ $name>](class); }
				)+
			}
		}};
	}

	// The order here is very important.
	//
	// java.lang.Class MUST be loaded last, as its presence is used by the ClassLoader to determine
	// whether it is safe to create mirror instances yet. All classes that come before it are ones
	// that java.lang.Class depends on.
	//
	// After this point, any other class loads can appear in an arbitrary order.
	load!(
		java_lang_Object,
		java_lang_String,
		java_lang_ClassLoader,
		java_lang_Module,
		java_lang_Class,
	);

	// Pre-fire java.lang.Class field offset initialization, as it's needed by mirrors. All other
	// classes handle this in `init_field_offsets()`.
	unsafe {
		classes::java::lang::Class::init_offsets();
	}

	// Fixup mirrors, as we have classes that were loaded before java.lang.Class
	ClassLoader::fixup_mirrors();

	load!(
		jdk_internal_misc_UnsafeConstants,
		java_lang_System,
		java_lang_Thread,
		java_lang_Thread_FieldHolder,
		java_lang_ThreadGroup,
		java_lang_Throwable,
		java_lang_Cloneable,
		java_io_Serializable,
		java_lang_ref_Reference,
		java_lang_ref_Finalizer,
		java_lang_VirtualMachineError,
	);

	// MethodHandle stuff
	load!(
		java_lang_invoke_MethodHandle,
		java_lang_invoke_LambdaForm,
		jdk_internal_reflect_MethodAccessorImpl,
		jdk_internal_reflect_ConstantPool,
		java_lang_invoke_MethodHandleNatives,
		java_lang_invoke_MemberName,
		java_lang_invoke_ResolvedMethodName,
		java_lang_invoke_MethodType,
		java_lang_invoke_VarHandle,
		java_lang_reflect_Constructor,
		java_lang_reflect_Method,
		java_lang_reflect_Field,
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

	// Create the primitive mirrors (java.lang.Integer, etc...)
	crate::globals::mirrors::init_primitive_mirrors();

	// Primitive arrays
	load!(
		boolean_array,
		byte_array,
		character_array,
		double_array,
		float_array,
		integer_array,
		long_array,
		short_array,
		string_array,
	);

	Throws::Ok(())
}

fn init_field_offsets() {
	// java.lang.ClassLoader
	unsafe {
		classes::java::lang::ClassLoader::init_offsets();
	}

	// java.lang.String
	unsafe {
		classes::java::lang::String::init_offsets();
	}

	// java.lang.Module
	unsafe {
		classes::java::lang::Module::init_offsets();
	}

	// java.lang.ref.Reference
	unsafe {
		crate::classes::java::lang::r#ref::Reference::init_offsets();
	}

	// jdk.internal.misc.UnsafeConstants
	unsafe {
		misc::UnsafeConstants::init_offsets();
	}

	// java.lang.Thread
	unsafe {
		classes::java::lang::Thread::init_offsets();
	}

	// MethodHandle stuff
	{
		// java.lang.invoke.MethodHandle
		unsafe {
			classes::java::lang::invoke::MethodHandle::init_offsets();
		}

		// java.lang.invoke.LambdaForm
		unsafe {
			classes::java::lang::invoke::LambdaForm::init_offsets();
		}

		// java.lang.invoke.MemberName
		unsafe {
			classes::java::lang::invoke::MemberName::init_offsets();
		}

		// java.lang.invoke.ResolvedMethodName
		unsafe {
			classes::java::lang::invoke::ResolvedMethodName::init_offsets();
		}

		// java.lang.invoke.MethodType
		unsafe {
			classes::java::lang::invoke::MethodType::init_offsets();
		}
	}

	// Reflection stuff
	{
		// java.lang.reflect.Method
		unsafe {
			classes::java::lang::reflect::Method::init_offsets();
		}

		// java.lang.reflect.Field
		unsafe {
			classes::java::lang::reflect::Field::init_offsets();
		}

		// java.lang.reflect.Constructor
		unsafe {
			classes::java::lang::reflect::Constructor::init_offsets();
		}
	}
}

fn initialize_global_classes(thread: &'static JavaThread) -> Throws<()> {
	crate::globals::classes::java_lang_Object().initialize(thread)?;
	crate::globals::classes::java_lang_Class().initialize(thread)?;
	crate::globals::classes::java_lang_String().initialize(thread)?;

	crate::globals::classes::java_lang_Character().initialize(thread)?;

	crate::globals::classes::java_lang_Thread().initialize(thread)?;
	crate::globals::classes::java_lang_ThreadGroup().initialize(thread)?;
	crate::globals::classes::java_lang_ref_Finalizer().initialize(thread)?;

	crate::globals::classes::jdk_internal_misc_UnsafeConstants().initialize(thread)?;
	crate::globals::classes::java_lang_Module().initialize(thread)?;

	crate::globals::classes::java_lang_reflect_Method().initialize(thread)?;

	Throws::Ok(())
}

fn create_thread_object(thread: &'static JavaThread) -> bool {
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

	let name = StringInterner::intern("main");
	let result = java_call!(
		thread,
		init_method,
		Operand::Reference(Reference::clone(&system_thread_group_instance))
	);

	if thread.has_pending_exception() {
		return false;
	}

	unsafe {
		crate::globals::threads::set_main_thread_group(Reference::clone(
			&system_thread_group_instance,
		));
	}

	thread.init_obj(system_thread_group_instance);
	true
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
fn init_phase_1(thread: &'static JavaThread) -> Result<(), JniError> {
	let system_class = crate::globals::classes::java_lang_System();
	let init_phase_1;
	match system_class.resolve_method(sym!(initPhase1_name), sym!(void_method_signature)) {
		Throws::Ok(method) => init_phase_1 = method,
		Throws::Exception(e) => {
			e.throw(thread);
			return Err(JniError::ExceptionThrown);
		},
	}

	java_call!(thread, init_phase_1);

	if thread.has_pending_exception() {
		return Err(JniError::ExceptionThrown);
	}

	Ok(())
}

/// Call `java.lang.System#initPhase2`
///
/// This is responsible for initializing the module system. Prior to this point, the only module
/// available to us is `java.base`.
fn init_phase_2(thread: &'static JavaThread) -> Result<(), JniError> {
	let system_class = crate::globals::classes::java_lang_System();

	// TODO: Actually set these arguments accordingly
	let display_vm_output_to_stderr = false;
	let print_stacktrace_on_exception = true;

	let init_phase_2;
	match system_class.resolve_method(sym!(initPhase2_name), sym!(bool_bool_int_signature)) {
		Throws::Ok(method) => init_phase_2 = method,
		Throws::Exception(e) => {
			e.throw(thread);
			return Err(JniError::ExceptionThrown);
		},
	}

	let result = java_call!(
		thread,
		init_phase_2,
		display_vm_output_to_stderr as s4,
		print_stacktrace_on_exception as s4
	);

	if thread.has_pending_exception() {
		return Err(JniError::ExceptionThrown);
	}

	let ret = result.unwrap().expect_int();
	if ret != JNI_OK {
		return Err(JniError::Unknown);
	}

	Ok(())
}

/// Call `java.lang.System#initPhase3`
///
/// This is responsible for the following:
///
/// * Initialization of and setting the security manager
/// * Setting the system class loader
/// * Setting the thread context class loader
fn init_phase_3(thread: &'static JavaThread) -> Result<(), JniError> {
	let system_class = crate::globals::classes::java_lang_System();

	let init_phase_3;
	match system_class.resolve_method(sym!(initPhase3_name), sym!(void_method_signature)) {
		Throws::Ok(method) => init_phase_3 = method,
		Throws::Exception(e) => {
			e.throw(thread);
			return Err(JniError::ExceptionThrown);
		},
	}

	java_call!(thread, init_phase_3);

	if thread.has_pending_exception() {
		return Err(JniError::ExceptionThrown);
	}

	Ok(())
}
