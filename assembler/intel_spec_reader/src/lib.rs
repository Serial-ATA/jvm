mod page;

use crate::page::Page;
use pdf::file::{File, FileOptions, ObjectCache, StreamCache};
use std::fs::OpenOptions;
use std::path::Path;

const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub fn generate() -> bool {
	let spec_path = Path::new(CRATE_ROOT).join("vol-2abcd.pdf");
	if !spec_path.exists() {
		eprintln!("WARN: Missing instruction set reference, downloading from Intel.");

		let mut response;
		match reqwest::blocking::get("https://cdrdv2.intel.com/v1/dl/getContent/671110") {
			Ok(r) => response = r,
			Err(e) => {
				eprintln!("ERROR: Failed to download instruction set reference from Intel: {e}");
				return false;
			},
		}

		let mut pdf_file;
		match OpenOptions::new().create(true).write(true).open(&spec_path) {
			Ok(f) => pdf_file = f,
			Err(e) => {
				eprintln!("ERROR: Failed to create instruction set PDF: {e}");
				return false;
			},
		}

		if let Err(e) = response.copy_to(&mut pdf_file) {
			eprintln!("ERROR: Failed to write to instruction set PDF: {e}");
			return false;
		}
	}

	let pdf = FileOptions::cached().open(&spec_path).unwrap();
	parse(pdf)
}

fn parse(pdf: File<Vec<u8>, ObjectCache, StreamCache>) -> bool {
	for page in pdf.pages().map(Result::unwrap) {
		let Some(parsed) = Page::parse(page) else {
			return false;
		};
	}

	true
}
