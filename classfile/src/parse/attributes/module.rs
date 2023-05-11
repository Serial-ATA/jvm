use super::Location;
use crate::error::Result;
use crate::{AttributeTag, AttributeType, ModuleExport, ModuleOpen, ModuleProvide, ModuleRequire};

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[Location::ClassFile];

pub fn read<R>(reader: &mut R, location: Location) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::Module, VALID_LOCATIONS)?;

	let module_name_index = reader.read_u2()?;
	let module_flags = reader.read_u2()?;
	let module_version_index = reader.read_u2()?;

	let requires_count = reader.read_u2()?;
	let mut requires = Vec::with_capacity(requires_count as usize);
	for _ in 0..requires_count {
		let requires_index = reader.read_u2()?;
		let requires_flags = reader.read_u2()?;
		let requires_version_index = reader.read_u2()?;

		requires.push(ModuleRequire {
			requires_index,
			requires_flags,
			requires_version_index,
		});
	}

	let exports_count = reader.read_u2()?;
	let mut exports = Vec::with_capacity(exports_count as usize);
	for _ in 0..exports_count {
		let exports_index = reader.read_u2()?;
		let exports_flags = reader.read_u2()?;

		let exports_to_count = reader.read_u2()?;
		let mut exports_to_index = Vec::with_capacity(exports_to_count as usize);
		for _ in 0..exports_to_count {
			exports_to_index.push(reader.read_u2()?);
		}

		exports.push(ModuleExport {
			exports_index,
			exports_flags,
			exports_to_index,
		});
	}

	let opens_count = reader.read_u2()?;
	let mut opens = Vec::with_capacity(opens_count as usize);
	for _ in 0..opens_count {
		let opens_index = reader.read_u2()?;
		let opens_flags = reader.read_u2()?;

		let opens_to_count = reader.read_u2()?;
		let mut opens_to_index = Vec::with_capacity(opens_to_count as usize);
		for _ in 0..opens_to_count {
			opens_to_index.push(reader.read_u2()?);
		}

		opens.push(ModuleOpen {
			opens_index,
			opens_flags,
			opens_to_index,
		});
	}

	let uses_count = reader.read_u2()?;
	let mut uses_index = Vec::with_capacity(uses_count as usize);
	for _ in 0..uses_count {
		uses_index.push(reader.read_u2()?);
	}

	let provides_count = reader.read_u2()?;
	let mut provides = Vec::with_capacity(provides_count as usize);
	for _ in 0..provides_count {
		let provides_index = reader.read_u2()?;

		let provides_with_count = reader.read_u2()?;
		let mut provides_with_index = Vec::with_capacity(provides_with_count as usize);
		for _ in 0..provides_with_count {
			provides_with_index.push(reader.read_u2()?);
		}

		provides.push(ModuleProvide {
			provides_index,
			provides_with_index,
		});
	}

	Ok(AttributeType::Module {
		module_name_index,
		module_flags,
		module_version_index,
		requires,
		exports,
		opens,
		uses_index,
		provides,
	})
}

pub fn read_packages<R>(reader: &mut R, location: Location) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::ModulePackages, VALID_LOCATIONS)?;

	let package_count = reader.read_u2()?;
	let mut package_index = Vec::with_capacity(package_count as usize);
	for _ in 0..package_count {
		package_index.push(reader.read_u2()?);
	}

	Ok(AttributeType::ModulePackages { package_index })
}

pub fn read_main_class<R>(reader: &mut R, location: Location) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::ModuleMainClass, VALID_LOCATIONS)?;
	Ok(AttributeType::ModuleMainClass {
		main_class_index: reader.read_u2()?,
	})
}
