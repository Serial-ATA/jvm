use crate::objects::class::ClassPtr;
use crate::objects::instance::array::{
	ObjectArrayInstance, PrimitiveArrayInstance, PrimitiveArrayInstanceRef,
};
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::{handle_exception, throw_and_return_null};

use jni::env::JniEnv;
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};

include_generated!("native/java/lang/reflect/def/Array.definitions.rs");

/// Maximum number of array dimensions
pub const MAX_DIM: u8 = 255;

// throws IllegalArgumentException
pub fn getLength(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
) -> jint {
	unimplemented!("java.lang.reflect.Array#getLength");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn get(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.reflect.Array#get");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getBoolean(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jboolean {
	unimplemented!("java.lang.reflect.Array#getBoolean");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getByte(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jbyte {
	unimplemented!("java.lang.reflect.Array#getByte");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getChar(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jchar {
	unimplemented!("java.lang.reflect.Array#getChar");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getShort(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jshort {
	unimplemented!("java.lang.reflect.Array#getShort");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getInt(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jint {
	unimplemented!("java.lang.reflect.Array#getInt");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getLong(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jlong {
	unimplemented!("java.lang.reflect.Array#getLong");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getFloat(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jfloat {
	unimplemented!("java.lang.reflect.Array#getFloat");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getDouble(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jdouble {
	unimplemented!("java.lang.reflect.Array#getDouble");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn set(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: Reference, // java.lang.Object
) {
	unimplemented!("java.lang.reflect.Array#set");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setBoolean(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jboolean,
) {
	unimplemented!("java.lang.reflect.Array#setBoolean");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setByte(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jbyte,
) {
	unimplemented!("java.lang.reflect.Array#setByte");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setChar(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jchar,
) {
	unimplemented!("java.lang.reflect.Array#setChar");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setShort(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jshort,
) {
	unimplemented!("java.lang.reflect.Array#setShort");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setInt(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jint,
) {
	unimplemented!("java.lang.reflect.Array#setInt");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setLong(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jlong,
) {
	unimplemented!("java.lang.reflect.Array#setLong");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setFloat(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jfloat,
) {
	unimplemented!("java.lang.reflect.Array#setFloat");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setDouble(
	_env: JniEnv,
	_class: ClassPtr,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jdouble,
) {
	unimplemented!("java.lang.reflect.Array#setDouble");
}

// throws NegativeArraySizeException
pub fn newArray(
	env: JniEnv,
	_class: ClassPtr,
	component_type: Reference, // java.lang.Class<?>
	length: jint,
) -> Reference /* java.lang.Object */ {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	if component_type.is_null() {
		throw_and_return_null!(thread, NullPointerException);
	}

	if length.is_negative() {
		throw_and_return_null!(thread, NegativeArraySizeException, "{length}");
	}

	let mirror = component_type.extract_mirror();
	if mirror.is_primitive() {
		let type_code = mirror
			.primitive_target()
			.as_array_type_code()
			.expect("should be a valid array type code");

		let array = PrimitiveArrayInstance::new_from_type(type_code, length);
		let array_instance: PrimitiveArrayInstanceRef =
			handle_exception!(Reference::null(), thread, array);
		return Reference::array(array_instance);
	}

	let class = mirror.target_class();
	if class.is_array() && class.unwrap_array_instance().dimensions >= MAX_DIM {
		throw_and_return_null!(thread, IllegalArgumentException);
	}

	let array = ObjectArrayInstance::new(length, class);
	let array_instance = handle_exception!(Reference::null(), thread, array);
	Reference::object_array(array_instance)
}

// throws IllegalArgumentException, NegativeArraySizeException
pub fn multiNewArray(
	_env: JniEnv,
	_class: ClassPtr,
	_component_type: Reference, // java.lang.Class<?>
	_dimensions: Reference,     // int[]
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.reflect.Array#multiNewArray");
}
