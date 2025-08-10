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
				FieldType::Byte => Operand::Int({
					if field.is_volatile() {
						jint::from(self.atomic_get::<jbyte>(field.offset()))
					} else {
						jint::from(self.get::<jbyte>(field.offset()))
					}
				}),
				FieldType::Character => Operand::Int({
					if field.is_volatile() {
						jint::from(self.atomic_get::<jchar>(field.offset()))
					} else {
						jint::from(self.get::<jchar>(field.offset()))
					}
				}),
				FieldType::Integer => Operand::Int({
					if field.is_volatile() {
						self.atomic_get::<jint>(field.offset())
					} else {
						self.get::<jint>(field.offset())
					}
				}),
				FieldType::Short => Operand::Int({
					if field.is_volatile() {
						jint::from(self.atomic_get::<jshort>(field.offset()))
					} else {
						jint::from(self.get::<jshort>(field.offset()))
					}
				}),
				FieldType::Boolean => Operand::Int({
					if field.is_volatile() {
						jint::from(self.atomic_get::<jboolean>(field.offset()))
					} else {
						jint::from(self.get::<jboolean>(field.offset()))
					}
				}),

				FieldType::Double => Operand::Double({
					if field.is_volatile() {
						self.atomic_get::<jdouble>(field.offset())
					} else {
						self.get::<jdouble>(field.offset())
					}
				}),
				FieldType::Float => Operand::Float({
					if field.is_volatile() {
						self.atomic_get::<jfloat>(field.offset())
					} else {
						self.get::<jfloat>(field.offset())
					}
				}),

				FieldType::Long => Operand::Long({
					if field.is_volatile() {
						self.atomic_get::<jlong>(field.offset())
					} else {
						self.get::<jlong>(field.offset())
					}
				}),

				FieldType::Object(_) | FieldType::Array(_) => Operand::Reference({
					if field.is_volatile() {
						Reference::from_raw(self.atomic_get::<usize>(field.offset()) as *mut ())
					} else {
						Reference::from_raw(self.get::<usize>(field.offset()) as *mut ())
					}
				}),

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
					FieldType::Byte => {
						if field.is_volatile() {
							self.atomic_store::<jbyte>(int as jbyte, field.offset())
						} else {
							self.put::<jbyte>(int as jbyte, field.offset())
						}
					},
					FieldType::Character => {
						if field.is_volatile() {
							self.atomic_store::<jchar>(int as jchar, field.offset())
						} else {
							self.put::<jchar>(int as jchar, field.offset())
						}
					},
					FieldType::Integer => {
						if field.is_volatile() {
							self.atomic_store::<jint>(int, field.offset())
						} else {
							self.put::<jint>(int, field.offset())
						}
					},
					FieldType::Short => {
						if field.is_volatile() {
							self.atomic_store::<jshort>(int as jshort, field.offset())
						} else {
							self.put::<jshort>(int as jshort, field.offset())
						}
					},
					FieldType::Boolean => {
						if field.is_volatile() {
							self.atomic_store::<jboolean>(int != 0, field.offset())
						} else {
							self.put::<jboolean>(int != 0, field.offset())
						}
					},
					_ => incompatible(field, value),
				},
				Operand::Float(float) if field.descriptor == FieldType::Float => {
					if field.is_volatile() {
						self.atomic_store::<jfloat>(float, field.offset())
					} else {
						self.put::<jfloat>(float, field.offset())
					}
				},
				Operand::Double(double) if field.descriptor == FieldType::Double => {
					if field.is_volatile() {
						self.atomic_store::<jdouble>(double, field.offset())
					} else {
						self.put::<jdouble>(double, field.offset())
					}
				},
				Operand::Long(long) if field.descriptor == FieldType::Long => {
					if field.is_volatile() {
						self.atomic_store::<jlong>(long, field.offset())
					} else {
						self.put::<jlong>(long, field.offset())
					}
				},
				// TODO: Verify the reference type?
				Operand::Reference(reference) => {
					if field.is_volatile() {
						self.atomic_store::<usize>(reference.raw_tagged() as usize, field.offset())
					} else {
						self.put::<usize>(reference.raw_tagged() as usize, field.offset())
					}
				},
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
		u8::from(self.locked)
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
