#![cfg_attr(rustfmt, rustfmt_skip)]

// TODO

use std::ptr::NonNull;

//         @Native private static final int _display_country_NDX = 0;
//         @Native private static final int _display_language_NDX = 1 + _display_country_NDX;
//         @Native private static final int _display_script_NDX = 1 + _display_language_NDX;
//         @Native private static final int _display_variant_NDX = 1 + _display_script_NDX;
//         @Native private static final int _file_encoding_NDX = 1 + _display_variant_NDX;
//         @Native private static final int _format_country_NDX = 1 + _file_separator_NDX;
//         @Native private static final int _format_language_NDX = 1 + _format_country_NDX;
//         @Native private static final int _format_script_NDX = 1 + _format_language_NDX;
//         @Native private static final int _format_variant_NDX = 1 + _format_script_NDX;
//         @Native private static final int _ftp_nonProxyHosts_NDX = 1 + _format_variant_NDX;
//         @Native private static final int _ftp_proxyHost_NDX = 1 + _ftp_nonProxyHosts_NDX;
//         @Native private static final int _ftp_proxyPort_NDX = 1 + _ftp_proxyHost_NDX;
//         @Native private static final int _http_nonProxyHosts_NDX = 1 + _ftp_proxyPort_NDX;
//         @Native private static final int _http_proxyHost_NDX = 1 + _http_nonProxyHosts_NDX;
//         @Native private static final int _http_proxyPort_NDX = 1 + _http_proxyHost_NDX;
//         @Native private static final int _https_proxyHost_NDX = 1 + _http_proxyPort_NDX;
//         @Native private static final int _https_proxyPort_NDX = 1 + _https_proxyHost_NDX;
//         @Native private static final int _java_io_tmpdir_NDX = 1 + _https_proxyPort_NDX;
pub const OS_NAME: &str = "Mac OS X";
//         @Native private static final int _os_version_NDX = 1 + _os_name_NDX;
//         @Native private static final int _socksNonProxyHosts_NDX = 1 + _path_separator_NDX;
//         @Native private static final int _socksProxyHost_NDX = 1 + _socksNonProxyHosts_NDX;
//         @Native private static final int _socksProxyPort_NDX = 1 + _socksProxyHost_NDX;
//         @Native private static final int _stderr_encoding_NDX = 1 + _socksProxyPort_NDX;
//         @Native private static final int _stdout_encoding_NDX = 1 + _stderr_encoding_NDX;
//         @Native private static final int _sun_arch_abi_NDX = 1 + _stdout_encoding_NDX;
//         @Native private static final int _sun_arch_data_model_NDX = 1 + _sun_arch_abi_NDX;
//         @Native private static final int _sun_cpu_isalist_NDX = 1 + _sun_cpu_endian_NDX;
pub const SUN_JNU_ENCODING: &str = "UTF-8";
//         @Native private static final int _sun_os_patch_level_NDX = 1 + _sun_jnu_encoding_NDX;
//         @Native private static final int _user_dir_NDX = 1 + _sun_os_patch_level_NDX;
//         @Native private static final int _user_home_NDX = 1 + _user_dir_NDX;
//         @Native private static final int _user_name_NDX = 1 + _user_home_NDX;

pub fn fill_properties_impl(props: &mut crate::properties::PropertySet) -> Result<(), crate::properties::Error> {
    props.os_name = OS_NAME;
    props.sun_jnu_encoding = SUN_JNU_ENCODING;
    unimplemented!("macOS property set");
}
