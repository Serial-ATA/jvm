mod header;
pub(crate) mod index;

use crate::header::JImageHeader;

use std::io::Read;

use common::endian::Endian;
use common::int_types::u1;

pub(crate) fn parse<R>(reader: &mut R) -> (JImageHeader, Endian, Vec<u1>)
where
	R: Read,
{
	let mut jimage_bytes = Vec::new();
	reader.read_to_end(&mut jimage_bytes).unwrap();

	let reader = &mut &jimage_bytes[..];
	let (header, endian) = header::read_header(reader);

	(header, endian, jimage_bytes)
}
