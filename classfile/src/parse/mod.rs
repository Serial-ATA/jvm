mod attributes;
mod constant_pool;
pub mod error;
mod fieldinfo;
mod methodinfo;

use crate::accessflags::ClassAccessFlags;
use crate::classfile::ClassFile;
use crate::constant_pool::ConstantPool;
use crate::parse::attributes::Location;
use crate::parse::error::Result;

use std::io::Read;

use common::int_types::u2;
use common::traits::JavaReadExt;

pub fn parse_class<R>(reader: &mut R) -> Result<ClassFile>
where
	R: Read,
{
	let magic = reader.read_u4()?;
	assert_eq!(magic, 0xCAFE_BABE, "No magic found!");

	let minor_version = reader.read_u2()?;
	let major_version = reader.read_u2()?;

	let constant_pool_count = reader.read_u2()?;
	let mut constant_pool = ConstantPool::with_capacity(constant_pool_count as usize);

	constant_pool::read_cp_info(reader, &mut constant_pool, constant_pool_count)?;

	let access_flags = reader.read_u2()?;
	let this_class = reader.read_u2()?;
	let super_class = reader.read_u2()?;

	let interfaces_count = reader.read_u2()?;
	let mut interfaces = Vec::<u2>::with_capacity(interfaces_count as usize);

	for _ in 0..interfaces_count {
		interfaces.push(reader.read_u2()?);
	}

	let fields_count = reader.read_u2()?;
	let mut fields = Vec::with_capacity(fields_count as usize);

	for _ in 0..fields_count {
		fields.push(fieldinfo::read_field_info(reader, &constant_pool)?);
	}

	let methods_count = reader.read_u2()?;
	let mut methods = Vec::with_capacity(methods_count as usize);

	for _ in 0..methods_count {
		methods.push(methodinfo::read_method_info(reader, &constant_pool)?);
	}

	let attributes_count = reader.read_u2()?;
	let mut attributes = Vec::with_capacity(attributes_count as usize);

	for _ in 0..attributes_count {
		attributes.push(attributes::read_attribute(
			reader,
			&constant_pool,
			Location::ClassFile,
		)?);
	}

	Ok(ClassFile {
		minor_version,
		major_version,
		constant_pool,
		access_flags: ClassAccessFlags::from(access_flags),
		this_class,
		super_class,
		interfaces,
		fields,
		methods,
		attributes,
	})
}
