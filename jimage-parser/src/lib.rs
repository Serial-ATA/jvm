mod header;
mod index;

use std::io::Read;

use common::int_types::u1;
use jimage::{JImage, JImageBuilder};

pub fn parse<R>(reader: &mut R) -> JImage
where
	R: Read,
{
	let mut jimage_bytes = Vec::new();
	reader.read_to_end(&mut jimage_bytes).unwrap();

	let reader = &mut &jimage_bytes[..];
	let (header, endian) = header::read_header(reader);

	JImageBuilder {
		endian,
		data: jimage_bytes,
		header,
		index_builder: |data: &Vec<u1>| index::read_index(data.as_slice(), header, endian),
	}
	.build()
}
