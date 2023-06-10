use crate::parse::{Class, Member};
use crate::{field, parse, registernatives, util};

use std::path::Path;

use walkdir::WalkDir;

static METHOD_DEFINITION_DIR_NAME: &str = "def";

pub(crate) struct Module {
	pub name: String,
	pub components: Vec<String>,
	pub classes: Vec<Class>,
}

impl Module {
	fn from_path(root: &Path) -> Self {
		let components = util::create_relative_path_components(root, true);

		let mut name = String::new();
		for comp in &components {
			name.push_str(comp);
			name.push('/');
		}

		let mut classes = Vec::new();
		for entry in std::fs::read_dir(root).unwrap().map(Result::unwrap) {
			if !entry.file_type().unwrap().is_file() {
				continue;
			}

			let path = entry.path();
			if path.extension() != Some(METHOD_DEFINITION_DIR_NAME.as_ref()) {
				continue;
			}

			let file_contents = std::fs::read_to_string(&path).unwrap();
			let mut class = parse::Class::parse(
				file_contents,
				path.file_stem().unwrap().to_str().unwrap(),
				&name,
			);

			field::generate_native_constant_fields(&mut class, root);
			registernatives::generate_register_natives_table(&name, &mut class, root);

			classes.push(class);
		}

		Self {
			name,
			components,
			classes,
		}
	}

	pub fn for_each_class<F>(&self, mut map: F)
	where
		F: FnMut(&Class),
	{
		for class in &self.classes {
			map(class);
			for member in &class.members {
				if let Member::Class(subclass) = member {
					map(subclass)
				}
			}
		}
	}
}

pub(crate) fn get_modules_from(native_directory: &Path) -> Vec<Module> {
	let dirs_filtered = WalkDir::new(native_directory)
		.into_iter()
		.map(Result::unwrap)
		.filter(|entry| {
			entry.file_type().is_dir() && entry.file_name() == METHOD_DEFINITION_DIR_NAME
		});

	let mut modules = Vec::new();
	for dir in dirs_filtered {
		modules.push(Module::from_path(dir.path()))
	}

	modules
}
