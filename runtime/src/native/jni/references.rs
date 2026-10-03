use crate::objects::class::ClassPtr;
use crate::objects::field::Field;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;

use std::marker::ConstParamTy;
use std::ptr::NonNull;
use std::sync::{LazyLock, Mutex};

use ::jni::objects::{
	JArray, JBooleanArray, JByteArray, JCharArray, JClass, JDoubleArray, JFieldId, JFloatArray,
	JIntArray, JLongArray, JMethodId, JObject, JObjectArray, JShortArray, JString, JThrowable,
	JWeak,
};
use ::jni::sys::{JNI_ERR, JNI_OK, JNIEnv, jfieldID, jint, jmethodID, jobject};
use common::sync::ForceSendSync;
use jni::sys::jclass;

/// Types that can be converted into JNI objects
pub trait JniObjectType {
	type JniTarget;
	type JniSafeWrapper;

	fn ptr(self) -> Option<NonNull<()>>;

	fn from_ptr(ptr: *mut *mut ()) -> Self::JniTarget;

	fn wrap(raw: Self::JniTarget) -> Self::JniSafeWrapper;

	fn null() -> Self::JniTarget;
}

impl JniObjectType for Reference {
	type JniTarget = jobject;
	type JniSafeWrapper = JObject;

	fn ptr(self) -> Option<NonNull<()>> {
		if self.is_null() {
			return None;
		}

		// SAFETY: Verified non-null
		Some(unsafe { NonNull::new_unchecked(self.raw_tagged().cast_mut()) })
	}

	fn from_ptr(ptr: *mut *mut ()) -> Self::JniTarget {
		ptr as Self::JniTarget
	}

	fn wrap(raw: Self::JniTarget) -> Self::JniSafeWrapper {
		if raw.is_null() {
			return JObject::null();
		}

		unsafe { JObject::from_raw(raw) }
	}

	fn null() -> Self::JniTarget {
		std::ptr::null_mut() as Self::JniTarget
	}
}

impl JniObjectType for ClassPtr {
	type JniTarget = jclass;
	type JniSafeWrapper = JClass;

	fn ptr(self) -> Option<NonNull<()>> {
		<Reference as JniObjectType>::ptr(Reference::mirror(self.mirror()))
	}

	fn from_ptr(ptr: *mut *mut ()) -> Self::JniTarget {
		ptr as Self::JniTarget
	}

	fn wrap(raw: Self::JniTarget) -> Self::JniSafeWrapper {
		debug_assert!(!raw.is_null());
		unsafe { JClass::from_raw(raw) }
	}

	fn null() -> Self::JniTarget {
		unreachable!("null class pointers should never exist")
	}
}

impl JniObjectType for &'static Field {
	type JniTarget = jfieldID;
	type JniSafeWrapper = JFieldId;

	fn ptr(self) -> Option<NonNull<()>> {
		// SAFETY: Constructed from a valid reference
		Some(unsafe { NonNull::new_unchecked(std::ptr::from_ref(self).cast::<()>().cast_mut()) })
	}

	fn from_ptr(ptr: *mut *mut ()) -> Self::JniTarget {
		ptr as Self::JniTarget
	}

	fn wrap(raw: Self::JniTarget) -> Self::JniSafeWrapper {
		debug_assert!(!raw.is_null());
		unsafe { JFieldId::from_raw(raw) }
	}

	fn null() -> Self::JniTarget {
		unreachable!("null field pointers should never exist")
	}
}

impl JniObjectType for &'static Method {
	type JniTarget = jmethodID;
	type JniSafeWrapper = JMethodId;

	fn ptr(self) -> Option<NonNull<()>> {
		// SAFETY: Constructed from a valid reference
		Some(unsafe { NonNull::new_unchecked(std::ptr::from_ref(self).cast::<()>().cast_mut()) })
	}

	fn from_ptr(ptr: *mut *mut ()) -> Self::JniTarget {
		ptr as Self::JniTarget
	}

	fn wrap(raw: Self::JniTarget) -> Self::JniSafeWrapper {
		debug_assert!(!raw.is_null());
		unsafe { JMethodId::from_raw(raw) }
	}

	fn null() -> Self::JniTarget {
		unreachable!("null method pointers should never exist")
	}
}

/// The type of JNI object reference
#[derive(Copy, Clone, Debug, PartialEq, Eq, ConstParamTy)]
pub enum ObjectReferenceType {
	/// The reference is local to the current thread
	Local = 0b0,
	/// The reference is globally available
	Global = 0b1,
}

#[derive(Copy, Clone, Default, Debug)]
enum Slot {
	// We only ever access occupied slots by pointer. We *construct* `Occupied` slots, but never
	// read them as such, so the compiler doesn't know this is significant.
	#[allow(dead_code)]
	Occupied(ForceSendSync<NonNull<()>>),
	#[default]
	Empty,
}

const _: () = {
	assert!(
		size_of::<Slot>() == size_of::<usize>(),
		"`Slot` must be pointer-sized"
	);
};

// TODO: Make the storage growable
const CHUNK_SIZE: usize = 16384;

/// A collection of active JNI object references
pub struct JniObjectStorage<
	const REFERENCE_TYPE: ObjectReferenceType = { ObjectReferenceType::Local },
> {
	references: Mutex<[Slot; CHUNK_SIZE]>,
}

impl<const REFERENCE_TYPE: ObjectReferenceType> Default for JniObjectStorage<REFERENCE_TYPE> {
	fn default() -> Self {
		Self {
			references: Mutex::new([Slot::Empty; CHUNK_SIZE]),
		}
	}
}

impl<const REFERENCE_TYPE: ObjectReferenceType> JniObjectStorage<REFERENCE_TYPE> {
	/// Allocate a new JNI object reference for the given `target`
	pub fn allocate<T>(&self, target: T) -> T::JniTarget
	where
		T: JniObjectType,
	{
		let Some(target_ptr) = target.ptr() else {
			return T::null(); // No need to allocate nulls
		};

		let mut refs = self.references.lock().unwrap();
		let slot = Slot::Occupied(ForceSendSync(target_ptr));

		let target_entry = match refs.iter_mut().find(|s| matches!(s, Slot::Empty)) {
			Some(empty) => {
				*empty = slot;
				empty
			},
			None => panic!("JNI object storage exceeded"),
		};

		let new_obj_ptr =
			(std::ptr::from_ref(target_entry) as usize | REFERENCE_TYPE as usize) as *mut *mut ();
		debug_assert_eq!(split_jni_ref(new_obj_ptr).0, REFERENCE_TYPE);

		T::from_ptr(new_obj_ptr)
	}

	/// Same as [`Self::allocate()`], but returns the safe JNI wrapper
	pub fn allocate_wrapped<T>(&self, target: T) -> T::JniSafeWrapper
	where
		T: JniObjectType,
	{
		let raw = self.allocate(target);
		T::wrap(raw)
	}

	fn deallocate(&self, obj: jobject) {
		let (ty, target) = split_jni_ref(obj.cast());
		assert_eq!(ty, REFERENCE_TYPE);

		let mut refs = self.references.lock().unwrap();
		let slot = unsafe { (target as *mut Slot).offset_from_unsigned(refs.as_ptr()) };

		refs[slot] = Slot::Empty;
	}
}

static GLOBAL_REFERENCES: LazyLock<JniObjectStorage<{ ObjectReferenceType::Global }>> =
	LazyLock::new(|| JniObjectStorage::default());

/// Methods for JNI object handles
pub trait JObjectExt {
	/// Get the [`ObjectReferenceType`] of the object
	fn ref_ty(self) -> ObjectReferenceType;

	/// Attempt to derive a [`Reference`] from the object
	///
	/// This returns `None` for null pointers
	///
	/// # Safety
	///
	/// This must only be called with objects created via other JNI methods
	unsafe fn to_reference(self) -> Option<Reference>;

	/// Same as [`Self::to_reference()`], but null pointers return [`Reference::null()`]
	///
	/// # Safety
	///
	/// This must only be called with objects created via other JNI methods
	unsafe fn to_reference_maybe_null(self) -> Reference;
}

macro_rules! impl_jobject_ext {
    ($($ty:ty),* $(,)?) => {
        $(
        impl JObjectExt for $ty {
            fn ref_ty(self) -> ObjectReferenceType {
                self.raw().ref_ty()
            }

            unsafe fn to_reference(self) -> Option<Reference> {
                unsafe { self.raw().to_reference() }
            }

            unsafe fn to_reference_maybe_null(self) -> Reference {
                unsafe { self.raw().to_reference_maybe_null() }
            }
        }
        )*
    }
}

impl_jobject_ext!(
	JObject,
	JClass,
	JThrowable,
	JString,
	JWeak,
	JArray,
	JBooleanArray,
	JByteArray,
	JCharArray,
	JShortArray,
	JIntArray,
	JLongArray,
	JFloatArray,
	JDoubleArray,
	JObjectArray
);

impl JObjectExt for jobject {
	#[inline]
	fn ref_ty(self) -> ObjectReferenceType {
		let (ty, _) = split_jni_ref(self.cast());
		ty
	}

	#[inline]
	unsafe fn to_reference(self) -> Option<Reference> {
		let (_, obj) = split_jni_ref(self.cast());

		if obj.is_null() {
			return None;
		}

		unsafe { Some(Reference::from_raw(*obj)) }
	}

	#[inline]
	unsafe fn to_reference_maybe_null(self) -> Reference {
		let ref_ = unsafe { self.to_reference() };
		ref_.unwrap_or(Reference::null())
	}
}

/// Split a JNI reference into its parts
///
/// JNI reference pointers have the following structure:
///
/// ```text
/// [0-1 ] `ObjectReferenceType`
/// [1-63] Address
/// ```
///
/// Where `Address` is a pointer to the object's pointer (e.g., a [`Reference`] tagged pointer).
fn split_jni_ref(obj: *mut *mut ()) -> (ObjectReferenceType, *mut *mut ()) {
	const TYPE_MASK: usize = 0b1;
	const ADDRESS_MASK: usize = !TYPE_MASK;

	let raw = obj as usize;

	let ty = match raw & TYPE_MASK {
		0b0 => ObjectReferenceType::Local,
		0b1 => ObjectReferenceType::Global,
		_ => unreachable!(),
	};

	let ptr = (raw & ADDRESS_MASK) as *mut *mut ();

	(ty, ptr)
}

/// Create a `Field` from a `jfieldID`
pub unsafe fn field_ref_from_jfieldid(field: jfieldID) -> Option<&'static Field> {
	let (_, target) = split_jni_ref(field.cast());

	if target.is_null() {
		return None;
	}

	unsafe {
		let field_ptr = (*target) as *const Field;
		Some(&*field_ptr)
	}
}

/// Create a `Method` from a `jmethodID`
pub unsafe fn method_ref_from_jmethodid(method: jmethodID) -> Option<&'static Method> {
	let (_, target) = split_jni_ref(method.cast());

	if target.is_null() {
		return None;
	}

	unsafe {
		let method_ptr = (*target) as *const Method;
		Some(&*method_ptr)
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn PushLocalFrame(env: *mut JNIEnv, capacity: jint) -> jint {
	unimplemented!("jni::PushLocalFrame");
}

#[unsafe(no_mangle)]
pub extern "system" fn PopLocalFrame(env: *mut JNIEnv, result: jobject) -> jobject {
	unimplemented!("jni::PopLocalFrame");
}

#[unsafe(no_mangle)]
pub extern "system" fn NewGlobalRef(env: *mut JNIEnv, lobj: jobject) -> jobject {
	let Some(obj) = (unsafe { lobj.to_reference() }) else {
		return std::ptr::null_mut();
	};

	GLOBAL_REFERENCES.allocate(obj)
}

#[unsafe(no_mangle)]
pub extern "system" fn DeleteGlobalRef(env: *mut JNIEnv, gref: jobject) {
	if gref.is_null() {
		return;
	}

	GLOBAL_REFERENCES.deallocate(gref)
}

#[unsafe(no_mangle)]
pub extern "system" fn DeleteLocalRef(env: *mut JNIEnv, obj: jobject) {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	if obj.is_null() {
		return;
	}

	thread.jni_refs().deallocate(obj)
}

#[unsafe(no_mangle)]
pub extern "system" fn NewLocalRef(env: *mut JNIEnv, ref_: jobject) -> jobject {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let Some(obj) = (unsafe { ref_.to_reference() }) else {
		return std::ptr::null_mut();
	};

	thread.jni_refs().allocate(obj)
}

#[unsafe(no_mangle)]
pub extern "system" fn EnsureLocalCapacity(env: *mut JNIEnv, capacity: jint) -> jint {
	if capacity < 0 {
		return JNI_ERR;
	}

	// TODO: -Djdk.internal.MaxJNILocalCapacity
	JNI_OK
}
