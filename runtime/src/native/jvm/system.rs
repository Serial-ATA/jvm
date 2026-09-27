#![native_macros::jni_fn_module]

use crate::native::java::lang::String::StringInterner;
use crate::native::jni::references::JObjectExt;
use crate::objects::instance::array::{Array, ObjectArrayInstance};
use crate::objects::instance::object::Object;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw};

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use ::jni::env::JniEnv;
use ::jni::objects::{JClass, JObject, JObjectArray, JString};
use ::jni::sys::{jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_CurrentTimeMillis(_env: JniEnv, _unused: JClass) -> jlong {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NanoTime(_env: JniEnv, _unused: JClass) -> jlong {
	let time_nanos = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("current system time should not be before the UNIX epoch")
		.as_nanos();

	time_nanos as jlong
}

#[jni_call]
pub extern "C" fn JVM_GetNanoTimeAdjustment(
	_env: JniEnv,
	_unused: JClass,
	_offset_secs: jlong,
) -> jlong {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ArrayCopy(
	env: JniEnv,
	_unused: JClass,
	src: JObject,
	src_pos: jint,
	dst: JObject,
	dst_pos: jint,
	length: jint,
) {
	unsafe fn do_copy<T: Array>(src: T, src_pos: usize, dest: T, dest_pos: usize, length: usize) {
		unsafe {
			src.copy_into(src_pos, &dest, dest_pos, length);
		}
	}

	unsafe fn do_copy_within<T: Array>(src: T, src_pos: usize, dest_pos: usize, length: usize) {
		unsafe {
			src.copy_within(src_pos, dest_pos, length);
		}
	}

	let (Some(src), Some(dst)) = (unsafe { src.to_reference() }, unsafe { dst.to_reference() })
	else {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw!(thread, NullPointerException);
	};

	let src_len = match src.array_length() {
		Throws::Ok(len) => len,
		Throws::Exception(e) => {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			e.throw(thread);
			return;
		},
	};
	let dest_len = match dst.array_length() {
		Throws::Ok(len) => len,
		Throws::Exception(e) => {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			e.throw(thread);
			return;
		},
	};

	// TODO: Verify component types

	if src_pos < 0
		|| dst_pos < 0
		|| length < 0
		|| src_pos + length > src_len as jint
		|| dst_pos + length > dest_len as jint
	{
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw!(thread, IndexOutOfBoundsException);
	}

	if length == 0 {
		return;
	}

	if src == dst {
		if src.is_object_array() {
			unsafe {
				do_copy_within(
					src.extract_object_array(),
					src_pos as usize,
					dst_pos as usize,
					length as usize,
				)
			}
		} else {
			unsafe {
				do_copy_within(
					src.extract_primitive_array(),
					src_pos as usize,
					dst_pos as usize,
					length as usize,
				)
			}
		}

		return;
	}

	if src.is_object_array() {
		unsafe {
			do_copy(
				src.extract_object_array(),
				src_pos as usize,
				dst.extract_object_array(),
				dst_pos as usize,
				length as usize,
			)
		}
	} else {
		unsafe {
			do_copy(
				src.extract_primitive_array(),
				src_pos as usize,
				dst.extract_primitive_array(),
				dst_pos as usize,
				length as usize,
			)
		}
	}
}

const JAVA_VERSION: &str = env!("JAVA_VERSION");
const VM_SPECIFICATION_NAME: &str = env!("SYSTEM_PROPS_VM_SPECIFICATION_NAME");
const VM_NAME: &str = env!("SYSTEM_PROPS_VM_NAME");
const VM_VERSION: &str = env!("CARGO_PKG_VERSION");
const VM_VENDOR: &str = env!("SYSTEM_PROPS_VM_VENDOR");

/// All of the VM and CLI properties
///
/// See also: [`JvmOptions::load()`]
///
/// [`JvmOptions::load()`]: crate::options::JvmOptions::load
pub static SYSTEM_PROPERTIES: LazyLock<Mutex<HashMap<String, String>>> = LazyLock::new(|| {
	let mut m = HashMap::new();

	m.insert(String::from("java.version"), String::from(JAVA_VERSION));
	m.insert(
		String::from("java.vm.specification.name"),
		String::from(VM_SPECIFICATION_NAME),
	);
	m.insert(String::from("java.vm.name"), String::from(VM_NAME));
	m.insert(String::from("java.vm.version"), String::from(VM_VERSION));
	m.insert(String::from("java.vm.vendor"), String::from(VM_VENDOR));
	m.insert(
		String::from("java.library.path"),
		platform::env::java_library_path(),
	);
	if let Some(system_paths) = platform::env::SystemPaths::init() {
		m.insert(
			String::from("sun.boot.library.path"),
			system_paths
				.boot_library_path
				.to_string_lossy()
				.into_owned(),
		);
		m.insert(
			String::from("java.home"),
			system_paths.java_home.to_string_lossy().into_owned(),
		);
		m.insert(
			String::from("java.ext.dirs"),
			system_paths.extensions_dirs.clone(),
		);
	}

	Mutex::new(m)
});

#[jni_call]
pub extern "C" fn JVM_GetProperties(env: JniEnv) -> JObjectArray {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	let props = SYSTEM_PROPERTIES.lock().unwrap();
	let len = (props.len() * 2)
		.try_into()
		.expect("length should be verified beforehand");

	let string_array_class = crate::globals::classes::string_array();
	let prop_array;
	match ObjectArrayInstance::new(len, string_array_class) {
		Throws::Ok(array) => prop_array = array,
		Throws::Exception(e) => {
			e.throw(thread);

			// Doesn't matter what we return, this value will never be used.
			return JObjectArray::null();
		},
	}

	let mut index = 0;
	for (key, val) in props.iter() {
		let interned_key_string = StringInterner::intern(&**key);
		let interned_value_string = StringInterner::intern(&**val);
		if let Throws::Exception(e) = prop_array.store(index, Reference::class(interned_key_string))
		{
			e.throw(thread);
			return JObjectArray::null();
		}

		index += 1;
		if let Throws::Exception(e) =
			prop_array.store(index, Reference::class(interned_value_string))
		{
			e.throw(thread);
			return JObjectArray::null();
		}

		index += 1;
	}

	// TODO: Nicer way to convert `Reference` -> `JObjectArray`
	let raw = thread
		.jni_refs()
		.allocate(Reference::object_array(prop_array));
	unsafe { JObjectArray::from_raw(raw) }
}

#[jni_call]
pub extern "C" fn JVM_GetTemporaryDirectory(_env: JniEnv) -> JString {
	todo!()
}
