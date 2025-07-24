use crate::classes;
use crate::objects::class::ClassInitializationState;
use crate::objects::instance::array::{Array, ObjectArrayInstanceRef, PrimitiveType, TypeCode};
use crate::objects::instance::class::ClassInstance;
use crate::objects::instance::object::Object;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw, throw_with_ret};

use std::marker::PhantomData;
use std::sync::atomic::Ordering;

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
use common::atomic::{Atomic, AtomicCounterpart};
use instructions::Operand;

include_generated!("native/jdk/internal/misc/def/Unsafe.definitions.rs");
include_generated!("native/jdk/internal/misc/def/Unsafe.registerNatives.rs");

/// Wrapper for unsafe operations
///
/// This does all the work of and performing gets/sets.
///
/// If `object` is null, the offset is treated as a raw pointer.
struct UnsafeMemoryOp<T> {
	object: Reference,
	offset: isize,
	_phantom: PhantomData<T>,
}

impl<T> UnsafeMemoryOp<T> {
	fn new(object: Reference, offset: jlong) -> Self {
		let offset = offset as isize;
		Self {
			object,
			offset,
			_phantom: PhantomData,
		}
	}
}

impl<T> UnsafeMemoryOp<T>
where
	T: UnsafeOpImpl<Output = T>,
	T: PrimitiveType,
	T: AtomicCounterpart,
{
	unsafe fn get(&self) -> T {
		if self.object.is_null() {
			let offset = self.offset;
			let ptr = offset as *const T;
			return unsafe { ptr.read() };
		}

		unsafe { self.object.get::<T>(self.offset as usize) }
	}

	unsafe fn get_volatile(&self) -> T {
		if self.object.is_null() {
			let offset = self.offset;
			let ptr = offset as *const T::Counterpart;
			return unsafe { (&*ptr).load(Ordering::Acquire) };
		}

		let offset = self.offset;
		unsafe {
			let ptr = self.object.get_raw::<T>(offset as usize) as *const T::Counterpart;
			(&*ptr).load(Ordering::Acquire)
		}
	}

	unsafe fn put(&self, value: T) {
		if self.object.is_null() {
			let offset = self.offset;
			let ptr = offset as *mut T;
			unsafe {
				*ptr = value;
			}

			return;
		}

		unsafe { self.object.put::<T>(value, self.offset as usize) }
	}

	unsafe fn put_volatile(&self, value: T) {
		if self.object.is_null() {
			return unsafe { self.__put_raw_volatile(value) };
		}

		if self.object.is_primitive_array() {
			return unsafe { self.__put_array_volatile(value) };
		}

		assert!(self.object.is_class());
		unsafe { self.__put_field_volatile(value) }
	}

	#[doc(hidden)]
	unsafe fn __put_raw_volatile(&self, _value: T) {
		unimplemented!("Volatile raw pointer set")
	}

	#[doc(hidden)]
	unsafe fn __put_array_volatile(&self, _value: T) {
		unimplemented!("Volatile array set")
	}

	#[doc(hidden)]
	unsafe fn __put_field_volatile(&self, _value: T) {
		unimplemented!("Volatile field put")
	}
}

trait UnsafeOpImpl: Sized + AtomicCounterpart {
	type Output;

	unsafe fn get_field_volatile_impl(field_value: *mut Self) -> Self::Output;
	unsafe fn put_field_impl(field_value: *mut Self, value: Self::Output);
}

macro_rules! unsafe_ops {
	($($ty:ident => $operand_ty:ident),+) => {
		paste::paste! {
			$(
			impl UnsafeOpImpl for [<j $ty>] {
				type Output = [<j $ty>];

				#[allow(trivial_numeric_casts)]
				unsafe fn get_field_volatile_impl(field_value: *mut [<j $ty>]) -> Self::Output {
					let field_value_atomic = field_value.cast::<Self::Counterpart>();
					unsafe {
						(&*field_value_atomic).load(Ordering::Acquire)
					}
				}

				#[allow(dropping_copy_types)]
				unsafe fn put_field_impl(field_value: *mut [<j $ty>], value: Self::Output) {
					let old = unsafe { field_value.replace(value) };
					drop(old);
				}
			}
			)+
		}
	};
}

unsafe_ops! {
	boolean => int,
	byte => int,
	short => int,
	char => int,
	int => int,
	long => long,
	float => float,
	double => double
}

pub fn getUncompressedObject(
	_env: JniEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	_address: jlong,
) -> Reference {
	unimplemented!("jdk.internal.misc.Unsafe#getUncompressedObject")
}

pub fn writeback0(
	_env: JniEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	_address: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#writeback0")
}
pub fn writebackPreSync0(
	_env: JniEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
) {
	unimplemented!("jdk.internal.misc.Unsafe#writebackPreSync0")
}
pub fn writebackPostSync0(
	_env: JniEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
) {
	unimplemented!("jdk.internal.misc.Unsafe#writebackPostSync0")
}

pub fn defineClass0(
	_env: JniEnv,
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

// Creates an instance of `class` without running its constructor, and initializes the class
// if necessary.
//
// throws InstantiationException
pub fn allocateInstance(
	env: JniEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	class: Reference, // java.lang.Class
) -> Reference {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let target = class.extract_target_class();
	if let Throws::Exception(e) = target.initialize(thread) {
		e.throw(thread);
		return Reference::null();
	}

	Reference::class(ClassInstance::new(target))
}

pub fn throwException(
	_env: JniEnv,
	_this: Reference,      // jdk.internal.misc.Unsafe
	_exception: Reference, // java.lang.Throwable
) {
	unimplemented!("jdk.internal.misc.Unsafe#throwException")
}

pub fn compareAndSetInt(
	env: JniEnv,
	this: Reference,   // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: jint,
	value: jint,
) -> jboolean {
	compareAndExchangeInt(env, this, object, offset, expected, value) == expected
}

pub fn compareAndExchangeInt(
	_env: JniEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: jint,
	value: jint,
) -> jint {
	let op = UnsafeMemoryOp::<jint>::new(object, offset);
	unsafe {
		let current_field_value = op.get();
		if current_field_value == expected {
			op.put(value);
			return expected;
		}

		current_field_value
	}
}

pub fn compareAndSetLong(
	env: JniEnv,
	this: Reference,   // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: jlong,
	value: jlong,
) -> jboolean {
	compareAndExchangeLong(env, this, object, offset, expected, value) == expected
}

pub fn compareAndExchangeLong(
	_env: JniEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: jlong,
	value: jlong,
) -> jlong {
	let op = UnsafeMemoryOp::<jlong>::new(object, offset);

	unsafe {
		let current_field_value = op.get();
		if current_field_value == expected {
			op.put(value);
			return expected;
		}

		current_field_value
	}
}

pub fn compareAndSetReference(
	env: JniEnv,
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
	_env: JniEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	expected: Reference, // Object
	value: Reference,    // Object
) -> Reference {
	unsafe {
		let current_value = object.get_raw::<Reference>(offset as usize);
		if &*current_value == &expected {
			*current_value = value;
			return expected;
		}

		*current_value
	}
}

pub fn getReference(
	_env: JniEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
) -> Reference /* Object */ {
	unsafe { object.get::<Reference>(offset as usize) }
}

pub fn putReference(
	_env: JniEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
	value: Reference, // Object
) {
	// TODO: In hotspot, a mirror holds the static fields of a class. I guess we should do the same?
	if object.is_mirror() {
		let target_class = object.extract_target_class();
		unsafe {
			target_class.set_static_field(offset as usize, Operand::Reference(value));
		}
		return;
	}

	unsafe { object.put::<Reference>(value, offset as usize) }
}

pub fn getReferenceVolatile(
	_env: JniEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // Object
	offset: jlong,
) -> Reference /* Object */ {
	tracing::warn!("(!!!) Unsafe#getReferenceVolatile not actually volatile");
	getReference(_env, _this, object, offset)
}

pub fn putReferenceVolatile(
	_env: JniEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	object: Reference, // java.lang.Object
	offset: jlong,
	value: Reference, // java.lang.Object
) {
	tracing::warn!("(!!!) Unsafe#putReferenceVolatile not actually volatile");
	putReference(_env, _this, object, offset, value)
}

/// Creates the many `{get, put}Ty` and `{get, put}TyVolatile` methods
macro_rules! get_put_methods {
	($($ty:ident),+) => {
		$(
			paste::paste! {
				pub fn [<get $ty:camel>](
					_env: JniEnv,
					_this: Reference,  // jdk.internal.misc.Unsafe
					object: Reference, // Object
					offset: jlong
				) -> [<j $ty>] {
					let op = UnsafeMemoryOp::<[<j $ty>]>::new(object, offset);
					unsafe { op.get() }
				}

				pub fn [<put $ty:camel>](
					_env: JniEnv,
					_this: Reference,  // jdk.internal.misc.Unsafe
					object: Reference, // Object
					offset: jlong,
					value: [<j $ty>]
				) {
					let op = UnsafeMemoryOp::<[<j $ty>]>::new(object, offset);
					unsafe { op.put(value) }
				}

				pub fn [<get $ty:camel Volatile>](
					_env: JniEnv,
					_this: Reference,  // jdk.internal.misc.Unsafe
					object: Reference, // Object
					offset: jlong
				) -> [<j $ty>] {
					let op = UnsafeMemoryOp::<[<j $ty>]>::new(object, offset);
					unsafe { op.get_volatile() }
				}

				pub fn [<put $ty:camel Volatile>](
					_env: JniEnv,
					_this: Reference,  // jdk.internal.misc.Unsafe
					object: Reference, // Object
					offset: jlong,
					value: [<j $ty>]
				) {
					let op = UnsafeMemoryOp::<[<j $ty>]>::new(object, offset);
					unsafe { op.put_volatile(value) }
				}
			}
		)+
	};
}

get_put_methods! { boolean, byte, short, char, int, long, float, double }

pub fn unpark(
	_env: JniEnv,
	_this: Reference,   // jdk.internal.misc.Unsafe
	_thread: Reference, // Object
) {
	unimplemented!("jdk.internal.misc.Unsafe#unpark")
}

pub fn park(
	_env: JniEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	_is_absolute: bool,
	_time: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#park")
}

pub fn fullFence(
	_env: JniEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
) {
	platform::arch::ordering::fence();
}

pub fn allocateMemory0(
	_env: JniEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	_bytes: jlong,
) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#allocateMemory0")
}

pub fn reallocateMemory0(
	_env: JniEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	_address: jlong,
	_bytes: jlong,
) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#reallocateMemory0")
}

pub fn freeMemory0(
	_env: JniEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	_address: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#freeMemory0")
}

pub fn setMemory0(
	_env: JniEnv,
	_this: Reference,   // jdk.internal.misc.Unsafe
	_object: Reference, // Object
	_offset: jlong,
	_bytes: jlong,
	_value: jbyte,
) {
	unimplemented!("jdk.internal.misc.Unsafe#setMemory0")
}

pub fn copyMemory0(
	_env: JniEnv,
	_this: Reference,    // jdk.internal.misc.Unsafe
	src_base: Reference, // java.lang.Object
	src_offset: jlong,
	dest_base: Reference, // java.lang.Object
	dest_offset: jlong,
	bytes: jlong,
) {
	let size = bytes as usize;

	if src_base.is_primitive_array() {
		let src_base = src_base.extract_primitive_array();
		let dest_base = dest_base.extract_primitive_array();

		let src_base_element_size = src_base.scale();
		let dest_base_element_size = dest_base.scale();

		let src_base_offset = (src_offset as usize) * src_base_element_size;
		let dest_base_offset = (dest_offset as usize) * dest_base_element_size;

		unsafe {
			let src_base_ptr = src_base.field_base().add(src_base_offset);
			let dest_base_ptr = dest_base.field_base().add(dest_base_offset);
			src_base_ptr.copy_to(dest_base_ptr, size * src_base_element_size);
			return;
		}
	}

	todo!()
	// let src_base_ptr;
	// let dest_base_ptr;
	// unsafe {
	// 	src_base_ptr = src_base.get(src_offset as usize);
	// 	dest_base_ptr = dest_base.get(dest_offset as usize);
	// 	src_base_ptr.copy_to(dest_base_ptr, size);
	// }
}

pub fn copySwapMemory0(
	_env: JniEnv,
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
	_env: JniEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	_field: Reference, // java.lang.reflect.Field
) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#objectFieldOffset0")
}

pub fn objectFieldOffset1(
	env: JniEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	class: Reference, // java.lang.Class
	name: Reference,  // String
) -> jlong {
	let class = class.extract_mirror();

	let name_str = classes::java::lang::String::extract(name.extract_class());
	let classref = class.target_class();

	for field in classref.instance_fields() {
		if field.name.as_str() == name_str {
			return field.offset() as jlong;
		}
	}

	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	throw_with_ret!(0, thread, InternalError);
}

pub fn staticFieldOffset0(
	_env: JniEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	_field: Reference, // java.lang.reflect.Field
) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#staticFieldOffset0")
}

pub fn staticFieldBase0(
	_env: JniEnv,
	_this: Reference,  // jdk.internal.misc.Unsafe
	_field: Reference, // java.lang.reflect.Field
) -> Reference /* java.lang.Object */ {
	unimplemented!("jdk.internal.misc.Unsafe#staticFieldBase0")
}
pub fn shouldBeInitialized0(
	_env: JniEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	class: Reference, // java.lang.Class
) -> bool {
	assert!(!class.is_null(), "should be checked in the jdk");

	let class = class.extract_target_class();
	class.initialization_state() != ClassInitializationState::Init
}
pub fn ensureClassInitialized0(
	env: JniEnv,
	_this: Reference, // jdk.internal.misc.Unsafe
	class: Reference, // java.lang.Class
) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	let mirror = class.extract_mirror();

	let target_class = mirror.target_class();
	if let Throws::Exception(e) = target_class.initialize(thread) {
		e.throw(thread);
	}
}

fn scale_of(array_class: Reference) -> Throws<jint> {
	if array_class.is_null() {
		throw!(@DEFER InvalidClassException);
	}

	let array_class_mirror = array_class.extract_mirror();
	let array_class = array_class_mirror.target_class();
	if !array_class.is_array() {
		throw!(@DEFER InvalidClassException);
	}

	let array_descriptor = array_class.unwrap_array_instance();
	if array_descriptor.is_primitive() {
		// Safe to unwrap, just verified this is a primitive array
		let component_type_code = array_descriptor.component.as_array_type_code().unwrap();
		let type_code = TypeCode::from_u8(component_type_code);
		Throws::Ok(type_code.size() as jint)
	} else {
		let scale = size_of::<<ObjectArrayInstanceRef as Array>::Component>() as jint;
		Throws::Ok(scale)
	}
}

pub fn arrayBaseOffset0(
	env: JniEnv,
	_this: Reference,       // jdk.internal.misc.Unsafe
	array_class: Reference, // java.lang.Class
) -> jint {
	// Just here to verify that the array is valid
	if let Throws::Exception(e) = scale_of(array_class) {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		e.throw(thread);
		return 0;
	}

	// The Java code doesn't need to know the real base of the array. Object::get() and Object::put()
	// base on the start of the object's fields already, so the indexing is transparent.
	0
}
pub fn arrayIndexScale0(
	env: JniEnv,
	_this: Reference,       // jdk.internal.misc.Unsafe
	array_class: Reference, // java.lang.Class
) -> jint {
	match scale_of(array_class) {
		Throws::Ok(scale) => scale,
		Throws::Exception(e) => {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			e.throw(thread);
			0
		},
	}
}
pub fn getLoadAverage0(
	_env: JniEnv,
	_this: Reference,    // jdk.internal.misc.Unsafe
	_loadavg: Reference, // [D
	_nelems: jint,
) -> jint {
	unimplemented!("jdk.internal.misc.Unsafe#getLoadAverage0")
}
