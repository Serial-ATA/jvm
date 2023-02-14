mod error;

use crate::error::{JmodError, Result};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

use common::int_types::u1;
use zip::read::ZipFile;
use zip::ZipArchive;

const MAJOR_VERSION: u1 = 1;
const MINOR_VERSION: u1 = 0;

#[rustfmt::skip]
const JMOD_MAGIC: [u8; 4] = [
    b'J', b'M',
    MAJOR_VERSION, MINOR_VERSION
];

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Section {
	Classes,
	Config,
	HeaderFiles,
	LegalNotices,
	ManPages,
	NativeLibs,
	NativeCmds,
}

impl Section {
	/// Returns the directory name in the JMOD file corresponding to this section
	pub fn as_directory_name(self) -> &'static str {
		match self {
			Section::Classes => "classes",
			Section::Config => "conf",
			Section::HeaderFiles => "include",
			Section::LegalNotices => "legal",
			Section::ManPages => "man",
			Section::NativeLibs => "lib",
			Section::NativeCmds => "bin",
		}
	}
}

impl FromStr for Section {
	type Err = JmodError;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		match s {
			"classes" => Ok(Section::Classes),
			"conf" => Ok(Section::Config),
			"include" => Ok(Section::HeaderFiles),
			"legal" => Ok(Section::LegalNotices),
			"man" => Ok(Section::ManPages),
			"lib" => Ok(Section::NativeLibs),
			"bin" => Ok(Section::NativeCmds),
			_ => Err(JmodError::InvalidSectionName),
		}
	}
}

pub struct JmodEntry<'a> {
	zip_entry: ZipFile<'a>,
	section: Section,
	name_start: usize,
}

impl<'a> JmodEntry<'a> {
	fn new(entry: ZipFile<'a>, section: Option<Section>) -> Self {
		let slash_idx = entry
			.name()
			.find('/')
			.expect("JMOD entry name should have a '/'");

		let section = match section {
			Some(section) => section,
			None => Section::from_str(&entry.name()[..slash_idx]).unwrap(),
		};

		Self {
			zip_entry: entry,
			section,
			name_start: slash_idx + 1,
		}
	}

	/// Returns the contents of the entry
	///
	/// # Errors
	///
	/// * [`Read::read_exact`]
	pub fn content(&mut self) -> Result<Vec<u8>> {
		let size = self.zip_entry.size();
		let mut content = vec![0; size as usize];

		self.zip_entry.read_exact(&mut content)?;
		Ok(content)
	}

	/// Returns the section of this entry
	pub fn section(&self) -> Section {
		self.section
	}

	/// Returns the name of this entry
	pub fn name(&self) -> &str {
		&self.zip_entry.name()[self.name_start..]
	}

	/// Returns the name of the entry including the section directory name
	pub fn path(&self) -> &str {
		self.zip_entry.name()
	}

	/// Whether the entry is a directory in the JMOD file
	pub fn is_directory(&self) -> bool {
		self.zip_entry.is_dir()
	}

	/// Returns the size of this entry
	pub fn size(&self) -> u64 {
		self.zip_entry.size()
	}
}

pub struct JmodFile(ZipArchive<File>);

impl JmodFile {
	/// Read a [`JmodFile`] from a given path
	///
	/// # Errors
	///
	/// * [`File::open`]
	/// * JMOD file has no magic signature
	/// * JMOD file has an unsupported version
	/// * [`ZipArchive::new`]
	pub fn read_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
		let mut file = File::open(path)?;

		let mut magic_buf = [0; 4];
		file.read_exact(&mut magic_buf)
			.expect("Not enough bytes to read file magic");

		if JMOD_MAGIC[..2] != magic_buf[..2] {
			return Err(JmodError::MissingMagic);
		}

		if JMOD_MAGIC[2..] != magic_buf[2..] {
			return Err(JmodError::BadVersion(magic_buf[2], magic_buf[3]));
		}

		Ok(Self(ZipArchive::new(file)?))
	}

	/// Gets the entry for a resource in a JMOD file section
	pub fn get_entry<N: AsRef<str>>(&mut self, section: Section, name: N) -> Option<JmodEntry<'_>> {
		let entry_path = format!("{}/{}", section.as_directory_name(), name.as_ref());
		let entry = self.0.by_name(&entry_path).ok()?;
		Some(JmodEntry::new(entry, Some(section)))
	}

	/// Iterate each entry in the JMOD file, and perform an action on them
	pub fn for_each_entry<F>(&mut self, mut map: F)
	where
		F: FnMut(JmodEntry<'_>),
	{
		for i in 0..self.0.len() {
			let file_entry = self.0.by_index(i).unwrap();
			map(JmodEntry::new(file_entry, None))
		}
	}
}
