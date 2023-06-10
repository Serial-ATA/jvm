use super::Location;
use crate::attribute::BootstrapMethods;
use crate::error::Result;
use crate::{AttributeTag, AttributeType, BootstrapMethod};

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[Location::ClassFile];

pub fn read<R>(reader: &mut R, location: Location) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::BootstrapMethods, VALID_LOCATIONS)?;

	let num_bootstrap_methods = reader.read_u2()?;
	let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);

	for _ in 0..num_bootstrap_methods {
		let bootstrap_method_ref = reader.read_u2()?;

		let num_bootstrap_arguments = reader.read_u2()?;
		let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments as usize);

		for _ in 0..num_bootstrap_arguments {
			bootstrap_arguments.push(reader.read_u2()?);
		}

		bootstrap_methods.push(BootstrapMethod {
			bootstrap_method_ref,
			bootstrap_arguments,
		})
	}

	Ok(AttributeType::BootstrapMethods(BootstrapMethods {
		bootstrap_methods,
	}))
}
