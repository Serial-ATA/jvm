use super::{Module, ModuleLockGuard};
use crate::classes;
use crate::classpath::loader::ClassLoader;
use crate::symbols::Symbol;

use std::cell::SyncUnsafeCell;
use std::collections::LinkedList;

use jni::sys::jlong;

/// A list of [`Module`]s, synchronized with the global [module lock]
///
/// [module lock]: crate::modules::with_module_lock
pub struct ModuleSet {
	// A list of all currently loaded modules
	//
	// This is a `LinkedList`, as reads will do a lifetime-extension, and are not guarded. We cannot
	// risk a realloc invalidating a reference.
	list: SyncUnsafeCell<LinkedList<Module>>,
}

impl ModuleSet {
	pub fn new() -> Self {
		Self {
			list: SyncUnsafeCell::new(LinkedList::new()),
		}
	}

	pub fn add(&self, _guard: &ModuleLockGuard, mut module: Module) -> &'static Module {
		let obj = module.obj();

		// Set `Module::loader` field. This gives us access to the packages from `ClassLoader`
		let obj_class_loader = obj.extract_target_class().loader().obj();
		if obj_class_loader.is_null() {
			module.set_classloader(ClassLoader::bootstrap())
		} else {
			let classloader_ptr =
				classes::java::lang::ClassLoader::injected_loader_ptr_for(obj_class_loader)
					.expect("classloader should be initialized");
			module.set_classloader(unsafe { &*classloader_ptr })
		}

		let list = unsafe { &mut *self.list.get() };
		list.push_back(module);

		let ret = list.back().unwrap();

		// Store the pointer in the module, to make future lookups cheaper
		classes::java::lang::Module::set_injected_module_ptr_for(
			obj,
			std::ptr::from_ref(ret) as jlong,
		);

		ret
	}

	pub fn find(&self, _guard: &ModuleLockGuard, name: Symbol) -> Option<&'static Module> {
		let list = unsafe { &*self.list.get() };
		list.iter().find(|m| m.name() == Some(name))
	}
}
