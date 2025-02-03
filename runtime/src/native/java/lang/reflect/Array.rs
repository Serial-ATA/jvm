use crate::objects::class::Class;
use crate::objects::reference::Reference;

use jni::env::JniEnv;
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};

include_generated!("native/java/lang/reflect/def/Array.definitions.rs");

// throws IllegalArgumentException
pub fn getLength(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
) -> jint {
	unimplemented!("java.lang.reflect.Array#getLength");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn get(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.reflect.Array#get");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getBoolean(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jboolean {
	unimplemented!("java.lang.reflect.Array#getBoolean");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getByte(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jbyte {
	unimplemented!("java.lang.reflect.Array#getByte");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getChar(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jchar {
	unimplemented!("java.lang.reflect.Array#getChar");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getShort(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jshort {
	unimplemented!("java.lang.reflect.Array#getShort");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getInt(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jint {
	unimplemented!("java.lang.reflect.Array#getInt");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getLong(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jlong {
	unimplemented!("java.lang.reflect.Array#getLong");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getFloat(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jfloat {
	unimplemented!("java.lang.reflect.Array#getFloat");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn getDouble(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
) -> jdouble {
	unimplemented!("java.lang.reflect.Array#getDouble");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn set(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: Reference, // java.lang.Object
) {
	unimplemented!("java.lang.reflect.Array#set");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setBoolean(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jboolean,
) {
	unimplemented!("java.lang.reflect.Array#setBoolean");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setByte(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jbyte,
) {
	unimplemented!("java.lang.reflect.Array#setByte");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setChar(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jchar,
) {
	unimplemented!("java.lang.reflect.Array#setChar");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setShort(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jshort,
) {
	unimplemented!("java.lang.reflect.Array#setShort");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setInt(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jint,
) {
	unimplemented!("java.lang.reflect.Array#setInt");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setLong(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jlong,
) {
	unimplemented!("java.lang.reflect.Array#setLong");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setFloat(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jfloat,
) {
	unimplemented!("java.lang.reflect.Array#setFloat");
}

// throws IllegalArgumentException, ArrayIndexOutOfBoundsException
pub fn setDouble(
	_env: JniEnv,
	_class: &'static Class,
	_array: Reference, // java.lang.Object
	_index: jint,
	_value: jdouble,
) {
	unimplemented!("java.lang.reflect.Array#setDouble");
}

// throws NegativeArraySizeException
pub fn newArray(
	_env: JniEnv,
	_class: &'static Class,
	_component_type: Reference, // java.lang.Class<?>
	_length: jint,
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.reflect.Array#newArray");
}

// throws IllegalArgumentException, NegativeArraySizeException
pub fn multiNewArray(
	_env: JniEnv,
	_class: &'static Class,
	_component_type: Reference, // java.lang.Class<?>
	_dimensions: Reference,     // int[]
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.reflect.Array#multiNewArray");
}
