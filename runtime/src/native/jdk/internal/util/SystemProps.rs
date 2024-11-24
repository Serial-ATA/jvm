pub mod Raw {
	use crate::class_instance::ArrayInstance;
	use crate::classpath::classloader::ClassLoader;
	use crate::include_generated;
	use crate::reference::Reference;
	use crate::string_interner::StringInterner;

	use std::ptr::NonNull;

	use ::jni::env::JniEnv;
	use common::traits::PtrType;
	use instructions::Operand;
	use symbols::sym;

	include_generated!("native/jdk/internal/util/def/SystemProps$Raw.constants.rs");
	include_generated!("native/jdk/internal/util/def/SystemProps.definitions.rs");

	pub fn vmProperties(_env: NonNull<JniEnv>) -> Reference /* [Ljava/lang/String; */ {
		macro_rules! store_properties {
			($prop_array:ident; $($key:literal => $value:expr),+ $(,)?) => {
				let mut index = 0;
				$(
					let interned_key_string = StringInterner::intern_str($key);
					let interned_value_string = StringInterner::intern_string($value);
					$prop_array.store(index, Operand::Reference(Reference::class(interned_key_string)));
					index += 1;
					$prop_array.store(index, Operand::Reference(Reference::class(interned_value_string)));
					index += 1;
					let _ = index;
				)+
			};
		}

		let string_array_class = ClassLoader::Bootstrap.load(sym!(object_array)).unwrap();
		let prop_array = ArrayInstance::new_reference(FIXED_LENGTH, string_array_class);

		let prop_array_mut = prop_array.get_mut();
		store_properties!(prop_array_mut;
			// TODO: Something more ideal
			"java.home" => std::env::var("JAVA_HOME").unwrap(),
			// TODO: Set more properties
		);

		Reference::array(prop_array)
	}

	pub fn platformProperties(_env: NonNull<JniEnv>) -> Reference /* [Ljava/lang/String; */ {
		macro_rules! store_properties {
			($prop_array:ident; $($index:expr => $value:expr),+ $(,)?) => {
				$(
				let interned_string = StringInterner::intern_str($value);
				$prop_array.store($index, Operand::Reference(Reference::class(interned_string)));
				)+
			};
		}

		let string_array_class = ClassLoader::Bootstrap.load(sym!(string_array)).unwrap();
		let prop_array = ArrayInstance::new_reference(FIXED_LENGTH, string_array_class);

		let prop_array_mut = prop_array.get_mut();

		store_properties!(prop_array_mut;
			_display_country_NDX         => "TODO",
			_display_language_NDX        => "TODO",
			_display_script_NDX          => "TODO",
			_display_variant_NDX         => "TODO",
			_file_encoding_NDX           => "TODO",
			_file_separator_NDX          => "TODO",
			_format_country_NDX          => "TODO",
			_format_language_NDX         => "TODO",
			_format_script_NDX           => "TODO",
			_format_variant_NDX          => "TODO",
			_ftp_nonProxyHosts_NDX       => "TODO",
			_ftp_proxyHost_NDX           => "TODO",
			_ftp_proxyPort_NDX           => "TODO",
			_http_nonProxyHosts_NDX      => "TODO",
			_http_proxyHost_NDX          => "TODO",
			_http_proxyPort_NDX          => "TODO",
			_https_proxyHost_NDX         => "TODO",
			_https_proxyPort_NDX         => "TODO",
			_java_io_tmpdir_NDX          => "TODO",
			_line_separator_NDX          => "TODO",
			_os_arch_NDX                 => "TODO",
			_os_name_NDX                 => "TODO",
			_os_version_NDX              => "TODO",
			_path_separator_NDX          => "TODO",
			_socksNonProxyHosts_NDX      => "TODO",
			_socksProxyHost_NDX          => "TODO",
			_socksProxyPort_NDX          => "TODO",
			_stderr_encoding_NDX         => "TODO",
			_stdout_encoding_NDX         => "TODO",
			_sun_arch_abi_NDX            => "TODO",
			_sun_arch_data_model_NDX     => "TODO",
			_sun_cpu_endian_NDX          => "TODO",
			_sun_cpu_isalist_NDX         => "TODO",
			_sun_io_unicode_encoding_NDX => "TODO",
			_sun_jnu_encoding_NDX        => "TODO",
			_sun_os_patch_level_NDX      => "TODO",
			_user_dir_NDX                => "TODO",
			_user_home_NDX               => "TODO",
			_user_name_NDX               => "TODO",
		);

		Reference::array(prop_array)
	}
}
