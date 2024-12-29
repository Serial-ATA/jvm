pub mod Raw {
	use crate::classpath::classloader::ClassLoader;
	use crate::include_generated;
	use crate::objects::array::ArrayInstance;
	use crate::objects::reference::Reference;
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

	pub fn platformProperties(env: NonNull<JniEnv>) -> Reference /* [Ljava/lang/String; */ {
		macro_rules! store_properties {
			($prop_array:ident; $($index:expr => $value:expr),+ $(,)?) => {
				$(
				if let Some(val) = Option::<String>::from($value) {
					let interned_string = StringInterner::intern_string(val);
					$prop_array.store($index, Operand::Reference(Reference::class(interned_string)));
				}
				)+
			};
		}

		let string_array_class = ClassLoader::Bootstrap.load(sym!(string_array)).unwrap();
		let prop_array = ArrayInstance::new_reference(FIXED_LENGTH, string_array_class);

		let prop_array_mut = prop_array.get_mut();

		let mut system_properties = platform::properties::PropertySet::default();
		system_properties.fill().unwrap();

		store_properties!(prop_array_mut;
			_display_country_NDX         => system_properties.display_country,
			_display_language_NDX        => system_properties.display_language,
			_display_script_NDX          => system_properties.display_script,
			_display_variant_NDX         => system_properties.display_variant,
			_file_encoding_NDX           => system_properties.file_encoding,
			_file_separator_NDX          => system_properties.file_separator,
			_format_country_NDX          => system_properties.format_country,
			_format_language_NDX         => system_properties.format_language,
			_format_script_NDX           => system_properties.format_script,
			_format_variant_NDX          => system_properties.format_variant,
			_ftp_nonProxyHosts_NDX       => system_properties.ftp_nonProxyHosts,
			_ftp_proxyHost_NDX           => system_properties.ftp_proxyHost,
			_ftp_proxyPort_NDX           => system_properties.ftp_proxyPort,
			_http_nonProxyHosts_NDX      => system_properties.http_nonProxyHosts,
			_http_proxyHost_NDX          => system_properties.http_proxyHost,
			_http_proxyPort_NDX          => system_properties.http_proxyPort,
			_https_proxyHost_NDX         => system_properties.https_proxyHost,
			_https_proxyPort_NDX         => system_properties.https_proxyPort,
			_java_io_tmpdir_NDX          => system_properties.java_io_tmpdir,
			_line_separator_NDX          => system_properties.line_separator,
			_os_arch_NDX                 => system_properties.os_arch,
			_os_name_NDX                 => system_properties.os_name,
			_os_version_NDX              => system_properties.os_version,
			_path_separator_NDX          => system_properties.path_separator,
			_socksNonProxyHosts_NDX      => system_properties.socksNonProxyHosts,
			_socksProxyHost_NDX          => system_properties.socksProxyHost,
			_socksProxyPort_NDX          => system_properties.socksProxyPort,
			_stderr_encoding_NDX         => system_properties.stderr_encoding,
			_stdout_encoding_NDX         => system_properties.stdout_encoding,
			_sun_arch_abi_NDX            => system_properties.sun_arch_abi,
			_sun_arch_data_model_NDX     => system_properties.sun_arch_data_model,
			_sun_cpu_endian_NDX          => system_properties.sun_cpu_endian,
			_sun_cpu_isalist_NDX         => system_properties.sun_cpu_isalist,
			_sun_io_unicode_encoding_NDX => system_properties.sun_io_unicode_encoding,
			_sun_jnu_encoding_NDX        => system_properties.sun_jnu_encoding,
			_sun_os_patch_level_NDX      => system_properties.sun_os_patch_level,
			_user_dir_NDX                => system_properties.user_dir,
			_user_home_NDX               => system_properties.user_home,
			_user_name_NDX               => system_properties.user_name,
		);

		Reference::array(prop_array)
	}
}
