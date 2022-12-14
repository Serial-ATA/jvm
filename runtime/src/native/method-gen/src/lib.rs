mod parse;

use std::path::PathBuf;

use walkdir::WalkDir;

static CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
static METHOD_DEFINITION_DIR_NAME: &str = "def";

pub fn run() {
	let crate_root = PathBuf::from(CRATE_ROOT);
	let native_directory = crate_root.parent().unwrap();

	let dirs_filtered = WalkDir::new(native_directory)
		.into_iter()
		.map(Result::unwrap)
		.filter(|entry| {
			entry.file_type().is_dir() && entry.file_name() == METHOD_DEFINITION_DIR_NAME
		});

	for dir in dirs_filtered {
		for entry in std::fs::read_dir(dir.path()).unwrap().map(Result::unwrap) {
			let file_contents = std::fs::read_to_string(entry.path()).unwrap();
			let class = parse::Class::parse(file_contents);

			dbg!(class);
		}
	}
}

#[test]
fn test_parse() {
	run();
}
