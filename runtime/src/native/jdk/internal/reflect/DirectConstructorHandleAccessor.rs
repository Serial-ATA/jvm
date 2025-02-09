pub mod NativeAccessor {
	include_generated!(
		"native/jdk/internal/reflect/def/DirectConstructorHandleAccessor.definitions.rs"
	);

	use crate::objects::class::Class;
	use crate::objects::reference::Reference;

	use jni::env::JniEnv;

	// throws InstantiationException, InvocationTargetException
	pub fn newInstance0(
		_env: JniEnv,
		_class: &'static Class,
		_c: Reference,    // java.lang.reflect.Constructor
		_args: Reference, // java.lang.Object[]
	) -> Reference /* java.lang.Object */ {
		unimplemented!(
			"jdk.internal.reflect.DirectConstructorHandleAccessor$NativeAccessor#newInstance0"
		)
	}
}
