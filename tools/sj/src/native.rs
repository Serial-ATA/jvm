use crate::error::Result;

use std::ops::Deref;

use jni::env::JniEnv;
use jni::error::JniError;
use jni::java_vm::{JavaVm, JavaVmBuilder, VmInitArgs};
use jni::objects::{JClass, JObjectArray};
use jni::sys::jsize;
use jni::version::JniVersion;

const MAIN_METHOD_SIGNATURE: &str = "([Ljava/lang/String;)V";

pub fn init_java_vm(
	system_properties: impl IntoIterator<Item = String>,
) -> Result<(JavaVm, JniEnv)> {
	let init_args = VmInitArgs::new(JniVersion::LATEST).options(system_properties);
	Ok(JavaVmBuilder::new().args(init_args).build()?)
}

/// Wrapper to force a type to implement `Send`
///
/// This is necessary for our JNI types, which are just pointer wrappers.
struct IsSend<T>(T);

unsafe impl<T> Send for IsSend<T> {}

impl<T> Deref for IsSend<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

/// Invoke the main method in a new thread
///
/// This is the last stop in the launcher, all remaining action occurs in the VM runtime.
pub fn invoke_main_method(env: JniEnv, main_class: JClass, args: Vec<String>) -> Result<()> {
	let env = IsSend(env);
	let main_class = IsSend(main_class);

	let method_id = env.get_static_method_id(*main_class, "main", MAIN_METHOD_SIGNATURE)?;
	let args = args_as_jstring_array(*env, args)?;

	env.call_static_void_method(*main_class, method_id, [args])?;
	if env.exception_check() {
		return Err(JniError::ExceptionThrown.into());
	}

	Ok(())
}

fn args_as_jstring_array(env: JniEnv, args: Vec<String>) -> Result<JObjectArray> {
	let string_class = env.find_class("java/lang/String")?;
	let array = env.new_object_array(args.len() as jsize, string_class, None)?;
	for (i, arg) in args.iter().enumerate() {
		let string_obj = env.new_string_utf(arg)?;
		env.set_object_array_element(array, i as jsize, Some(string_obj))?;
		// TODO: DeleteLocalRef
	}

	Ok(array)
}
