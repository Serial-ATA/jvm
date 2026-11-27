use ::jni::env::{JniEnv, JniNativeMethod};
use ::jni::objects::{JObject, JObjectArray};
use native_macros::jni_call;

#[jni_call]
extern "C" fn var_handle_stub(
	_env: JniEnv,
	_this: JObject,
	_args: JObjectArray, // Object...
) -> JObject {
	unreachable!(
		"java.lang.invoke.VarHandle method called through JNI. should have been intercepted"
	);
}

#[rustfmt::skip]
#[allow(trivial_casts)]
pub(super) fn methods() -> [JniNativeMethod; 31] {
    [
        JniNativeMethod::from("get",                       "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("set",                       "([Ljava/lang/Object;)V",                  var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getVolatile",               "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("setVolatile",               "([Ljava/lang/Object;)V",                  var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAcquire",                "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("setRelease",                "([Ljava/lang/Object;)V",                  var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getOpaque",                 "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("setOpaque",                 "([Ljava/lang/Object;)V",                  var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("compareAndSet",             "([Ljava/lang/Object;)Z",                  var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("compareAndExchange",        "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("compareAndExchangeAcquire", "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("compareAndExchangeRelease", "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("weakCompareAndSetPlain",    "([Ljava/lang/Object;)Z",                  var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("weakCompareAndSet",         "([Ljava/lang/Object;)Z",                  var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("weakCompareAndSetAcquire",  "([Ljava/lang/Object;)Z",                  var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("weakCompareAndSetRelease",  "([Ljava/lang/Object;)Z",                  var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndSet",                 "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndSetAcquire",          "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndSetRelease",          "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndAdd",                 "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndAddAcquire",          "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndAddRelease",          "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndBitwiseOr",           "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndBitwiseOrAcquire",    "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndBitwiseOrRelease",    "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndBitwiseAnd",          "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndBitwiseAndAcquire",   "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndBitwiseAndRelease",   "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndBitwiseXor",          "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndBitwiseXorAcquire",   "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("getAndBitwiseXorRelease",   "([Ljava/lang/Object;)Ljava/lang/Object;", var_handle_stub as extern "C" fn(_, _, _) -> JObject),
    ]
}
