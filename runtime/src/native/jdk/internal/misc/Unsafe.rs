use crate::classes;
use crate::native::jni::JniEnvExt;
use crate::native::jni::references::JObjectExt;
use crate::objects::class::ClassInitializationState;
use crate::objects::instance::array::{Array, ObjectArrayInstanceRef, PrimitiveType, TypeCode};
use crate::objects::instance::class::ClassInstance;
use crate::objects::instance::object::Object;
use crate::objects::reference::Reference;
use crate::thread::exceptions::{Throws, throw, throw_with_ret};

use std::alloc::{AllocError, Layout};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::atomic::Ordering;

use ::jni::env::{JniEnv, JniNativeMethod};
use ::jni::objects::{JArray, JByteArray, JClass, JDoubleArray, JObject, JString, JThrowable};
use ::jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
use classfile::accessflags::FieldAccessFlags;
use common::atomic::{Atomic, AtomicCounterpart};
use native_macros::jni_call;

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
	fn new(object: JObject, offset: jlong) -> Self {
		let object = unsafe { object.to_reference_maybe_null() };

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
		unsafe { self.object.atomic_get::<T>(offset as usize) }
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

#[jni_call]
extern "C" fn getUncompressedObject(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	_address: jlong,
) -> JObject {
	unimplemented!("jdk.internal.misc.Unsafe#getUncompressedObject")
}

#[jni_call]
extern "C" fn writeBack0(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	_address: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#writeBack0")
}

#[jni_call]
extern "C" fn writeBackPreSync0(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
) {
	unimplemented!("jdk.internal.misc.Unsafe#writeBackPreSync0")
}

#[jni_call]
extern "C" fn writeBackPostSync0(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
) {
	unimplemented!("jdk.internal.misc.Unsafe#writeBackPostSync0")
}

#[jni_call]
extern "C" fn defineClass0(
	_env: JniEnv,
	_this: JObject,     // jdk.internal.misc.Unsafe
	_name: JString,     // java.lang.String
	_bytes: JByteArray, // [B
	_offset: jint,
	_length: jint,
	_loader: JObject,            // java.lang.ClassLoader
	_protection_domain: JObject, // java.security.ProtectionDomain
) -> JObject {
	unimplemented!("jdk.internal.misc.Unsafe#defineClass0")
}

// Creates an instance of `class` without running its constructor, and initializes the class
// if necessary.
//
// throws InstantiationException
#[jni_call]
extern "C" fn allocateInstance(
	env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	class: JClass,
) -> JObject {
	let thread = unsafe { env.thread() };

	let class = unsafe { class.to_reference() }.expect("class should be non-null");

	let target = class.extract_target_class();
	if let Throws::Exception(e) = target.initialize(thread) {
		e.throw(thread);
		return JObject::null();
	}

	thread
		.jni_refs()
		.allocate_wrapped(Reference::class(ClassInstance::new(target)))
}

#[jni_call]
extern "C" fn throwException(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	_exception: JThrowable,
) {
	unimplemented!("jdk.internal.misc.Unsafe#throwException")
}

#[jni_call]
extern "C" fn compareAndSetInt(
	env: JniEnv,
	this: JObject, // jdk.internal.misc.Unsafe
	object: JObject,
	offset: jlong,
	expected: jint,
	value: jint,
) -> jboolean {
	compareAndExchangeInt(env, this, object, offset, expected, value) == expected
}

#[jni_call]
extern "C" fn compareAndExchangeInt(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	object: JObject,
	offset: jlong,
	expected: jint,
	value: jint,
) -> jint {
	let object = unsafe { object.to_reference_maybe_null() };

	if object.is_null() {
		let atomic = unsafe {
			let raw = offset as *const <jint as AtomicCounterpart>::Counterpart;
			&*raw
		};
		return atomic
			.compare_exchange(expected, value, Ordering::Acquire, Ordering::Relaxed)
			.unwrap_or_else(|v| v);
	}

	unsafe { object.compare_exchange::<jint>(offset as usize, expected, value) }
}

#[jni_call]
extern "C" fn compareAndSetLong(
	env: JniEnv,
	this: JObject, // jdk.internal.misc.Unsafe
	object: JObject,
	offset: jlong,
	expected: jlong,
	value: jlong,
) -> jboolean {
	compareAndExchangeLong(env, this, object, offset, expected, value) == expected
}

#[jni_call]
extern "C" fn compareAndExchangeLong(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	object: JObject,
	offset: jlong,
	expected: jlong,
	value: jlong,
) -> jlong {
	let object = unsafe { object.to_reference_maybe_null() };

	if object.is_null() {
		let atomic = unsafe {
			let raw = offset as *const <jlong as AtomicCounterpart>::Counterpart;
			&*raw
		};
		return atomic
			.compare_exchange(expected, value, Ordering::Acquire, Ordering::Relaxed)
			.unwrap_or_else(|v| v);
	}

	unsafe { object.compare_exchange::<jlong>(offset as usize, expected, value) }
}

#[jni_call]
extern "C" fn compareAndSetReference(
	env: JniEnv,
	this: JObject, // jdk.internal.misc.Unsafe
	object: JObject,
	offset: jlong,
	expected: JObject,
	value: JObject,
) -> jboolean {
	compareAndExchangeReference(env, this, object, offset, expected, value) == expected
}

#[jni_call]
extern "C" fn compareAndExchangeReference(
	env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	object: JObject,
	offset: jlong,
	expected: JObject,
	value: JObject,
) -> JObject {
	let thread = unsafe { env.thread() };

	let Some(object) = (unsafe { object.to_reference() }) else {
		panic!("should be checked by the jdk");
	};

	let expected = unsafe { expected.to_reference_maybe_null() };
	let value = unsafe { value.to_reference_maybe_null() };

	let expected_usize = expected.raw_tagged() as usize;
	let value_usize = value.raw_tagged() as usize;

	let ret =
		unsafe { object.compare_exchange::<usize>(offset as usize, expected_usize, value_usize) };

	// SAFETY: Both the `current` and `new` values are valid references
	let r = unsafe { Reference::from_raw(ret as *mut ()) };
	thread.jni_refs().allocate_wrapped(r)
}

#[jni_call]
extern "C" fn getReference(env: JniEnv, _this: JObject, object: JObject, offset: jlong) -> JObject {
	let thread = unsafe { env.thread() };

	let Some(object) = (unsafe { object.to_reference() }) else {
		panic!("should be checked by the jdk");
	};

	let r = unsafe { object.get::<Reference>(offset as usize) };
	thread.jni_refs().allocate_wrapped(r)
}

#[jni_call]
extern "C" fn putReference(
	_env: JniEnv,
	_this: JObject,
	object: JObject,
	offset: jlong,
	value: JObject,
) {
	let Some(object) = (unsafe { object.to_reference() }) else {
		panic!("should be checked by the jdk");
	};

	let value = unsafe { value.to_reference_maybe_null() };

	unsafe { object.put::<Reference>(value, offset as usize) }
}

#[jni_call]
extern "C" fn getReferenceVolatile(
	env: JniEnv,
	_this: JObject,
	object: JObject,
	offset: jlong,
) -> JObject {
	let thread = unsafe { env.thread() };

	let Some(object) = (unsafe { object.to_reference() }) else {
		panic!("should be checked by the jdk");
	};

	// SAFETY: Assuming that the caller provided the offset to a real, tagged reference
	let r = unsafe {
		let raw_tagged_ref = object.atomic_get::<usize>(offset as usize);
		Reference::from_raw(raw_tagged_ref as *mut ())
	};
	thread.jni_refs().allocate_wrapped(r)
}

#[jni_call]
extern "C" fn putReferenceVolatile(
	_env: JniEnv,
	_this: JObject,
	object: JObject,
	offset: jlong,
	value: JObject,
) {
	let Some(object) = (unsafe { object.to_reference() }) else {
		panic!("should be checked by the jdk");
	};

	let value = unsafe { value.to_reference_maybe_null() };

	unsafe { object.atomic_store::<usize>(value.raw_tagged() as usize, offset as usize) }
}

/// Creates the many `{get, put}Ty` and `{get, put}TyVolatile` methods
macro_rules! get_put_methods {
	($($ty:ident),+) => {
		$(
			paste::paste! {
				#[jni_call]
                extern "C" fn [<get $ty:camel>](
					_env: JniEnv,
					_this: JObject,  // jdk.internal.misc.Unsafe
					object: JObject,
					offset: jlong
				) -> [<j $ty>] {
					let op = UnsafeMemoryOp::<[<j $ty>]>::new(object, offset);
					unsafe { op.get() }
				}

				#[jni_call]
                extern "C" fn [<put $ty:camel>](
					_env: JniEnv,
					_this: JObject,  // jdk.internal.misc.Unsafe
					object: JObject,
					offset: jlong,
					value: [<j $ty>]
				) {
					let op = UnsafeMemoryOp::<[<j $ty>]>::new(object, offset);
					unsafe { op.put(value) }
				}

				#[jni_call]
                extern "C" fn [<get $ty:camel Volatile>](
					_env: JniEnv,
					_this: JObject,  // jdk.internal.misc.Unsafe
					object: JObject,
					offset: jlong
				) -> [<j $ty>] {
					let op = UnsafeMemoryOp::<[<j $ty>]>::new(object, offset);
					unsafe { op.get_volatile() }
				}

				#[jni_call]
                extern "C" fn [<put $ty:camel Volatile>](
					_env: JniEnv,
					_this: JObject,  // jdk.internal.misc.Unsafe
					object: JObject,
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

#[jni_call]
extern "C" fn valueHeaderSize(_env: JniEnv, _this: JObject, _class: JClass) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#valueHeaderSize")
}

#[jni_call]
extern "C" fn isFlatField(_env: JniEnv, _this: JObject, _obj: JObject) -> jboolean {
	unimplemented!("jdk.internal.misc.Unsafe#isFlatField")
}

#[jni_call]
extern "C" fn hasNullMarker(_env: JniEnv, _this: JObject, _obj: JObject) -> jboolean {
	unimplemented!("jdk.internal.misc.Unsafe#hasNullMarker")
}

#[jni_call]
extern "C" fn nullMarkerOffset(_env: JniEnv, _this: JObject, _obj: JObject) -> jint {
	unimplemented!("jdk.internal.misc.Unsafe#nullMarkerOffset")
}

#[jni_call]
extern "C" fn arrayLayout(_env: JniEnv, _this: JObject, _array: JArray) -> jint {
	unimplemented!("jdk.internal.misc.Unsafe#arrayLayout")
}

#[jni_call]
extern "C" fn fieldLayout(_env: JniEnv, _this: JObject, _obj: JObject) -> jint {
	unimplemented!("jdk.internal.misc.Unsafe#fieldLayout")
}

#[jni_call]
extern "C" fn newSpecialArray(
	_env: JniEnv,
	_this: JObject,
	_elem_class: JClass,
	_len: jint,
	_layout_kind: jint,
) -> JArray {
	unimplemented!("jdk.internal.misc.Unsafe#newSpecialArray")
}

#[jni_call]
extern "C" fn getFlatValue(
	_env: JniEnv,
	_this: JObject,
	_obj: JObject,
	_offset: jlong,
	_layout_kind: jint,
	_vc: JClass,
) -> JObject {
	unimplemented!("jdk.internal.misc.Unsafe#getFlatValue")
}

#[jni_call]
extern "C" fn putFlatValue(
	_env: JniEnv,
	_this: JObject,
	_obj: JObject,
	_offset: jlong,
	_layout_kind: jint,
	_vc: JClass,
	_value: JObject,
) {
	unimplemented!("jdk.internal.misc.Unsafe#putFlatValue")
}

#[jni_call]
extern "C" fn unpark(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	_thread: JObject,
) {
	unimplemented!("jdk.internal.misc.Unsafe#unpark")
}

#[jni_call]
extern "C" fn park(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	_is_absolute: jboolean,
	_time: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#park")
}

#[jni_call]
extern "C" fn fullFence(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
) {
	std::sync::atomic::fence(Ordering::SeqCst);
}

/// Allocator used in `{allocate,reallocate,free}Memory0`
struct UnsafeAllocator;

impl UnsafeAllocator {
	const HEADER_LAYOUT: Layout = {
		let Ok(layout) = Layout::new::<usize>().align_to(Self::ALIGNMENT) else {
			panic!();
		};

		layout
	};
	const ALIGNMENT: usize = align_of::<libc::max_align_t>();

	fn allocate(&self, size: usize) -> Result<NonNull<u8>, AllocError> {
		let layout = Layout::from_size_align(size, Self::ALIGNMENT).map_err(|_| AllocError)?;
		let (layout, _) = Self::HEADER_LAYOUT.extend(layout).map_err(|_| AllocError)?;

		let ptr = unsafe {
			let ptr = NonNull::new(std::alloc::alloc(layout)).ok_or(AllocError)?;
			ptr.cast::<usize>().write(layout.size());
			ptr.byte_add(Self::HEADER_LAYOUT.size())
		};

		Ok(ptr)
	}

	unsafe fn read_header(ptr: *mut u8) -> (*mut u8, Layout) {
		// SAFETY: Assuming this pointer came from `allocateMemory0()`
		let (header_start, alloc_size) = unsafe {
			let header_start = ptr.byte_sub(Self::HEADER_LAYOUT.size());
			(header_start, header_start.cast::<usize>().read())
		};

		// NOTE: `alloc_size` includes itself
		(
			header_start,
			Layout::from_size_align(alloc_size, Self::ALIGNMENT).unwrap(),
		)
	}

	unsafe fn deallocate(&self, ptr: *mut u8) {
		let (header_start, layout) = unsafe { Self::read_header(ptr) };

		unsafe {
			std::alloc::dealloc(header_start, layout);
		}
	}
}

static UNSAFE_ALLOCATOR: UnsafeAllocator = UnsafeAllocator;

#[jni_call]
extern "C" fn allocateMemory0(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	bytes: jlong,
) -> jlong {
	let Ok(size) = usize::try_from(bytes) else {
		return 0; // 0 throws an OutOfMemoryError
	};

	UNSAFE_ALLOCATOR
		.allocate(size)
		.map_or(0, |ptr| ptr.as_ptr() as jlong)
}

#[jni_call]
extern "C" fn reallocateMemory0(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	_address: jlong,
	_bytes: jlong,
) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#reallocateMemory0")
}

#[jni_call]
extern "C" fn freeMemory0(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	address: jlong,
) {
	if address == 0 {
		return; // Just in case...
	}

	unsafe { UNSAFE_ALLOCATOR.deallocate(address as *mut u8) }
}

#[jni_call]
extern "C" fn setMemory0(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	_object: JObject,
	_offset: jlong,
	_bytes: jlong,
	_value: jbyte,
) {
	unimplemented!("jdk.internal.misc.Unsafe#setMemory0")
}

#[jni_call]
extern "C" fn copyMemory0(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	src_base: JObject,
	src_offset: jlong,
	dest_base: JObject,
	dest_offset: jlong,
	size: jlong,
) {
	debug_assert!(size >= 0 && src_offset >= 0 && dest_offset >= 0);
	if size == 0 {
		return; // Nothing to copy
	}

	let src_base = unsafe { src_base.to_reference_maybe_null() };
	let dest_base = unsafe { dest_base.to_reference_maybe_null() };

	let size = size as usize;

	let src = unsafe { src_base.field_base().add(src_offset as usize) };
	let dest = unsafe { dest_base.field_base().add(dest_offset as usize) };
	unsafe { src.copy_to(dest, size) };
}

#[jni_call]
extern "C" fn copySwapMemory0(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	_src_base: JObject,
	_src_offset: jlong,
	_dest_base: JObject,
	_dest_offset: jlong,
	_bytes: jlong,
	_elem_size: jlong,
) {
	unimplemented!("jdk.internal.misc.Unsafe#copyMemory0")
}

#[jni_call]
extern "C" fn objectFieldOffset0(
	env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	field: JObject, // java.lang.reflect.Field
) -> jlong {
	let thread = unsafe { env.thread() };

	let Some(field) = (unsafe { field.to_reference() }) else {
		panic!("should be checked by the jdk");
	};

	let instance = field.extract_class();

	let target = classes::java::lang::reflect::Field::clazz(instance);
	let slot = classes::java::lang::reflect::Field::slot(instance);

	let field = target
		.target_class()
		.fields()
		.nth(slot as usize)
		.expect("malformed java.lang.reflect.Field");
	if field.is_static() {
		throw_with_ret!(0, thread, IllegalArgumentException);
	}

	field.offset() as jlong
}

#[jni_call]
extern "C" fn knownObjectFieldOffset0(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	class: JClass,
	name: JString,
) -> jlong {
	// error codes
	const NOT_FOUND: jlong = -1;
	const IS_STATIC: jlong = -2;

	let Some(class) = (unsafe { class.to_reference() }) else {
		panic!("should be checked by the jdk");
	};

	let Some(name) = (unsafe { name.to_reference() }) else {
		panic!("should be checked by the jdk");
	};

	let class = class.extract_mirror();

	let name_str = classes::java::lang::String::extract(name.extract_class());
	let classref = class.target_class();

	for field in classref.fields() {
		if field.name.as_str() != name_str {
			continue;
		}

		if field.is_static() {
			return IS_STATIC;
		}

		return field.offset() as jlong;
	}

	NOT_FOUND
}

#[jni_call]
extern "C" fn staticFieldOffset0(
	env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	field: JObject, // java.lang.reflect.Field
) -> jlong {
	let thread = unsafe { env.thread() };

	let Some(field) = (unsafe { field.to_reference() }) else {
		throw_with_ret!(0, thread, NullPointerException);
	};
	let field = field.extract_class();

	let modifiers = classes::java::lang::reflect::Field::modifiers(field);
	if modifiers & FieldAccessFlags::ACC_STATIC.as_u2() as jint == 0 {
		throw_with_ret!(0, thread, IllegalArgumentException);
	}

	let slot = classes::java::lang::reflect::Field::slot(field);
	let class = classes::java::lang::reflect::Field::clazz(field);

	let field = class
		.target_class()
		.fields()
		.nth(slot as usize)
		.expect("malformed java.lang.reflect.Field");
	field.offset() as jlong
}

#[jni_call]
extern "C" fn staticFieldBase0(
	env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	field: JObject, // java.lang.reflect.Field
) -> JObject {
	let thread = unsafe { env.thread() };

	let Some(field) = (unsafe { field.to_reference() }) else {
		throw_with_ret!(JObject::null(), thread, NullPointerException);
	};
	let field = field.extract_class();

	let modifiers = classes::java::lang::reflect::Field::modifiers(field);
	if modifiers & FieldAccessFlags::ACC_STATIC.as_u2() as jint == 0 {
		throw_with_ret!(JObject::null(), thread, IllegalArgumentException);
	}

	let class = classes::java::lang::reflect::Field::clazz(field);
	thread.jni_refs().allocate_wrapped(class).into()
}
#[jni_call]
extern "C" fn shouldBeInitialized0(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	class: JClass,
) -> jboolean {
	let Some(class) = (unsafe { class.to_reference() }) else {
		panic!("should be checked by the jdk");
	};

	let class = class.extract_target_class();
	class.initialization_state() != ClassInitializationState::Init
}
#[jni_call]
extern "C" fn notifyStrictStaticAccess0(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	_class: JClass,
	_sfoffset: jlong,
	_writing: jboolean,
) -> jboolean {
	unimplemented!("jdk.internal.misc.Unsafe#notifyStrictStaticAccess0")
}
#[jni_call]
extern "C" fn ensureClassInitialized0(
	env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	class: JClass,
) {
	let thread = unsafe { env.thread() };

	let Some(class) = (unsafe { class.to_reference() }) else {
		panic!("should be checked by the jdk");
	};

	let mirror = class.extract_mirror();

	let target_class = mirror.target_class();
	if let Throws::Exception(e) = target_class.initialize(thread) {
		e.throw(thread);
	}
}

fn scale_of(array_class: JClass) -> Throws<jint> {
	let Some(array_class) = (unsafe { array_class.to_reference() }) else {
		throw!(@DEFER InvalidClassException);
	};

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

#[jni_call]
extern "C" fn arrayBaseOffset0(
	env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	array_class: JClass,
) -> jint {
	let thread = unsafe { env.thread() };

	// Just here to verify that the array is valid
	if let Throws::Exception(e) = scale_of(array_class) {
		e.throw(thread);
		return 0;
	}

	// The Java code doesn't need to know the real base of the array. Object::get() and Object::put()
	// base on the start of the object's fields already, so the indexing is transparent.
	0
}
#[jni_call]
extern "C" fn arrayInstanceBaseOffset0(_env: JniEnv, _this: JObject, _array: JArray) -> jint {
	unimplemented!("jdk.internal.misc.Unsafe#arrayInstanceBaseOffset0")
}
#[jni_call]
extern "C" fn arrayIndexScale0(
	env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	array_class: JClass,
) -> jint {
	let thread = unsafe { env.thread() };

	match scale_of(array_class) {
		Throws::Ok(scale) => scale,
		Throws::Exception(e) => {
			e.throw(thread);
			0
		},
	}
}
#[jni_call]
extern "C" fn arrayInstanceIndexScale0(_env: JniEnv, _this: JObject, _array: JArray) -> jint {
	unimplemented!("jdk.internal.misc.Unsafe#arrayInstanceIndexScale0")
}
#[jni_call]
extern "C" fn getFieldMap0(_env: JniEnv, _this: JObject, _class: JClass) -> JArray {
	unimplemented!("jdk.internal.misc.Unsafe#getFieldMap0")
}
#[jni_call]
extern "C" fn getObjectSize0(_env: JniEnv, _this: JObject, _obj: JObject) -> jlong {
	unimplemented!("jdk.internal.misc.Unsafe#getObjectSize0")
}
#[jni_call]
extern "C" fn getLoadAverage0(
	_env: JniEnv,
	_this: JObject, // jdk.internal.misc.Unsafe
	_loadavg: JDoubleArray,
	_nelems: jint,
) -> jint {
	unimplemented!("jdk.internal.misc.Unsafe#getLoadAverage0")
}

#[jni_call]
#[unsafe(export_name = "Java_jdk_internal_misc_Unsafe_registerNatives")]
pub extern "C" fn registerNatives(env: JniEnv, class: JClass) {
	macro_rules! methods {
        (
            GET_PUT_TYPES: [$(($get_put_ty:ident, $get_put_ty_code:ident)),* $(,)?],
            $(($name:expr, $signature:expr, $fn_ptr:expr)),* $(,)?
        ) => {
            paste::paste! {
                [
                    $(
                        JniNativeMethod::from(stringify!([<get $get_put_ty>]),          const_format::concatcp!("(Ljava/lang/Object;J)", stringify!($get_put_ty_code)),      [<get $get_put_ty>] as extern "C" fn (_, _, _, _) -> _),
                        JniNativeMethod::from(stringify!([<put $get_put_ty>]),          const_format::concatcp!("(Ljava/lang/Object;J", stringify!($get_put_ty_code), ")V"), [<put $get_put_ty>] as extern "C" fn (_, _, _, _, _)),
                        JniNativeMethod::from(stringify!([<get $get_put_ty Volatile>]), const_format::concatcp!("(Ljava/lang/Object;J)", stringify!($get_put_ty_code)),     [<get $get_put_ty Volatile>] as extern "C" fn (_, _, _, _) -> _),
                        JniNativeMethod::from(stringify!([<put $get_put_ty Volatile>]), const_format::concatcp!("(Ljava/lang/Object;J", stringify!($get_put_ty_code), ")V"), [<put $get_put_ty Volatile>] as extern "C" fn (_, _, _, _, _)),
                    )*
                    $(
                        JniNativeMethod::from($name, $signature, $fn_ptr),
                    )*
                ]
            }
        }
    }

	#[allow(trivial_casts)]
	let methods = methods! {
		GET_PUT_TYPES: [
			(Boolean, Z),
			(Byte, B),
			(Short, S),
			(Char, C),
			(Int, I),
			(Long, J),
			(Float, F),
			(Double, D),
		],

		("getReference",                "(Ljava/lang/Object;J)Ljava/lang/Object;",                                                          getReference as extern "C" fn(_, _, _, _) -> JObject),
		("putReference",                "(Ljava/lang/Object;JLjava/lang/Object;)V",                                                         putReference as extern "C" fn(_, _, _, _, _)),
		("getReferenceVolatile",        "(Ljava/lang/Object;J)Ljava/lang/Object;",                                                          getReferenceVolatile as extern "C" fn(_, _, _, _) -> JObject),
		("putReferenceVolatile",        "(Ljava/lang/Object;JLjava/lang/Object;)V",                                                         putReferenceVolatile as extern "C" fn(_, _, _, _, _)),
		("isFlatField0",                "(Ljava/lang/Object;)Z",                                                                            isFlatField as extern "C" fn(_, _, _) -> jboolean),
		("hasNullMarker0",              "(Ljava/lang/Object;)Z",                                                                            hasNullMarker as extern "C" fn(_, _, _) -> jboolean),
		("nullMarkerOffset0",           "(Ljava/lang/Object;)I",                                                                            nullMarkerOffset as extern "C" fn(_, _, _) -> jint),
		("arrayLayout0",                "([Ljava/lang/Object;)I",                                                                           arrayLayout as extern "C" fn(_, _, _) -> jint),
		("fieldLayout0",                "(Ljava/lang/Object;)I",                                                                            fieldLayout as extern "C" fn(_, _, _) -> jint),
		("newSpecialArray",             "(Ljava/lang/Class;II)[Ljava/lang/Object;",                                                         newSpecialArray as extern "C" fn(_, _, _, _, _) -> JArray),
		("getFlatValue",                "(Ljava/lang/Object;JILjava/lang/Class;)Ljava/lang/Object;",                                        getFlatValue as extern "C" fn(_, _, _, _, _, _) -> JObject),
		("putFlatValue",                "(Ljava/lang/Object;JILjava/lang/Class;Ljava/lang/Object;)V",                                       putFlatValue as extern "C" fn(_, _, _, _, _, _, _)),
		("valueHeaderSize",             "(Ljava/lang/Class;)J",                                                                             valueHeaderSize as extern "C" fn(_, _, _) -> jlong),
		("getUncompressedObject",       "(J)Ljava/lang/Object;",                                                                            getUncompressedObject as extern "C" fn(_, _, _) -> JObject),
		("allocateMemory0",             "(J)J",                                                                                             allocateMemory0 as extern "C" fn(_, _, _) -> jlong),
		("reallocateMemory0",           "(JJ)J",                                                                                            reallocateMemory0 as extern "C" fn(_, _, _, _) -> jlong),
		("freeMemory0",                 "(J)V",                                                                                             freeMemory0 as extern "C" fn(_, _, _)),
		("objectFieldOffset0",          "(Ljava/lang/reflect/Field;)J",                                                                     objectFieldOffset0 as extern "C" fn(_, _, _) -> jlong),
		("knownObjectFieldOffset0",     "(Ljava/lang/Class;Ljava/lang/String;)J",                                                           knownObjectFieldOffset0 as extern "C" fn(_, _, _, _) -> jlong),
		("staticFieldOffset0",          "(Ljava/lang/reflect/Field;)J",                                                                     staticFieldOffset0 as extern "C" fn(_, _, _) -> jlong),
		("staticFieldBase0",            "(Ljava/lang/reflect/Field;)Ljava/lang/Object;",                                                    staticFieldBase0 as extern "C" fn(_, _, _) -> JObject),
		("ensureClassInitialized0",     "(Ljava/lang/Class;)V",                                                                             ensureClassInitialized0 as extern "C" fn(_, _, _)),
		("arrayBaseOffset0",            "(Ljava/lang/Class;)I",                                                                             arrayBaseOffset0 as extern "C" fn(_, _, _) -> jint),
		("arrayInstanceBaseOffset0",    "([Ljava/lang/Object;)I",                                                                           arrayInstanceBaseOffset0 as extern "C" fn(_, _, _) -> jint),
		("arrayIndexScale0",            "(Ljava/lang/Class;)I",                                                                             arrayIndexScale0 as extern "C" fn(_, _, _) -> jint),
		("arrayInstanceIndexScale0",    "([Ljava/lang/Object;)I",                                                                           arrayInstanceIndexScale0 as extern "C" fn(_, _, _) -> jint),
		("getFieldMap0",                "(Ljava/lang/Class;)[I",                                                                            getFieldMap0 as extern "C" fn(_, _, _) -> JArray),
		("getObjectSize0",              "(Ljava/lang/Object;)J",                                                                            getObjectSize0 as extern "C" fn(_, _, _) -> jlong),
		("defineClass0",                "(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;", defineClass0 as extern "C" fn(_, _, _, _, _, _, _, _) -> JObject),
		("allocateInstance",            "(Ljava/lang/Class;)Ljava/lang/Object;",                                                            allocateInstance as extern "C" fn(_, _, _) -> JObject),
		("throwException",              "(Ljava/lang/Throwable;)V",                                                                         throwException as extern "C" fn(_, _, _)),
		("compareAndSetReference",      "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",                                       compareAndSetReference as extern "C" fn(_, _, _, _, _, _) -> jboolean),
		("compareAndSetInt",            "(Ljava/lang/Object;JII)Z",                                                                         compareAndSetInt as extern "C" fn(_, _, _, _, _, _) -> jboolean),
		("compareAndSetLong",           "(Ljava/lang/Object;JJJ)Z",                                                                         compareAndSetLong as extern "C" fn(_, _, _, _, _, _) -> jboolean),
		("compareAndExchangeReference", "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",                      compareAndExchangeReference as extern "C" fn(_, _, _, _, _, _) -> JObject),
		("compareAndExchangeInt",       "(Ljava/lang/Object;JII)I",                                                                         compareAndExchangeInt as extern "C" fn(_, _, _, _, _, _) -> jint),
		("compareAndExchangeLong",      "(Ljava/lang/Object;JJJ)J",                                                                         compareAndExchangeLong as extern "C" fn(_, _, _, _, _, _) -> jlong),
		("park",                        "(ZJ)V",                                                                                            park as extern "C" fn(_, _, _, _)),
		("unpark",                      "(Ljava/lang/Object;)V",                                                                            unpark as extern "C" fn(_, _, _)),
		("getLoadAverage0",             "([DI)I",                                                                                           getLoadAverage0 as extern "C" fn(_, _, _, _) -> jint),
		("copyMemory0",                 "(Ljava/lang/Object;JLjava/lang/Object;JJ)V",                                                       copyMemory0 as extern "C" fn(_, _, _, _, _, _, _)),
		("copySwapMemory0",             "(Ljava/lang/Object;JLjava/lang/Object;JJJ)V",                                                      copySwapMemory0 as extern "C" fn(_, _, _, _, _, _, _, _)),
		("writeback0",                  "(J)V",                                                                                             writeBack0 as extern "C" fn(_, _, _)),
		("writebackPreSync0",           "()V",                                                                                              writeBackPreSync0 as extern "C" fn(_, _)),
		("writebackPostSync0",          "()V",                                                                                              writeBackPostSync0 as extern "C" fn(_, _)),
		("setMemory0",                  "(Ljava/lang/Object;JJB)V",                                                                         setMemory0 as extern "C" fn(_, _, _, _, _, _)),
		("shouldBeInitialized0",        "(Ljava/lang/Class;)Z",                                                                             shouldBeInitialized0 as extern "C" fn(_, _, _) -> jboolean),
		("notifyStrictStaticAccess0",   "(Ljava/lang/Class;JZ)V",                                                                           notifyStrictStaticAccess0 as extern "C" fn(_, _, _, _, _) -> jboolean),
		("fullFence",                   "()V",                                                                                              fullFence as extern "C" fn(_, _)),
	};

	env.register_natives(class, &methods)
		.expect("should register")
}
