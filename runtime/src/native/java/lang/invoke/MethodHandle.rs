use crate::objects::boxing::Boxable;
use crate::objects::class::Class;
use crate::objects::method::Method;
use crate::objects::reference::{ClassInstanceRef, Reference};
use crate::stack::local_stack::LocalStack;
use crate::thread::exceptions::{handle_exception, Throws};
use crate::thread::JavaThread;
use crate::{classes, java_call};

use common::traits::PtrType;
use instructions::Operand;
use jni::env::JniEnv;

include_generated!("native/java/lang/invoke/def/MethodHandle.definitions.rs");

pub fn get_target_method(handle: ClassInstanceRef) -> Throws<&'static Method> {
	let form = classes::java_lang_invoke_MethodHandle::form(handle.get());
	let vmentry = classes::java_lang_invoke_LambdaForm::vmentry(form.get());
	let vmindex = classes::java_lang_invoke_MemberName::vmindex(vmentry.get());

	let defining_class_mirror = classes::java_lang_invoke_MemberName::clazz(vmentry.get())?;
	let defining_class = defining_class_mirror.get().target_class();

	Throws::Ok(&defining_class.vtable()[vmindex as usize])
}

pub fn invokeExact(
	_env: JniEnv,
	_this: Reference,      // java.lang.invoke.MethodHandle
	_args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.invoke.MethodHandle#invokeExact")
}

pub fn invoke(
	_env: JniEnv,
	_this: Reference,      // java.lang.invoke.MethodHandle
	_args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.invoke.MethodHandle#invoke")
}

pub fn invokeBasic(
	env: JniEnv,
	this: Reference,      // java.lang.invoke.MethodHandle
	args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let method = get_target_method(this.extract_class()).unwrap();

	// TODO: This is terrible. Recreating the LocalStack we just deconstructed.
	let mut args = args.into_iter().map(Operand::Reference).collect::<Vec<_>>();
	args.insert(0, Operand::Reference(this));

	// SAFETY: Every operand is a reference
	let call_args = unsafe { LocalStack::new_with_args(args, method.code.max_locals as usize) };
	let ret = java_call!(@WITH_ARGS_LIST thread, method, call_args);

	match ret {
		Some(ret) => {
			handle_exception!(Reference::null(), thread, ret.into_box(thread))
		},
		None => Reference::null(),
	}
}

pub fn linkToStatic(
	env: JniEnv,
	_class: &'static Class,
	mut args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let appendix = args.pop().expect("appendix is required").extract_class();

	let vmindex = classes::java_lang_invoke_MemberName::vmindex(appendix.get());
	let defining_class_mirror = match classes::java_lang_invoke_MemberName::clazz(appendix.get()) {
		Throws::Ok(mirror) => mirror,
		Throws::Exception(e) => {
			e.throw(thread);
			return Reference::null();
		},
	};

	let defining_class = defining_class_mirror.get().target_class();

	let target_method = &defining_class.vtable()[vmindex as usize];

	// TODO: This is terrible. Recreating the LocalStack we just deconstructed.
	let args = args.into_iter().map(Operand::Reference).collect::<Vec<_>>();

	// SAFETY: Every operand is a reference
	let call_args =
		unsafe { LocalStack::new_with_args(args, target_method.code.max_locals as usize) };
	let ret = java_call!(@WITH_ARGS_LIST thread, target_method, call_args);

	match ret {
		Some(ret) => {
			handle_exception!(Reference::null(), thread, ret.into_box(thread))
		},
		None => Reference::null(),
	}
}

pub fn linkToSpecial(
	env: JniEnv,
	class: &'static Class,
	args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	// TODO: This is valid, right?
	linkToStatic(env, class, args)
}

pub fn linkToInterface(
	_env: JniEnv,
	_class: &'static Class,
	_args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.invoke.MethodHandle#linkToInterface")
}

pub fn linkToNative(
	_env: JniEnv,
	_class: &'static Class,
	_args: Vec<Reference>, // Object...
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.invoke.MethodHandle#linkToNative")
}
