use crate::parse::Method;

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::{Component, Path};

/// Create a `NativeMethodDef` for a method
pub(crate) fn method_table_entry(module: &str, class_name: &str, method: &Method) -> String {
	format!(
		"NativeMethodDef {{ class: &{:?}, name: &{:?}, descriptor: &{:?} }}, \
		 crate::native::{}{}::{} as NativeMethodPtr",
		format!("{}{}", module, class_name).as_bytes(),
		method.name().as_bytes(),
		method.descriptor.as_bytes(),
		module.replace('/', "::"),
		class_name.replace('$', "::"),
		method.name()
	)
}

/// Split a path into its components, stripping the `runtime/src/native` prefix
///
/// For example, `runtime/src/native/java/lang/Object.rs` would be split into `["java", "lang", "Object.rs"]`
pub(crate) fn create_relative_path_components(path: &Path, skip_first: bool) -> Vec<String> {
	let mut components = path
		.components()
		.rev()
		.skip(if skip_first { 1 } else { 0 })
		.map(Component::as_os_str)
		.map(OsStr::to_string_lossy)
		.take_while(|comp| comp != "native")
		.map(Cow::into_owned)
		.collect::<Vec<String>>();

	components.reverse();
	components
}
