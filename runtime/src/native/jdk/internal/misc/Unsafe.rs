use crate::class_instance::{ArrayInstance, Instance};
use crate::native::JniEnv;
use crate::objects::class::ClassInitializationState;
use crate::reference::Reference;
use crate::string_interner::StringInterner;
use crate::thread::JavaThread;

use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::atomic::{
	AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicU16, Ordering,
};

use ::jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
use common::atomic::{Atomic, AtomicF32, AtomicF64};
use common::traits::PtrType;
use instructions::Operand;

include_generated!("native/jdk/internal/misc/def/Unsafe.definitions.rs");
include_generated!("native/jdk/internal/misc/def/Unsafe.registerNatives.rs");

/// Wrapper for unsafe operations
///
/// This does all the work of checking the object type (class, array, or null) and performing gets/sets.
struct UnsafeMemoryOp<T, A> {
	object: Reference,
	offset: usize,
	_phantom: PhantomData<(T, A)>,
}

impl<T, A> UnsafeMemoryOp<T, A> {
	fn new(object: Reference, offset: jlong) -> Self {
		let offset = offset as usize;
		Self {
			object,
			offset,
			_phantom: PhantomData,
		}
	}
}

impl<T, A> UnsafeMemoryOp<T, A>
where
	T: UnsafeOpImpl<Output = T>,
	A: Atomic<Output = T>,
{
	unsafe fn get(&self) -> T {
		if self.object.is_null() {
			return self.__get_raw();
		}

		if self.object.is_array() {
			return self.__get_array();
		}

		assert!(self.object.is_class());
		self.__get_field()
	}

	#[doc(hidden)]
	unsafe fn __get_raw(&self) -> T {
		let offset = self.offset;
		let ptr = offset as *const T;
		unsafe { ptr.read() }
	}

	#[doc(hidden)]
	unsafe fn __get_array(&self) -> T {
		let offset = self.offset;
		let instance = self.object.extract_array();
		let array_mut = instance.get_mut();
		unsafe { <T as UnsafeOpImpl>::get_array_impl(array_mut, offset) }
	}

	#[doc(hidden)]
	unsafe fn __get_field(&self) -> T {
		let offset = self.offset;
		let instance = self.object.extract_class();
		unsafe {
			let field_value = instance.get_mut().get_field_value_raw(offset).as_ptr();
			<T as UnsafeOpImpl>::get_field_impl(field_value)
		}
	}

	unsafe fn get_volatile(&self) -> T {
		if self.object.is_null() {
			return unsafe { self.__get_raw_volatile() };
		}

		if self.object.is_array() {
			return unsafe { self.__get_array_volatile() };
		}

		assert!(self.object.is_class());
		unsafe { self.__get_field_volatile() }
	}

	#[doc(hidden)]
	unsafe fn __get_raw_volatile(&self) -> T {
		let offset = self.offset;
		let ptr = offset as *const T;
		let atomic_ptr: &A = unsafe { &*ptr.cast() };
		unsafe { atomic_ptr.load(Ordering::Acquire) }
	}

	#[doc(hidden)]
	unsafe fn __get_array_volatile(&self) -> T {
		unimplemented!("Volatile array access")
	}

	#[doc(hidden)]
	unsafe fn __get_field_volatile(&self) -> T {
		unimplemented!("Volatile field access")
	}

	unsafe fn put(&self, value: T) {
		if self.object.is_null() {
			return unsafe { self.__put_raw(value) };
		}

		if self.object.is_array() {
			return unsafe { self.__put_array(value) };
		}

		assert!(self.object.is_class());
		unsafe { self.__put_field(value) }
	}

	#[doc(hidden)]
	unsafe fn __put_raw(&self, value: T) {
		let offset = self.offset;
		let ptr = offset as *mut T;
		unsafe {
			*ptr = value;
		}
	}

	#[doc(hidden)]
	unsafe fn __put_array(&self, value: T) {
		let offset = self.offset;
		let instance = self.object.extract_array();
		let array_mut = instance.get_mut();
		unsafe { <T as UnsafeOpImpl>::put_array_impl(array_mut, offset, value) }
	}

	#[doc(hidden)]
	unsafe fn __put_field(&self, value: T) {
		let offset = self.offset;
		let instance = self.object.extract_class();
		unsafe {
			let field_value = instance.get_mut().get_field_value_raw(offset).as_ptr();
			<T as UnsafeOpImpl>::put_field_impl(field_value, value)
		}
	}

	unsafe fn put_volatile(&self, value: T) {
		if self.object.is_null() {
			return unsafe { self.__put_raw_volatile(value) };
		}

		if self.object.is_array() {
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

trait UnsafeOpImpl: Sized {
	type Output;
	unsafe fn get_array_impl(array: &mut ArrayInstance, offset: usize) -> Self::Output;
	unsafe fn put_array_impl(array: &mut ArrayInstance, offset: usize, value: Self::Output);
	unsafe fn get_field_impl(field_value: *mut Operand<Reference>) -> Self::Output;
	unsafe fn put_field_impl(field_value: *mut Operand<Reference>, value: Self::Output);
}

// bool implemented separated due to `get_field_impl` cast
impl UnsafeOpImpl for jboolean {
	type Output = jboolean;

	unsafe fn get_array_impl(array: &mut ArrayInstance, offset: usize) -> jboolean {
		let raw = array.get_content_mut().get_boolean_raw(offset).as_ptr();
		unsafe { (&*raw).clone() }
	}

	#[allow(dropping_copy_types)]
	unsafe fn put_array_impl(array: &mut ArrayInstance, offset: usize, value: jboolean) {
		let raw = array.get_content_mut().get_boolean_raw(offset);
		let old = unsafe { raw.replace(value) };
		drop(old);
	}

	unsafe fn get_field_impl(field_value: *mut Operand<Reference>) -> jboolean {
		unsafe { (*field_value).expect_int() != 0 }
	}

	#[allow(dropping_copy_types)]
	unsafe fn put_field_impl(field_value: *mut Operand<Reference>, value: Self::Output) {
		let old = unsafe { field_value.replace(Operand::from(value)) };
		drop(old);
	}
}

macro_rules! unsafe_ops {
	($($ty:ident => $operand_ty:ident),+) => {
		paste::paste! {
			$(
			impl UnsafeOpImpl for [<j $ty>] {
				type Output = [<j $ty>];

				unsafe fn get_array_impl(array: &mut ArrayInstance, offset: usize) -> Self::Output {
					let raw = array
						.get_content_mut()
						.[<get_ $ty _raw>](offset).as_ptr();
					unsafe { (&*raw).clone() }
				}

				#[allow(dropping_copy_types)]
				unsafe fn put_array_impl(
					array: &mut ArrayInstance,
					offset: usize,
					value: Self::Output,
				) {
					let raw = array
						.get_content_mut()
						.[<get_ $ty _raw>](offset);
					let old = unsafe { raw.replace(value) };
					drop(old);
				}

				#[allow(trivial_numeric_casts)]
				unsafe fn get_field_impl(field_value: *mut Operand<Reference>) -> Self::Output {
					unsafe {
						(*field_value).[<expect_ $operand_ty>]() as [<j $ty>]
					}
				}

				#[allow(dropping_copy_types)]
				unsafe fn put_field_impl(field_value: *mut Operand<Reference>, value: Self::Output) {
					let old = unsafe { field_value.replace(Operand::from(value)) };
					drop(old);
				}
			}
			)+
		}
	};
}

unsafe_ops! {
	byte => int,
	short => int,
	char => int,
	int => int,
	long => long,
	float => float,
	double => double
}

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
	let op = UnsafeMemoryOp::<jint, AtomicI32>::new(object, offset);
	unsafe {
		let current_field_value = op.get();
		if current_field_value == expected {
			op.put(value);
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
	let op = UnsafeMemoryOp::<jlong, AtomicI64>::new(object, offset);

	unsafe {
		let current_field_value = op.get();
		if current_field_value == expected {
			op.put(value);
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
	tracing::warn!("(!!!) Unsafe#getReferenceVolatile not actually volatile");
	getReference(_env, _this, object, offset)
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
	($(($ty:ident; $atomic_ty:ident)),+) => {
		$(
			paste::paste! {
				pub fn [<get $ty:camel>](
					_env: NonNull<JniEnv>,
					_this: Reference,  // jdk.internal.misc.Unsafe
					object: Reference, // Object
					offset: jlong
				) -> [<j $ty>] {
					let op = UnsafeMemoryOp::<[<j $ty>], $atomic_ty>::new(object, offset);
					unsafe { op.get() }
				}

				pub fn [<put $ty:camel>](
					_env: NonNull<JniEnv>,
					_this: Reference,  // jdk.internal.misc.Unsafe
					object: Reference, // Object
					offset: jlong,
					value: [<j $ty>]
				) {
					let op = UnsafeMemoryOp::<[<j $ty>], $atomic_ty>::new(object, offset);
					unsafe { op.put(value) }
				}

				pub fn [<get $ty:camel Volatile>](
					_env: NonNull<JniEnv>,
					_this: Reference,  // jdk.internal.misc.Unsafe
					object: Reference, // Object
					offset: jlong
				) -> [<j $ty>] {
					let op = UnsafeMemoryOp::<[<j $ty>], $atomic_ty>::new(object, offset);
					unsafe { op.get_volatile() }
				}

				pub fn [<put $ty:camel Volatile>](
					_env: NonNull<JniEnv>,
					_this: Reference,  // jdk.internal.misc.Unsafe
					object: Reference, // Object
					offset: jlong,
					value: [<j $ty>]
				) {
					let op = UnsafeMemoryOp::<[<j $ty>], $atomic_ty>::new(object, offset);
					unsafe { op.put_volatile(value) }
				}
			}
		)+
	};
}

get_put_methods! { (boolean; AtomicBool), (byte; AtomicI8), (short; AtomicI16), (char; AtomicU16), (int; AtomicI32), (long; AtomicI64), (float; AtomicF32), (double; AtomicF64) }

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
	platform::arch::ordering::fence();
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

		if field.name.as_str() == name_str {
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
		ClassInitializationState::Uninit => target_class.initialize(current_thread),
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
