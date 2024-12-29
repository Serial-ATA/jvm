#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::properties::{Error, PropertySet};

use core::ffi::CStr;
use std::io::BufRead;
use core::mem::MaybeUninit;
use core::ptr::{self, NonNull};

//         @Native private static final int _display_country_NDX = 0;
//         @Native private static final int _display_language_NDX = 1 + _display_country_NDX;
//         @Native private static final int _display_script_NDX = 1 + _display_language_NDX;
//         @Native private static final int _display_variant_NDX = 1 + _display_script_NDX;
//         @Native private static final int _file_encoding_NDX = 1 + _display_variant_NDX;
//         @Native private static final int _format_country_NDX = 1 + _file_separator_NDX;
//         @Native private static final int _format_language_NDX = 1 + _format_country_NDX;
//         @Native private static final int _format_script_NDX = 1 + _format_language_NDX;
//         @Native private static final int _format_variant_NDX = 1 + _format_script_NDX;
//         @Native private static final int _java_io_tmpdir_NDX = 1 + _https_proxyPort_NDX;
const JAVA_IO_TMPDIR: &str = "/var/tmp";
//         @Native private static final int _os_name_NDX = 1 + _os_arch_NDX;
//         @Native private static final int _os_version_NDX = 1 + _os_name_NDX;
//         @Native private static final int _stderr_encoding_NDX = 1 + _socksProxyPort_NDX;
//         @Native private static final int _stdout_encoding_NDX = 1 + _stderr_encoding_NDX;
//         @Native private static final int _sun_arch_abi_NDX = 1 + _stdout_encoding_NDX;
//         @Native private static final int _sun_arch_data_model_NDX = 1 + _sun_arch_abi_NDX;
//         @Native private static final int _sun_cpu_isalist_NDX = 1 + _sun_cpu_endian_NDX;
//         @Native private static final int _sun_jnu_encoding_NDX = 1 + _sun_io_unicode_encoding_NDX;
//         @Native private static final int _sun_os_patch_level_NDX = 1 + _sun_jnu_encoding_NDX;
//         @Native private static final int _user_dir_NDX = 1 + _sun_os_patch_level_NDX;
//         @Native private static final int _user_home_NDX = 1 + _user_dir_NDX;
//         @Native private static final int _user_name_NDX = 1 + _user_home_NDX;

pub fn fill_properties_impl(props: &mut PropertySet) -> Result<(), Error> {
    props.java_io_tmpdir = JAVA_IO_TMPDIR.into();

    // OS props
    {
        let mut utsname = unsafe { MaybeUninit::<libc::utsname>::zeroed().assume_init() };
        unsafe {
            libc::uname(&raw mut utsname);
        }

        let sysname_raw = unsafe {
            CStr::from_ptr(utsname.sysname.as_ptr())
        };
        props.os_name = sysname_raw.to_str().expect("TODO: Other string encodings").to_owned();

        let release_raw = unsafe {
            CStr::from_ptr(utsname.release.as_ptr())
        };
        props.os_version = release_raw.to_str().expect("TODO: Other string encodings").to_owned();
    }

    // Locale props
    {
        unsafe { libc::setlocale(libc::LC_ALL, c"".as_ptr()) };
        init_locale(
            libc::LC_CTYPE,
            Some(&mut props.format_language),
            Some(&mut props.format_script),
            Some(&mut props.format_country),
            Some(&mut props.format_variant),
            Some(&mut props.file_encoding)
        );
        init_locale(
            libc::LC_MESSAGES,
            Some(&mut props.display_language),
            Some(&mut props.display_script),
            Some(&mut props.display_country),
            Some(&mut props.display_variant),
            None
        );

        props.sun_jnu_encoding = props.file_encoding.clone();
    }

    // User props
    {
        let uid = unsafe { libc::getuid() };
        let passwd = unsafe { libc::getpwuid(uid) };

        let user_name;
        let user_home;
        if passwd.is_null() {
            user_name = String::from("?");
            user_home = None;
        } else {
            let user_name_raw = unsafe {
                CStr::from_ptr((*passwd).pw_name)
            };
            user_name = user_name_raw.to_str().expect("TODO: Other string encodings").to_owned();

            let user_home_raw = unsafe {
                CStr::from_ptr((*passwd).pw_dir)
            };

            if user_home_raw.to_bytes().len() < 3 {
                user_home = None;
            } else {
                user_home = Some(user_home_raw.to_str().expect("TODO: Other string encodings").to_owned());
            }
        }

        let user_home = match user_home {
            Some(user_home) => user_home,
            None => {
                match std::env::var("HOME") {
                    Ok(env_home) if env_home.len() > 2 => {
                        env_home
                    }
                    _ => String::from("?")
                }
            }
        };
    }

    // Current directory
    {
        let Ok(dir) = std::env::current_dir() else {
            return Err(Error::WorkingDir)
        };

        props.user_dir = dir.to_str().expect("TODO: Other path encodings").to_owned();
    }

    Ok(())
}

fn init_locale(category: i32, language_field: Option<&mut String>, script_field: Option<&mut String>, country_field: Option<&mut String>, variant_field: Option<&mut String>, encoding_field: Option<&mut String>) {
    // Parses the locale in the form: `language_country.encoding@variant`
    //
    // `language_country`: Required
    // `encoding`: Optional, preceded by .
    // `variant`: Optional, preceded by @
    fn split_locale(locale: &CStr) -> (&[u8], &[u8], Option<&[u8]>, Option<&[u8]>) {
        let bytes = locale.to_bytes();

        let mut has_encoding_or_variant = false;
        let mut language_country_end_pos = bytes.len();

        // Encoding
        let mut encoding = None;
        let variant_position = bytes.iter().position(|b| *b == b'@');
        if let Some(pos) = bytes.iter().position(|b| *b == b'.') {
            language_country_end_pos = pos;

            has_encoding_or_variant = true;
            let end = variant_position.unwrap_or(bytes.len());
            encoding = Some(&bytes[pos..end]);
        }

        // Variant
        let mut variant = None;
        if let Some(pos) = variant_position {
            if encoding.is_none() {
                language_country_end_pos = pos;
            }

            has_encoding_or_variant = true;
            variant = Some(&bytes[pos..]);
        }

        let mut language_country = bytes[..language_country_end_pos].split(|b| *b == b'_');

        let language = language_country.next().expect("TODO: Nice error");
        let country = language_country.next().expect("TODO: Nice error");

        (language, country, encoding, variant)
    }

    let locale_raw = unsafe { libc::setlocale(category, ptr::null()) };

    let locale;
    if locale_raw.is_null() {
        locale = c"en_US";
    } else {
        locale = unsafe { CStr::from_ptr(locale_raw) };
    }

    let (mut language, mut country, mut encoding, mut variant) = split_locale(locale);

    if encoding.is_none() && variant.is_none() {
        if let Some((_, new_locale)) = super::locale::locale_aliases().find(|(k, _)| *k == locale) {
            (language, country, encoding, variant) = split_locale(new_locale);
        }
    }

    // Normalize the language name
    if let Some(language_field) = language_field {
        // In case we can't find anything...
        *language_field = String::from("en");

        if let Some((_, language)) = super::locale::language_names().find(|(k, _)| k.to_bytes() == language) {
            let language = language.to_str().expect("normalized table entries should never be invalid");
            *language_field = String::from(language);
        }
    }

    // Normalize the country name
    if let Some(country_field) = country_field {
        if let Some((_, country)) = super::locale::country_names().find(|(k, _)| k.to_bytes() == country) {
            let country = country.to_str().expect("normalized table entries should never be invalid");
            *country_field = String::from(country);
        }
    }

    // Normalize the script and variant name.
    // Note that we only use variants listed in the mapping array; others are ignored.
    if let Some(variant) = variant {
        if let Some(script_field) = script_field {
            if let Some((_, script)) = super::locale::script_names().find(|(k, _)| k.to_bytes() == variant) {
                let script = script.to_str().expect("normalized table entries should never be invalid");
                *script_field = String::from(script);
            }
        }

        if let Some(variant_field) = variant_field {
            if let Some((_, variant)) = crate::locale::base_variant_names().find(|(k, _)| k.to_bytes() == variant) {
                let variant = variant.to_str().expect("normalized table entries should never be invalid");
                *variant_field = String::from(variant);
            }
        }
    }

    // Normalize the encoding name.  Note that we IGNORE the string 'encoding' extracted from the
    // locale name above.  Instead, we use the more reliable method of calling `nl_langinfo(CODESET)`.
    // This function returns an empty string if no encoding is set for the given
    // locale (e.g., the C or POSIX locales); we use the default ISO 8859-1 converter for such locales.
    if let Some(encoding) = encoding {
        if let Some(encoding_field) = encoding_field {
            let ret_encoding;
            match encoding {
                b"ISO8859-15" => ret_encoding = "ISO8859-15",
                // Remap the encoding string to a different value for japanese locales on linux so that
                // customized converters are used instead of the default converter for "EUC-JP"
                b"EUC-JP" => ret_encoding = "EUC-JP-LINUX",
                _ => {
                    let lang = unsafe { libc::nl_langinfo(libc::CODESET) };
                    if lang.is_null() {
                        ret_encoding = "ISO8859-1";
                    } else {
                        let cstr = unsafe { CStr::from_ptr(lang) };
                        ret_encoding = cstr.to_str().expect("TODO: Nice error");
                    }
                }
            }

            *encoding_field = String::from(ret_encoding);
        }
    }
}
