use classfile::{Code, ConstantPool, MethodDescriptor, MethodInfo};
use common::types::{u1, u2};

#[derive(Debug, Clone, PartialEq)]
pub struct Method {
	pub access_flags: u2,
	pub name: Vec<u1>,
	pub descriptor: MethodDescriptor,
	pub code: Option<Code>,
}

impl Method {
	pub fn new(method_info: &MethodInfo, constant_pool: &ConstantPool) -> Self {
		let access_flags = method_info.access_flags;

		let name_index = method_info.name_index;
		let name = constant_pool.get_class_name(name_index).to_vec();

		let descriptor_index = method_info.descriptor_index;
		let mut descriptor_bytes = constant_pool.get_constant_utf8(descriptor_index);

		let descriptor = MethodDescriptor::parse(&mut descriptor_bytes);

		let code = method_info.get_code_attribute();

		Self {
			access_flags,
			name,
			descriptor,
			code,
		}
	}
}
