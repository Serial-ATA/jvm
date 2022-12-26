use crate::class::Class;
use crate::classpath::classloader::ClassLoader;
use crate::thread::{Thread, ThreadRef};

use std::sync::atomic::{AtomicBool, Ordering};

static JVM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub(crate) fn initialize(thread: ThreadRef) {
	if JVM_INITIALIZED.compare_exchange(false, true, Ordering::SeqCst, Ordering::Relaxed)
		!= Ok(false)
	{
		// We've already initialized!
		return;
	}

	// Load some important classes first
	ClassLoader::Bootstrap.load(b"java/lang/Object").unwrap();
	ClassLoader::Bootstrap.load(b"java/lang/String").unwrap();

	let system_class = ClassLoader::Bootstrap.load(b"java/lang/System").unwrap();
	ClassLoader::Bootstrap.load(b"java/lang/Class").unwrap();

	// Call `java.lang.System#initPhase1`
	let init_phase_1 =
		Class::resolve_method_step_two(system_class.unwrap_class_instance(), b"initPhase1", b"()V")
			.unwrap();
	Thread::pre_main_invoke_method(thread, init_phase_1);

	// TODO: initPhase2: https://github.com/openjdk/jdk/blob/04591595374e84cfbfe38d92bff4409105b28009/src/hotspot/share/runtime/threads.cpp#L298
	// TODO: initPhase3: https://github.com/openjdk/jdk/blob/04591595374e84cfbfe38d92bff4409105b28009/src/hotspot/share/runtime/threads.cpp#L322
}
