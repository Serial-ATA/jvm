//! The Invocation API allows software vendors to load the Java VM into an arbitrary native application.
//!
//! Vendors can deliver Java-enabled applications without having to link with the Java VM source code.

use crate::thread::{JavaThread, JavaThreadBuilder};
use crate::{classes, initialization};

use core::ffi::c_void;
use std::ffi::CStr;

use jni::error::JniError;
use jni::java_vm::JavaVm;
use jni::sys::{
	JNI_EVERSION, JNI_OK, JNIInvokeInterface_, JNINativeInterface_, JavaVM, jint, jsize,
};
use jni::version::JniVersion;

pub mod library;

#[unsafe(no_mangle)]
pub extern "system" fn DestroyJavaVM(vm: *mut JavaVM) -> jint {
	{
		JavaThread::current().exit(false)
	}

	// SAFETY: No active references to the thread exist now
	unsafe { JavaThread::unset_current_thread() };

	JNI_OK
}

#[unsafe(no_mangle)]
pub extern "system" fn AttachCurrentThread(
	vm: *mut JavaVM,
	penv: *mut *mut c_void,
	args: *mut c_void,
) -> jint {
	attach_current_thread_impl(vm, penv, args, false)
}

#[unsafe(no_mangle)]
pub extern "system" fn DetachCurrentThread(vm: *mut JavaVM) -> jint {
	unimplemented!("jni::DetachCurrentThread")
}

#[unsafe(no_mangle)]
pub extern "system" fn GetEnv(vm: *mut JavaVM, penv: *mut *mut c_void, version: jint) -> jint {
	unimplemented!("jni::GetEnv")
}

#[unsafe(no_mangle)]
pub extern "system" fn AttachCurrentThreadAsDaemon(
	vm: *mut JavaVM,
	penv: *mut *mut c_void,
	args: *mut c_void,
) -> jint {
	attach_current_thread_impl(vm, penv, args, true)
}

#[unsafe(no_mangle)]
pub extern "system" fn JNI_GetDefaultJavaVMInitArgs(args: *mut c_void) -> jint {
	unimplemented!("jni::JNI_GetDefaultJavaVMInitArgs")
}

#[unsafe(no_mangle)]
pub extern "system" fn JNI_CreateJavaVM(
	pvm: *mut *mut JavaVM,
	penv: *mut *mut c_void,
	args: *mut c_void,
) -> jint {
	if args.is_null() {
		return jni::sys::JNI_EINVAL;
	}

	let args = unsafe { &*args.cast::<jni::sys::JavaVMInitArgs>() };
	if args.version == jni::sys::JNI_VERSION_1_1 {
		return jni::sys::JNI_EVERSION;
	}

	let vm;
	match initialization::create_java_vm(Some(args)) {
		Ok(java_vm) => vm = java_vm,
		Err((e, exception)) => {
			if let JniError::ExceptionThrown = e {
				eprintln!("Error occurred during initialization of VM");
				if let Some(exception) = exception {
					classes::java::lang::Throwable::print_stack_trace_without_java_system(
						exception,
						JavaThread::current(),
					);

					// If a VM was created and initialized to the point that an exception was thrown,
					// the entire process just gets aborted like Hotspot.
					std::process::abort();
				}
			}

			// Otherwise, the error can just be returned to the caller
			return e.as_jint();
		},
	}

	let current_thread = JavaThread::current();
	let env = current_thread.env();
	unsafe {
		*pvm = vm.raw() as _;
		*penv = env.raw() as _;
	}

	JNI_OK
}

#[unsafe(no_mangle)]
pub extern "system" fn JNI_GetCreatedJavaVMs(
	vmBuf: *mut *mut JavaVM,
	bufLen: jsize,
	nVMs: *mut jsize,
) -> jint {
	unimplemented!("jni::JNI_GetCreatedJavaVMs")
}

fn attach_current_thread_impl(
	vm: *mut JavaVM,
	penv: *mut *mut c_void,
	args: *mut c_void,
	daemon: bool,
) -> jint {
	// We're already attached
	if let Some(thread) = JavaThread::current_opt() {
		let env = thread.env();
		unsafe {
			*penv = env.raw() as _;
		}
		return JNI_OK;
	}

	let mut name = None;
	let mut group = None;
	if !args.is_null() {
		let args = unsafe { &*args.cast::<jni::sys::JavaVMAttachArgs>() };
		name = Some(unsafe { CStr::from_ptr(args.name) });
		let Some(_version) = JniVersion::from_raw(args.version) else {
			return JNI_EVERSION;
		};
		group = Some(args.group);
	}

	if group.is_none() {
		group = Some(todo!("crate::globals::threads::main_thread_group()"));
	}

	let thread = JavaThreadBuilder::new().finish();

	todo!()
}

#[allow(trivial_casts)]
pub unsafe fn main_java_vm() -> JavaVm {
	let raw = &RAW_INVOKE_INTERFACE.0 as *const _;
	unsafe { JavaVm::from_raw(raw as *mut _) }
}

struct InvokeInterface(jni::sys::JNIInvokeInterface_);

unsafe impl Sync for InvokeInterface {}
unsafe impl Send for InvokeInterface {}

static RAW_INVOKE_INTERFACE: InvokeInterface = InvokeInterface(JNIInvokeInterface_ {
	reserved0: core::ptr::null_mut(),
	reserved1: core::ptr::null_mut(),
	reserved2: core::ptr::null_mut(),

	DestroyJavaVM,
	AttachCurrentThread,
	DetachCurrentThread,
	GetEnv,
	AttachCurrentThreadAsDaemon,
});

pub unsafe fn new_env() -> jni::sys::JNIEnv {
	let native_interface = JNINativeInterface_ {
		reserved0: core::ptr::null_mut(),
		reserved1: core::ptr::null_mut(),
		reserved2: core::ptr::null_mut(),
		reserved3: core::ptr::null_mut(),

		GetVersion: super::version::GetVersion,
		DefineClass: super::class::DefineClass,
		FindClass: super::class::FindClass,
		FromReflectedMethod: super::reflection::FromReflectedMethod,
		FromReflectedField: super::reflection::FromReflectedField,
		ToReflectedMethod: super::reflection::ToReflectedMethod,
		GetSuperclass: super::class::GetSuperclass,
		IsAssignableFrom: super::class::IsAssignableFrom,
		ToReflectedField: super::reflection::ToReflectedField,
		Throw: super::exceptions::Throw,
		ThrowNew: super::exceptions::ThrowNew,
		ExceptionOccurred: super::exceptions::ExceptionOccurred,
		ExceptionDescribe: super::exceptions::ExceptionDescribe,
		ExceptionClear: super::exceptions::ExceptionClear,
		FatalError: super::exceptions::FatalError,
		PushLocalFrame: super::references::PushLocalFrame,
		PopLocalFrame: super::references::PopLocalFrame,
		NewGlobalRef: super::references::NewGlobalRef,
		DeleteGlobalRef: super::references::DeleteGlobalRef,
		DeleteLocalRef: super::references::DeleteLocalRef,
		IsSameObject: super::object::IsSameObject,
		NewLocalRef: super::references::NewLocalRef,
		EnsureLocalCapacity: super::references::EnsureLocalCapacity,
		AllocObject: super::object::AllocObject,
		NewObject: super::object::NewObject,
		NewObjectV: super::object::NewObjectV,
		NewObjectA: super::object::NewObjectA,
		GetObjectClass: super::object::GetObjectClass,
		IsInstanceOf: super::object::IsInstanceOf,
		GetMethodID: super::method::GetMethodID,
		CallObjectMethod: super::method::CallObjectMethod,
		CallObjectMethodV: super::method::CallObjectMethodV,
		CallObjectMethodA: super::method::CallObjectMethodA,
		CallBooleanMethod: super::method::CallBooleanMethod,
		CallBooleanMethodV: super::method::CallBooleanMethodV,
		CallBooleanMethodA: super::method::CallBooleanMethodA,
		CallByteMethod: super::method::CallByteMethod,
		CallByteMethodV: super::method::CallByteMethodV,
		CallByteMethodA: super::method::CallByteMethodA,
		CallCharMethod: super::method::CallCharMethod,
		CallCharMethodV: super::method::CallCharMethodV,
		CallCharMethodA: super::method::CallCharMethodA,
		CallShortMethod: super::method::CallShortMethod,
		CallShortMethodV: super::method::CallShortMethodV,
		CallShortMethodA: super::method::CallShortMethodA,
		CallIntMethod: super::method::CallIntMethod,
		CallIntMethodV: super::method::CallIntMethodV,
		CallIntMethodA: super::method::CallIntMethodA,
		CallLongMethod: super::method::CallLongMethod,
		CallLongMethodV: super::method::CallLongMethodV,
		CallLongMethodA: super::method::CallLongMethodA,
		CallFloatMethod: super::method::CallFloatMethod,
		CallFloatMethodV: super::method::CallFloatMethodV,
		CallFloatMethodA: super::method::CallFloatMethodA,
		CallDoubleMethod: super::method::CallDoubleMethod,
		CallDoubleMethodV: super::method::CallDoubleMethodV,
		CallDoubleMethodA: super::method::CallDoubleMethodA,
		CallVoidMethod: super::method::CallVoidMethod,
		CallVoidMethodV: super::method::CallVoidMethodV,
		CallVoidMethodA: super::method::CallVoidMethodA,
		CallNonvirtualObjectMethod: super::method::CallNonvirtualObjectMethod,
		CallNonvirtualObjectMethodV: super::method::CallNonvirtualObjectMethodV,
		CallNonvirtualObjectMethodA: super::method::CallNonvirtualObjectMethodA,
		CallNonvirtualBooleanMethod: super::method::CallNonvirtualBooleanMethod,
		CallNonvirtualBooleanMethodV: super::method::CallNonvirtualBooleanMethodV,
		CallNonvirtualBooleanMethodA: super::method::CallNonvirtualBooleanMethodA,
		CallNonvirtualByteMethod: super::method::CallNonvirtualByteMethod,
		CallNonvirtualByteMethodV: super::method::CallNonvirtualByteMethodV,
		CallNonvirtualByteMethodA: super::method::CallNonvirtualByteMethodA,
		CallNonvirtualCharMethod: super::method::CallNonvirtualCharMethod,
		CallNonvirtualCharMethodV: super::method::CallNonvirtualCharMethodV,
		CallNonvirtualCharMethodA: super::method::CallNonvirtualCharMethodA,
		CallNonvirtualShortMethod: super::method::CallNonvirtualShortMethod,
		CallNonvirtualShortMethodV: super::method::CallNonvirtualShortMethodV,
		CallNonvirtualShortMethodA: super::method::CallNonvirtualShortMethodA,
		CallNonvirtualIntMethod: super::method::CallNonvirtualIntMethod,
		CallNonvirtualIntMethodV: super::method::CallNonvirtualIntMethodV,
		CallNonvirtualIntMethodA: super::method::CallNonvirtualIntMethodA,
		CallNonvirtualLongMethod: super::method::CallNonvirtualLongMethod,
		CallNonvirtualLongMethodV: super::method::CallNonvirtualLongMethodV,
		CallNonvirtualLongMethodA: super::method::CallNonvirtualLongMethodA,
		CallNonvirtualFloatMethod: super::method::CallNonvirtualFloatMethod,
		CallNonvirtualFloatMethodV: super::method::CallNonvirtualFloatMethodV,
		CallNonvirtualFloatMethodA: super::method::CallNonvirtualFloatMethodA,
		CallNonvirtualDoubleMethod: super::method::CallNonvirtualDoubleMethod,
		CallNonvirtualDoubleMethodV: super::method::CallNonvirtualDoubleMethodV,
		CallNonvirtualDoubleMethodA: super::method::CallNonvirtualDoubleMethodA,
		CallNonvirtualVoidMethod: super::method::CallNonvirtualVoidMethod,
		CallNonvirtualVoidMethodV: super::method::CallNonvirtualVoidMethodV,
		CallNonvirtualVoidMethodA: super::method::CallNonvirtualVoidMethodA,
		GetFieldID: super::field::GetFieldID,
		GetObjectField: super::field::GetObjectField,
		GetBooleanField: super::field::GetBooleanField,
		GetByteField: super::field::GetByteField,
		GetCharField: super::field::GetCharField,
		GetShortField: super::field::GetShortField,
		GetIntField: super::field::GetIntField,
		GetLongField: super::field::GetLongField,
		GetFloatField: super::field::GetFloatField,
		GetDoubleField: super::field::GetDoubleField,
		SetObjectField: super::field::SetObjectField,
		SetBooleanField: super::field::SetBooleanField,
		SetByteField: super::field::SetByteField,
		SetCharField: super::field::SetCharField,
		SetShortField: super::field::SetShortField,
		SetIntField: super::field::SetIntField,
		SetLongField: super::field::SetLongField,
		SetFloatField: super::field::SetFloatField,
		SetDoubleField: super::field::SetDoubleField,
		GetStaticMethodID: super::method::GetStaticMethodID,
		CallStaticObjectMethod: super::method::CallStaticObjectMethod,
		CallStaticObjectMethodV: super::method::CallStaticObjectMethodV,
		CallStaticObjectMethodA: super::method::CallStaticObjectMethodA,
		CallStaticBooleanMethod: super::method::CallStaticBooleanMethod,
		CallStaticBooleanMethodV: super::method::CallStaticBooleanMethodV,
		CallStaticBooleanMethodA: super::method::CallStaticBooleanMethodA,
		CallStaticByteMethod: super::method::CallStaticByteMethod,
		CallStaticByteMethodV: super::method::CallStaticByteMethodV,
		CallStaticByteMethodA: super::method::CallStaticByteMethodA,
		CallStaticCharMethod: super::method::CallStaticCharMethod,
		CallStaticCharMethodV: super::method::CallStaticCharMethodV,
		CallStaticCharMethodA: super::method::CallStaticCharMethodA,
		CallStaticShortMethod: super::method::CallStaticShortMethod,
		CallStaticShortMethodV: super::method::CallStaticShortMethodV,
		CallStaticShortMethodA: super::method::CallStaticShortMethodA,
		CallStaticIntMethod: super::method::CallStaticIntMethod,
		CallStaticIntMethodV: super::method::CallStaticIntMethodV,
		CallStaticIntMethodA: super::method::CallStaticIntMethodA,
		CallStaticLongMethod: super::method::CallStaticLongMethod,
		CallStaticLongMethodV: super::method::CallStaticLongMethodV,
		CallStaticLongMethodA: super::method::CallStaticLongMethodA,
		CallStaticFloatMethod: super::method::CallStaticFloatMethod,
		CallStaticFloatMethodV: super::method::CallStaticFloatMethodV,
		CallStaticFloatMethodA: super::method::CallStaticFloatMethodA,
		CallStaticDoubleMethod: super::method::CallStaticDoubleMethod,
		CallStaticDoubleMethodV: super::method::CallStaticDoubleMethodV,
		CallStaticDoubleMethodA: super::method::CallStaticDoubleMethodA,
		CallStaticVoidMethod: super::method::CallStaticVoidMethod,
		CallStaticVoidMethodV: super::method::CallStaticVoidMethodV,
		CallStaticVoidMethodA: super::method::CallStaticVoidMethodA,
		GetStaticFieldID: super::field::GetStaticFieldID,
		GetStaticObjectField: super::field::GetStaticObjectField,
		GetStaticBooleanField: super::field::GetStaticBooleanField,
		GetStaticByteField: super::field::GetStaticByteField,
		GetStaticCharField: super::field::GetStaticCharField,
		GetStaticShortField: super::field::GetStaticShortField,
		GetStaticIntField: super::field::GetStaticIntField,
		GetStaticLongField: super::field::GetStaticLongField,
		GetStaticFloatField: super::field::GetStaticFloatField,
		GetStaticDoubleField: super::field::GetStaticDoubleField,
		SetStaticObjectField: super::field::SetStaticObjectField,
		SetStaticBooleanField: super::field::SetStaticBooleanField,
		SetStaticByteField: super::field::SetStaticByteField,
		SetStaticCharField: super::field::SetStaticCharField,
		SetStaticShortField: super::field::SetStaticShortField,
		SetStaticIntField: super::field::SetStaticIntField,
		SetStaticLongField: super::field::SetStaticLongField,
		SetStaticFloatField: super::field::SetStaticFloatField,
		SetStaticDoubleField: super::field::SetStaticDoubleField,
		NewString: super::string::NewString,
		GetStringLength: super::string::GetStringLength,
		GetStringChars: super::string::GetStringChars,
		ReleaseStringChars: super::string::ReleaseStringChars,
		NewStringUTF: super::string::NewStringUTF,
		GetStringUTFLength: super::string::GetStringUTFLength,
		GetStringUTFChars: super::string::GetStringUTFChars,
		ReleaseStringUTFChars: super::string::ReleaseStringUTFChars,
		GetArrayLength: super::array::GetArrayLength,
		NewObjectArray: super::array::NewObjectArray,
		GetObjectArrayElement: super::array::GetObjectArrayElement,
		SetObjectArrayElement: super::array::SetObjectArrayElement,
		NewBooleanArray: super::array::NewBooleanArray,
		NewByteArray: super::array::NewByteArray,
		NewCharArray: super::array::NewCharArray,
		NewShortArray: super::array::NewShortArray,
		NewIntArray: super::array::NewIntArray,
		NewLongArray: super::array::NewLongArray,
		NewFloatArray: super::array::NewFloatArray,
		NewDoubleArray: super::array::NewDoubleArray,
		GetBooleanArrayElements: super::array::GetBooleanArrayElements,
		GetByteArrayElements: super::array::GetByteArrayElements,
		GetCharArrayElements: super::array::GetCharArrayElements,
		GetShortArrayElements: super::array::GetShortArrayElements,
		GetIntArrayElements: super::array::GetIntArrayElements,
		GetLongArrayElements: super::array::GetLongArrayElements,
		GetFloatArrayElements: super::array::GetFloatArrayElements,
		GetDoubleArrayElements: super::array::GetDoubleArrayElements,
		ReleaseBooleanArrayElements: super::array::ReleaseBooleanArrayElements,
		ReleaseByteArrayElements: super::array::ReleaseByteArrayElements,
		ReleaseCharArrayElements: super::array::ReleaseCharArrayElements,
		ReleaseShortArrayElements: super::array::ReleaseShortArrayElements,
		ReleaseIntArrayElements: super::array::ReleaseIntArrayElements,
		ReleaseLongArrayElements: super::array::ReleaseLongArrayElements,
		ReleaseFloatArrayElements: super::array::ReleaseFloatArrayElements,
		ReleaseDoubleArrayElements: super::array::ReleaseDoubleArrayElements,
		GetBooleanArrayRegion: super::array::GetBooleanArrayRegion,
		GetByteArrayRegion: super::array::GetByteArrayRegion,
		GetCharArrayRegion: super::array::GetCharArrayRegion,
		GetShortArrayRegion: super::array::GetShortArrayRegion,
		GetIntArrayRegion: super::array::GetIntArrayRegion,
		GetLongArrayRegion: super::array::GetLongArrayRegion,
		GetFloatArrayRegion: super::array::GetFloatArrayRegion,
		GetDoubleArrayRegion: super::array::GetDoubleArrayRegion,
		SetBooleanArrayRegion: super::array::SetBooleanArrayRegion,
		SetByteArrayRegion: super::array::SetByteArrayRegion,
		SetCharArrayRegion: super::array::SetCharArrayRegion,
		SetShortArrayRegion: super::array::SetShortArrayRegion,
		SetIntArrayRegion: super::array::SetIntArrayRegion,
		SetLongArrayRegion: super::array::SetLongArrayRegion,
		SetFloatArrayRegion: super::array::SetFloatArrayRegion,
		SetDoubleArrayRegion: super::array::SetDoubleArrayRegion,
		RegisterNatives: super::register::RegisterNatives,
		UnregisterNatives: super::register::UnregisterNatives,
		MonitorEnter: super::monitor::MonitorEnter,
		MonitorExit: super::monitor::MonitorExit,
		GetJavaVM: super::vm::GetJavaVM,
		GetStringRegion: super::string::GetStringRegion,
		GetStringUTFRegion: super::string::GetStringUTFRegion,
		GetPrimitiveArrayCritical: super::array::GetPrimitiveArrayCritical,
		ReleasePrimitiveArrayCritical: super::array::ReleasePrimitiveArrayCritical,
		GetStringCritical: super::string::GetStringCritical,
		ReleaseStringCritical: super::string::ReleaseStringCritical,
		NewWeakGlobalRef: super::weak::NewWeakGlobalRef,
		DeleteWeakGlobalRef: super::weak::DeleteWeakGlobalRef,
		ExceptionCheck: super::exceptions::ExceptionCheck,
		NewDirectByteBuffer: super::nio::NewDirectByteBuffer,
		GetDirectBufferAddress: super::nio::GetDirectBufferAddress,
		GetDirectBufferCapacity: super::nio::GetDirectBufferCapacity,
		GetObjectRefType: super::object::GetObjectRefType,
	};

	Box::into_raw(Box::new(native_interface)) as jni::sys::JNIEnv
}
