use crate::modules::{Module, Package};
use crate::objects::class::Class;
use crate::objects::reference::Reference;
use crate::string_interner::StringInterner;
use crate::thread::exceptions::{handle_exception, throw};
use crate::thread::JavaThread;

use ::jni::env::JniEnv;
use common::traits::PtrType;
use symbols::Symbol;

include_generated!("native/java/lang/def/Module.definitions.rs");

pub fn defineModule0(
	env: JniEnv,
	_class: &'static Class,
	module: Reference, // java.lang.Module
	is_open: bool,
	version: Reference,  // java.lang.String
	location: Reference, // java.lang.String
	pns: Reference,      // java.lang.Object[]
) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let mut version_sym = None;
	if !version.is_null() {
		let version_str = StringInterner::rust_string_from_java_string(version.extract_class());
		version_sym = Some(Symbol::intern(version_str));
	}

	let mut location_sym = None;
	if !location.is_null() {
		let location_str = StringInterner::rust_string_from_java_string(location.extract_class());
		location_sym = Some(Symbol::intern(location_str));
	}

	let mut package_names = Vec::new();
	if !pns.is_null() {
		let package_names_obj = pns.extract_array();
		let package_names_ref = package_names_obj.get().get_content().expect_reference();

		for package_name in package_names_ref {
			if package_name.is_null() {
				throw!(thread, IllegalArgumentException, "Bad package name");
			}

			let package_name =
				StringInterner::rust_string_from_java_string(package_name.extract_class());
			package_names.push(Package::name_to_internal(package_name));
		}
	}

	handle_exception!(
		thread,
		Module::named(
			module.clone(),
			is_open,
			version_sym,
			location_sym,
			package_names,
		)
	);
}

pub fn addReads0(
	env: JniEnv,
	_class: &'static Class,
	from: Reference, // java.lang.Module
	to: Reference,   // java.lang.Module
) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	if from.is_null() {
		throw!(thread, NullPointerException, "from_module is null");
	}

	if from == to {
		// Nothing to do if the modules are the same
		return;
	}

	let Some(from_ptr) = crate::globals::fields::java_lang_Module::injected_module_ptr_for(from)
	else {
		throw!(thread, IllegalArgumentException, "from_module is not valid");
	};

	let from_module = unsafe { &*from_ptr };
	if from_module.name().is_none() {
		// Nothing to do if `from` is unnamed
		return;
	}

	let mut to_module = None;
	if !to.is_null() {
		let Some(to_ptr) = crate::globals::fields::java_lang_Module::injected_module_ptr_for(to)
		else {
			throw!(thread, IllegalArgumentException, "to_module is not valid");
		};

		to_module = Some(unsafe { &*to_ptr });
	}

	from_module.add_reads(to_module);
}

pub fn addExports0(
	env: JniEnv,
	_class: &'static Class,
	from: Reference, // java.lang.Module
	pn: Reference,   // java.lang.String
	to: Reference,   // java.lang.Module
) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	if from.is_null() {
		throw!(thread, NullPointerException, "from_module is null");
	}

	if to.is_null() {
		throw!(thread, NullPointerException, "to_module is null");
	}

	if pn.is_null() {
		throw!(thread, NullPointerException, "package is null");
	}

	let Some(from_ptr) = crate::globals::fields::java_lang_Module::injected_module_ptr_for(from)
	else {
		throw!(thread, IllegalArgumentException, "from_module is not valid");
	};

	let Some(to_ptr) = crate::globals::fields::java_lang_Module::injected_module_ptr_for(to) else {
		throw!(thread, IllegalArgumentException, "to_module is not valid");
	};

	let package_name = StringInterner::rust_string_from_java_string(pn.extract_class());
	let package_name = Package::name_to_internal(package_name);

	let from_module = unsafe { &*from_ptr };
	let to_module = unsafe { &*to_ptr };
	from_module.add_exports(Some(to_module), package_name);
}

pub fn addExportsToAll0(
	env: JniEnv,
	_class: &'static Class,
	from: Reference, // java.lang.Module
	pn: Reference,   // java.lang.String
) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	if from.is_null() {
		throw!(thread, NullPointerException, "from_module is null");
	}

	if pn.is_null() {
		throw!(thread, NullPointerException, "package is null");
	}

	let Some(from_ptr) = crate::globals::fields::java_lang_Module::injected_module_ptr_for(from)
	else {
		throw!(thread, IllegalArgumentException, "from_module is not valid");
	};

	let package_name = StringInterner::rust_string_from_java_string(pn.extract_class());
	let package_name = Package::name_to_internal(package_name);

	let from_module = unsafe { &*from_ptr };
	from_module.add_exports(None, package_name);
}

pub fn addExportsToAllUnnamed0(
	_: JniEnv,
	_class: &'static Class,
	_from: Reference, // java.lang.Module
	_pn: Reference,   // java.lang.String
) {
	unimplemented!("java.lang.Module#addExportsToAllUnnamed0");
}
