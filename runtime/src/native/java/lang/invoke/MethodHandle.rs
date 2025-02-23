use crate::globals::fields;
use crate::java_call;
use crate::objects::class::Class;
use crate::objects::method::Method;
use crate::objects::reference::{ClassInstanceRef, Reference};
use crate::thread::exceptions::Throws;

use common::traits::PtrType;
use jni::env::JniEnv;

include_generated!("native/java/lang/invoke/def/MethodHandle.definitions.rs");

pub fn get_target_method(handle: ClassInstanceRef) -> Throws<&'static Method> {
	let form = fields::java_lang_invoke_MethodHandle::form(handle.get());
	let vmentry = fields::java_lang_invoke_LambdaForm::vmentry(form.get());
	let vmindex = fields::java_lang_invoke_MemberName::vmindex(vmentry.get());

	let defining_class_mirror = fields::java_lang_invoke_MemberName::clazz(vmentry.get());
	let defining_class = defining_class_mirror.get().target_class();

	Throws::Ok(&defining_class.vtable()[vmindex as usize])
}

pub fn invokeExact(
	_env: JniEnv,
	_this: Reference, // java.lang.invoke.MethodHandle
	_args: Reference, // Object[]
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.invoke.MethodHandle#invokeExact")
}

pub fn invoke(
	_env: JniEnv,
	_this: Reference, // java.lang.invoke.MethodHandle
	_args: Reference, // Object[]
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.invoke.MethodHandle#invoke")
}

pub fn invokeBasic(
	_env: JniEnv,
	_this: Reference, // java.lang.invoke.MethodHandle
	_args: Reference, // Object[]
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.invoke.MethodHandle#invokeBasic")
}

pub fn linkToStatic(
	_env: JniEnv,
	_class: &'static Class,
	_args: Reference, // Object[]
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.invoke.MethodHandle#linkToStatic")
}

pub fn linkToSpecial(
	_env: JniEnv,
	_class: &'static Class,
	_args: Reference, // Object[]
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.invoke.MethodHandle#linkToSpecial")
}

pub fn linkToInterface(
	_env: JniEnv,
	_class: &'static Class,
	_args: Reference, // Object[]
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.invoke.MethodHandle#linkToInterface")
}

pub fn linkToNative(
	_env: JniEnv,
	_class: &'static Class,
	_args: Reference, // Object[]
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.invoke.MethodHandle#linkToNative")
}
