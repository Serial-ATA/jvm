use crate::class::Class;
use crate::class_instance::Instance;
use crate::native::JniEnv;
use crate::objects::spec::class::ClassInitializationState;
use crate::reference::Reference;
use crate::string_interner::StringInterner;
use crate::thread::JavaThread;

use std::ptr::NonNull;

use ::jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
use common::traits::PtrType;
use instructions::Operand;

include_generated!("native/jdk/internal/misc/def/Unsafe.definitions.rs");
include_generated!("native/jdk/internal/misc/def/Unsafe.registerNatives.rs");

pub fn getUncompressedObject(
	_env: NonNull<JniEnv>,
	_this: Reference, // jdk.internal.misc.Unsafe
	_address: jlong,
) -> Reference {
	unimplemented!("jdk.internal.misc.Unsafe#getUncompressedObject")
}

pub fn writeback0(
	_env: NonNull<JniEnv>,
	_this: Reference, // jdk.internal.misc.Unsafe
	_address: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#writeback0")
}
pub fn writebackPreSync0(
	_env: NonNull<JniEnv>,
	_this: Reference, // jdk.internal.misc.Unsafe
) {
	unimplemented!("jdk.internal.misc.Unsafe#writebackPreSync0")
}
pub fn writebackPostSync0(
	_env: NonNull<JniEnv>,
	_this: Reference, // jdk.internal.misc.Unsafe
) {
	unimplemented!("jdk.internal.misc.Unsafe#writebackPostSync0")
}

pub fn defineClass0(
	_env: NonNull<JniEnv>,
	_this: Reference,  // jdk.internal.misc.Unsafe
	_name: Reference,  // java.lang.String
	_bytes: Reference, // [B
	_offset: jint,
	_length: jint,
	_loader: Reference,            // java.lang.ClassLoader
	_protection_domain: Reference, // java.security.ProtectionDomain
) -> Reference {
	unimplemented!("jdk.internal.misc.Unsafe#defineClass0")
}

// throws InstantiationException
pub fn allocateInstance(
	_env: NonNull<JniEnv>,
	_this: Reference,  // jdk.internal.misc.Unsafe
	_class: Reference, // java.lang.Class
) -> Reference {
	unimplemented!("jdk.internal.misc.Unsafe#allocateInstance")
}

pub fn throwException(
	_env: NonNull<JniEnv>,
	_this: Reference,      // jdk.internal.misc.Unsafe
	_exception: Reference, // java.lang.Throwable
) {
	unimplemented!("jdk.internal.misc.Unsafe#throwException")
}

pub fn compareAndSetInt(
	env: NonNull<JniEnv>,
	this: Reference,   // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: jint,
	value: jint,
) -> jboolean {
	compareAndExchangeInt(env, this, object, offset, expected, value) == value
}

pub fn compareAndExchangeInt(
	_env: NonNull<JniEnv>,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: jint,
	value: jint,
) -> jint {
	let instance = object.extract_class();

	unsafe {
		let field_value = instance
			.get_mut()
			.get_field_value_raw(offset as usize)
			.as_ptr();

		let current_field_value = (*field_value).expect_int();
		if current_field_value == expected {
			*field_value = Operand::Int(value);
			return value;
		}

		current_field_value
	}
}

pub fn compareAndSetLong(
	env: NonNull<JniEnv>,
	this: Reference,   // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: jlong,
	value: jlong,
) -> jboolean {
	compareAndExchangeLong(env, this, object, offset, expected, value) == value
}

pub fn compareAndExchangeLong(
	_env: NonNull<JniEnv>,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: jlong,
	value: jlong,
) -> jlong {
	let instance = object.extract_class();

	unsafe {
		let field_value = instance
			.get_mut()
			.get_field_value_raw(offset as usize)
			.as_ptr();

		let current_field_value = (*field_value).expect_long();
		if current_field_value == expected {
			*field_value = Operand::Long(value);
			return value;
		}

		current_field_value
	}
}

pub fn compareAndSetReference(
	env: NonNull<JniEnv>,
	this: Reference,   // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: Reference, // Object
	value: Reference,    // Object
) -> jboolean {
	compareAndExchangeReference(
		env,
		this,
		object,
		offset,
		Reference::clone(&expected),
		value,
	) == expected
}

pub fn compareAndExchangeReference(
	_env: NonNull<JniEnv>,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: Reference, // Object
	value: Reference,    // Object
) -> Reference {
	if object.is_array() {
		let instance = object.extract_array();
		unsafe {
			let array_mut = instance.get_mut();
			let mut current_field_value = array_mut
				.get_content_mut()
				.get_reference_raw(offset as usize);
			if current_field_value.as_ref() == &expected {
				*current_field_value.as_mut() = Reference::clone(&value);
				return value;
			}

			return Reference::clone(current_field_value.as_ref());
		}
	}

	let instance = object.extract_class();
	unsafe {
		let field_value = instance
			.get_mut()
			.get_field_value_raw(offset as usize)
			.as_ptr();

		let current_field_value = (*field_value).expect_reference();
		if current_field_value == expected {
			*field_value = Operand::Reference(Reference::clone(&value));
			return value;
		}

		current_field_value
	}
}

pub fn getReference(
	_env: NonNull<JniEnv>,
	_this: Reference,   // jdk.internal.misc.Unsafe
	_object: Reference, // Object
	_offset: jlong,
) -> Reference /* Object */ {
	unimplemented!("jdk.internal.misc.Unsafe#getReference")
}

pub fn putReference(
	_env: NonNull<JniEnv>,
	_this: Reference,   // jdk.internal.misc.Unsafe
	_object: Reference, // Object
	_offset: jlong,
	_value: Reference, // Object
) -> Reference /* Object */ {
	unimplemented!("jdk.internal.misc.Unsafe#putReference")
}

pub fn getReferenceVolatile(
	_env: NonNull<JniEnv>,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
) -> Reference /* Object */ {
	if object.is_array() {
		let instance = object.extract_array();
		unsafe {
			let array_mut = instance.get_mut();
			return Reference::clone(
				array_mut
					.get_content_mut()
					.get_reference_raw(offset as usize)
					.as_ref(),
			);
		}
	}

	let instance = object.extract_class();
	unsafe {
		let field_value = instance
			.get_mut()
			.get_field_value_raw(offset as usize)
			.as_ptr();
		(*field_value).expect_reference()
	}
}

pub fn putReferenceVolatile(
	_env: NonNull<JniEnv>,
	_this: Reference,   // jdk.internal.misc.Unsafe
	_object: Reference, // java.lang.Object
	_offset: jlong,
	_value: Reference, // java.lang.Object
) -> Reference /* java.lang.Object */ {
	unimplemented!("jdk.internal.misc.Unsafe#putReferenceVolatile")
}

/// Creates the many `{get, put}Ty` and `{get, put}TyVolatile` methods
macro_rules! get_put_methods {
	($($ty:ident),+) => {
		$(
			paste::paste! {
				pub fn [<get $ty:camel>](
					_env: NonNull<JniEnv>,
					_this: Reference,  // jdk.internal.misc.Unsafe
					_object: Reference, // Object
					_offset: jlong
				) -> [<j $ty>] {
					unimplemented!()
				}

				pub fn [<put $ty:camel>](
					_env: NonNull<JniEnv>,
					_this: Reference,  // jdk.internal.misc.Unsafe
					_object: Reference, // Object
					_offset: jlong,
					_value: [<j $ty>]
				) {
					unimplemented!()
				}

				pub fn [<get $ty:camel Volatile>](
					_env: NonNull<JniEnv>,
					_this: Reference,  // jdk.internal.misc.Unsafe
					_object: Reference, // Object
					_offset: jlong
				) -> [<j $ty>] {
					unimplemented!()
				}

				pub fn [<put $ty:camel Volatile>](
					_env: NonNull<JniEnv>,
					_this: Reference,  // jdk.internal.misc.Unsafe
					_object: Reference, // Object
					_offset: jlong,
					_value: [<j $ty>]
				) {
					unimplemented!()
				}

			}
		)+
	};
}

get_put_methods! { boolean, byte, short, char, int, long, float, double }

pub fn unpark(
	_env: NonNull<JniEnv>,
	_this: Reference,   // jdk.internal.misc.Unsafe
	_thread: Reference, // Object
) {
	unimplemented!("jdk.internal.misc.Unsafe#unpark")
}

pub fn park(
	_env: NonNull<JniEnv>,
	_this: Reference, // jdk.internal.misc.Unsafe
	_is_absolute: bool,
	_time: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#park")
}

pub fn fullFence(
	_env: NonNull<JniEnv>,
	_this: Reference, // jdk.internal.misc.Unsafe
) {
	platform::os_arch::ordering::fence();
}

pub fn allocateMemory0(
	_env: NonNull<JniEnv>,
	_this: Reference, // jdk.internal.misc.Unsafe
	_bytes: jlong,
) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#allocateMemory0")
}

pub fn reallocateMemory0(
	_env: NonNull<JniEnv>,
	_this: Reference, // jdk.internal.misc.Unsafe
	_address: jlong,
	_bytes: jlong,
) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#reallocateMemory0")
}

pub fn freeMemory0(
	_env: NonNull<JniEnv>,
	_this: Reference, // jdk.internal.misc.Unsafe
	_address: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#freeMemory0")
}

pub fn setMemory0(
	_env: NonNull<JniEnv>,
	_this: Reference,   // jdk.internal.misc.Unsafe
	_object: Reference, // Object
	_offset: jlong,
	_bytes: jlong,
	_value: jbyte,
) {
	unimplemented!("jdk.internal.misc.Unsafe#setMemory0")
}

pub fn copyMemory0(
	_env: NonNull<JniEnv>,
	_this: Reference,     // jdk.internal.misc.Unsafe
	_src_base: Reference, // Object
	_src_offset: jlong,
	_dest_base: Reference, // Object
	_dest_offset: jlong,
	_bytes: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#copyMemory0")
}

pub fn copySwapMemory0(
	_env: NonNull<JniEnv>,
	_this: Reference,     // jdk.internal.misc.Unsafe
	_src_base: Reference, // Object
	_src_offset: jlong,
	_dest_base: Reference, // Object,
	_dest_offset: jlong,
	_bytes: jlong,
	_elem_size: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#copyMemory0")
}

pub fn objectFieldOffset0(
	_env: NonNull<JniEnv>,
	_this: Reference,  // jdk.internal.misc.Unsafe
	_field: Reference, // java.lang.reflect.Field
) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#objectFieldOffset0")
}

pub fn objectFieldOffset1(
	_env: NonNull<JniEnv>,
	_this: Reference, // jdk.internal.misc.Unsafe
	class: Reference, // java.lang.Class
	name: Reference,  // String
) -> jlong {
	let class = class.extract_mirror();

	let name_str = StringInterner::rust_string_from_java_string(name.extract_class());
	let classref = class.get().expect_class();

	let mut offset = 0;
	for field in classref.fields() {
		if field.is_static() {
			continue;
		}

		if field.name == name_str.as_bytes() {
			return (offset as jlong).into();
		}

		offset += 1;
	}

	// TODO
	panic!("InternalError")
}

pub fn staticFieldOffset0(
	_env: NonNull<JniEnv>,
	_this: Reference,  // jdk.internal.misc.Unsafe
	_field: Reference, // java.lang.reflect.Field
) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#staticFieldOffset0")
}

pub fn staticFieldBase0(
	_env: NonNull<JniEnv>,
	_this: Reference,  // jdk.internal.misc.Unsafe
	_field: Reference, // java.lang.reflect.Field
) -> Reference /* java.lang.Object */ {
	unimplemented!("jdk.internal.misc.Unsafe#staticFieldBase0")
}
pub fn shouldBeInitialized0(
	_env: NonNull<JniEnv>,
	_this: Reference,  // jdk.internal.misc.Unsafe
	_class: Reference, // java.lang.Class
) -> bool {
	unimplemented!("jdk.internal.misc.Unsafe#shouldBeInitialized0")
}
pub fn ensureClassInitialized0(
	env: NonNull<JniEnv>,
	_this: Reference, // jdk.internal.misc.Unsafe
	class: Reference, // java.lang.Class
) {
	let current_thread = unsafe { &mut *JavaThread::for_env(env.as_ptr() as _) };
	let mirror = class.extract_mirror();

	let target_class = mirror.get().expect_class();
	match target_class.initialization_state() {
		ClassInitializationState::Uninit => Class::initialize(&target_class, current_thread),
		ClassInitializationState::InProgress | ClassInitializationState::Init => {},
		// TODO: Is this the best we can do?
		ClassInitializationState::Failed => unreachable!("Failed to ensure class initialization"),
	}
}
pub fn arrayBaseOffset0(
	_env: NonNull<JniEnv>,
	_this: Reference,       // jdk.internal.misc.Unsafe
	array_class: Reference, // java.lang.Class
) -> jint {
	let mirror = array_class.extract_mirror();
	// TODO: InvalidClassException
	let _array = mirror.get().expect_class().unwrap_array_instance();

	// TODO: We don't do byte packing like Hotspot
	0
}
pub fn arrayIndexScale0(
	_env: NonNull<JniEnv>,
	_this: Reference,       // jdk.internal.misc.Unsafe
	array_class: Reference, // java.lang.Class
) -> jint {
	let mirror = array_class.extract_mirror();
	// TODO: InvalidClassException
	let _array = mirror.get().expect_class().unwrap_array_instance();

	// TODO: We don't do byte packing like Hotspot
	1
}
pub fn getLoadAverage0(
	_env: NonNull<JniEnv>,
	_this: Reference,    // jdk.internal.misc.Unsafe
	_loadavg: Reference, // [D
	_nelems: jint,
) -> jint {
	unimplemented!("jdk.internal.misc.Unsafe#getLoadAverage0")
}
