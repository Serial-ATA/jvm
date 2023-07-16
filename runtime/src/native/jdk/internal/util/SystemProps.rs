pub mod Raw {
	use crate::class_instance::ArrayInstance;
	use crate::classpath::classloader::ClassLoader;
	use crate::include_generated;
	use crate::native::JNIEnv;
	use crate::reference::Reference;

	use common::traits::PtrType;
	use symbols::sym;

	include_generated!("native/jdk/internal/util/def/SystemProps$Raw.constants.rs");
	include_generated!("native/jdk/internal/util/def/SystemProps.definitions.rs");

	// TODO
	pub fn vmProperties(_env: JNIEnv) -> Reference /* [Ljava/lang/String; */ {
		let string_array_class = ClassLoader::Bootstrap.load(sym!(object_array)).unwrap();
		let prop_array = ArrayInstance::new_reference(FIXED_LENGTH, string_array_class);
		Reference::Array(prop_array)
	}

	// TODO: https://github.com/openjdk/jdk/blob/19373b2ff0cd795afa262c17dcb3388fd6a5be59/src/java.base/share/native/libjava/System.c#L107
	pub fn platformProperties(_env: JNIEnv) -> Reference /* [Ljava/lang/String; */ {
		macro_rules! store_property {
			($prop_array:ident, $index:expr, $value:expr) => {
				let interned_string = StringInterner::intern($value.as_bytes());
				$prop_array.store($index, Operand::Reference(interned_string));
			};
		}

		let string_array_class = ClassLoader::Bootstrap.load(sym!(string_array)).unwrap();
		let prop_array = ArrayInstance::new_reference(FIXED_LENGTH, string_array_class);

		let prop_array_mut = prop_array.get_mut();

		// OS properties
		// store_property!(prop_array_mut, _os_name_NDX, "TODO");
		// store_property!(prop_array_mut, _os_version_NDX, "TODO");
		// store_property!(prop_array_mut, _os_arch_NDX, "TODO");

		Reference::Array(prop_array)
	}
}
