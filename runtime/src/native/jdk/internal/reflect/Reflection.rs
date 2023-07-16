use crate::native::JNIEnv;
use crate::reference::Reference;

use ::jni::sys::{jboolean, jint};
use common::traits::PtrType;
use instructions::Operand;

include_generated!("native/jdk/internal/reflect/def/Reflection.definitions.rs");

#[expect(clippy::match_same_arms)]
pub fn getCallerClass(env: JNIEnv) -> Reference {
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
				return Reference::Mirror(method.class.get_mirror());
			},
		}
	}

	Reference::Null
}

pub fn getClassAccessFlags(_env: JNIEnv, class: Reference) -> jint {
	unimplemented!("jdk.internal.reflect.Reflection#getClassAccessFlags")
}

pub fn areNestMates(_env: JNIEnv, current_class: Reference, member_class: Reference) -> jboolean {
	unimplemented!("jdk.internal.reflect.Reflection#areNestMates")
}
