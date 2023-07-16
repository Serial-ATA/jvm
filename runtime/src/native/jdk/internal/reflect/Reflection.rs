use crate::native::jni::jclass_from_classref;
use crate::native::JNIEnv;

use ::jni::sys::{jboolean, jclass, jint};
use common::traits::PtrType;

include_generated!("native/jdk/internal/reflect/def/Reflection.definitions.rs");

#[expect(clippy::match_same_arms)]
pub fn getCallerClass(env: JNIEnv) -> jclass {
	for (n, frame) in env
		.current_thread
		.get()
		.frame_stack
		.iter()
		.rev()
		.enumerate()
	{
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
				return jclass_from_classref(method.class.get_mirror());
			},
		}
	}

	return core::ptr::null() as jclass;
}

pub fn getClassAccessFlags(_env: JNIEnv, class: jclass) -> jint {
	unimplemented!("jdk.internal.reflect.Reflection#getClassAccessFlags")
}

pub fn areNestMates(_env: JNIEnv, current_class: jclass, member_class: jclass) -> jboolean {
	unimplemented!("jdk.internal.reflect.Reflection#areNestMates")
}
