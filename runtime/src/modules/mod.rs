use crate::objects::reference::Reference;

use std::cell::SyncUnsafeCell;

use symbols::sym;

mod entry;

pub use entry::Module;

mod package;
pub use package::Package;

/// Special case for `java.base`, as it is heavily used during VM initialization, and is initially
/// created in an **invalid state**.
static JAVA_BASE: SyncUnsafeCell<Option<Module>> = SyncUnsafeCell::new(None);

fn set_java_base(entry: Module) {
	let ptr = JAVA_BASE.get();
	assert!(unsafe { &*ptr }.is_none(), "java.base can only be set once");
	unsafe {
		*ptr = Some(entry);
	}
}

pub fn java_base() -> &'static Module {
	let opt = unsafe { &*JAVA_BASE.get() };
	opt.as_ref().expect("java.base should be set")
}

/// Create the entry for `java.base`
///
/// This is only useful for bootstrapping purposes, very early in VM initialization. The [`Module`]
/// produced is **not valid**.
pub fn create_java_base() {
	// Don't use the constructors, since they do validation.
	let java_base_module = Module {
		name: Some(sym!(java_base)),
		open: true,
		obj: Reference::null(),
		version: None,
		location: None,
	};

	set_java_base(java_base_module);
}
