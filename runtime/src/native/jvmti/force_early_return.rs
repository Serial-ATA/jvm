use jni::objects::JObject;
use jni::sys::{jdouble, jfloat, jint, jlong};
use jvmti::env::JvmtiEnv;
use jvmti::error::JvmtiError;
use jvmti::objects::JThread;
use native_macros::jvmti_call;

macro_rules! force_early_return {
	($($fun:ident => $ty:ty),+ $(,)?) => {
		$(
        paste::paste! {
			#[jvmti_call]
			pub extern "system" fn $fun(
				_env: JvmtiEnv,
				_thread: JThread,
				_value: $ty,
			) -> JvmtiError {
				unimplemented!("jvmtiEnv::{}", stringify!($ty))
			}
		}
        )+
	};
}

force_early_return! {
	ForceEarlyReturnObject => JObject,
	ForceEarlyReturnInt => jint,
	ForceEarlyReturnLong => jlong,
	ForceEarlyReturnFloat => jfloat,
	ForceEarlyReturnDouble => jdouble,
}

#[jvmti_call]
pub extern "system" fn ForceEarlyReturnVoid(_env: JvmtiEnv, _thread: JThread) -> JvmtiError {
	unimplemented!("jvmtiEnv::ForceEarlyReturnVoid")
}
