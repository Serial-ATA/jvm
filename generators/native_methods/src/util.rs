use crate::parse::{Class, Method};

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::{Component, Path};

/// Create a `NativeMethodDef` for a method
pub(crate) fn method_table_entry(module: &str, class: &Class, method: &Method) -> String {
	let ptr = if method.is_static() {
		format!(
			"NativeMethodPtr::new_static(crate::native::{}{}::definitions::_{})",
			escape_module_name(module).replace('/', "::"),
			class.class_name.replace('$', "::"),
			method.name()
		)
	} else {
		format!(
			"NativeMethodPtr::new_non_static(crate::native::{}{}::definitions::_{})",
			escape_module_name(module).replace('/', "::"),
			class.class_name.replace('$', "::"),
			method.name()
		)
	};

	format!(
		"NativeMethodDef {{ class: sym!({}), name: sym!({}), descriptor: sym!({}), is_static: {} \
		 }}, {ptr}",
		format!("{}{}", module, class.class_name)
			.replace('/', "_")
			.replace('$', "_"),
		method.name_symbol(),
		method.signature_symbol_name(),
		method.is_static(),
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

/// Convert a module name into a valid rust module name
///
/// This is needed for java/lang/ref
pub(crate) fn escape_module_name(name: &str) -> String {
	name.replace("ref", "r#ref")
}
