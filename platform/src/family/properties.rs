#![cfg_attr(rustfmt, rustfmt_skip)]

// Properties shared between all `target_family`s

pub const CPU_ENDIAN: &str = if cfg!(target_endian = "big") { "big" } else { "little" };

// TODO: Document
// TODO: Probably make all fields `Option`, to avoid setting empty values at runtime
#[allow(non_snake_case)]
pub struct PropertySet {
    pub display_country: String,
    pub display_language: String,
    pub display_script: String,
    pub display_variant: String,
    pub file_encoding: String,
    pub file_separator: String,
    pub format_country: String,
    pub format_language: String,
    pub format_script: String,
    pub format_variant: String,
    pub ftp_nonProxyHosts: Option<String>,
    pub ftp_proxyHost: Option<String>,
    pub ftp_proxyPort: Option<String>,
    pub http_nonProxyHosts: Option<String>,
    pub http_proxyHost: Option<String>,
    pub http_proxyPort: Option<String>,
    pub https_proxyHost: Option<String>,
    pub https_proxyPort: Option<String>,
    pub java_io_tmpdir: String,
    pub line_separator: String,
    pub os_arch: String,
    pub os_name: String,
    pub os_version: String,
    pub path_separator: String,
    pub socksNonProxyHosts: Option<String>,
    pub socksProxyHost: Option<String>,
    pub socksProxyPort: Option<String>,
    pub stderr_encoding: Option<String>,
    pub stdout_encoding: Option<String>,
    pub sun_arch_abi: Option<String>,
    pub sun_arch_data_model: String,
    pub sun_cpu_endian: String,
    pub sun_cpu_isalist: Option<String>,
    pub sun_io_unicode_encoding: String,
    pub sun_jnu_encoding: String,
    pub sun_os_patch_level: String,
    pub user_dir: String,
    pub user_home: String,
    pub user_name: String,
}

impl PropertySet {
    pub fn fill(&mut self) -> Result<(), Error> {
        fill_properties_impl(self)
    }
}

impl Default for PropertySet {
    fn default() -> Self {
        PropertySet {
            // Set the constant properties, exported below
            file_separator: FILE_SEPARATOR.into(),
            line_separator: LINE_SEPARATOR.into(),
            os_arch: OS_ARCH.into(),
            path_separator: PATH_SEPARATOR.into(),
            sun_cpu_endian: CPU_ENDIAN.into(),
            sun_io_unicode_encoding: UNICODE_ENCODING.into(),

            // Others will need to be filled by their platform-specific implementations
            display_country: String::new(),
            display_language: String::new(),
            display_script: String::new(),
            display_variant: String::new(),
            file_encoding: String::new(),
            format_country: String::new(),
            format_language: String::new(),
            format_script: String::new(),
            format_variant: String::new(),
            ftp_nonProxyHosts: None,
            ftp_proxyHost: None,
            ftp_proxyPort: None,
            http_nonProxyHosts: None,
            http_proxyHost: None,
            http_proxyPort: None,
            https_proxyHost: None,
            https_proxyPort: None,
            java_io_tmpdir: String::new(),
            os_name: String::new(),
            os_version: String::new(),
            socksNonProxyHosts: None,
            socksProxyHost: None,
            socksProxyPort: None,
            stderr_encoding: None,
            stdout_encoding: None,
            sun_arch_abi: None,
            sun_arch_data_model: String::new(),
            sun_cpu_isalist: None,
            sun_jnu_encoding: String::new(),
            sun_os_patch_level: String::new(),
            user_dir: String::new(),
            user_home: String::new(),
            user_name: String::new(),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    WorkingDir,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::WorkingDir => f.write_str("Could not determine currenting working directory")
        }
    }
}

impl core::error::Error for Error {}

// Export family specific properties

use std::fmt::{Display, Formatter};
#[cfg(target_family = "unix")]
pub use super::unix::properties::*;
#[cfg(target_family = "windows")]
pub use super::windows::properties::*;