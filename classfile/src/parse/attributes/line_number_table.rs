use super::Location;
use crate::{AttributeType, LineNumber};

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[Location::Code];

pub fn read<R>(reader: &mut R, location: Location) -> AttributeType
where
	R: Read,
{
	location.verify_valid(VALID_LOCATIONS);

	let line_number_table_length = reader.read_u2();
	let mut line_number_table = Vec::with_capacity(line_number_table_length as usize);

	for _ in 0..line_number_table_length {
		line_number_table.push(LineNumber {
			start_pc: reader.read_u2(),
			line_number: reader.read_u2(),
		})
	}

	AttributeType::LineNumberTable { line_number_table }
}
