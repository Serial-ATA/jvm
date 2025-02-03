use crate::objects::class::Class;
use crate::objects::reference::Reference;
use crate::thread::exceptions::throw_and_return_null;
use crate::thread::JavaThread;

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint};

include_generated!("native/jdk/internal/reflect/def/Reflection.definitions.rs");

#[expect(clippy::match_same_arms)]
pub fn getCallerClass(env: JniEnv, _class: &'static Class) -> Reference {
	let current_thread = unsafe { &*JavaThread::for_env(env.raw() as _) };

	// The call stack at this point looks something like this:
	//
	// [0] [ @CallerSensitive public jdk.internal.reflect.Reflection.getCallerClass ]
	// [1] [ @CallerSensitive API.method                                   ]
	// [.] [ (skipped intermediate frames)                                 ]
	// [n] [ caller                                                        ]
	for (n, frame) in current_thread.frame_stack().iter().enumerate() {
		let method = frame.method();

		// TODO:
		//   https://github.com/openjdk/jdk/blob/6a44120a16d0f06b4ed9f0ebf6b0919da7070287/src/hotspot/share/prims/jvm.cpp#L742-L744
		//   https://github.com/openjdk/jdk/blob/6a44120a16d0f06b4ed9f0ebf6b0919da7070287/src/java.base/share/classes/java/lang/invoke/MethodHandleNatives.java#L117
		if n == 0 || n == 1 {
			if n == 0 {
				// TODO
				tracing::warn!(
					"(!!!) UNIMPLEMENTED `getCallerClass` not verifying call from Reflection"
				);
			}

			if !method.is_caller_sensitive() {
				throw_and_return_null!(
					current_thread,
					InternalError,
					"`getCallerClass` is not called from a @CallerSensitive method"
				);
			}

			continue;
		}

		if !method.is_stack_walk_ignored() {
			return Reference::mirror(method.class().mirror());
		}
	}

	Reference::null()
}

pub fn getClassAccessFlags(_env: JniEnv, _this_class: &'static Class, _class: Reference) -> jint {
	unimplemented!("jdk.internal.reflect.Reflection#getClassAccessFlags")
}

pub fn areNestMates(
	_env: JniEnv,
	_class: &'static Class,
	_current_class: Reference,
	_member_class: Reference,
) -> jboolean {
	unimplemented!("jdk.internal.reflect.Reflection#areNestMates")
}
