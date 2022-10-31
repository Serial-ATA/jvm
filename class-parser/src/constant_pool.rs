use std::io::Read;

use classfile::traits::JavaReadExt;
use classfile::{ConstantPool, ConstantPoolTag, ConstantPoolValueInfo};

pub fn read_cp_info<R>(reader: &mut R, constant_pool: &mut ConstantPool, constant_pool_count: u16)
where
	R: Read,
{
	// The constant_pool table is indexed from 1 to constant_pool_count - 1, so
	// just make an invalid entry for index 0.
	constant_pool.push(ConstantPoolValueInfo::Unusable);

	let mut i = 0;

	while i < constant_pool_count - 1 {
		let tag = ConstantPoolTag::from(reader.read_u1());

		let cp_value_info = match tag {
			ConstantPoolTag::Class => ConstantPoolValueInfo::Class {
				name_index: reader.read_u2(),
			},
			ConstantPoolTag::FieldRef
			| ConstantPoolTag::MethodRef
			| ConstantPoolTag::InterfaceMethodref => {
				let class_index = reader.read_u2();
				let name_and_type_index = reader.read_u2();

				match tag {
					ConstantPoolTag::FieldRef => ConstantPoolValueInfo::Fieldref {
						class_index,
						name_and_type_index,
					},
					ConstantPoolTag::MethodRef => ConstantPoolValueInfo::Methodref {
						class_index,
						name_and_type_index,
					},
					ConstantPoolTag::InterfaceMethodref => {
						ConstantPoolValueInfo::InterfaceMethodref {
							class_index,
							name_and_type_index,
						}
					},
					_ => unreachable!(),
				}
			},
			ConstantPoolTag::String => ConstantPoolValueInfo::String {
				string_index: reader.read_u2(),
			},
			ConstantPoolTag::Integer => ConstantPoolValueInfo::Integer {
				bytes: reader.read_u4(),
			},
			ConstantPoolTag::Float => ConstantPoolValueInfo::Float {
				bytes: reader.read_u4(),
			},
			ConstantPoolTag::Long => ConstantPoolValueInfo::Long {
				high_bytes: reader.read_u4(),
				low_bytes: reader.read_u4(),
			},
			ConstantPoolTag::Double => ConstantPoolValueInfo::Double {
				high_bytes: reader.read_u4(),
				low_bytes: reader.read_u4(),
			},
			ConstantPoolTag::NameAndType => ConstantPoolValueInfo::NameAndType {
				name_index: reader.read_u2(),
				descriptor_index: reader.read_u2(),
			},
			ConstantPoolTag::Utf8 => {
				let length = reader.read_u2();
				let mut bytes = vec![0; length as usize];
				reader.read_exact(&mut bytes).unwrap();

				ConstantPoolValueInfo::Utf8 { length, bytes }
			},
			ConstantPoolTag::MethodHandle => ConstantPoolValueInfo::MethodHandle {
				reference_kind: reader.read_u1(),
				reference_index: reader.read_u2(),
			},
			ConstantPoolTag::MethodType => ConstantPoolValueInfo::MethodType {
				descriptor_index: reader.read_u2(),
			},
			ConstantPoolTag::InvokeDynamic => ConstantPoolValueInfo::InvokeDynamic {
				bootstrap_method_attr_index: reader.read_u2(),
				name_and_type_index: reader.read_u2(),
			},
			// Doesn't actually exist
			ConstantPoolTag::Unusable => unreachable!(),
		};

		constant_pool.push(cp_value_info);
		i += 1;

		// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.5:
		// All 8-byte constants take up two entries in the constant_pool table of the class file.
		let needs_unusable_entry = matches!(tag, ConstantPoolTag::Long | ConstantPoolTag::Double);

		if needs_unusable_entry {
			constant_pool.push(ConstantPoolValueInfo::Unusable);
			i += 1;
		}
	}
}
