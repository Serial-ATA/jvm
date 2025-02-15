pub mod NativeAccessor {
	include_generated!(
		"native/jdk/internal/reflect/def/DirectConstructorHandleAccessor.definitions.rs"
	);

	use crate::globals::fields;
	use crate::java_call;
	use crate::objects::array::Array;
	use crate::objects::class::Class;
	use crate::objects::class_instance::ClassInstance;
	use crate::objects::reference::{MirrorInstanceRef, Reference};
	use crate::stack::local_stack::LocalStack;
	use crate::symbols::sym;
	use crate::thread::exceptions::throw_and_return_null;
	use crate::thread::JavaThread;

	use classfile::FieldType;
	use common::traits::PtrType;
	use instructions::Operand;
	use jni::env::JniEnv;

	// throws InstantiationException, InvocationTargetException
	pub fn newInstance0(
		env: JniEnv,
		_class: &'static Class,
		c: Reference,    // java.lang.reflect.Constructor
		args: Reference, // java.lang.Object[]
	) -> Reference /* java.lang.Object */ {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };

		let constructor = c.extract_class();
		let args = args.extract_object_array();

		let clazz = fields::java_lang_reflect_Constructor::clazz(constructor.get());
		let slot = fields::java_lang_reflect_Constructor::slot(constructor.get());
		let parameter_types =
			fields::java_lang_reflect_Constructor::parameterTypes(constructor.get());

		let class = clazz.get().target_class();
		let method = &class.vtable()[slot as usize];

		assert_eq!(method.name, sym!(object_initializer_name));

		// Ensure the class is initialized
		class.initialize(thread);

		let new_instance = Reference::class(ClassInstance::new(class));

		// method parameters == parameterTypes.len() == args.len()
		let expected_arg_count = method.descriptor.parameters.len();
		assert_eq!(expected_arg_count, parameter_types.get().len());
		assert_eq!(parameter_types.get().len(), args.get().len());

		// + 1 to account for the `this` argument
		let mut call_args = LocalStack::new(expected_arg_count + 1);
		call_args[0] = Operand::Reference(new_instance.clone());

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
					FieldType::Byte => {
						Operand::from(fields::java_lang_Byte::value(&instance.get()))
					},
					FieldType::Char => {
						Operand::from(fields::java_lang_Character::value(&instance.get()))
					},
					FieldType::Double => {
						Operand::from(fields::java_lang_Double::value(&instance.get()))
					},
					FieldType::Float => {
						Operand::from(fields::java_lang_Float::value(&instance.get()))
					},
					FieldType::Int => {
						Operand::from(fields::java_lang_Integer::value(&instance.get()))
					},
					FieldType::Long => {
						Operand::from(fields::java_lang_Long::value(&instance.get()))
					},
					FieldType::Short => {
						Operand::from(fields::java_lang_Short::value(&instance.get()))
					},
					FieldType::Boolean => {
						Operand::from(fields::java_lang_Boolean::value(&instance.get()))
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

		java_call!(@WITH_ARGS_LIST thread, method, call_args);
		if thread.has_pending_exception() {
			return Reference::null();
		}

		new_instance
	}
}
