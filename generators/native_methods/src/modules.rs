use crate::parse::{Class, Member};
use crate::{SymbolCollector, definitions, field, parse, registernatives, util};

use indexmap::IndexMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

static METHOD_DEFINITION_DIR_NAME: &str = "def";

#[derive(Debug)]
pub struct ModuleComponent {
	pub name: String,
	pub classes: Vec<Class>,
}

impl PartialEq for ModuleComponent {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Eq for ModuleComponent {}

impl PartialOrd for ModuleComponent {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for ModuleComponent {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.name.cmp(&other.name)
	}
}

impl ModuleComponent {
	/// The escaped rust name for this module component
	///
	/// This is needed for java/lang/ref
	pub fn rust_name(&self) -> String {
		if self.name == "ref" {
			return String::from("r#ref");
		}

		self.name.clone()
	}
}

#[derive(Debug)]
pub(crate) struct Module {
	pub name: String,
	pub components: Vec<ModuleComponent>,
}

impl Module {
	fn from_path(
		generated_directory: &Path,
		root: &Path,
		symbol_collector: &mut SymbolCollector,
	) -> Self {
		let component_names = util::create_relative_path_components(root, true);

		let mut name = String::new();
		for comp in &component_names {
			name.push_str(comp);
			name.push('/');
		}

		// Skip over /home/.../ until we make it to `native`
		let non_absolute_generated_path = root
			.components()
			.skip_while(|c| c.as_os_str().to_str().unwrap() != "native")
			.skip(1)
			.collect::<PathBuf>();

		let generated_root = format!(
			"{}{}{}",
			generated_directory.display(),
			std::path::MAIN_SEPARATOR,
			non_absolute_generated_path.display()
		);
		std::fs::create_dir_all(&generated_root).unwrap();

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

			definitions::generate_definitions_for_class(Path::new(&generated_root), &class);
			field::generate_native_constant_fields(&mut class, Path::new(&generated_root));
			registernatives::generate_register_natives_table(
				&name,
				&mut class,
				Path::new(&generated_root),
				symbol_collector,
			);

			classes.push(class);
		}

		let mut components = Vec::with_capacity(component_names.len());
		for component in component_names {
			components.push(ModuleComponent {
				name: component,
				classes: Vec::new(),
			})
		}

		// We don't have access to the full module yet, we can only define the classes available at the tail
		components.last_mut().unwrap().classes = classes;

		Self { name, components }
	}

	fn merge_with(&mut self, other: Self) {
		assert!(self.components.len() >= other.components.len());
		for (i, component) in other.components.into_iter().enumerate() {
			assert_eq!(self.components[i].name, component.name);
			self.components[i].classes.extend(component.classes);
		}
	}

	fn is_submodule_of(&self, other: &Self) -> bool {
		if other.components.len() < self.components.len() {
			return false;
		}

		for (i, component) in self.components.iter().enumerate() {
			if component != &other.components[i] {
				return false;
			}
		}

		true
	}

	pub fn common_root_depth(&self, other: &Self) -> u8 {
		let mut count = 0;

		for (c1, c2) in self.components.iter().zip(other.components.iter()) {
			if c1 != c2 {
				break;
			}

			count += 1;
		}

		count
	}

	pub fn for_each_class<F>(&self, mut map: F)
	where
		F: FnMut(&Class),
	{
		for component in &self.components {
			for class in &component.classes {
				map(class);
				for member in &class.members {
					if let Member::Class(subclass) = member {
						map(subclass)
					}
				}
			}
		}
	}

	pub fn name_for_class(&self, class_name: &str) -> String {
		format!("{}{}", self.name, class_name)
	}
}

pub(crate) fn get_modules_from(
	generated_directory: &Path,
	native_directory: &Path,
	symbol_collector: &mut SymbolCollector,
) -> Vec<Module> {
	let dirs_filtered = WalkDir::new(native_directory)
		.into_iter()
		.map(Result::unwrap)
		.filter(|entry| {
			entry.file_type().is_dir() && entry.file_name() == METHOD_DEFINITION_DIR_NAME
		});

	let mut modules = Vec::new();
	for dir in dirs_filtered {
		modules.push(Module::from_path(
			generated_directory,
			dir.path(),
			symbol_collector,
		))
	}

	let mut modules_by_root: IndexMap<String, Vec<Module>> = IndexMap::new();
	for module in modules {
		let entry = modules_by_root
			.entry(module.components[0].name.clone())
			.or_default();
		entry.push(module);
	}

	for modules in modules_by_root.values_mut() {
		let mut i = modules.len() - 1;
		loop {
			if i == 0 {
				break;
			}

			let mut to_merge = Vec::new();

			for (idx, module) in modules[..i].iter().enumerate() {
				if module.is_submodule_of(&modules[i]) {
					to_merge.push(idx)
				}
			}

			for from_idx in to_merge.into_iter().rev() {
				i -= 1;

				let from = modules.remove(from_idx);
				modules[i].merge_with(from);
			}

			i -= 1;
		}
	}

	// Need to make sure the roots are sorted so we aren't constantly making unnecessary module updates
	modules_by_root.sort_keys();

	// Flatten back to a Vec, sorted by root
	let mut modules = Vec::new();
	for (_, mods) in modules_by_root {
		modules.extend(mods);
	}

	modules
}
