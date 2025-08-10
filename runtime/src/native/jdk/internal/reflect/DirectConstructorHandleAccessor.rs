pub mod NativeAccessor {
	include_generated!(
		"native/jdk/internal/reflect/def/DirectConstructorHandleAccessor.definitions.rs"
	);

	use crate::classes;
	use crate::objects::class::ClassPtr;
	use crate::objects::instance::class::ClassInstance;
	use crate::objects::reference::Reference;
	use crate::symbols::sym;

	use jni::env::JniEnv;

	// throws InstantiationException, InvocationTargetException
	pub fn newInstance0(
		env: JniEnv,
		_class: ClassPtr,
		c: Reference,    // java.lang.reflect.Constructor
		args: Reference, // java.lang.Object[]
	) -> Reference /* java.lang.Object */ {
		let constructor = c.extract_class();
		let args = args.extract_object_array();

		let clazz = classes::java::lang::reflect::Constructor::clazz(constructor);
		let slot = classes::java::lang::reflect::Constructor::slot(constructor);
		let parameter_types =
			classes::java::lang::reflect::Constructor::parameterTypes(constructor);

		let class = clazz.target_class();
		let method = &class.vtable()[slot as usize];

		assert_eq!(method.name, sym!(object_initializer_name));

		let new_instance = Reference::class(ClassInstance::new(class));

		// Won't return anything
		let _ = crate::native::jdk::internal::reflect::DirectMethodHandleAccessor::do_invoke(
			env,
			class,
			method,
			parameter_types,
			args,
			new_instance,
		);

		new_instance
	}
}
