use crate::ImageStrings;
use crate::error::{Error, Result};

use std::io::Write;

use common::endian::Endian;
use common::int_types::{u1, u4, u8};
use common::traits::JavaEndianAwareRead;

pub struct ResourceHeader {
	pub(crate) __magic: u4,
	pub size: u8,
	pub uncompressed_size: u8,
	pub decompressor_name_offset: u4,
	pub decompressor_config_offset: u4,
	pub is_terminal: u1,
}

impl ResourceHeader {
	pub const RESOURCE_HEADER_MAGIC: u4 = 0xCAFE_FAFA;
}

// https://github.com/openjdk/jdk/blob/f80faced6e6c6c1b10541a8b0c91625215c9ef43/src/java.base/share/native/libjimage/imageDecompressor.cpp#L136
/// Decompression entry point. Called from [`ImageFileReader::get_resource`].
#[allow(clippy::size_of_in_element_count)]
pub fn decompress_resource(
	compressed: &mut &[u1],
	mut uncompressed: &mut [u1],
	uncompressed_size: u8,
	strings: ImageStrings<'_>,
	endian: Endian,
) -> Result<()> {
	let mut has_header;

	let mut resource = compressed.as_ptr();

	// Resource could have been transformed by a stack of decompressors.
	// Iterate and decompress resources until there is no more header.
	loop {
		let magic = endian.read_u4(compressed)?;
		let size = endian.read_u8(compressed)?;
		let uncompressed_size = endian.read_u8(compressed)?;
		let decompressor_name_offset = endian.read_u4(compressed)?;
		let decompressor_config_offset = endian.read_u4(compressed)?;
		let is_terminal = endian.read_u1(compressed)?;

		let header = ResourceHeader {
			__magic: magic,
			size,
			uncompressed_size,
			decompressor_name_offset,
			decompressor_config_offset,
			is_terminal,
		};

		resource = unsafe { resource.add(size_of::<ResourceHeader>()) };

		has_header = header.__magic == ResourceHeader::RESOURCE_HEADER_MAGIC;
		if !has_header {
			resource = unsafe { resource.sub(size_of::<ResourceHeader>()) };
			break;
		}

		// Retrieve the decompressor name
		let decompressor_name = strings.get(header.decompressor_name_offset);

		// Retrieve the decompressor instance
		// Ask the decompressor to decompress the compressed content
		let decompressed_resource;
		match decompressor_name {
			b"zip" => decompressed_resource = decompress_zip(compressed, &header, strings),
			b"compact-cp" => {
				decompressed_resource = decompress_string(compressed, &header, strings)
			},
			_ => {
				return Err(Error::DecompressorNotFound(
					String::from_utf8_lossy(decompressor_name).into_owned(),
				));
			},
		}

		// We need to reconstruct our box and drop it in the next iteration
		let decompressed_resource = Box::leak(decompressed_resource);

		// Drop the previous iteration's decompressed contents
		unsafe {
			let _ = Box::from_raw(resource as *mut u1);
		}

		// Preserve this iteration's decompressed contents for the next round
		resource = decompressed_resource.as_mut_ptr();
	}

	// Now we can write the resource to our uncompressed buffer
	uncompressed.write_all(unsafe {
		std::slice::from_raw_parts(resource, uncompressed_size.try_into().unwrap())
	})?;

	Ok(())
}

fn decompress_zip(
	_compressed: &mut &[u1],
	_header: &ResourceHeader,
	_strings: ImageStrings<'_>,
) -> Box<[u1]> {
	unimplemented!("zip decompression")
}

fn decompress_string(
	_compressed: &mut &[u1],
	_header: &ResourceHeader,
	_strings: ImageStrings<'_>,
) -> Box<[u1]> {
	unimplemented!("string decompression")
}
