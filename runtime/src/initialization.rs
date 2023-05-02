use crate::class::Class;
use crate::classpath::classloader::ClassLoader;
use crate::thread::{Thread, ThreadRef};

use std::sync::atomic::{AtomicBool, Ordering};

use classfile::FieldType;

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
	ClassLoader::Bootstrap.load(b"java/lang/Object").unwrap();
	ClassLoader::Bootstrap.load(b"java/lang/Class").unwrap();
	let string_class = ClassLoader::Bootstrap.load(b"java/lang/String").unwrap();
	{
		let string_value_field = string_class
			.unwrap_class_instance()
			.find_field(|field| {
				!field.is_static()
					&& field.name == b"value"
					&& matches!(field.descriptor, FieldType::Array(ref val) if **val == FieldType::Byte)
			})
			.expect("java.lang.String should have a value field");

		unsafe {
			crate::globals::STRING_VALUE_FIELD_OFFSET = string_value_field.idx;
		}
	}

	// Fixup mirrors, as we have classes that were loaded before java.lang.Class
	ClassLoader::fixup_mirrors();

	let system_class = ClassLoader::Bootstrap.load(b"java/lang/System").unwrap();

	// Call `java.lang.System#initPhase1`
	let init_phase_1 = Class::resolve_method_step_two(system_class, b"initPhase1", b"()V").unwrap();
	Thread::pre_main_invoke_method(thread, init_phase_1);

	// TODO: initPhase2: https://github.com/openjdk/jdk/blob/04591595374e84cfbfe38d92bff4409105b28009/src/hotspot/share/runtime/threads.cpp#L298
	// TODO: initPhase3: https://github.com/openjdk/jdk/blob/04591595374e84cfbfe38d92bff4409105b28009/src/hotspot/share/runtime/threads.cpp#L322

	// TODO: https://github.com/openjdk/jdk/blob/04591595374e84cfbfe38d92bff4409105b28009/src/java.base/share/native/libjli/java.c#L1805

	JVM_INITIALIZED.store(true, Ordering::SeqCst);
}
