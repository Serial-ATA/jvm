use crate::parse::{AccessFlags, Class, Member};
use crate::util;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

macro_rules! native_method_table_file_header {
	() => {
		r#"use crate::native::{{NativeMethodDef, NativeMethodPtr}};

use std::sync::atomic::{{AtomicBool, Ordering}};

static NATIVES_REGISTERED: AtomicBool = AtomicBool::new(false);

#[allow(trivial_casts)]
pub fn registerNatives(_: JNIEnv, _: crate::stack::local_stack::LocalStack) -> NativeReturn {{
	if NATIVES_REGISTERED.compare_exchange(false, true, Ordering::SeqCst, Ordering::Acquire) != Ok(false) {{
		return None;
	}}
	
	let natives: [(NativeMethodDef<'static>, NativeMethodPtr); {}] = [
"#
	};
}

pub(crate) fn generate_register_natives_table(module: &str, class: &mut Class, def_path: &Path) {
	if !class
		.members
		.iter()
		.any(|member| matches!(member, Member::Method(method) if method.name == "registerNatives"))
	{
		return;
	}

	let native_method_table_path = def_path.join(format!("{}.registerNatives", class.class_name));
	let mut native_method_table_file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(native_method_table_path)
		.unwrap();

	write!(
        native_method_table_file,
        "{}",
        format_args!(
            native_method_table_file_header!(),
            class
                .members
                .iter()
                .filter(|member| matches!(member, Member::Method(method) if !method.modifiers.contains(AccessFlags::ACC_STATIC)))
                .count()
        )
    )
        .unwrap();

	for ref member in class.members.drain_filter(|member| {
        matches!(member, Member::Method(method) if method.name != "registerNatives" && !method.modifiers.contains(AccessFlags::ACC_STATIC))
    }) {
        match member {
            Member::Method(method) => {
                writeln!(
                    native_method_table_file,
                    "\t\t({}),",
                    util::method_table_entry(module, &class.class_name, method)
                )
                    .unwrap();
            }
            _ => unreachable!()
        }
    }

	write!(
		native_method_table_file,
		"\t];\n\n\tfor method in natives \
		 {{\n\t\tcrate::native::insert_method(method);\n\t}}\nNone\n}}"
	)
	.unwrap();
}
