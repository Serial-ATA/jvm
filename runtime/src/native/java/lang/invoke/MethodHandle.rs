use crate::objects::class::ClassPtr;
use crate::objects::reference::Reference;

use jni::env::JniEnv;

include_generated!("native/java/lang/invoke/def/MethodHandle.definitions.rs");

pub fn invokeExact(
	_env: JniEnv,
	_this: Reference,      // java.lang.invoke.MethodHandle
	_args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	unreachable!(
		"java.lang.invoke.MethodHandle#invokeExact called through native method. should have been \
		 intercepted"
	);
}

pub fn invoke(
	_env: JniEnv,
	_this: Reference,      // java.lang.invoke.MethodHandle
	_args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	unreachable!(
		"java.lang.invoke.MethodHandle#invoke called through native method. should have been \
		 intercepted"
	);
}

pub fn invokeBasic(
	_env: JniEnv,
	_this: Reference,      // java.lang.invoke.MethodHandle
	_args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	unreachable!(
		"java.lang.invoke.MethodHandle#invokeBasic called through native method. should have been \
		 intercepted"
	);
}

pub fn linkToVirtual(
	_env: JniEnv,
	_class: ClassPtr,
	_args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	unreachable!(
		"java.lang.invoke.MethodHandle#linkToVirtual called through native method. should have \
		 been intercepted"
	);
}

pub fn linkToStatic(
	_env: JniEnv,
	_class: ClassPtr,
	_args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	unreachable!(
		"java.lang.invoke.MethodHandle#linkToStatic called through native method. should have \
		 been intercepted"
	);
}

pub fn linkToSpecial(
	_env: JniEnv,
	_class: ClassPtr,
	_args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	unreachable!(
		"java.lang.invoke.MethodHandle#linkToSpecial called through native method. should have \
		 been intercepted"
	);
}

pub fn linkToInterface(
	_env: JniEnv,
	_class: ClassPtr,
	_args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	unreachable!(
		"java.lang.invoke.MethodHandle#linkToInterface called through native method. should have \
		 been intercepted"
	);
}

pub fn linkToNative(
	_env: JniEnv,
	_class: ClassPtr,
	_args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	unreachable!(
		"java.lang.invoke.MethodHandle#linkToNative called through native method. should have \
		 been intercepted"
	);
}
