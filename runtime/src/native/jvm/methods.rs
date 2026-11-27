#![native_macros::jni_fn_module]

use crate::native::jni::references::JObjectExt;
use crate::native::jvm::reflection::ensure_class_mirror;
use crate::native::jvm::strings::StringInterner;
use crate::objects::boxing::Boxable;
use crate::objects::class::ClassPtr;
use crate::objects::instance::array::{Array, ObjectArrayInstance, ObjectArrayInstanceRef};
use crate::objects::instance::class::ClassInstance;
use crate::objects::instance::mirror::MirrorInstanceRef;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::symbols::sym;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, handle_exception, throw_with_ret};
use crate::{classes, java_call};

use std::ffi::{c_char, c_int, c_uchar, c_ushort};

use ::jni::env::JniEnv;
use ::jni::objects::{JClass, JObject, JObjectArray};
use ::jni::sys::{jboolean, jint};
use classfile::FieldType;
use native_macros::jni_call;

fn do_invoke(
	env: JniEnv,
	target_class: ClassPtr,
	target_method: &'static Method,
	parameter_types: ObjectArrayInstanceRef,
	args: Option<ObjectArrayInstanceRef>,
	receiver: Reference,
) -> JObject {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	// Ensure the class is initialized
	handle_exception!(JObject::null(), thread, target_class.initialize(thread));

	// method parameters == parameterTypes.len() == args.len()
	let expected_arg_count = target_method.descriptor.parameters.len();
	if expected_arg_count != parameter_types.len()
		|| args.is_some_and(|args| parameter_types.len() != args.len())
	{
		throw_with_ret!(
			JObject::null(),
			thread,
			IllegalArgumentException,
			"wrong number of arguments"
		);
	}

	let stack = thread.stack();
	if !target_method.is_static() {
		stack.push_reference(receiver);
	}

	if let Some(args) = args {
		for (arg, parameter_mirror) in args.as_slice().iter().zip(parameter_types.as_slice()) {
			let parameter_mirror: MirrorInstanceRef = parameter_mirror.extract_mirror();

			if parameter_mirror.is_primitive() {
				let instance = arg.extract_class();
				match parameter_mirror.primitive_target() {
					FieldType::Byte => {
						stack.push_int(classes::java::lang::Byte::value(instance) as jint)
					},
					FieldType::Character => {
						stack.push_int(classes::java::lang::Character::value(instance) as jint);
					},
					FieldType::Double => {
						stack.push_double(classes::java::lang::Double::value(instance))
					},
					FieldType::Float => {
						stack.push_float(classes::java::lang::Float::value(instance))
					},
					FieldType::Integer => {
						stack.push_int(classes::java::lang::Integer::value(instance))
					},
					FieldType::Long => stack.push_long(classes::java::lang::Long::value(instance)),
					FieldType::Short => {
						stack.push_int(classes::java::lang::Short::value(instance) as jint)
					},
					FieldType::Boolean => {
						stack.push_int(classes::java::lang::Boolean::value(instance) as jint)
					},
					_ => throw_with_ret!(
						JObject::null(),
						thread,
						IllegalArgumentException,
						"argument type mismatch"
					),
				}

				continue;
			}

			let target_class = parameter_mirror.target_class();
			if !arg.is_instance_of(target_class) {
				throw_with_ret!(
					JObject::null(),
					thread,
					IllegalArgumentException,
					"argument type mismatch"
				)
			}

			stack.push_reference(*arg);
		}
	}

	let ret = java_call!(thread, target_method);
	if thread.has_pending_exception() {
		return JObject::null();
	}

	match ret {
		Some(r) => match r.into_box(thread) {
			Throws::Ok(ret) => thread.jni_refs().allocate_wrapped(ret),
			Throws::Exception(e) => {
				e.throw(thread);
				JObject::null()
			},
		},
		None => JObject::null(),
	}
}

#[jni_call]
pub extern "C" fn JVM_InvokeMethod(
	env: JniEnv,
	method: JObject,     // java.lang.reflect.Method
	obj: JObject,        // java.lang.Object
	args0: JObjectArray, // Object[]
) -> JObject {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	let method_ref = unsafe { method.to_reference() }.expect("null method");
	let method = method_ref.extract_class();

	let obj = unsafe { obj.to_reference_maybe_null() };

	let args = unsafe { args0.to_reference() }.map(Reference::extract_object_array);

	let class = classes::java::lang::reflect::Method::clazz(method);
	let Some(target_method) = classes::java::lang::reflect::Method::vmtarget(method) else {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw_with_ret!(JObject::null(), thread, InternalError, "invoke");
	};

	let parameter_types = classes::java::lang::reflect::Method::parameterTypes(method);

	do_invoke(
		env,
		class.target_class(),
		target_method,
		parameter_types,
		args,
		obj,
	)
}

#[jni_call]
pub extern "C" fn JVM_NewInstanceFromConstructor(
	env: JniEnv,
	c: JObject,          // java.lang.reflect.Constructor
	args0: JObjectArray, // java.lang.Object[]
) -> JObject {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	let constructor_ref = unsafe { c.to_reference() }.expect("null constructor");
	let constructor = constructor_ref.extract_class();

	let args = unsafe { args0.to_reference() }.map(Reference::extract_object_array);

	let clazz = classes::java::lang::reflect::Constructor::clazz(constructor);
	let slot = classes::java::lang::reflect::Constructor::slot(constructor);
	let parameter_types = classes::java::lang::reflect::Constructor::parameterTypes(constructor);

	let class = clazz.target_class();
	let method = &class.vtable()[slot as usize];

	assert_eq!(method.name, sym!(object_initializer_name));

	let new_instance = Reference::class(ClassInstance::new(class));

	// Won't return anything
	let _ = do_invoke(env, class, method, parameter_types, args, new_instance);

	thread.jni_refs().allocate_wrapped(new_instance)
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxExceptionIndexes(
	_env: JniEnv,
	_cb: JClass,
	_method_index: jint,
	_exceptions: *mut c_ushort,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetMethodIxExceptionsCount(
	_env: JniEnv,
	_cb: JClass,
	_method_index: jint,
) -> jint {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxByteCode(
	_env: JniEnv,
	_cb: JClass,
	_method_index: jint,
	_code: *mut c_uchar,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetMethodIxByteCodeLength(
	_env: JniEnv,
	_cb: JClass,
	_method_index: jint,
) -> jint {
	todo!()
}

#[repr(C)]
pub struct JVM_ExceptionTableEntryType {
	pub start_pc: jint,
	pub end_pc: jint,
	pub handler_pc: jint,
	pub catch_type: jint,
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxExceptionTableEntry(
	_env: JniEnv,
	_cb: JClass,
	_method_index: jint,
	_entry_index: jint,
	_entry: *mut JVM_ExceptionTableEntryType,
) {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxExceptionTableLength(
	_env: JniEnv,
	_cb: JClass,
	_index: c_int,
) -> jint {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxModifiers(_env: JniEnv, _cb: JClass, _index: c_int) -> jint {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxLocalsCount(_env: JniEnv, _cb: JClass, _index: c_int) -> jint {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxArgsSize(_env: JniEnv, _cb: JClass, _index: c_int) -> jint {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxMaxStack(_env: JniEnv, _cb: JClass, _index: c_int) -> jint {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_IsConstructorIx(_env: JniEnv, _cb: JClass, _index: c_int) -> jboolean {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_IsVMGeneratedMethodIx(_env: JniEnv, _cb: JClass, _index: c_int) -> jboolean {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxNameUTF(_env: JniEnv, _cb: JClass, _index: jint) -> *const c_char {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxSignatureUTF(
	_env: JniEnv,
	_cb: JClass,
	_index: jint,
) -> *const c_char {
	todo!()
}

// Returns Object[3] where:
//
// [0]: The class holding the enclosing method (not null, java.lang.Class<?>)
// [1]: The enclosing method's name (nullable, java.lang.String)
// [2]: The enclosing method's descriptor (nullable. java.lang.String)
#[jni_call]
pub extern "C" fn JVM_GetEnclosingMethodInfo(env: JniEnv, class: JClass) -> JObjectArray {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	let this = unsafe { class.to_reference() }.expect("invalid class reference");
	let Some(mirror) = handle_exception!(JObjectArray::null(), thread, ensure_class_mirror(this))
	else {
		return JObjectArray::null();
	};

	let array = ObjectArrayInstance::new(3, crate::globals::classes::java_lang_Object());
	let array_instance: ObjectArrayInstanceRef =
		handle_exception!(JObjectArray::null(), thread, array);

	let target_class = mirror.target_class();

	let Some(enclosing_method) = target_class.unwrap_class_instance().enclosing_method else {
		// Class has no immediate enclosing method/class information
		return JObjectArray::null();
	};

	let enclosing_class = enclosing_method.class;

	// SAFETY: We know that the array has a length of 3
	unsafe {
		array_instance.store_unchecked(0, Reference::mirror(enclosing_class.mirror()));
	}

	let Some(enclosing_method) = enclosing_method.method else {
		// There is no immediate enclosing method
		return thread.jni_refs().allocate_wrapped(array_instance);
	};

	unsafe {
		array_instance.store_unchecked(
			1,
			Reference::class(StringInterner::intern(enclosing_method.name)),
		);

		array_instance.store_unchecked(
			2,
			Reference::class(StringInterner::intern(enclosing_method.descriptor_sym())),
		);
	}

	thread.jni_refs().allocate_wrapped(array_instance)
}
