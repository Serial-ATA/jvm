use super::{read_attribute, Location};
use crate::{AttributeType, Code, CodeException, ConstantPool};

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[Location::MethodInfo];

pub fn read<R>(reader: &mut R, constant_pool: &ConstantPool, location: Location) -> AttributeType
where
	R: Read,
{
	location.verify_valid(VALID_LOCATIONS);

	let max_stack = reader.read_u2();
	let max_locals = reader.read_u2();

	let code_length = reader.read_u4();
	let mut code = vec![0; code_length as usize];
	reader.read_exact(&mut code).unwrap();

	let exception_table_length = reader.read_u2();
	let mut exception_table = Vec::with_capacity(exception_table_length as usize);

	for _ in 0..exception_table_length {
		exception_table.push(CodeException {
			start_pc: reader.read_u2(),
			end_pc: reader.read_u2(),
			handler_pc: reader.read_u2(),
			catch_type: reader.read_u2(),
		})
	}

	let attributes_count = reader.read_u2();
	let mut attributes = Vec::with_capacity(attributes_count as usize);

	for _ in 0..attributes_count {
		attributes.push(read_attribute(reader, constant_pool, Location::Code));
	}

	AttributeType::Code(Code {
		max_stack,
		max_locals,
		code,
		exception_table,
		attributes,
	})
}
