use crate::reference::Reference;
use crate::thread::JavaThread;

use std::ptr::NonNull;

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint};

include_generated!("native/jdk/internal/reflect/def/Reflection.definitions.rs");

#[expect(clippy::match_same_arms)]
pub fn getCallerClass(env: NonNull<JniEnv>) -> Reference {
	let current_thread = unsafe { &*JavaThread::for_env(env.as_ptr() as _) };
	for (n, frame) in current_thread.frames().rev().enumerate() {
		let method = frame.method();
		match n {
			// TODO:
			//   https://github.com/openjdk/jdk/blob/6a44120a16d0f06b4ed9f0ebf6b0919da7070287/src/hotspot/share/prims/jvm.cpp#L742-L744
			//   https://github.com/openjdk/jdk/blob/6a44120a16d0f06b4ed9f0ebf6b0919da7070287/src/java.base/share/classes/java/lang/invoke/MethodHandleNatives.java#L117
			0 => {},
			// TODO:
			//   https://github.com/openjdk/jdk/blob/6a44120a16d0f06b4ed9f0ebf6b0919da7070287/src/hotspot/share/prims/jvm.cpp#L748-L750
			//   https://github.com/openjdk/jdk/blob/6a44120a16d0f06b4ed9f0ebf6b0919da7070287/src/java.base/share/classes/java/lang/invoke/MethodHandleNatives.java#L117
			1 => {},
			_ => {
				// TODO: https://github.com/openjdk/jdk/blob/6a44120a16d0f06b4ed9f0ebf6b0919da7070287/src/hotspot/share/oops/method.cpp#L1378
				return Reference::mirror(method.class.mirror());
			},
		}
	}

	Reference::null()
}

pub fn getClassAccessFlags(_env: NonNull<JniEnv>, _class: Reference) -> jint {
	unimplemented!("jdk.internal.reflect.Reflection#getClassAccessFlags")
}

pub fn areNestMates(
	_env: NonNull<JniEnv>,
	_current_class: Reference,
	_member_class: Reference,
) -> jboolean {
	unimplemented!("jdk.internal.reflect.Reflection#areNestMates")
}
