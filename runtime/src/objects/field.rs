use super::reference::Reference;
use crate::objects::class::Class;
use crate::objects::constant_pool::{cp_types, ConstantPool};

use std::fmt::{Debug, Formatter};

use classfile::accessflags::FieldAccessFlags;
use classfile::{FieldInfo, FieldType};
use common::int_types::u2;
use instructions::Operand;
use symbols::Symbol;

// TODO: Make more fields private
#[derive(Clone, PartialEq)]
pub struct Field {
	pub idx: usize, // Used to set the value on `ClassInstance`s
	pub class: &'static Class,
	pub access_flags: FieldAccessFlags,
	pub name: Symbol,
	pub descriptor: FieldType,
	pub constant_value_index: Option<u2>,
	// TODO
}

impl Debug for Field {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}#{} (index: {})",
			self.class.name.as_str(),
			self.name.as_str(),
			self.idx
		)
	}
}

impl Field {
	pub fn is_static(&self) -> bool {
		self.access_flags.is_static()
	}

	pub fn is_final(&self) -> bool {
		self.access_flags.is_final()
	}

	pub fn is_volatile(&self) -> bool {
		self.access_flags.is_volatile()
	}
}

impl Field {
	/// Create a new `Field` instance
	///
	/// NOTE: This will leak the `Field` and return a reference. It is important that this only
	///       be called once per field. It should never be used outside of class loading.
	pub(super) fn new(
		idx: usize,
		class: &'static Class,
		field_info: &FieldInfo,
		constant_pool: &ConstantPool,
	) -> &'static Field {
		let access_flags = field_info.access_flags;

		let name_index = field_info.name_index;
		let name = constant_pool.get::<cp_types::ConstantUtf8>(name_index);

		let descriptor_index = field_info.descriptor_index;
		let descriptor = constant_pool.get::<cp_types::ConstantUtf8>(descriptor_index);

		let descriptor = FieldType::parse(&mut descriptor.as_bytes()).unwrap(); // TODO: Error handling
		let constant_value_index = field_info
			.get_constant_value_attribute()
			.map(|constant_value| constant_value.constantvalue_index);

		Box::leak(Box::new(Self {
			idx,
			class,
			access_flags,
			name,
			descriptor,
			constant_value_index,
		}))
	}

	pub fn get_static_value(&self) -> Operand<Reference> {
		if self.is_volatile() {
			self.class.static_field_value_volatile(self.idx)
		} else {
			self.class.static_field_value(self.idx)
		}
	}

	pub fn set_static_value(&self, value: Operand<Reference>) {
		if self.is_volatile() {
			unsafe { self.class.set_static_field_volatile(self.idx, value) }
		} else {
			unsafe {
				self.class.set_static_field(self.idx, value);
			}
		}
	}

	pub fn default_value_for_ty(ty: &FieldType) -> Operand<Reference> {
		match ty {
			FieldType::Byte
			| FieldType::Boolean
			| FieldType::Short
			| FieldType::Int
			| FieldType::Char => Operand::Int(0),
			FieldType::Long => Operand::Long(0),
			FieldType::Float => Operand::Float(0.),
			FieldType::Double => Operand::Double(0.),
			FieldType::Void | FieldType::Object(_) | FieldType::Array(_) => {
				Operand::Reference(Reference::null())
			},
		}
	}
}
