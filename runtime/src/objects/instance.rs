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

/// An instance of an [`Object`]
///
/// [`Object`]: object::Object
pub trait Instance: object::Object {
	fn get_field_value(&self, field: &Field) -> Operand<Reference> {
		// Only mirrors can hold static fields
		if field.is_static() && !self.is_mirror() {
			return self.class().mirror().get_field_value(field);
		}

		unsafe { get_field_value_impl(self, &field, field.offset()) }
	}

	/// Get the value of a field by its index
	fn get_field_value0(&self, field_idx: usize) -> Operand<Reference> {
		let Some(field) = self.class().fields().nth(field_idx) else {
			panic!(
				"Failed to resolve field index: {:?}, in class: {:?}",
				field_idx,
				self.class()
			);
		};

		self.get_field_value(field)
	}

	fn put_field_value(&self, field: &Field, value: Operand<Reference>) {
		// Only mirrors can hold static fields
		if field.is_static() && !self.is_mirror() {
			return self.class().mirror().put_field_value(field, value);
		}

		unsafe { put_field_value_impl(self, &field, field.offset(), value) }
	}

	/// Set the value of a field by its index
	fn put_field_value0(&self, field_idx: usize, value: Operand<Reference>) {
		let Some(field) = self.class().fields().nth(field_idx) else {
			panic!(
				"Failed to resolve field index: {:?}, in class: {:?}",
				field_idx,
				self.class()
			);
		};

		self.put_field_value(field, value);
	}
}

/// Get the current value of a field for this object
///
/// This allows specifying a custom offset to interpret as the provided field.
/// See `MirrorInstanceRef::get_static_field_value()`.
///
/// # Safety
///
/// The caller must verify that the provided `offset` is within the current object's allocation
unsafe fn get_field_value_impl(
	obj: &impl object::Object,
	field: &Field,
	offset: usize,
) -> Operand<Reference> {
	unsafe {
		match field.descriptor {
			FieldType::Byte => Operand::Int({
				if field.is_volatile() {
					jint::from(obj.atomic_get::<jbyte>(offset))
				} else {
					jint::from(obj.get::<jbyte>(offset))
				}
			}),
			FieldType::Character => Operand::Int({
				if field.is_volatile() {
					jint::from(obj.atomic_get::<jchar>(offset))
				} else {
					jint::from(obj.get::<jchar>(offset))
				}
			}),
			FieldType::Integer => Operand::Int({
				if field.is_volatile() {
					obj.atomic_get::<jint>(offset)
				} else {
					obj.get::<jint>(offset)
				}
			}),
			FieldType::Short => Operand::Int({
				if field.is_volatile() {
					jint::from(obj.atomic_get::<jshort>(offset))
				} else {
					jint::from(obj.get::<jshort>(offset))
				}
			}),
			FieldType::Boolean => Operand::Int({
				if field.is_volatile() {
					jint::from(obj.atomic_get::<jboolean>(offset))
				} else {
					jint::from(obj.get::<jboolean>(offset))
				}
			}),

			FieldType::Double => Operand::Double({
				if field.is_volatile() {
					obj.atomic_get::<jdouble>(offset)
				} else {
					obj.get::<jdouble>(offset)
				}
			}),
			FieldType::Float => Operand::Float({
				if field.is_volatile() {
					obj.atomic_get::<jfloat>(offset)
				} else {
					obj.get::<jfloat>(offset)
				}
			}),

			FieldType::Long => Operand::Long({
				if field.is_volatile() {
					obj.atomic_get::<jlong>(offset)
				} else {
					obj.get::<jlong>(offset)
				}
			}),

			FieldType::Object(_) | FieldType::Array(_) => Operand::Reference({
				if field.is_volatile() {
					Reference::from_raw(obj.atomic_get::<usize>(offset) as *mut ())
				} else {
					Reference::from_raw(obj.get::<usize>(offset) as *mut ())
				}
			}),

			FieldType::Void => unreachable!(),
		}
	}
}

/// Put a value into a field of an object
///
/// This allows specifying a custom offset to interpret as the provided field.
/// See `MirrorInstanceRef::put_static_field_value()`.
///
/// # Safety
///
/// See [`get_field_value_impl()`]
unsafe fn put_field_value_impl(
	obj: &impl object::Object,
	field: &Field,
	offset: usize,
	value: Operand<Reference>,
) {
	fn incompatible(field: &Field, value: Operand<Reference>) -> ! {
		panic!(
			"Expected type compatible with: {:?}, found: {value:?} (class: {}, field index: {})",
			field.descriptor,
			field.class.name(),
			field.index(),
		)
	}

	unsafe {
		match value {
			Operand::Int(int) => match field.descriptor {
				FieldType::Byte => {
					if field.is_volatile() {
						obj.atomic_store::<jbyte>(int as jbyte, offset)
					} else {
						obj.put::<jbyte>(int as jbyte, offset)
					}
				},
				FieldType::Character => {
					if field.is_volatile() {
						obj.atomic_store::<jchar>(int as jchar, offset)
					} else {
						obj.put::<jchar>(int as jchar, offset)
					}
				},
				FieldType::Integer => {
					if field.is_volatile() {
						obj.atomic_store::<jint>(int, offset)
					} else {
						obj.put::<jint>(int, offset)
					}
				},
				FieldType::Short => {
					if field.is_volatile() {
						obj.atomic_store::<jshort>(int as jshort, offset)
					} else {
						obj.put::<jshort>(int as jshort, offset)
					}
				},
				FieldType::Boolean => {
					if field.is_volatile() {
						obj.atomic_store::<jboolean>(int != 0, offset)
					} else {
						obj.put::<jboolean>(int != 0, offset)
					}
				},
				_ => incompatible(field, value),
			},
			Operand::Float(float) if field.descriptor == FieldType::Float => {
				if field.is_volatile() {
					obj.atomic_store::<jfloat>(float, offset)
				} else {
					obj.put::<jfloat>(float, offset)
				}
			},
			Operand::Double(double) if field.descriptor == FieldType::Double => {
				if field.is_volatile() {
					obj.atomic_store::<jdouble>(double, offset)
				} else {
					obj.put::<jdouble>(double, offset)
				}
			},
			Operand::Long(long) if field.descriptor == FieldType::Long => {
				if field.is_volatile() {
					obj.atomic_store::<jlong>(long, offset)
				} else {
					obj.put::<jlong>(long, offset)
				}
			},
			// TODO: Verify the reference type?
			Operand::Reference(reference) => {
				if field.is_volatile() {
					obj.atomic_store::<usize>(reference.raw_tagged() as usize, offset)
				} else {
					obj.put::<usize>(reference.raw_tagged() as usize, offset)
				}
			},
			_ => incompatible(field, value),
		}
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
