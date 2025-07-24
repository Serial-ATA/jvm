pub mod Raw {
	use crate::include_generated;
	use crate::native::java::lang::String::StringInterner;
	use crate::objects::class::ClassPtr;
	use crate::objects::instance::array::{Array, ObjectArrayInstance};
	use crate::objects::reference::Reference;
	use crate::thread::JavaThread;
	use crate::thread::exceptions::Throws;

	use std::collections::HashMap;
	use std::sync::{LazyLock, Mutex};

	use ::jni::env::JniEnv;

	include_generated!("native/jdk/internal/util/def/SystemProps$Raw.constants.rs");
	include_generated!("native/jdk/internal/util/def/SystemProps.definitions.rs");

	const JAVA_VERSION: &str = env!("SYSTEM_PROPS_JAVA_VERSION");
	const VM_SPECIFICATION_NAME: &str = env!("SYSTEM_PROPS_VM_SPECIFICATION_NAME");
	const VM_NAME: &str = env!("SYSTEM_PROPS_VM_NAME");
	const VM_VERSION: &str = env!("CARGO_PKG_VERSION");
	const VM_VENDOR: &str = env!("SYSTEM_PROPS_VM_VENDOR");

	pub static SYSTEM_PROPERTIES: LazyLock<Mutex<HashMap<String, String>>> = LazyLock::new(|| {
		let mut m = HashMap::new();

		m.insert(String::from("java.version"), String::from(JAVA_VERSION));
		m.insert(
			String::from("java.vm.specification.name"),
			String::from(VM_SPECIFICATION_NAME),
		);
		m.insert(String::from("java.vm.name"), String::from(VM_NAME));
		m.insert(String::from("java.vm.version"), String::from(VM_VERSION));
		m.insert(String::from("java.vm.vendor"), String::from(VM_VENDOR));
		m.insert(
			String::from("java.library.path"),
			platform::env::java_library_path(),
		);
		m.insert(String::from("java.home"), platform::env::java_home());
		Mutex::new(m)
	});

	pub fn vmProperties(env: JniEnv, _class: ClassPtr) -> Reference /* [Ljava/lang/String; */ {
		// TODO: FIXED_LENGTH is not the correct size here
		let string_array_class = crate::globals::classes::string_array();
		let prop_array;
		match ObjectArrayInstance::new(FIXED_LENGTH, string_array_class) {
			Throws::Ok(array) => prop_array = array,
			Throws::Exception(e) => {
				let thread = unsafe { &*JavaThread::for_env(env.raw()) };
				e.throw(thread);

				// Doesn't matter what we return, this value will never be used.
				return Reference::null();
			},
		}

		let mut index = 0;
		for (key, val) in SYSTEM_PROPERTIES.lock().unwrap().iter() {
			let interned_key_string = StringInterner::intern(&**key);
			let interned_value_string = StringInterner::intern(&**val);
			if let Throws::Exception(e) =
				prop_array.store(index, Reference::class(interned_key_string))
			{
				let thread = unsafe { &*JavaThread::for_env(env.raw()) };
				e.throw(thread);
				return Reference::null();
			}

			index += 1;
			if let Throws::Exception(e) =
				prop_array.store(index, Reference::class(interned_value_string))
			{
				let thread = unsafe { &*JavaThread::for_env(env.raw()) };
				e.throw(thread);
				return Reference::null();
			}

			index += 1;
		}

		Reference::object_array(prop_array)
	}

	pub fn platformProperties(env: JniEnv, _class: ClassPtr) -> Reference /* [Ljava/lang/String; */
	{
		macro_rules! store_properties {
			($prop_array:ident; $($index:expr => $value:expr),+ $(,)?) => {
				$(
				if let Some(val) = Option::<String>::from($value) {
					let interned_string = StringInterner::intern(&*val);
					if let Throws::Exception(e) = $prop_array.store($index, Reference::class(interned_string)) {
						let thread = unsafe { &*JavaThread::for_env(env.raw()) };
						e.throw(thread);
						return Reference::null();
					}
				}
				)+
			};
		}

		let string_array_class = crate::globals::classes::string_array();
		let prop_array;
		match ObjectArrayInstance::new(FIXED_LENGTH, string_array_class) {
			Throws::Ok(array) => prop_array = array,
			Throws::Exception(e) => {
				let thread = unsafe { &*JavaThread::for_env(env.raw()) };
				e.throw(thread);

				// Doesn't matter what we return, this value will never be used.
				return Reference::null();
			},
		}

		let mut platform_properties = platform::properties::PropertySet::default();
		platform_properties.fill().unwrap();

		store_properties!(prop_array;
			_display_country_NDX         => platform_properties.display_country,
			_display_language_NDX        => platform_properties.display_language,
			_display_script_NDX          => platform_properties.display_script,
			_display_variant_NDX         => platform_properties.display_variant,
			_file_separator_NDX          => platform_properties.file_separator,
			_format_country_NDX          => platform_properties.format_country,
			_format_language_NDX         => platform_properties.format_language,
			_format_script_NDX           => platform_properties.format_script,
			_format_variant_NDX          => platform_properties.format_variant,
			_ftp_nonProxyHosts_NDX       => platform_properties.ftp_nonProxyHosts,
			_ftp_proxyHost_NDX           => platform_properties.ftp_proxyHost,
			_ftp_proxyPort_NDX           => platform_properties.ftp_proxyPort,
			_http_nonProxyHosts_NDX      => platform_properties.http_nonProxyHosts,
			_http_proxyHost_NDX          => platform_properties.http_proxyHost,
			_http_proxyPort_NDX          => platform_properties.http_proxyPort,
			_https_proxyHost_NDX         => platform_properties.https_proxyHost,
			_https_proxyPort_NDX         => platform_properties.https_proxyPort,
			_java_io_tmpdir_NDX          => platform_properties.java_io_tmpdir,
			_line_separator_NDX          => platform_properties.line_separator,
			_native_encoding_NDX         => platform_properties.native_encoding,
			_os_arch_NDX                 => platform_properties.os_arch,
			_os_name_NDX                 => platform_properties.os_name,
			_os_version_NDX              => platform_properties.os_version,
			_path_separator_NDX          => platform_properties.path_separator,
			_socksNonProxyHosts_NDX      => platform_properties.socksNonProxyHosts,
			_socksProxyHost_NDX          => platform_properties.socksProxyHost,
			_socksProxyPort_NDX          => platform_properties.socksProxyPort,
			_stderr_encoding_NDX         => platform_properties.stderr_encoding,
			_stdin_encoding_NDX          => platform_properties.stdin_encoding,
			_stdout_encoding_NDX         => platform_properties.stdout_encoding,
			_sun_arch_abi_NDX            => platform_properties.sun_arch_abi,
			_sun_arch_data_model_NDX     => platform_properties.sun_arch_data_model,
			_sun_cpu_endian_NDX          => platform_properties.sun_cpu_endian,
			_sun_cpu_isalist_NDX         => platform_properties.sun_cpu_isalist,
			_sun_io_unicode_encoding_NDX => platform_properties.sun_io_unicode_encoding,
			_sun_jnu_encoding_NDX        => platform_properties.sun_jnu_encoding,
			_sun_os_patch_level_NDX      => platform_properties.sun_os_patch_level,
			_user_dir_NDX                => platform_properties.user_dir,
			_user_home_NDX               => platform_properties.user_home,
			_user_name_NDX               => platform_properties.user_name,
		);

		Reference::object_array(prop_array)
	}
}
