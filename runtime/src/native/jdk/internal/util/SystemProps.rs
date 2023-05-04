pub mod Raw {
	include!("def/SystemProps$Raw.constants");

	use crate::class_instance::ArrayInstance;
	use crate::classpath::classloader::ClassLoader;
	use crate::native::{JNIEnv, NativeReturn};
	use crate::reference::Reference;
	use crate::stack::local_stack::LocalStack;

	use instructions::Operand;

	// TODO
	pub fn vmProperties(_: JNIEnv, _: LocalStack) -> NativeReturn {
		let string_array_class = ClassLoader::Bootstrap.load(b"[Ljava/lang/Object;").unwrap();
		let prop_array = ArrayInstance::new_reference(FIXED_LENGTH, string_array_class);
		Some(Operand::Reference(Reference::Array(prop_array)))
	}

	// TODO: https://github.com/openjdk/jdk/blob/19373b2ff0cd795afa262c17dcb3388fd6a5be59/src/java.base/share/native/libjava/System.c#L107
	pub fn platformProperties(_: JNIEnv, _: LocalStack) -> NativeReturn {
		let string_array_class = ClassLoader::Bootstrap.load(b"[Ljava/lang/String;").unwrap();
		let prop_array = ArrayInstance::new_reference(FIXED_LENGTH, string_array_class);
		Some(Operand::Reference(Reference::Array(prop_array)))
	}
}
