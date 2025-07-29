use super::ClassLoader;
use crate::classes;
use crate::objects::reference::Reference;

use std::cell::SyncUnsafeCell;
use std::collections::LinkedList;
use std::sync::{LazyLock, Mutex};

use jni::sys::jlong;

pub struct ClassLoaderSet {
	// A list of all currently alive classloaders.
	//
	// This is a `LinkedList`, as reads will do a lifetime-extension, and are not guarded. We cannot
	// risk a realloc invalidating a reference.
	list: SyncUnsafeCell<LinkedList<ClassLoader>>,
	write_mutex: Mutex<()>,
}

static CLASS_LOADER_SET: LazyLock<ClassLoaderSet> = LazyLock::new(|| ClassLoaderSet {
	list: SyncUnsafeCell::new(LinkedList::new()),
	write_mutex: Mutex::new(()),
});

impl ClassLoaderSet {
	pub fn add(loader: Reference, is_hidden: bool) -> &'static ClassLoader {
		#[cfg(test)]
		{
			ClassLoader::bootstrap().assert_not_sealed()
		}

		let _guard = CLASS_LOADER_SET.write_mutex.lock().unwrap();

		// TODO: These hidden classloaders will forever be in the set, need to sweep.
		if is_hidden {
			let class_loader = ClassLoader::new_hidden(loader);
			let list = unsafe { &mut *CLASS_LOADER_SET.list.get() };
			list.push_back(class_loader);
			return list.back().unwrap();
		}

		let class_loader = ClassLoader::from_obj(loader.clone());

		let list = unsafe { &mut *CLASS_LOADER_SET.list.get() };
		list.push_back(class_loader);

		let ret = list.back().unwrap();

		// Store the pointer in the classloader, to make future lookups cheaper
		classes::java::lang::ClassLoader::set_injected_loader_ptr_for(
			loader,
			ret as *const ClassLoader as jlong,
		);

		ret
	}

	pub fn find(loader: Reference, is_hidden: bool) -> Option<&'static ClassLoader> {
		if loader.is_null() {
			return Some(ClassLoader::bootstrap());
		}

		let list = unsafe { &*CLASS_LOADER_SET.list.get() };
		list.iter()
			.find(|cl| cl.obj == loader && cl.is_hidden() == is_hidden)
	}

	pub fn find_or_add(loader: Reference, is_hidden: bool) -> &'static ClassLoader {
		if loader.is_null() {
			return ClassLoader::bootstrap();
		}

		if let Some(class_loader_ptr) =
			classes::java::lang::ClassLoader::injected_loader_ptr_for(loader.clone())
		{
			return unsafe { &*class_loader_ptr };
		}

		match Self::find(loader.clone(), is_hidden) {
			Some(cl) => cl,
			None => Self::add(loader, is_hidden),
		}
	}
}
