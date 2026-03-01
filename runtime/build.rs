#![feature(result_option_map_or_default)]

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use syn::Type;

fn main() {
	println!("cargo::rerun-if-changed=../generators/native_methods/");
	println!("cargo::rerun-if-changed=../generated");
	build_deps::rerun_if_changed_paths("src/native/**/*.def").unwrap();

	if let Err(e) = native_methods::generate() {
		println!("cargo::error=Failed to generate native methods: {e}");
		std::process::exit(1);
	}

	collect_jvm_h()
}

/// Compares the function definitions in `./vm_functions.txt` to the ones defined in `./src/native/jvm`.
///
/// The actual generation is handled by `../scripts/jvm_h.py`.
fn collect_jvm_h() {
	fn collect_files(src_dir: &Path) -> std::io::Result<Vec<PathBuf>> {
		let mut ret = Vec::new();
		for entry in std::fs::read_dir(src_dir)? {
			let entry = entry?;

			let path = entry.path();
			if path.is_dir() {
				ret.append(&mut collect_files(&path)?);
				return Ok(ret);
			}

			if path.extension().and_then(OsStr::to_str) != Some("rs") {
				continue;
			}

			ret.push(path);
		}

		Ok(ret)
	}

	#[derive(Debug, PartialEq)]
	struct JniFunction {
		name: String,
		parameters: Vec<String>,
		return_type: Option<String>,
	}

	impl Display for JniFunction {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			let ret_ty = self
				.return_type
				.as_ref()
				.map_or_default(|ret| format!(" -> {ret}"));
			write!(f, "{}({}){ret_ty}", self.name, self.parameters.join(", "))
		}
	}

	let vm_functions = Path::new(env!("CARGO_MANIFEST_DIR")).join("vm_functions.txt");
	let jvm_h_py = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("..")
		.join("scripts")
		.join("jvm_h.py");

	println!("cargo:rerun-if-changed={}", vm_functions.display());
	println!("cargo:rerun-if-changed={}", jvm_h_py.display());
	if !vm_functions.exists() {
		panic!(
			"Expected VM function list at `{}`. Regenerate it with `{}`",
			vm_functions.display(),
			jvm_h_py.canonicalize().unwrap().display()
		);
	}

	let expected_vm_functions =
		std::fs::read_to_string(&vm_functions).expect("failed to read VM functions");

	let src_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("src")
		.join("native")
		.join("jvm");
	println!("cargo:rerun-if-changed={}", src_dir.display());

	let mut defined_jvm_functions = HashMap::new();
	for file in collect_files(&src_dir).unwrap() {
		let content = std::fs::read_to_string(&file).expect("Failed to read file");

		let file = match syn::parse_file(&content) {
			Ok(file) => file,
			Err(e) => {
				panic!("Failed to parse file at '{}': {e}", file.display());
			},
		};

		for item in file.items {
			fn encode_type(ty: &Type) -> String {
				match ty {
					Type::Ptr(ty) => {
						let mutability = if ty.const_token.is_some() {
							"const"
						} else {
							"mut"
						};
						let elem = &ty.elem;
						format!("*{mutability} {}", quote::quote!(#elem))
					},
					_ => quote::quote!(#ty).to_string(),
				}
			}

			let syn::Item::Fn(item_fn) = item else {
				continue;
			};

			let is_jni_call = item_fn
				.attrs
				.iter()
				.any(|attr| attr.meta.path().is_ident("jni_call"));

			if !is_jni_call {
				continue;
			}

			let name = item_fn.sig.ident.to_string();
			let mut parameters = Vec::new();

			for input in item_fn.sig.inputs.iter() {
				if let syn::FnArg::Typed(pat_type) = input {
					parameters.push(encode_type(&pat_type.ty));
				}
			}

			let return_type = match &item_fn.sig.output {
				syn::ReturnType::Default => None,
				syn::ReturnType::Type(_, ty) => Some(encode_type(ty)),
			};

			let existing = defined_jvm_functions.insert(
				name.clone(),
				JniFunction {
					name,
					parameters,
					return_type,
				},
			);

			assert!(existing.is_none());
		}
	}

	for line in expected_vm_functions.lines() {
		if line.starts_with('#') {
			continue;
		}

		let mut parts = line.split('\t');
		let name = parts.next().unwrap();
		let params = parts.next().unwrap();
		let return_type = parts
			.next()
			.map(|ret| ret.strip_prefix("-> ").unwrap().to_string());

		let expected = JniFunction {
			name: name.to_string(),
			parameters: params
				.split(',')
				.filter(|p| !p.is_empty())
				.map(ToString::to_string)
				.collect(),
			return_type,
		};

		let Some(defined) = defined_jvm_functions.get(&*name) else {
			println!("cargo::warning=JVM function {name} not found",);
			continue;
		};

		if &expected != defined {
			println!(
				"cargo::warning=JVM function {} has the wrong signature. (found: {defined}, \
				 expected: {expected})",
				defined.name
			);
		}
	}
}
