use crate::native::Reference;
use crate::string_interner::StringInterner;

use std::ptr::NonNull;

use ::jni::env::JniEnv;
use ::jni::sys::{jint, jlong};

include_generated!("native/jdk/internal/misc/def/Signal.definitions.rs");

pub fn findSignal0(_: NonNull<JniEnv>, sig_name: Reference /* java.lang.String */) -> jint {
	let sig_name_string = sig_name.extract_class();
	let sig_name = StringInterner::rust_string_from_java_string(sig_name_string);

	match platform::Signal::from_name(sig_name) {
		Some(signal) => signal.value(),
		None => -1,
	}
}

pub fn handle0(_: NonNull<JniEnv>, sig: jint, native_h: jlong) -> jlong {
	let signal = platform::Signal::from(sig);

	if !signal.registration_allowed() {
		return -1;
	}

	let handler = match native_h {
		2 => platform::SignalHandler::user_handler(),
		_ => unsafe { platform::SignalHandler::from_raw(native_h as usize) },
	};

	let old = unsafe { signal.install(handler) };
	let Some(old) = old else {
		// Registration failed
		return -1;
	};

	if old == platform::SignalHandler::user_handler() {
		return 2;
	}

	old.as_usize() as jlong
}

pub fn raise0(_: NonNull<JniEnv>, _sig: jint) {
	unimplemented!("jdk.internal.misc.Signal#raise0");
}
