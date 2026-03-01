#![native_macros::jni_fn_module]

use jni::sys::jboolean;
use native_macros::jni_call;

#[jni_call(no_env)]
pub extern "C" fn JVM_PrintWarningAtDynamicAgentLoad() -> jboolean {
	todo!()
}
