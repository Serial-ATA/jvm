use crate::classpath::classloader::ClassLoader;

use std::sync::atomic::{AtomicBool, Ordering};

static JVM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub(crate) fn initialize() {
	if JVM_INITIALIZED.compare_exchange(false, true, Ordering::SeqCst, Ordering::Relaxed)
		!= Ok(false)
	{
		// We've already initialized!
		return;
	}

	// Load some important classes first
	ClassLoader::Bootstrap.load(b"java/lang/Object").unwrap();
	ClassLoader::Bootstrap.load(b"java/lang/Class").unwrap();
	ClassLoader::Bootstrap.load(b"java/lang/String").unwrap();
}
