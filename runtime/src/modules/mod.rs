use crate::classpath::classloader::ClassLoader;
use crate::objects::reference::Reference;

use std::cell::SyncUnsafeCell;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use symbols::{sym, Symbol};

mod entry;
pub use entry::Module;

mod package;
pub use package::Package;

static MODULE_LOCK: Mutex<()> = Mutex::new(());

pub struct ModuleLockGuard(MutexGuard<'static, ()>);

/// Run the provided function while holding the [`ModuleLockGuard`]
///
/// This is the only way to interact with the module system.
pub fn with_module_lock<F>(f: F)
where
	F: FnOnce(ModuleLockGuard),
{
	let _guard = MODULE_LOCK.lock().unwrap();
	f(ModuleLockGuard(_guard));
}

impl ModuleLockGuard {
	/// Create the entry for `java.base`
	///
	/// This is only useful for bootstrapping purposes, very early in VM initialization. The [`Module`]
	/// produced is **not valid**.
	pub fn create_java_base(&self) {
		// Don't use the constructors, since they do validation.
		let java_base_module = Module {
			name: Some(sym!(java_base)),
			open: false,
			obj: SyncUnsafeCell::new(Reference::null()),
			version: SyncUnsafeCell::new(None),
			location: SyncUnsafeCell::new(None),
		};

		ClassLoader::bootstrap().set_java_base(java_base_module);
	}

	pub fn fixup_java_base(
		&self,
		java_base: &Module,
		obj: Reference,
		version: Option<Symbol>,
		location: Option<Symbol>,
	) {
		assert!(
			!java_base.has_obj(),
			"java.base module can only be initialized once"
		);

		assert!(obj.is_class(), "invalid associated object for module");
		unsafe {
			*java_base.obj.get() = obj.clone();
			*java_base.version.get() = version;
			*java_base.location.get() = location;
		}

		// All classes we've loaded up to this point need to be added to `java.base`
		ClassLoader::fixup_modules(obj);
	}
}
