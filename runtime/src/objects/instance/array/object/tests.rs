use crate::globals::classes;
use crate::objects::instance::CloneableInstance;
use crate::objects::instance::array::{Array, ObjectArrayInstance};
use crate::objects::reference::Reference;
use crate::test_utils::init_basic_shared_runtime;

use jni::sys::jint;

#[test]
fn clone() {
	init_basic_shared_runtime();

	let array =
		ObjectArrayInstance::new(20, classes::java_lang_Class()).expect("should be a valid array");
	let cloned = unsafe { CloneableInstance::clone(&array) };

	assert_eq!(array.len(), 20);
	assert_eq!(array.len(), cloned.len());

	assert!(array.as_slice().iter().copied().all(Reference::is_null));
	assert!(cloned.as_slice().iter().copied().all(Reference::is_null));

	array
		.store(1, Reference::mirror(classes::java_lang_String().mirror()))
		.expect("store should work");
	assert_eq!(
		cloned.array_get(1).expect("get should work"),
		Reference::null(),
		"cloned array shouldn't be updated"
	);

	let cloned = unsafe { CloneableInstance::clone(&array) };
	assert_eq!(
		cloned.array_get(1).expect("get should work"),
		Reference::mirror(classes::java_lang_String().mirror()),
		"second cloned array should be updated"
	);
}

#[test]
fn slice() {
	init_basic_shared_runtime();

	let array =
		ObjectArrayInstance::new(20, classes::java_lang_Class()).expect("should be a valid array");
	let slice = array.as_slice();

	assert!(slice.iter().copied().all(Reference::is_null));

	let mirrors = [
		classes::java_lang_String().mirror(),
		classes::java_lang_Object().mirror(),
		classes::java_lang_Integer().mirror(),
		classes::java_lang_Long().mirror(),
		classes::java_lang_Boolean().mirror(),
	];

	for (i, mirror) in mirrors.into_iter().enumerate() {
		array
			.store(i as jint, Reference::mirror(mirror))
			.expect("store should work");
	}

	for (i, mirror) in mirrors.into_iter().enumerate() {
		assert_eq!(
			array.array_get(i as jint).expect("get should work"),
			Reference::mirror(mirror)
		);
	}
}
