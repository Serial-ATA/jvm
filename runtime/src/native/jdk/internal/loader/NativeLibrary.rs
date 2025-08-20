use crate::classes;
use crate::objects::class::ClassPtr;
use crate::objects::reference::Reference;

use std::ffi::{CString, c_void};
use std::str::FromStr;

use jni::env::JniEnv;
use jni::sys::jlong;

include_generated!("native/jdk/internal/loader/def/NativeLibrary.definitions.rs");

pub fn findEntry0(
	_env: JniEnv,
	_class: ClassPtr,
	handle: jlong,
	name: Reference, // java.lang.String
) -> jlong {
	let name = classes::java::lang::String::extract(name.extract_class());
	let Ok(name_c) = CString::from_str(&name) else {
		return 0;
	};

	let lib = unsafe { platform::libs::Library::from_raw(handle as *mut c_void) };
	let Ok(sym) = (unsafe { lib.symbol::<c_void>(&name_c) }) else {
		return 0;
	};

	sym.raw() as jlong
}
