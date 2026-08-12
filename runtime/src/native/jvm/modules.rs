#![native_macros::jni_fn_module]

use crate::classes;
use crate::classpath::loader::ClassLoader;
use crate::modules::{Module, Package};
use crate::native::jni::{
	JniObjectArrayExt, JniStringExt, reference_from_jobject, reference_from_jobject_maybe_null,
};
use crate::symbols::Symbol;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, handle_exception, throw};

use jni::env::JniEnv;
use jni::objects::{JClass, JObject, JObjectArray, JString};
use jni::sys::jboolean;
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_DefineModule(
	env: JniEnv,
	module: JObject,
	is_open: jboolean,
	version: JString,
	location: JString,
	packages: JObjectArray,
) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let mut version_sym = None;
	if !version.is_null() {
		let version_str = unsafe { version.extract() };
		version_sym = Some(Symbol::intern(version_str));
	}

	let mut location_sym = None;
	if !location.is_null() {
		let location_str = unsafe { location.extract() };
		location_sym = Some(Symbol::intern(location_str));
	}

	let mut package_names = Vec::new();
	if !packages.is_null() {
		let package_names_obj = unsafe { packages.extract_object_array() };
		let package_names_ref = package_names_obj.as_slice();

		for package_name in package_names_ref {
			if package_name.is_null() {
				throw!(thread, IllegalArgumentException, "Bad package name");
			}

			let package_name = classes::java::lang::String::extract(package_name.extract_class());
			package_names.push(Package::name_to_internal(&package_name));
		}
	}

	let module = unsafe { reference_from_jobject_maybe_null(module.raw()) };
	handle_exception!(
		thread,
		Module::named(module, is_open, version_sym, location_sym, package_names,)
	);
}

#[jni_call]
pub extern "C" fn JVM_SetBootLoaderUnnamedModule(env: JniEnv, module: JObject) {
	let Some(module) = (unsafe { reference_from_jobject(module.raw()) }) else {
		panic!("Attempting to SetBootLoaderUnnamedModule with a null reference");
	};

	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let module_entry_result = Module::unnamed(module);
	let module_entry = handle_exception!(thread, module_entry_result);

	let loader = classes::java::lang::Module::loader(module);
	if !loader.is_null() {
		throw!(
			thread,
			IllegalArgumentException,
			"Class loader must be the boot class loader"
		);
	}

	ClassLoader::set_bootloader_unnamed_module(module_entry);
}

#[jni_call]
pub extern "C" fn JVM_AddModuleExports(
	env: JniEnv,
	from_module: JObject,
	package: JString,
	to_module: JObject,
) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let Some(from_module) = (unsafe { reference_from_jobject(from_module.raw()) }) else {
		throw!(thread, NullPointerException, "from_module is null");
	};

	let Some(to_module) = (unsafe { reference_from_jobject(to_module.raw()) }) else {
		throw!(thread, NullPointerException, "to_module is null");
	};

	if package.is_null() {
		throw!(thread, NullPointerException, "package is null");
	}

	let Some(from_ptr) = classes::java::lang::Module::injected_module_ptr_for(from_module) else {
		throw!(thread, IllegalArgumentException, "from_module is not valid");
	};

	let Some(to_ptr) = classes::java::lang::Module::injected_module_ptr_for(to_module) else {
		throw!(thread, IllegalArgumentException, "to_module is not valid");
	};

	let package_name = unsafe { package.extract() };
	let package_name = Package::name_to_internal(&package_name);

	let from_module = unsafe { &*from_ptr };
	let to_module = unsafe { &*to_ptr };
	from_module.add_exports(Some(to_module), package_name);
}

#[jni_call]
pub extern "C" fn JVM_AddModuleExportsToAllUnnamed(
	_env: JniEnv,
	_from_module: JObject,
	_package: JString,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_AddModuleExportsToAll(env: JniEnv, from_module: JObject, package: JString) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let Some(from_module) = (unsafe { reference_from_jobject(from_module.raw()) }) else {
		throw!(thread, NullPointerException, "from_module is null");
	};

	if package.is_null() {
		throw!(thread, NullPointerException, "package is null");
	}

	let Some(from_ptr) = classes::java::lang::Module::injected_module_ptr_for(from_module) else {
		throw!(thread, IllegalArgumentException, "from_module is not valid");
	};

	let package_name = unsafe { package.extract() };
	let package_name = Package::name_to_internal(&package_name);

	let from_module = unsafe { &*from_ptr };
	from_module.add_exports(None, package_name);
}

#[jni_call]
pub extern "C" fn JVM_AddReadsModule(env: JniEnv, from_module: JObject, source_module: JObject) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let Some(from_module) = (unsafe { reference_from_jobject(from_module.raw()) }) else {
		throw!(thread, NullPointerException, "from_module is null");
	};

	let Some(from_ptr) = classes::java::lang::Module::injected_module_ptr_for(from_module) else {
		throw!(thread, IllegalArgumentException, "from_module is not valid");
	};

	let from_module = unsafe { &*from_ptr };
	if from_module.name().is_none() {
		// Nothing to do if `from` is unnamed
		return;
	}

	let mut source_module_instance = None;
	if let Some(source_module) = unsafe { reference_from_jobject(source_module.raw()) } {
		let Some(to_ptr) = classes::java::lang::Module::injected_module_ptr_for(source_module)
		else {
			throw!(
				thread,
				IllegalArgumentException,
				"source_module is not valid"
			);
		};

		source_module_instance = Some(unsafe { &*to_ptr });
	}

	if let Throws::Exception(e) = from_module.add_reads(source_module_instance) {
		e.throw(thread);
	}
}

#[jni_call]
pub extern "C" fn JVM_DefineArchivedModules(
	_env: JniEnv,
	_platform_loader: JObject,
	_system_loader: JObject,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetSystemPackage(_env: JniEnv, _name: JString) -> JString {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetSystemPackages(_env: JniEnv) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_IsSameClassPackage(
	_env: JniEnv,
	_class1: JClass,
	_class2: JClass,
) -> jboolean {
	todo!()
}
