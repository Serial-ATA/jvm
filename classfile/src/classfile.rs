use crate::attribute::Attribute;
use crate::constant_pool::ConstantPool;
use crate::fieldinfo::FieldInfo;
use crate::methodinfo::MethodInfo;
use crate::types::u2;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.1
#[derive(Debug, Clone, PartialEq)]
pub struct ClassFile {
	pub minor_version: u2,
	pub major_version: u2,
	pub constant_pool: ConstantPool,
	pub access_flags: u2,
	pub this_class: u2,
	pub super_class: u2,
	pub interfaces: Vec<u2>,
	pub fields: Vec<FieldInfo>,
	pub methods: Vec<MethodInfo>,
	pub attributes: Vec<Attribute>,
}
