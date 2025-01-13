use crate::modules::{Module, Package};
use crate::objects::class::Class;
use crate::objects::reference::Reference;
use crate::string_interner::StringInterner;
use crate::thread::exceptions::{handle_exception, throw};
use crate::thread::JavaThread;

use std::ptr::NonNull;

use ::jni::env::JniEnv;
use common::traits::PtrType;

include_generated!("native/java/lang/def/Module.definitions.rs");

pub fn defineModule0(
	env: NonNull<JniEnv>,
	_class: &'static Class,
	module: Reference, // java.lang.Module
	is_open: bool,
	version: Reference,  // java.lang.String
	location: Reference, // java.lang.String
	pns: Reference,      // java.lang.Object[]
) {
	let mut version_sym = None;
	if !version.is_null() {
		version_sym = Some(StringInterner::symbol_from_java_string(
			version.extract_class(),
		));
	}

	let mut location_sym = None;
	if !location.is_null() {
		location_sym = Some(StringInterner::symbol_from_java_string(
			version.extract_class(),
		));
	}

	let mut package_names = Vec::new();
	if !pns.is_null() {
		let package_names_obj = pns.extract_array();
		let package_names_ref = package_names_obj.get().get_content().expect_reference();

		for package_name in package_names_ref {
			let package_name =
				StringInterner::rust_string_from_java_string(package_name.extract_class());
			package_names.push(Package::name_to_internal(package_name));
		}
	}

	let module_entry_result = Module::named(
		module.clone(),
		is_open,
		version_sym,
		location_sym,
		package_names,
	);

	let thread = unsafe { &*JavaThread::for_env(env.as_ptr()) };
	let Some(module) = handle_exception!(thread, module_entry_result) else {
		// `Module::named` returns `None` in the case of `java.base`, so we have nothing left
		// to do.
		return;
	};

	unimplemented!("java.lang.Module#defineModule0");
}

pub fn addReads0(
	_: NonNull<JniEnv>,
	_class: &'static Class,
	_from: Reference, // java.lang.Module
	_to: Reference,   // java.lang.Module
) {
	unimplemented!("java.lang.Module#addReads0");
}

pub fn addExports0(
	_: NonNull<JniEnv>,
	_class: &'static Class,
	_from: Reference, // java.lang.Module
	_pn: Reference,   // java.lang.String
	_to: Reference,   // java.lang.Module
) {
	unimplemented!("java.lang.Module#addExports0");
}

pub fn addExportsToAll0(
	_: NonNull<JniEnv>,
	_class: &'static Class,
	_from: Reference, // java.lang.Module
	_pn: Reference,   // java.lang.String
) {
	unimplemented!("java.lang.Module#addExportsToAll0");
}

pub fn addExportsToAllUnnamed0(
	_: NonNull<JniEnv>,
	_class: &'static Class,
	_from: Reference, // java.lang.Module
	_pn: Reference,   // java.lang.String
) {
	unimplemented!("java.lang.Module#addExportsToAllUnnamed0");
}
