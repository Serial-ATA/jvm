use super::Location;
use crate::{AttributeType, StackMapFrame, VerificationTypeInfo};

use std::io::Read;

use common::int_types::u2;
use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[Location::Code];

pub fn read<R>(reader: &mut R, location: Location) -> AttributeType
where
	R: Read,
{
	location.verify_valid(VALID_LOCATIONS);

	let number_of_entries = reader.read_u2();
	let mut entries = Vec::with_capacity(number_of_entries as usize);

	for _ in 0..number_of_entries {
		let frame_type = reader.read_u1();

		let stack_map_frame = match frame_type {
			0..=63 => StackMapFrame::SameFrame {
				offset_delta: u2::from(frame_type),
			},
			64..=127 => StackMapFrame::SameLocals1StackItemFrame {
				offset_delta: u2::from(frame_type - 64),
				verification_type_info: [read_attribute_verification_type_info(reader)],
			},
			247 => StackMapFrame::SameLocals1StackItemFrameExtended {
				offset_delta: reader.read_u2(),
				verification_type_info: [read_attribute_verification_type_info(reader)],
			},
			248..=250 => StackMapFrame::ChopFrame {
				offset_delta: reader.read_u2(),
			},
			251 => StackMapFrame::SameFrameExtended {
				offset_delta: reader.read_u2(),
			},
			252..=254 => {
				let offset_delta = reader.read_u2();

				let locals_len = frame_type - 251;
				let mut locals = Vec::with_capacity(locals_len as usize);

				for _ in 0..locals_len {
					locals.push(read_attribute_verification_type_info(reader));
				}

				StackMapFrame::AppendFrame {
					offset_delta,
					locals,
				}
			},
			255 => {
				let offset_delta = reader.read_u2();

				let number_of_locals = reader.read_u2();
				let mut locals = Vec::with_capacity(number_of_locals as usize);

				for _ in 0..number_of_locals {
					locals.push(read_attribute_verification_type_info(reader));
				}

				let number_of_stack_items = reader.read_u2();
				let mut stack = Vec::with_capacity(number_of_stack_items as usize);

				for _ in 0..number_of_stack_items {
					stack.push(read_attribute_verification_type_info(reader));
				}

				StackMapFrame::FullFrame {
					offset_delta,
					locals,
					stack,
				}
			},
			_ => unreachable!(),
		};

		entries.push(stack_map_frame);
	}

	AttributeType::StackMapTable { entries }
}

fn read_attribute_verification_type_info<R>(reader: &mut R) -> VerificationTypeInfo
where
	R: Read,
{
	let tag = reader.read_u1();

	match tag {
		0 => VerificationTypeInfo::TopVariableInfo,
		1 => VerificationTypeInfo::IntegerVariableInfo,
		2 => VerificationTypeInfo::FloatVariableInfo,
		4 => VerificationTypeInfo::LongVariableInfo,
		3 => VerificationTypeInfo::DoubleVariableInfo,
		5 => VerificationTypeInfo::NullVariableInfo,
		6 => VerificationTypeInfo::UninitializedThisVariableInfo,
		7 => VerificationTypeInfo::ObjectVariableInfo {
			cpool_index: reader.read_u2(),
		},
		8 => VerificationTypeInfo::UninitializedVariableInfo {
			offset: reader.read_u2(),
		},
		_ => panic!("Encountered invalid verification type info tag"),
	}
}
