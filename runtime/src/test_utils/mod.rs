use crate::classpath::loader::ClassLoader;
use crate::initialization::initialize_thread;
use crate::native::jvm::system::SYSTEM_PROPERTIES;
use crate::options::JvmOptions;
use crate::thread::{JavaThread, JavaThreadBuilder};

use std::sync::Once;

mod loader;
mod thread;

/// Initialize a shared runtime
///
/// As this is shared, it has the following conditions:
///
/// * No Java code can be executed on the returned [`JavaThread`]
/// * No additional classes can be loaded
///
/// This is only useful for tests that require access to basic classes like `java.lang.Class`, but
/// don't need a fully functional runtime.
///
/// All tests will block on this function until the runtime is initialized. Afterwards, all tests can
/// safely execute in parallel.
pub fn init_basic_shared_runtime() -> &'static JavaThread {
	static ONCE: Once = Once::new();

	ONCE.call_once(|| {
		let test_java_home = std::env::var("TEST_JAVA_HOME")
			.or_else(|_| std::env::var("JAVA_HOME"))
			.expect("TEST_JAVA_HOME not set!");

		{
			let mut guard = SYSTEM_PROPERTIES.lock().unwrap();
			guard.insert(String::from("java.home"), test_java_home);
		}

		crate::classpath::jimage::lookup_vm_options();

		let options = JvmOptions::default();
		options.logs.apply();

		let thread = JavaThreadBuilder::new()
			.finish(false)
			.expect("failed to allocate thread");

		JavaThread::set_current_thread(thread);
		unsafe {
			JavaThread::set_shared_thread(thread);
		}

		initialize_thread(thread, false).unwrap();

		// Seal the thread so it can't execute anymore Java code
		thread.seal();
		// Seal the bootstrap loader so it can't load anymore classes
		ClassLoader::bootstrap().seal();
	});

	JavaThread::shared()
}
