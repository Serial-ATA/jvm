mod header;
pub(crate) mod index;
mod tests;

use crate::error::Result;
use crate::header::JImageHeader;

use std::io::Read;

use common::endian::Endian;
use common::int_types::u1;

pub(crate) fn parse<R>(reader: &mut R) -> Result<(JImageHeader, Endian, Vec<u1>)>
where
	R: Read,
{
	let mut jimage_bytes = Vec::new();
	reader.read_to_end(&mut jimage_bytes)?;

	let reader = &mut &jimage_bytes[..];
	let (header, endian) = header::read_header(reader)?;

	Ok((header, endian, jimage_bytes))
}
