use super::ClassLoader;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;

use std::cell::SyncUnsafeCell;
use std::collections::LinkedList;
use std::sync::{LazyLock, Mutex};

use crate::classes;
use common::traits::PtrType;
use instructions::Operand;
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
	pub fn add(loader: Reference) -> &'static ClassLoader {
		let _guard = CLASS_LOADER_SET.write_mutex.lock().unwrap();

		let class_loader = ClassLoader::from_obj(loader.clone());

		let list = unsafe { &mut *CLASS_LOADER_SET.list.get() };
		list.push_back(class_loader);

		let ret = list.back().unwrap();

		// Store the pointer in the classloader, to make future lookups cheaper
		loader.extract_class().get_mut().put_field_value0(
			classes::java_lang_ClassLoader::loader_ptr_field_offset(),
			Operand::Long(ret as *const ClassLoader as jlong),
		);

		ret
	}

	pub fn find(loader: Reference) -> Option<&'static ClassLoader> {
		if loader.is_null() {
			return Some(ClassLoader::bootstrap());
		}

		let list = unsafe { &*CLASS_LOADER_SET.list.get() };
		list.iter().find(|cl| cl.obj == loader)
	}

	pub fn find_or_add(loader: Reference) -> &'static ClassLoader {
		if loader.is_null() {
			return ClassLoader::bootstrap();
		}

		if let Some(class_loader_ptr) =
			classes::java_lang_ClassLoader::injected_loader_ptr_for(loader.clone())
		{
			return unsafe { &*class_loader_ptr };
		}

		match Self::find(loader.clone()) {
			Some(cl) => cl,
			None => Self::add(loader),
		}
	}
}
