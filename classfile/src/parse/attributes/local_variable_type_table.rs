use super::Location;
use crate::attribute::{AttributeTag, AttributeType, LocalVariableType, LocalVariableTypeTable};
use crate::error::Result;

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[Location::Code];

pub fn read<R>(reader: &mut R, location: Location) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::LocalVariableTypeTable, VALID_LOCATIONS)?;

	let local_variable_table_length = reader.read_u2()?;
	let mut local_variable_type_table = Vec::with_capacity(local_variable_table_length as usize);

	for _ in 0..local_variable_table_length {
		local_variable_type_table.push(LocalVariableType {
			start_pc: reader.read_u2()?,
			length: reader.read_u2()?,
			name_index: reader.read_u2()?,
			signature_index: reader.read_u2()?,
			index: reader.read_u2()?,
		})
	}

	Ok(AttributeType::LocalVariableTypeTable(
		LocalVariableTypeTable {
			local_variable_type_table,
		},
	))
}
