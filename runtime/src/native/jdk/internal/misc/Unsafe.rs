use crate::class::Class;
use crate::heap::spec::class::ClassInitializationState;
use crate::reference::Reference;
use crate::stack::local_stack::LocalStack;
use crate::string_interner::StringInterner;

use std::sync::Arc;

use ::jni::env::JNIEnv;
use ::jni::sys::{jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jobject, jshort};
use common::traits::PtrType;
use instructions::Operand;

include_generated!("native/jdk/internal/misc/def/Unsafe.definitions.rs");
include_generated!("native/jdk/internal/misc/def/Unsafe.registerNatives.rs");

pub fn getUncompressedObject(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	address: jlong,
) -> jobject {
	unimplemented!("jdk.internal.misc.Unsafe#getUncompressedObject")
}

pub fn writeback0(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	address: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#writeback0")
}
pub fn writebackPreSync0(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
) {
	unimplemented!("jdk.internal.misc.Unsafe#writebackPreSync0")
}
pub fn writebackPostSync0(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
) {
	unimplemented!("jdk.internal.misc.Unsafe#writebackPostSync0")
}

pub fn defineClass0(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	name: Reference,  // java.lang.String
	bytes: Reference, // [B
	offset: jint,
	length: jint,
	loader: Reference,            // java.lang.ClassLoader
	protection_domain: Reference, // java.security.ProtectionDomain
) -> jclass {
	unimplemented!("jdk.internal.misc.Unsafe#defineClass0")
}

// throws InstantiationException
pub fn allocateInstance(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	class: Reference, // java.lang.Class
) -> jobject {
	unimplemented!("jdk.internal.misc.Unsafe#allocateInstance")
}

pub fn throwException(
	_env: JNIEnv,
	_this: Reference,     // jdk.internal.misc.Unsafe
	exception: Reference, // java.lang.Throwable
) {
	unimplemented!("jdk.internal.misc.Unsafe#throwException")
}

pub fn compareAndSetInt(
	_env: JNIEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: jint,
	value: jint,
) {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndSetInt")
}

pub fn compareAndExchangeInt(
	_env: JNIEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: jint,
	value: jint,
) {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndExchangeInt")
}

pub fn compareAndSetLong(
	_env: JNIEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: jlong,
	value: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndSetLong")
}

pub fn compareAndExchangeLong(
	_env: JNIEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: jlong,
	value: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndExchangeLong")
}

pub fn compareAndSetReference(
	_env: JNIEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: Reference, // Object
	value: Reference,    // Object
) {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndSetReference")
}

pub fn compareAndExchangeReference(
	_env: JNIEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: Reference, // Object
	value: Reference,    // Object
) {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndExchangeReference")
}

pub fn getReference(
	_env: JNIEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
) -> Reference /* Object */ {
	unimplemented!("jdk.internal.misc.Unsafe#getReference")
}

pub fn putReference(
	_env: JNIEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	value: Reference, // Object
) -> Reference /* Object */ {
	unimplemented!("jdk.internal.misc.Unsafe#putReference")
}

pub fn getReferenceVolatile(
	_env: JNIEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
) -> Reference /* Object */ {
	unimplemented!("jdk.internal.misc.Unsafe#getReferenceVolatile")
}

pub fn putReferenceVolatile(
	_env: JNIEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // java.lang.Object
	offset: jlong,
	value: Reference, // java.lang.Object
) -> Reference /* java.lang.Object */ {
	unimplemented!("jdk.internal.misc.Unsafe#putReferenceVolatile")
}

/// Creates the many `{get, put}Ty` and `{get, put}TyVolatile` methods
macro_rules! get_put_methods {
	($($ty:ident),+) => {
		$(
			paste::paste! {
				pub fn [<get $ty:camel>](
					_env: JNIEnv,
					_this: Reference,  // jdk.internal.misc.Unsafe
					object: Reference, // Object
					offset: jlong
				) -> [<j $ty>] {
					unimplemented!()
				}

				pub fn [<put $ty:camel>](
					_env: JNIEnv,
					_this: Reference,  // jdk.internal.misc.Unsafe
					object: Reference, // Object
					offset: jlong,
					value: [<j $ty>]
				) {
					unimplemented!()
				}

				pub fn [<get $ty:camel Volatile>](
					_env: JNIEnv,
					_this: Reference,  // jdk.internal.misc.Unsafe
					object: Reference, // Object
					offset: jlong
				) -> [<j $ty>] {
					unimplemented!()
				}

				pub fn [<put $ty:camel Volatile>](
					_env: JNIEnv,
					_this: Reference,  // jdk.internal.misc.Unsafe
					object: Reference, // Object
					offset: jlong,
					value: [<j $ty>]
				) {
					unimplemented!()
				}

			}
		)+
	};
}

get_put_methods! { boolean, byte, short, char, int, long, float, double }

pub fn unpark(
	_env: JNIEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	thread: Reference, // Object
) {
	unimplemented!("jdk.internal.misc.Unsafe#unpark")
}

pub fn park(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	is_absolute: bool,
	time: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#park")
}

pub fn fullFence(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
) {
	platform::os_arch::ordering::fence();
}

pub fn allocateMemory0(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	bytes: jlong,
) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#allocateMemory0")
}

pub fn reallocateMemory0(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	address: jlong,
	bytes: jlong,
) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#reallocateMemory0")
}

pub fn freeMemory0(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	address: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#freeMemory0")
}

pub fn setMemory0(
	_env: JNIEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	bytes: jlong,
	value: jbyte,
) {
	unimplemented!("jdk.internal.misc.Unsafe#setMemory0")
}

pub fn copyMemory0(
	_env: JNIEnv,
	_this: Reference,    // jdk.internal.misc.Unsafe
	src_base: Reference, // Object
	src_offset: jlong,
	dest_base: Reference, // Object
	dest_offset: jlong,
	bytes: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#copyMemory0")
}

pub fn copySwapMemory0(
	_env: JNIEnv,
	_this: Reference,    // jdk.internal.misc.Unsafe
	src_base: Reference, // Object
	src_offset: jlong,
	dest_base: Reference, // Object,
	dest_offset: jlong,
	bytes: jlong,
	elem_size: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#copyMemory0")
}

pub fn objectFieldOffset0(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	field: Reference, // java.lang.reflect.Field
) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#objectFieldOffset0")
}

pub fn objectFieldOffset1(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	class: Reference, // java.lang.Class
	name: Reference,  // String
) -> jlong {
	let class = class.extract_mirror();

	let name_str = StringInterner::rust_string_from_java_string(name.extract_class());
	let classref = class.get().expect_class();
	for (offset, field) in classref.unwrap_class_instance().fields.iter().enumerate() {
		if field.name == name_str.as_bytes() {
			return (offset as jlong).into();
		}
	}

	// TODO
	panic!("InternalError")
}

pub fn staticFieldOffset0(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	field: Reference, // java.lang.reflect.Field
) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#staticFieldOffset0")
}

pub fn staticFieldBase0(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	field: Reference, // java.lang.reflect.Field
) -> Reference /* java.lang.Object */ {
	unimplemented!("jdk.internal.misc.Unsafe#staticFieldBase0")
}
pub fn shouldBeInitialized0(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	class: Reference, // java.lang.Class
) -> bool {
	unimplemented!("jdk.internal.misc.Unsafe#shouldBeInitialized0")
}
pub fn ensureClassInitialized0(
	env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	class: Reference, // java.lang.Class
) {
	let mirror = class.extract_mirror();

	let target_class = mirror.get().expect_class();
	match target_class.initialization_state() {
		ClassInitializationState::Uninit => {
			Class::initialize(&target_class, Arc::clone(&env.current_thread))
		},
		ClassInitializationState::InProgress | ClassInitializationState::Init => {},
		// Is this the best we can do?
		ClassInitializationState::Failed => unreachable!("Failed to ensure class initialization"),
	}
}
pub fn arrayBaseOffset0(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	locals: LocalStack,
) -> jint {
	let array_class = locals[1].expect_reference(); // java.lang.Class

	let mirror = array_class.extract_mirror();
	// TODO: InvalidClassException
	let _array = mirror.get().expect_class().unwrap_array_instance();

	// TODO: We don't do byte packing like Hotspot
	0
}
pub fn arrayIndexScale0(
	_env: JNIEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	locals: LocalStack,
) -> jint {
	let array_class = locals[1].expect_reference(); // java.lang.Class

	let mirror = array_class.extract_mirror();
	// TODO: InvalidClassException
	let _array = mirror.get().expect_class().unwrap_array_instance();

	// TODO: We don't do byte packing like Hotspot
	core::mem::size_of::<Operand<Reference>>() as jint
}
pub fn getLoadAverage0(
	_env: JNIEnv,
	_this: Reference,   // jdk.internal.misc.Unsafe
	loadavg: Reference, // [D
	nelems: jint,
) -> jint {
	unimplemented!("jdk.internal.misc.Unsafe#getLoadAverage0")
}
