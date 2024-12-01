mod header;
pub(crate) mod index;
#[cfg(test)]
mod tests;

use crate::error::{Error, Result};
use crate::image::JImage;
use crate::JImageHeader;

use std::io::Read;

pub(crate) fn parse<R>(reader: &mut R) -> Result<JImage>
where
	R: Read,
{
	let (header, endian) = header::read_header(reader)?;
	let index = index::read_index(reader, header, endian)?;

	if header.index_length() < size_of::<JImageHeader>() {
		return Err(Error::InvalidTableSize);
	}

	// `index_length()` contains the length of the header as well for some reason, need to subtract it.
	// Copying this behavior for parity with OpenJDK.
	if index.len() != (header.index_length() - size_of::<JImageHeader>()) {
		return Err(Error::BadIndexSize);
	}

	// Everything left is the resources section
	let mut resources = Vec::new();
	reader.read_to_end(&mut resources)?;

	Ok(JImage {
		endian,
		header,
		index,
		resources: resources.into(),
	})
}
