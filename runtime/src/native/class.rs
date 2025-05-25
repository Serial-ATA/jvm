use crate::native::jni::reference_from_jobject;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;

use jni::env::JniEnv;
use jni::objects::JValue;

/// Construct a new instance of `class` with the given constructor signature
///
/// This will return `None` if:
///
/// * The class cannot be found
/// * The constructor cannot be found
/// * Some exception occurs in [`JniEnv::new_object()`]
///
/// In any case, the thread will have a pending exception. It **MUST** be handled.
pub fn construct_class(
	thread: &'static JavaThread,
	class: impl Into<String>,
	signature: impl Into<String>,
	args: impl IntoIterator<Item = impl Into<JValue>>,
) -> Option<Reference> {
	let env = thread.env();

	let Ok(class) = env.find_class(class.into()) else {
		// Any error case will have already set a pending exception
		return None;
	};

	let Ok(constructor) = env.get_method_id(class, "<init>", signature.into()) else {
		return None;
	};

	let Ok(obj) = env.new_object(class, constructor, args) else {
		return None;
	};

	unsafe { reference_from_jobject(obj.raw()) }
}
