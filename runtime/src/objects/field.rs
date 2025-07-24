use super::reference::Reference;
use crate::objects::class::ClassPtr;
use crate::objects::constant_pool::{ConstantPool, cp_types};
use crate::symbols::Symbol;

use std::cell::SyncUnsafeCell;
use std::fmt::{Debug, Formatter};

use classfile::accessflags::FieldAccessFlags;
use classfile::{FieldInfo, FieldType};
use common::int_types::u2;
use instructions::Operand;

// TODO: Make more fields private
pub struct Field {
	/// Used to set the value on `ClassInstance`s
	/// This is an `UnsafeCell`, as the index will be mutated for injected fields.
	idx: SyncUnsafeCell<usize>,
	/// The byte offset from an object's field base where this field's value lives
	offset: SyncUnsafeCell<usize>,
	pub class: ClassPtr,
	pub access_flags: FieldAccessFlags,
	pub name: Symbol,
	pub descriptor: FieldType,
	pub descriptor_sym: Symbol,
	pub constant_value_index: Option<u2>,
}

impl Debug for Field {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}#{} (index: {})",
			self.class.name(),
			self.name.as_str(),
			self.index()
		)
	}
}

impl Field {
	#[inline]
	pub fn is_static(&self) -> bool {
		self.access_flags.is_static()
	}

	#[inline]
	pub fn is_final(&self) -> bool {
		self.access_flags.is_final()
	}

	#[inline]
	pub fn is_volatile(&self) -> bool {
		self.access_flags.is_volatile()
	}

	#[inline]
	pub fn is_trusted_final(&self) -> bool {
		self.is_final() && (self.is_static() || self.class.is_record())
	}
}

impl Field {
	pub fn index(&self) -> usize {
		unsafe { *self.idx.get() }
	}

	pub(super) unsafe fn set_index(&self, index: usize) {
		unsafe { *self.idx.get() = index };
	}

	pub fn offset(&self) -> usize {
		unsafe { *self.offset.get() }
	}

	pub(super) fn set_offset(&self, offset: usize) {
		unsafe { *self.offset.get() = offset };
	}
}

impl Field {
	/// Create a new `Field` instance
	///
	/// NOTES:
	///
	/// * `offset` is the direct, **possibly unaligned** end of the previous field. This constructor
	///   automatically handles padding if necessary.
	/// * This will leak the `Field` and return a reference. It is important that this only be called
	///   once per field. It should never be used outside of class loading.
	pub(super) fn new(
		idx: usize,
		offset: usize,
		class: ClassPtr,
		field_info: &FieldInfo,
		constant_pool: &ConstantPool,
	) -> &'static Field {
		fn padding(is_volatile: bool, offset: usize, align_of_field: usize) -> usize {
			debug_assert!(align_of_field <= 8);

			if !is_volatile {
				return 0;
			}

			offset % align_of_field
		}

		let access_flags = field_info.access_flags;

		let name_index = field_info.name_index;
		let name = constant_pool
			.get::<cp_types::ConstantUtf8>(name_index)
			.expect("field name should always resolve");

		let descriptor_index = field_info.descriptor_index;
		let descriptor_sym = constant_pool
			.get::<cp_types::ConstantUtf8>(descriptor_index)
			.expect("field descriptor should always resolve");

		let descriptor = FieldType::parse(&mut descriptor_sym.as_bytes()).unwrap(); // TODO: Error handling
		let constant_value_index = field_info
			.get_constant_value_attribute()
			.map(|constant_value| constant_value.constantvalue_index);

		let offset = offset + padding(access_flags.is_volatile(), offset, descriptor.align());
		Box::leak(Box::new(Self {
			idx: SyncUnsafeCell::new(idx),
			offset: SyncUnsafeCell::new(offset),
			class,
			access_flags,
			name,
			descriptor,
			descriptor_sym,
			constant_value_index,
		}))
	}

	pub fn new_injected(class: ClassPtr, name: Symbol, descriptor: FieldType) -> &'static Field {
		let descriptor_sym = Symbol::intern(descriptor.as_signature());

		Box::leak(Box::new(Self {
			idx: SyncUnsafeCell::new(0),
			offset: SyncUnsafeCell::new(0),
			class,
			access_flags: FieldAccessFlags::NONE,
			name,
			descriptor,
			descriptor_sym,
			constant_value_index: None,
		}))
	}

	pub fn get_static_value(&self) -> Operand<Reference> {
		if self.is_volatile() {
			self.class.static_field_value_volatile(self.index())
		} else {
			self.class.static_field_value(self.index())
		}
	}

	pub fn set_static_value(&self, value: Operand<Reference>) {
		if self.is_volatile() {
			unsafe { self.class.set_static_field_volatile(self.index(), value) }
		} else {
			unsafe {
				self.class.set_static_field(self.index(), value);
			}
		}
	}

	pub fn default_value_for_ty(ty: &FieldType) -> Operand<Reference> {
		match ty {
			FieldType::Byte
			| FieldType::Boolean
			| FieldType::Short
			| FieldType::Integer
			| FieldType::Character => Operand::Int(0),
			FieldType::Long => Operand::Long(0),
			FieldType::Float => Operand::Float(0.),
			FieldType::Double => Operand::Double(0.),
			FieldType::Void | FieldType::Object(_) | FieldType::Array(_) => {
				Operand::Reference(Reference::null())
			},
		}
	}
}
