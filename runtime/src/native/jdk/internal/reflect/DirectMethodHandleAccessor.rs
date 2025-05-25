use crate::objects::array::Array;
use crate::objects::boxing::Boxable;
use crate::objects::class::Class;
use crate::objects::method::Method;
use crate::objects::reference::{MirrorInstanceRef, ObjectArrayInstanceRef, Reference};
use crate::stack::local_stack::LocalStack;
use crate::thread::JavaThread;
use crate::thread::exceptions::{handle_exception, throw_and_return_null};
use crate::{classes, java_call};

use classfile::FieldType;
use common::traits::PtrType;
use instructions::Operand;
use jni::env::JniEnv;

pub mod NativeAccessor {
	include_generated!("native/jdk/internal/reflect/def/DirectMethodHandleAccessor.definitions.rs");

	use crate::classes;
	use crate::objects::class::Class;
	use crate::objects::reference::Reference;
	use crate::thread::JavaThread;
	use crate::thread::exceptions::throw_and_return_null;

	use common::traits::PtrType;
	use jni::env::JniEnv;

	pub fn invoke0(
		env: JniEnv,
		_class: &'static Class,
		m: Reference,    // java.lang.reflect.Method
		obj: Reference,  // java.lang.Object
		args: Reference, // Object[]
	) -> Reference /* java.lang.Object */ {
		let m = m.extract_class();
		let class = classes::java::lang::reflect::Method::clazz(m.get());
		let Some(target_method) = classes::java::lang::reflect::Method::vmtarget(m.get()) else {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			throw_and_return_null!(thread, InternalError, "invoke");
		};

		let parameter_types = classes::java::lang::reflect::Method::parameterTypes(m.get());

		super::do_invoke(
			env,
			class.get().target_class(),
			target_method,
			parameter_types,
			args.extract_object_array(),
			obj,
		)
	}
}

// Shared with DirectConstructorHandleAccessor
pub(super) fn do_invoke(
	env: JniEnv,
	target_class: &'static Class,
	target_method: &'static Method,
	parameter_types: ObjectArrayInstanceRef,
	args: ObjectArrayInstanceRef,
	receiver: Reference,
) -> Reference {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	// Ensure the class is initialized
	handle_exception!(Reference::null(), thread, target_class.initialize(thread));

	// method parameters == parameterTypes.len() == args.len()
	let expected_arg_count = target_method.descriptor.parameters.len();
	if expected_arg_count != parameter_types.get().len()
		|| parameter_types.get().len() != args.get().len()
	{
		throw_and_return_null!(
			thread,
			IllegalArgumentException,
			"wrong number of arguments"
		);
	}

	// + 1 to account for the `this` argument
	let mut call_args = LocalStack::new(expected_arg_count + 1);
	call_args[0] = Operand::Reference(receiver);

	for (index, (arg, parameter_mirror)) in args
		.get()
		.as_slice()
		.into_iter()
		.zip(parameter_types.get().as_slice())
		.enumerate()
	{
		let parameter_mirror_instance: MirrorInstanceRef = parameter_mirror.extract_mirror();
		let parameter_mirror = parameter_mirror_instance.get();

		if parameter_mirror.is_primitive() {
			let instance = arg.extract_class();
			let value = match parameter_mirror.primitive_target() {
				FieldType::Byte => Operand::from(classes::java::lang::Byte::value(&instance.get())),
				FieldType::Character => {
					Operand::from(classes::java::lang::Character::value(&instance.get()))
				},
				FieldType::Double => {
					Operand::from(classes::java::lang::Double::value(&instance.get()))
				},
				FieldType::Float => {
					Operand::from(classes::java::lang::Float::value(&instance.get()))
				},
				FieldType::Integer => {
					Operand::from(classes::java::lang::Integer::value(&instance.get()))
				},
				FieldType::Long => Operand::from(classes::java::lang::Long::value(&instance.get())),
				FieldType::Short => {
					Operand::from(classes::java::lang::Short::value(&instance.get()))
				},
				FieldType::Boolean => {
					Operand::from(classes::java::lang::Boolean::value(&instance.get()))
				},
				_ => throw_and_return_null!(
					thread,
					IllegalArgumentException,
					"argument type mismatch"
				),
			};

			call_args[index + 1] = value;
			continue;
		}

		let target_class = parameter_mirror.target_class();
		if !arg.is_instance_of(target_class) {
			throw_and_return_null!(thread, IllegalArgumentException, "argument type mismatch")
		}

		call_args[index + 1] = Operand::Reference(Reference::clone(arg));
	}

	let ret = java_call!(@WITH_ARGS_LIST thread, target_method, call_args);
	if thread.has_pending_exception() {
		return Reference::null();
	}

	match ret {
		Some(r) => handle_exception!(Reference::null(), thread, r.into_box(thread)),
		None => Reference::null(),
	}
}
