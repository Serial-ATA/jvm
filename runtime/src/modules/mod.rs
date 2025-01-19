use std::sync::{Mutex, MutexGuard};

mod entry;
pub use entry::Module;

mod package;
pub use package::{Package, PackageExportType};

mod set;
pub use set::ModuleSet;

static MODULE_LOCK: Mutex<()> = Mutex::new(());

#[expect(dead_code)] // Never actually need to use the guard
pub struct ModuleLockGuard(MutexGuard<'static, ()>);

/// Run the provided function while holding the [`ModuleLockGuard`]
///
/// This is the only way to interact with the module system.
pub fn with_module_lock<F>(f: F)
where
	F: FnOnce(&ModuleLockGuard),
{
	let _guard = MODULE_LOCK.lock().unwrap();
	f(&ModuleLockGuard(_guard));
}
