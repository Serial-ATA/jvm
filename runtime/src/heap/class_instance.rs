use crate::reference::{ArrayInstanceRef, ClassInstanceRef, ClassRef};
use crate::stack::operand_stack::Operand;

use common::int_types::{s8, u1, u2, u4};

#[derive(Debug, Clone, PartialEq)]
pub struct ClassInstance {
	pub class: ClassRef,
	pub fields: Box<[Operand]>,
}

impl ClassInstance {
	pub fn new(class: ClassRef) -> ClassInstanceRef {
		let class_instance = class.unwrap_class_instance();
		let field_count = class_instance.instance_field_count;

		let fields = vec![Operand::Empty; field_count as usize].into_boxed_slice();

		ClassInstanceRef::new(Self { class, fields })
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayInstance {
	class: ClassRef,
	elements: ArrayContent,
}

impl ArrayInstance {
	pub fn new(class: ClassRef, elements: ArrayContent) -> ArrayInstanceRef {
		ArrayInstanceRef::new(Self { class, elements })
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrayContent {
	Byte(Box<[u1]>),
	Bool(Box<[u1]>),
	Short(Box<[u2]>),
	Char(Box<[u2]>),
	Int(Box<[u4]>),
	Float(Box<[f32]>),
	Double(Box<[f64]>),
	Long(Box<[s8]>),
}
