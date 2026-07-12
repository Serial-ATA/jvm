#![native_macros::jni_fn_module]

use crate::classes;
use crate::classpath::loader::{ClassLoader, ClassLoaderSet};
use crate::native::RawSymbolExt;
use crate::native::jni::{IntoJni, reference_from_jobject_maybe_null};
use crate::objects::reference::Reference;
use crate::symbols::Symbol;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, handle_exception, throw_with_ret};

use std::ffi::{CStr, c_char, c_int};

use common::int_types::s4;
use common::unicode;
use jni::env::JniEnv;
use jni::objects::{JByteArray, JClass, JObject, JObjectArray, JString};
use jni::sys::{jboolean, jbyte, jint, jsize};
use native_macros::jni_call;

const MN_NESTMATE_CLASS: s4 = 0x0000_0001;
const MN_HIDDEN_CLASS: s4 = 0x0000_0002;
const MN_STRONG_LOADER_LINK: s4 = 0x0000_0004;
const MN_ACCESS_VM_ANNOTATIONS: s4 = 0x0000_0008;

#[jni_call]
pub extern "C" fn JVM_GetCallerClass(_env: JniEnv) -> JClass {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_FindPrimitiveClass(_env: JniEnv, _utf: *const c_char) -> JClass {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_FindClassFromBootLoader(_env: JniEnv, name: *const c_char) -> JClass {
	let name_c = unsafe { CStr::from_ptr(name) };
	let name = unicode::decode(name_c.to_bytes()).unwrap();

	match ClassLoader::bootstrap().lookup_class(Symbol::intern(name)) {
		Some(class) => unsafe { JClass::from_raw(class.into_jni()) },
		None => JClass::null(),
	}
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_FindClassFromLoader(
	_env: JniEnv,
	_name: *const c_char,
	_init: jboolean,
	_loader: JObject,
) -> JClass {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_FindClassFromClass(
	_env: JniEnv,
	_name: *const c_char,
	_init: jboolean,
	_from: JClass,
) -> JClass {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_DefineClass(
	_env: JniEnv,
	_name: *const c_char,
	_loader: JObject,
	_buf: *const jbyte,
	_len: jsize,
	_protection_domain: JObject,
) -> JClass {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_LookupDefineClass(
	env: JniEnv,
	lookup: JClass,
	name: *const c_char,
	buf: *const jbyte,
	len: jsize,
	_protection_domain: JObject,
	initialize: jboolean,
	flags: c_int,
	class_data: JObject,
) -> JClass {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	let lookup = unsafe { reference_from_jobject_maybe_null(lookup.raw()) };
	if lookup.is_null() {
		throw_with_ret!(
			JClass::null(),
			thread,
			IllegalArgumentException,
			"Lookup class is null"
		);
	}

	let lookup = lookup.extract_target_class();

	let buf = unsafe {
		// SAFETY: `i8` and `u8` have the same size and alignment
		let ptr = buf.cast::<u8>();
		std::slice::from_raw_parts(ptr, len as usize)
	};

	let is_nestmate = (flags & MN_NESTMATE_CLASS) == MN_NESTMATE_CLASS;
	let is_hidden = (flags & MN_HIDDEN_CLASS) == MN_HIDDEN_CLASS;
	let is_strong = (flags & MN_STRONG_LOADER_LINK) == MN_STRONG_LOADER_LINK;
	let vm_annotations = (flags & MN_ACCESS_VM_ANNOTATIONS) == MN_ACCESS_VM_ANNOTATIONS;

	let nest_host_class;
	if is_nestmate {
		let host = handle_exception!(JClass::null(), thread, lookup.nest_host(thread));
		nest_host_class = Some(host);
	} else {
		nest_host_class = None;
	}

	if !is_hidden {
		if !class_data.is_null() {
			throw_with_ret!(
				JClass::null(),
				thread,
				IllegalArgumentException,
				"classData is only applicable for hidden classes"
			);
		}

		if is_nestmate {
			throw_with_ret!(
				JClass::null(),
				thread,
				IllegalArgumentException,
				"dynamic nestmate is only applicable for hidden classes"
			);
		}

		if !is_strong {
			throw_with_ret!(
				JClass::null(),
				thread,
				IllegalArgumentException,
				"an ordinary class must be strongly referenced by its defining loader"
			);
		}

		if vm_annotations {
			throw_with_ret!(
				JClass::null(),
				thread,
				IllegalArgumentException,
				"vm annotations only allowed for hidden classes"
			);
		}

		if flags != MN_STRONG_LOADER_LINK {
			throw_with_ret!(
				JClass::null(),
				thread,
				IllegalArgumentException,
				"invalid flag 0x{flags}"
			);
		}
	}

	if name.is_null() {
		// TODO: In this case, the name is just to be derived from the class file itself.
		//       Will need to train the loader to be able to do that.
		todo!()
	}

	let name = unsafe { Symbol::intern_mutf_c_str(name) };

	let class = match lookup.loader().derive_class(name, Some(buf), is_hidden) {
		Throws::Ok(class) => class,
		Throws::Exception(e) => {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			e.throw(thread);
			return JClass::null();
		},
	};

	if let Some(host) = nest_host_class {
		unsafe {
			class.set_nest_host(host);
		}
	}

	if is_hidden {
		let class_data = unsafe { reference_from_jobject_maybe_null(class_data.raw()) };
		class.mirror().set_class_data(class_data);
	}

	if initialize && let Throws::Exception(e) = class.initialize(thread) {
		e.throw(thread);
		return JClass::null();
	}

	// TODO: Parallel class loaders
	unsafe { JClass::from_raw(Reference::mirror(class.mirror()).into_jni()) }
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_DefineClassWithSource(
	env: JniEnv,
	name: *const c_char,
	loader: JObject,
	buf: *const jbyte,
	len: jsize,
	_protection_domain: JObject,
	source: *const c_char,
) -> JClass {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	let buf = unsafe {
		// SAFETY: `i8` and `u8` have the same size and alignment
		let ptr = buf.cast::<u8>();
		std::slice::from_raw_parts(ptr, len as usize)
	};

	let name_sym;
	if name.is_null() {
		// TODO: In this case, the name is just to be derived from the class file itself.
		//       Will need to train the loader to be able to do that.
		todo!()
	} else {
		let external_name_c = unsafe { CStr::from_ptr(name) };
		let external_name = unicode::decode(external_name_c.to_bytes()).unwrap();
		let internal_name = external_name.replace('.', "/");
		name_sym = Symbol::intern(internal_name);
	}

	let loader = unsafe { reference_from_jobject_maybe_null(loader.raw()) };

	let source_str;
	if source.is_null() {
		source_str = None;
	} else {
		let source_c = unsafe { CStr::from_ptr(source) };
		source_str = unicode::decode(source_c.to_bytes()).ok(); // TODO: maybe panic?
	}

	// Not a thing in defineClass1
	let is_hidden = false;

	let loader = ClassLoaderSet::find_or_add(loader, is_hidden);
	let class = match loader.derive_class(name_sym, Some(buf), is_hidden) {
		Throws::Ok(class) => class,
		Throws::Exception(e) => {
			e.throw(thread);
			return JClass::null();
		},
	};

	unsafe { JClass::from_raw(Reference::mirror(class.mirror()).into_jni()) }
}

#[jni_call]
pub extern "C" fn JVM_FindLoadedClass(_env: JniEnv, loader: JObject, name: JString) -> JClass {
	let name = unsafe { reference_from_jobject_maybe_null(name.raw()) };
	if name.is_null() {
		return JClass::null();
	}

	let name_str = classes::java::lang::String::extract(name.extract_class());
	let internal_name = name_str.replace('.', "/");

	let internal_name_sym = Symbol::intern(internal_name);

	let loader_obj = unsafe { reference_from_jobject_maybe_null(loader.raw()) };
	let Some(loader) = ClassLoaderSet::find(loader_obj, false) else {
		// Unknown loader
		return JClass::null();
	};

	match loader.lookup_class(internal_name_sym) {
		None => JClass::null(),
		Some(class) => unsafe { JClass::from_raw(Reference::mirror(class.mirror()).into_jni()) },
	}
}

#[jni_call]
pub extern "C" fn JVM_InitClassName(_env: JniEnv, _class: JClass) -> JString {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassInterfaces(_env: JniEnv, _class: JClass) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_IsInterface(_env: JniEnv, _class: JClass) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_IsHiddenClass(_env: JniEnv, _class: JClass) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_FindScopedValueBindings(_env: JniEnv, _class: JClass) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetDeclaredClasses(_env: JniEnv, _class: JClass) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetDeclaringClass(_env: JniEnv, _class: JClass) -> JClass {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetSimpleBinaryName(_env: JniEnv, _class: JClass) -> JString {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassSignature(_env: JniEnv, _class: JClass) -> JString {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassAnnotations(_env: JniEnv, _class: JClass) -> JByteArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassTypeAnnotations(_env: JniEnv, _class: JClass) -> JByteArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetMethodTypeAnnotations(_env: JniEnv, _method: JObject) -> JByteArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetFieldTypeAnnotations(_env: JniEnv, _field: JObject) -> JByteArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetMethodParameters(_env: JniEnv, _method: JObject) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassDeclaredFields(
	_env: JniEnv,
	_class: JClass,
	_public_only: jboolean,
) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_IsRecord(_env: JniEnv, _class: JClass) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetRecordComponents(_env: JniEnv, _class: JClass) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassDeclaredMethods(
	_env: JniEnv,
	_class: JClass,
	_public_only: jboolean,
) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassDeclaredConstructors(
	_env: JniEnv,
	_class: JClass,
	_public_only: jboolean,
) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_AreNestMates(_env: JniEnv, _current: JClass, _member: JClass) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetNestHost(_env: JniEnv, _current: JClass) -> JClass {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetNestMembers(_env: JniEnv, _current: JClass) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetPermittedSubclasses(_env: JniEnv, _current: JClass) -> JObjectArray {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetClassNameUTF(_env: JniEnv, _cb: JClass) -> *const c_char {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassFieldsCount(_env: JniEnv, _cb: JClass) -> jint {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassMethodsCount(_env: JniEnv, _cb: JClass) -> jint {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassFileVersion(_env: JniEnv, _current: JClass) -> jint {
	todo!()
}
