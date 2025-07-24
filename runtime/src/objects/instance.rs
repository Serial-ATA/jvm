pub mod array;
pub mod class;
pub mod mirror;
pub mod object;

use crate::objects::field::Field;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;

use std::mem::offset_of;
use std::sync::atomic::{AtomicI32, Ordering};

use classfile::FieldType;
use instructions::Operand;
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};

pub trait Instance: object::Object {
	fn get_field_value(&self, field: &Field) -> Operand<Reference> {
		assert!(!field.is_static());

		unsafe {
			match field.descriptor {
				FieldType::Byte => Operand::Int(self.get::<jbyte>(field.offset()) as jint),
				FieldType::Character => Operand::Int(self.get::<jchar>(field.offset()) as jint),
				FieldType::Integer => Operand::Int(self.get::<jint>(field.offset())),
				FieldType::Short => Operand::Int(self.get::<jshort>(field.offset()) as jint),
				FieldType::Boolean => Operand::Int(self.get::<jboolean>(field.offset()) as jint),

				FieldType::Double => Operand::Double(self.get::<jdouble>(field.offset())),
				FieldType::Float => Operand::Float(self.get::<jfloat>(field.offset())),

				FieldType::Long => Operand::Long(self.get::<jlong>(field.offset())),

				FieldType::Object(_) | FieldType::Array(_) => {
					Operand::Reference(self.get::<Reference>(field.offset()))
				},

				FieldType::Void => unreachable!(),
			}
		}
	}

	/// Read the value of a field by its index
	fn get_field_value0(&self, field_idx: usize) -> Operand<Reference> {
		let Some(field) = self.class().instance_fields().nth(field_idx) else {
			panic!(
				"Failed to resolve field index: {:?}, in class: {:?}",
				field_idx,
				self.class()
			);
		};

		self.get_field_value(field)
	}

	fn put_field_value(&self, field: &Field, value: Operand<Reference>) {
		fn incompatible(field: &Field, value: Operand<Reference>) -> ! {
			panic!(
				"Expected type compatible with: {:?}, found: {value:?} (class: {}, field index: \
				 {})",
				field.descriptor,
				field.class.name(),
				field.index(),
			)
		}

		assert!(!field.is_static());

		unsafe {
			match value {
				Operand::Int(int) => match field.descriptor {
					FieldType::Byte => self.put::<jbyte>(int as jbyte, field.offset()),
					FieldType::Character => self.put::<jchar>(int as jchar, field.offset()),
					FieldType::Integer => self.put::<jint>(int, field.offset()),
					FieldType::Short => self.put::<jshort>(int as jshort, field.offset()),
					FieldType::Boolean => self.put::<jboolean>(int != 0, field.offset()),
					_ => incompatible(field, value),
				},
				Operand::Float(float) if field.descriptor == FieldType::Float => {
					self.put::<jfloat>(float, field.offset())
				},
				Operand::Double(double) if field.descriptor == FieldType::Double => {
					self.put::<jdouble>(double, field.offset())
				},
				Operand::Long(long) if field.descriptor == FieldType::Long => {
					self.put::<jlong>(long, field.offset())
				},
				// TODO: Verify the reference type?
				Operand::Reference(reference) => self.put::<Reference>(reference, field.offset()),
				_ => incompatible(field, value),
			}
		}
	}

	/// Set the value of a field by its index
	fn put_field_value0(&self, field_idx: usize, value: Operand<Reference>) {
		let Some(field) = self.class().instance_fields().nth(field_idx) else {
			panic!(
				"Failed to resolve field index: {:?}, in class: {:?}",
				field_idx,
				self.class()
			);
		};

		self.put_field_value(field, value);
	}
}

pub trait CloneableInstance {
	type ReferenceTy;

	// TODO: Throw for OOM
	/// Clone the object that this reference points to
	///
	/// # Safety
	///
	/// The caller **must** verify that the object is cloneable. This should rarely, if ever, be
	/// used directly.
	unsafe fn clone(&self) -> Self::ReferenceTy;
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq)]
struct HeaderFlags {
	locked: bool,
}

impl HeaderFlags {
	pub fn encode(self) -> u8 {
		0u8 | self.locked as u8
	}
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Header {
	flags: HeaderFlags,
	hash: jint,
}

const _: () = {
	assert!(size_of::<Header>() == 8, "Header size changed!");
};

impl Header {
	const HASH_OFFSET: usize = offset_of!(Header, hash);

	pub fn new() -> Self {
		Header {
			flags: HeaderFlags::default(),
			hash: 0,
		}
	}

	pub fn hash(&self) -> Option<jint> {
		let atomic_hash = unsafe {
			let hash_ptr = std::ptr::from_ref(self).byte_offset(Self::HASH_OFFSET as isize);
			&*hash_ptr.cast::<AtomicI32>()
		};

		let ret = atomic_hash.load(Ordering::Acquire);
		if ret == 0 {
			return None;
		}

		Some(ret)
	}

	fn set_hash(&self, hash: jint) -> bool {
		let atomic_hash = unsafe {
			let hash_ptr = std::ptr::from_ref(self).byte_offset(Self::HASH_OFFSET as isize);
			&*hash_ptr.cast::<AtomicI32>()
		};

		atomic_hash
			.compare_exchange(0, hash, Ordering::Acquire, Ordering::Acquire)
			.is_ok()
	}

	// TODO: This only supports the HotSpot default hash code generation, there are still 4 other possible algorithms.
	// https://github.com/openjdk/jdk/blob/807f6f7fb868240cba5ba117c7059216f69a53f9/src/hotspot/share/runtime/synchronizer.cpp#L935
	pub fn generate_hash(&self, thread: &'static JavaThread) -> jint {
		loop {
			let current_hash = self.hash();
			if let Some(hash) = current_hash {
				return hash;
			}

			let hash = thread.marsaglia_xor_shift_hash() as jint;
			if self.set_hash(hash) {
				return hash;
			}
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn header_hash_set_single_thread() {
		todo!()
	}

	#[test]
	fn header_hash_set_contention() {
		todo!()
	}
}
