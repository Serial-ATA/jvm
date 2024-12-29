#![cfg_attr(rustfmt, rustfmt_skip)]

use std::ptr::NonNull;
use crate::macros::match_cfg_meta;

// TODO

//         @Native private static final int _display_country_NDX = 0;
//         @Native private static final int _display_language_NDX = 1 + _display_country_NDX;
//         @Native private static final int _display_script_NDX = 1 + _display_language_NDX;
//         @Native private static final int _display_variant_NDX = 1 + _display_script_NDX;
//         @Native private static final int _file_encoding_NDX = 1 + _display_variant_NDX;
pub const FILE_SEPARATOR: &str = "\\";
//         @Native private static final int _format_country_NDX = 1 + _file_separator_NDX;
//         @Native private static final int _format_language_NDX = 1 + _format_country_NDX;
//         @Native private static final int _format_script_NDX = 1 + _format_language_NDX;
//         @Native private static final int _format_variant_NDX = 1 + _format_script_NDX;
//         @Native private static final int _java_io_tmpdir_NDX = 1 + _https_proxyPort_NDX;
pub const LINE_SEPARATOR: &str = "\r\n";
// https://github.com/openjdk/jdk/blob/19373b2ff0cd795afa262c17dcb3388fd6a5be59/src/java.base/windows/native/libjava/java_props_md.c#L580-L588
match_cfg_meta! {
    match cfg(target_arch) {
        "x86_64" => {
            pub const OS_ARCH: &str = "amd64";
        },
        "x86" => {
            pub const OS_ARCH: &str = "x86";
        },
        "aarch64" => {
            pub const OS_ARCH: &str = "aarch64";
        },
        _ => {
            pub const OS_ARCH: &str = "unknown";
        }
    }
}
//         @Native private static final int _os_name_NDX = 1 + _os_arch_NDX;
//         @Native private static final int _os_version_NDX = 1 + _os_name_NDX;
pub const PATH_SEPARATOR: &str = ";";
//         @Native private static final int _stderr_encoding_NDX = 1 + _socksProxyPort_NDX;
//         @Native private static final int _stdout_encoding_NDX = 1 + _stderr_encoding_NDX;
//         @Native private static final int _sun_arch_abi_NDX = 1 + _stdout_encoding_NDX;
//         @Native private static final int _sun_arch_data_model_NDX = 1 + _sun_arch_abi_NDX;
//         @Native private static final int _sun_cpu_isalist_NDX = 1 + _sun_cpu_endian_NDX;
pub const UNICODE_ENCODING: &str = "UnicodeLittle";
//         @Native private static final int _sun_jnu_encoding_NDX = 1 + _sun_io_unicode_encoding_NDX;
//         @Native private static final int _sun_os_patch_level_NDX = 1 + _sun_jnu_encoding_NDX;
//         @Native private static final int _user_dir_NDX = 1 + _sun_os_patch_level_NDX;
//         @Native private static final int _user_home_NDX = 1 + _user_dir_NDX;
//         @Native private static final int _user_name_NDX = 1 + _user_home_NDX;

pub fn fill_properties_impl(props: &mut crate::properties::PropertySet) -> Result<(), crate::properties::Error> {
    unimplemented!("Windows property set");
}
