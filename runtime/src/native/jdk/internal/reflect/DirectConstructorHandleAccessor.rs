pub mod NativeAccessor {
	include_generated!(
		"native/jdk/internal/reflect/def/DirectConstructorHandleAccessor.definitions.rs"
	);

	use crate::objects::array::Array;
	use crate::objects::class::Class;
	use crate::objects::class_instance::ClassInstance;
	use crate::objects::reference::{MirrorInstanceRef, Reference};
	use crate::stack::local_stack::LocalStack;
	use crate::symbols::sym;
	use crate::thread::exceptions::{handle_exception, throw_and_return_null};
	use crate::thread::JavaThread;
	use crate::{classes, java_call};

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
		let constructor = c.extract_class();
		let args = args.extract_object_array();

		let clazz = classes::java_lang_reflect_Constructor::clazz(constructor.get());
		let slot = classes::java_lang_reflect_Constructor::slot(constructor.get());
		let parameter_types =
			classes::java_lang_reflect_Constructor::parameterTypes(constructor.get());

		let class = clazz.get().target_class();
		let method = &class.vtable()[slot as usize];

		assert_eq!(method.name, sym!(object_initializer_name));

		let new_instance = Reference::class(ClassInstance::new(class));

		// Won't return anything
		let _ = crate::native::jdk::internal::reflect::DirectMethodHandleAccessor::do_invoke(
			env,
			clazz.get().target_class(),
			method,
			parameter_types,
			args,
			new_instance.clone(),
		);

		new_instance
	}
}
