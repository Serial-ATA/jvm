mod attribute;
mod constant_pool;
mod fieldinfo;
mod methodinfo;

use std::io::{Read, Seek};

use classfile::{u1, u2, u4, ClassFile, ConstantPool};

pub fn parse_class<R>(reader: &mut R) -> ClassFile
where
	R: Read + Seek,
{
	let magic = reader.read_u4();
	assert_eq!(magic, 0xCAFE_BABE, "No magic found!");

	let minor_version = reader.read_u2();
	let major_version = reader.read_u2();

	let constant_pool_count = reader.read_u2().saturating_sub(1);
	let mut constant_pool = ConstantPool::with_capacity(constant_pool_count as usize);

	for _ in 0..constant_pool_count {
		constant_pool.push(constant_pool::read_cp_info(reader));
	}

	let access_flags = reader.read_u2();
	let this_class = reader.read_u2();
	let super_class = reader.read_u2();

	let interfaces_count = reader.read_u2();
	let mut interfaces = Vec::<u2>::with_capacity(interfaces_count as usize);

	for _ in 0..interfaces_count {
		interfaces.push(reader.read_u2());
	}

	let fields_count = reader.read_u2();
	let mut fields = Vec::with_capacity(fields_count as usize);

	for _ in 0..fields_count {
		fields.push(fieldinfo::read_field_info(reader, &constant_pool));
	}

	let methods_count = reader.read_u2();
	let mut methods = Vec::with_capacity(methods_count as usize);

	for _ in 0..methods_count {
		methods.push(methodinfo::read_method_info(reader, &constant_pool));
	}

	let attributes_count = reader.read_u2();
	let mut attributes = Vec::with_capacity(attributes_count as usize);

	for _ in 0..attributes_count {
		attributes.push(attribute::read_attribute(reader, &constant_pool));
	}

	ClassFile {
		minor_version,
		major_version,
		constant_pool,
		access_flags,
		this_class,
		super_class,
		interfaces,
		fields,
		methods,
		attributes,
	}
}

pub trait JavaReadExt: Read {
	fn read_u1(&mut self) -> u1 {
		let mut buf = [0u8; 1];
		self.read_exact(&mut buf).unwrap();
		buf[0]
	}

	fn read_u2(&mut self) -> u2 {
		let mut buf = [0u8; 2];
		self.read_exact(&mut buf).unwrap();
		u16::from_be_bytes(buf)
	}

	fn read_u4(&mut self) -> u4 {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf).unwrap();
		u32::from_be_bytes(buf)
	}
}

impl<R: Read> JavaReadExt for R {}
