use crate::ImageStrings;

use std::io::Write;
use std::ptr::read_unaligned as ptread;

use common::endian::Endian;
use common::int_types::{u1, u4, u8};

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

unsafe fn get_u8(ptr: *mut u1, endian: Endian) -> u8 {
	match endian {
		Endian::Little => {
			u8::from(ptread::<u1>(ptr))
				| u8::from(ptread::<u1>(ptr)) << 8
				| u8::from(ptread::<u1>(ptr)) << 16
				| u8::from(ptread::<u1>(ptr)) << 24
				| u8::from(ptread::<u1>(ptr)) << 32
				| u8::from(ptread::<u1>(ptr)) << 40
				| u8::from(ptread::<u1>(ptr)) << 48
				| u8::from(ptread::<u1>(ptr)) << 56
		},
		Endian::Big => {
			u8::from(ptread::<u1>(ptr)) << 56
				| u8::from(ptread::<u1>(ptr)) << 48
				| u8::from(ptread::<u1>(ptr)) << 40
				| u8::from(ptread::<u1>(ptr)) << 32
				| u8::from(ptread::<u1>(ptr)) << 24
				| u8::from(ptread::<u1>(ptr)) << 16
				| u8::from(ptread::<u1>(ptr)) << 8
				| u8::from(ptread::<u1>(ptr))
		},
	}
}

unsafe fn get_u4(ptr: *mut u1, endian: Endian) -> u4 {
	match endian {
		Endian::Little => {
			u4::from(ptread::<u1>(ptr))
				| u4::from(ptread::<u1>(ptr)) << 8
				| u4::from(ptread::<u1>(ptr)) << 16
				| u4::from(ptread::<u1>(ptr)) << 24
		},
		Endian::Big => {
			u4::from(ptread::<u1>(ptr)) << 24
				| u4::from(ptread::<u1>(ptr)) << 16
				| u4::from(ptread::<u1>(ptr)) << 8
				| u4::from(ptread::<u1>(ptr))
		},
	}
}

// https://github.com/openjdk/jdk/blob/f80faced6e6c6c1b10541a8b0c91625215c9ef43/src/java.base/share/native/libjimage/imageDecompressor.cpp#L136
/// Decompression entry point. Called from ImageFileReader::get_resource.
pub fn decompress_resource(
	compressed: &mut [u1],
	mut uncompressed: &mut [u1],
	uncompressed_size: u8,
	strings: ImageStrings<'_>,
	endian: Endian,
) {
	let mut has_header;

	let mut resource = compressed.as_mut_ptr();

	// Resource could have been transformed by a stack of decompressors.
	// Iterate and decompress resources until there is no more header.
	loop {
		let magic = unsafe { get_u4(resource, endian) };
		let size = unsafe { get_u8(resource, endian) };
		let uncompressed_size = unsafe { get_u8(resource, endian) };
		let decompressor_name_offset = unsafe { get_u4(resource, endian) };
		let decompressor_config_offset = unsafe { get_u4(resource, endian) };
		let is_terminal = unsafe { ptread::<u1>(resource) };

		let header = ResourceHeader {
			__magic: magic,
			size,
			uncompressed_size,
			decompressor_name_offset,
			decompressor_config_offset,
			is_terminal,
		};

		has_header = header.__magic == ResourceHeader::RESOURCE_HEADER_MAGIC;
		if has_header {
			// decompressed_resource array contains the result of decompression
			let decompressed_resource =
				vec![0; header.uncompressed_size.try_into().unwrap()].into_boxed_slice();

			// We need to reconstruct our box and drop it in the next iteration
			let decompressed_resource = Box::leak(decompressed_resource);

			// Retrieve the decompressor name
			let decompressor_name = strings.get(header.decompressor_name_offset);
			assert!(
				!decompressor_name.is_empty(),
				"image decompressor not found"
			);

			// Retrieve the decompressor instance
			// Ask the decompressor to decompress the compressed content
			match decompressor_name {
				b"zip" => decompress_zip(resource, decompressed_resource, &header, strings),
				b"compact-cp" => {
					decompress_string(resource, decompressed_resource, &header, strings)
				},
				_ => panic!(
					"image decompressor not found: {}",
					std::str::from_utf8(decompressor_name).unwrap()
				),
			}

			// Drop the previous iteration's decompressed contents
			unsafe {
				let _ = Box::from_raw(resource);
			}

			// Preserve this iteration's decompressed contents for the next round
			resource = decompressed_resource.as_mut_ptr();
			continue;
		}

		break;
	}

	// Now we can write the resource to our uncompressed buffer
	uncompressed
		.write_all(unsafe {
			std::slice::from_raw_parts(resource, uncompressed_size.try_into().unwrap())
		})
		.unwrap();
}

fn decompress_zip(
	_compressed: *mut u1,
	_uncompressed: &mut [u1],
	_header: &ResourceHeader,
	_strings: ImageStrings<'_>,
) {
	unimplemented!("zip decompression")
}

fn decompress_string(
	_compressed: *mut u1,
	_uncompressed: &mut [u1],
	_header: &ResourceHeader,
	_strings: ImageStrings<'_>,
) {
	unimplemented!("string decompression")
}
