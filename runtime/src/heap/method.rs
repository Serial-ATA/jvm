use super::reference::ClassRef;

use classfile::traits::PtrType;
use classfile::types::{u1, u2};
use classfile::{Code, MethodDescriptor, MethodInfo};

#[derive(Debug, Clone, PartialEq)]
pub struct Method {
	pub class: ClassRef,
	pub access_flags: u2,
	pub name: Vec<u1>,
	pub descriptor: MethodDescriptor,
	pub code: Code,
}

impl Method {
	pub fn new(class: ClassRef, method_info: &MethodInfo) -> Self {
		let constant_pool = &class.get().constant_pool;

		let access_flags = method_info.access_flags;

		let name_index = method_info.name_index;
		let name = constant_pool.get_constant_utf8(name_index).to_vec();

		let descriptor_index = method_info.descriptor_index;
		let mut descriptor_bytes = constant_pool.get_constant_utf8(descriptor_index);

		let descriptor = MethodDescriptor::parse(&mut descriptor_bytes);

		let code = method_info.get_code_attribute().unwrap_or_default();

		Self {
			class,
			access_flags,
			name,
			descriptor,
			code,
		}
	}
}
