use crate::objects::instance::CloneableInstance;
use crate::objects::instance::array::{Array, PrimitiveArrayInstance};
use crate::objects::instance::object::Object;
use crate::test_utils::init_basic_shared_runtime;

use common::box_slice;
use instructions::Operand;
use jni::sys::{jbyte, jint};

#[test]
fn clone() {
	init_basic_shared_runtime();

	let array = PrimitiveArrayInstance::new(box_slice![0i8; 20]);
	let cloned = unsafe { CloneableInstance::clone(&array) };

	assert_eq!(array.len(), 20);
	assert_eq!(array.len(), cloned.len());

	assert!(array.as_slice::<jbyte>().iter().all(|i| *i == 0));
	assert!(cloned.as_slice::<jbyte>().iter().all(|i| *i == 0));

	array.store(1, Operand::Int(15)).expect("store should work");
	assert_eq!(
		cloned.array_get(1).expect("get should work"),
		Operand::Int(0),
		"cloned array shouldn't be updated"
	);

	let cloned = unsafe { CloneableInstance::clone(&array) };
	assert_eq!(
		cloned.array_get(1).expect("get should work"),
		Operand::Int(15),
		"second cloned array should be updated"
	);
}

#[test]
fn slice() {
	init_basic_shared_runtime();

	let array = PrimitiveArrayInstance::new(box_slice![1_i32, 2_i32, 3_i32, 4_i32, 5_i32]);
	let slice = array.as_slice::<jint>();

	for (value, expected) in slice.iter().zip(1..=5) {
		assert_eq!(*value, expected);
	}
}

#[test]
#[should_panic]
fn slice_wrong_type() {
	init_basic_shared_runtime();

	let array = PrimitiveArrayInstance::new(box_slice![1_i32, 2_i32, 3_i32, 4_i32, 5_i32]);
	let _slice = array.as_slice::<jbyte>();
}

#[test]
fn volatile() {
	init_basic_shared_runtime();

	let array = PrimitiveArrayInstance::new(box_slice![0_i32; 5]);

	let values = box_slice![1_i32, 2_i32, 3_i32, 4_i32, 5_i32];

	for (i, expected) in values.iter().enumerate() {
		let offset = i * size_of::<jint>();
		unsafe {
			array.atomic_store(*expected, offset);
		}
	}

	for (i, expected) in values.iter().enumerate() {
		let offset = i * size_of::<jint>();
		let value = unsafe { array.atomic_get::<jint>(offset) };

		assert_eq!(*expected, value);
	}
}
