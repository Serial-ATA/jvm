#![feature(extern_types)]
#![no_std]
#![allow(non_snake_case, non_camel_case_types)]

#[rustfmt::skip]
mod version;

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_void};

use jni_sys::{
	JNIEnv, JNINativeInterface_, JavaVM, jboolean, jchar, jclass, jdouble, jfieldID, jfloat, jint,
	jlong, jmethodID, jobject, jvalue,
};

pub const JVMTI_VERSION_1: c_int = 0x30010000;
pub const JVMTI_VERSION_1_0: c_int = 0x30010000;
pub const JVMTI_VERSION_1_1: c_int = 0x30010100;
pub const JVMTI_VERSION_1_2: c_int = 0x30010200;
pub const JVMTI_VERSION_9: c_int = 0x30090000;
pub const JVMTI_VERSION_11: c_int = 0x300B0000;
pub const JVMTI_VERSION_19: c_int = 0x30130000;
pub const JVMTI_VERSION_21: c_int = 0x30150000;
pub const JVMTI_VERSION: c_int = version::jvmti_version();

unsafe extern "system" {
	pub unsafe fn Agent_OnLoad(
		vm: *mut JavaVM,
		options: *mut c_char,
		reserved: *mut c_void,
	) -> jint;
	pub unsafe fn Agent_OnAttach(
		vm: *mut JavaVM,
		options: *mut c_char,
		reserved: *mut c_void,
	) -> jint;
	pub unsafe fn Agent_OnUnload(vm: *mut JavaVM);
}

// === Derived base types ===

pub type jthread = jobject;
pub type jthreadGroup = jobject;
pub type jlocation = jlong;
unsafe extern "system" {
	pub type _jrawMonitorID;
}
pub type jrawMonitorID = *mut _jrawMonitorID;
pub type jniNativeInterface = *mut JNINativeInterface_;

// === Constants ===

pub const JVMTI_THREAD_STATE_ALIVE: c_int = 0x0001;
pub const JVMTI_THREAD_STATE_TERMINATED: c_int = 0x0002;
pub const JVMTI_THREAD_STATE_RUNNABLE: c_int = 0x0004;
pub const JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER: c_int = 0x0400;
pub const JVMTI_THREAD_STATE_WAITING: c_int = 0x0080;
pub const JVMTI_THREAD_STATE_WAITING_INDEFINITELY: c_int = 0x0010;
pub const JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT: c_int = 0x0020;
pub const JVMTI_THREAD_STATE_SLEEPING: c_int = 0x0040;
pub const JVMTI_THREAD_STATE_IN_OBJECT_WAIT: c_int = 0x0100;
pub const JVMTI_THREAD_STATE_PARKED: c_int = 0x0200;
pub const JVMTI_THREAD_STATE_SUSPENDED: c_int = 0x100000;
pub const JVMTI_THREAD_STATE_INTERRUPTED: c_int = 0x200000;
pub const JVMTI_THREAD_STATE_IN_NATIVE: c_int = 0x400000;
pub const JVMTI_THREAD_STATE_VENDOR_1: c_int = 0x10000000;
pub const JVMTI_THREAD_STATE_VENDOR_2: c_int = 0x20000000;
pub const JVMTI_THREAD_STATE_VENDOR_3: c_int = 0x40000000;

pub const JVMTI_JAVA_LANG_THREAD_STATE_MASK: c_int = JVMTI_THREAD_STATE_TERMINATED
	| JVMTI_THREAD_STATE_ALIVE
	| JVMTI_THREAD_STATE_RUNNABLE
	| JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER
	| JVMTI_THREAD_STATE_WAITING
	| JVMTI_THREAD_STATE_WAITING_INDEFINITELY
	| JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT;
pub const JVMTI_JAVA_LANG_THREAD_STATE_NEW: c_int = 0;
pub const JVMTI_JAVA_LANG_THREAD_STATE_TERMINATED: c_int = JVMTI_THREAD_STATE_TERMINATED;
pub const JVMTI_JAVA_LANG_THREAD_STATE_RUNNABLE: c_int =
	JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_RUNNABLE;
pub const JVMTI_JAVA_LANG_THREAD_STATE_BLOCKED: c_int =
	JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER;
pub const JVMTI_JAVA_LANG_THREAD_STATE_WAITING: c_int =
	JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_WAITING | JVMTI_THREAD_STATE_WAITING_INDEFINITELY;
pub const JVMTI_JAVA_LANG_THREAD_STATE_TIMED_WAITING: c_int =
	JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_WAITING | JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT;

pub const JVMTI_THREAD_MIN_PRIORITY: c_int = 1;
pub const JVMTI_THREAD_NORM_PRIORITY: c_int = 5;
pub const JVMTI_THREAD_MAX_PRIORITY: c_int = 10;

pub const JVMTI_HEAP_FILTER_TAGGED: c_int = 0x4;
pub const JVMTI_HEAP_FILTER_UNTAGGED: c_int = 0x8;
pub const JVMTI_HEAP_FILTER_CLASS_TAGGED: c_int = 0x10;
pub const JVMTI_HEAP_FILTER_CLASS_UNTAGGED: c_int = 0x20;

pub const JVMTI_VISIT_OBJECTS: c_int = 0x100;
pub const JVMTI_VISIT_ABORT: c_int = 0x8000;

pub type jvmtiHeapReferenceKind = c_int;
pub const JVMTI_HEAP_REFERENCE_CLASS: jvmtiHeapReferenceKind = 1;
pub const JVMTI_HEAP_REFERENCE_FIELD: jvmtiHeapReferenceKind = 2;
pub const JVMTI_HEAP_REFERENCE_ARRAY_ELEMENT: jvmtiHeapReferenceKind = 3;
pub const JVMTI_HEAP_REFERENCE_CLASS_LOADER: jvmtiHeapReferenceKind = 4;
pub const JVMTI_HEAP_REFERENCE_SIGNERS: jvmtiHeapReferenceKind = 5;
pub const JVMTI_HEAP_REFERENCE_PROTECTION_DOMAIN: jvmtiHeapReferenceKind = 6;
pub const JVMTI_HEAP_REFERENCE_INTERFACE: jvmtiHeapReferenceKind = 7;
pub const JVMTI_HEAP_REFERENCE_STATIC_FIELD: jvmtiHeapReferenceKind = 8;
pub const JVMTI_HEAP_REFERENCE_CONSTANT_POOL: jvmtiHeapReferenceKind = 9;
pub const JVMTI_HEAP_REFERENCE_SUPERCLASS: jvmtiHeapReferenceKind = 10;
pub const JVMTI_HEAP_REFERENCE_JNI_GLOBAL: jvmtiHeapReferenceKind = 21;
pub const JVMTI_HEAP_REFERENCE_SYSTEM_CLASS: jvmtiHeapReferenceKind = 22;
pub const JVMTI_HEAP_REFERENCE_MONITOR: jvmtiHeapReferenceKind = 23;
pub const JVMTI_HEAP_REFERENCE_STACK_LOCAL: jvmtiHeapReferenceKind = 24;
pub const JVMTI_HEAP_REFERENCE_JNI_LOCAL: jvmtiHeapReferenceKind = 25;
pub const JVMTI_HEAP_REFERENCE_THREAD: jvmtiHeapReferenceKind = 26;
pub const JVMTI_HEAP_REFERENCE_OTHER: jvmtiHeapReferenceKind = 27;

pub type jvmtiPrimitiveType = c_int;
pub const JVMTI_PRIMITIVE_TYPE_BOOLEAN: jvmtiPrimitiveType = 90;
pub const JVMTI_PRIMITIVE_TYPE_BYTE: jvmtiPrimitiveType = 66;
pub const JVMTI_PRIMITIVE_TYPE_CHAR: jvmtiPrimitiveType = 67;
pub const JVMTI_PRIMITIVE_TYPE_SHORT: jvmtiPrimitiveType = 83;
pub const JVMTI_PRIMITIVE_TYPE_INT: jvmtiPrimitiveType = 73;
pub const JVMTI_PRIMITIVE_TYPE_LONG: jvmtiPrimitiveType = 74;
pub const JVMTI_PRIMITIVE_TYPE_FLOAT: jvmtiPrimitiveType = 70;
pub const JVMTI_PRIMITIVE_TYPE_DOUBLE: jvmtiPrimitiveType = 68;

pub type jvmtiHeapObjectFilter = c_int;
pub const JVMTI_HEAP_OBJECT_TAGGED: jvmtiHeapObjectFilter = 1;
pub const JVMTI_HEAP_OBJECT_UNTAGGED: jvmtiHeapObjectFilter = 2;
pub const JVMTI_HEAP_OBJECT_EITHER: jvmtiHeapObjectFilter = 3;

pub type jvmtiHeapRootKind = c_int;
pub const JVMTI_HEAP_ROOT_JNI_GLOBAL: jvmtiHeapRootKind = 1;
pub const JVMTI_HEAP_ROOT_SYSTEM_CLASS: jvmtiHeapRootKind = 2;
pub const JVMTI_HEAP_ROOT_MONITOR: jvmtiHeapRootKind = 3;
pub const JVMTI_HEAP_ROOT_STACK_LOCAL: jvmtiHeapRootKind = 4;
pub const JVMTI_HEAP_ROOT_JNI_LOCAL: jvmtiHeapRootKind = 5;
pub const JVMTI_HEAP_ROOT_THREAD: jvmtiHeapRootKind = 6;
pub const JVMTI_HEAP_ROOT_OTHER: jvmtiHeapRootKind = 7;

pub type jvmtiObjectReferenceKind = c_int;
pub const JVMTI_REFERENCE_CLASS: jvmtiObjectReferenceKind = 1;
pub const JVMTI_REFERENCE_FIELD: jvmtiObjectReferenceKind = 2;
pub const JVMTI_REFERENCE_ARRAY_ELEMENT: jvmtiObjectReferenceKind = 3;
pub const JVMTI_REFERENCE_CLASS_LOADER: jvmtiObjectReferenceKind = 4;
pub const JVMTI_REFERENCE_SIGNERS: jvmtiObjectReferenceKind = 5;
pub const JVMTI_REFERENCE_PROTECTION_DOMAIN: jvmtiObjectReferenceKind = 6;
pub const JVMTI_REFERENCE_INTERFACE: jvmtiObjectReferenceKind = 7;
pub const JVMTI_REFERENCE_STATIC_FIELD: jvmtiObjectReferenceKind = 8;
pub const JVMTI_REFERENCE_CONSTANT_POOL: jvmtiObjectReferenceKind = 9;

pub type jvmtiIterationControl = c_int;
pub const JVMTI_ITERATION_CONTINUE: jvmtiIterationControl = 1;
pub const JVMTI_ITERATION_IGNORE: jvmtiIterationControl = 2;
pub const JVMTI_ITERATION_ABORT: jvmtiIterationControl = 0;

pub const JVMTI_CLASS_STATUS_VERIFIED: c_int = 1;
pub const JVMTI_CLASS_STATUS_PREPARED: c_int = 2;
pub const JVMTI_CLASS_STATUS_INITIALIZED: c_int = 4;
pub const JVMTI_CLASS_STATUS_ERROR: c_int = 8;
pub const JVMTI_CLASS_STATUS_ARRAY: c_int = 16;
pub const JVMTI_CLASS_STATUS_PRIMITIVE: c_int = 32;

pub type jvmtiEventMode = c_int;
pub const JVMTI_ENABLE: jvmtiEventMode = 1;
pub const JVMTI_DISABLE: jvmtiEventMode = 0;

pub type jvmtiParamTypes = c_int;
pub const JVMTI_TYPE_JBYTE: jvmtiParamTypes = 101;
pub const JVMTI_TYPE_JCHAR: jvmtiParamTypes = 102;
pub const JVMTI_TYPE_JSHORT: jvmtiParamTypes = 103;
pub const JVMTI_TYPE_JINT: jvmtiParamTypes = 104;
pub const JVMTI_TYPE_JLONG: jvmtiParamTypes = 105;
pub const JVMTI_TYPE_JFLOAT: jvmtiParamTypes = 106;
pub const JVMTI_TYPE_JDOUBLE: jvmtiParamTypes = 107;
pub const JVMTI_TYPE_JBOOLEAN: jvmtiParamTypes = 108;
pub const JVMTI_TYPE_JOBJECT: jvmtiParamTypes = 109;
pub const JVMTI_TYPE_JTHREAD: jvmtiParamTypes = 110;
pub const JVMTI_TYPE_JCLASS: jvmtiParamTypes = 111;
pub const JVMTI_TYPE_JVALUE: jvmtiParamTypes = 112;
pub const JVMTI_TYPE_JFIELDID: jvmtiParamTypes = 113;
pub const JVMTI_TYPE_JMETHODID: jvmtiParamTypes = 114;
pub const JVMTI_TYPE_CCHAR: jvmtiParamTypes = 115;
pub const JVMTI_TYPE_CVOID: jvmtiParamTypes = 116;
pub const JVMTI_TYPE_JNIENV: jvmtiParamTypes = 117;

pub type jvmtiParamKind = c_int;
pub const JVMTI_KIND_IN: jvmtiParamKind = 91;
pub const JVMTI_KIND_IN_PTR: jvmtiParamKind = 92;
pub const JVMTI_KIND_IN_BUF: jvmtiParamKind = 93;
pub const JVMTI_KIND_ALLOC_BUF: jvmtiParamKind = 94;
pub const JVMTI_KIND_ALLOC_ALLOC_BUF: jvmtiParamKind = 95;
pub const JVMTI_KIND_OUT: jvmtiParamKind = 96;
pub const JVMTI_KIND_OUT_BUF: jvmtiParamKind = 97;

pub type jvmtiTimerKind = c_int;
pub const JVMTI_TIMER_USER_CPU: jvmtiTimerKind = 30;
pub const JVMTI_TIMER_TOTAL_CPU: jvmtiTimerKind = 31;
pub const JVMTI_TIMER_ELAPSED: jvmtiTimerKind = 32;

pub type jvmtiPhase = c_int;
pub const JVMTI_PHASE_ONLOAD: jvmtiPhase = 1;
pub const JVMTI_PHASE_PRIMORDIAL: jvmtiPhase = 2;
pub const JVMTI_PHASE_START: jvmtiPhase = 6;
pub const JVMTI_PHASE_LIVE: jvmtiPhase = 4;
pub const JVMTI_PHASE_DEAD: jvmtiPhase = 8;

pub const JVMTI_VERSION_INTERFACE_JNI: c_int = 0x00000000;
pub const JVMTI_VERSION_INTERFACE_JVMTI: c_int = 0x30000000;

pub const JVMTI_VERSION_MASK_INTERFACE_TYPE: c_int = 0x70000000;
pub const JVMTI_VERSION_MASK_MAJOR: c_int = 0x0FFF0000;
pub const JVMTI_VERSION_MASK_MINOR: c_int = 0x0000FF00;
pub const JVMTI_VERSION_MASK_MICRO: c_int = 0x000000FF;

pub const JVMTI_VERSION_SHIFT_MAJOR: c_int = 16;
pub const JVMTI_VERSION_SHIFT_MINOR: c_int = 8;
pub const JVMTI_VERSION_SHIFT_MICRO: c_int = 0;

pub type jvmtiVerboseFlag = c_int;
pub const JVMTI_VERBOSE_OTHER: jvmtiVerboseFlag = 0;
pub const JVMTI_VERBOSE_GC: jvmtiVerboseFlag = 1;
pub const JVMTI_VERBOSE_CLASS: jvmtiVerboseFlag = 2;
pub const JVMTI_VERBOSE_JNI: jvmtiVerboseFlag = 4;

pub type jvmtiJlocationFormat = c_int;
pub const JVMTI_JLOCATION_JVMBCI: jvmtiJlocationFormat = 1;
pub const JVMTI_JLOCATION_MACHINEPC: jvmtiJlocationFormat = 2;
pub const JVMTI_JLOCATION_OTHER: jvmtiJlocationFormat = 0;

pub const JVMTI_RESOURCE_EXHAUSTED_OOM_ERROR: c_int = 0x0001;
pub const JVMTI_RESOURCE_EXHAUSTED_JAVA_HEAP: c_int = 0x0002;
pub const JVMTI_RESOURCE_EXHAUSTED_THREADS: c_int = 0x0004;

pub type jvmtiError = c_int;
pub const JVMTI_ERROR_NONE: jvmtiError = 0;
pub const JVMTI_ERROR_INVALID_THREAD: jvmtiError = 10;
pub const JVMTI_ERROR_INVALID_THREAD_GROUP: jvmtiError = 11;
pub const JVMTI_ERROR_INVALID_PRIORITY: jvmtiError = 12;
pub const JVMTI_ERROR_THREAD_NOT_SUSPENDED: jvmtiError = 13;
pub const JVMTI_ERROR_THREAD_SUSPENDED: jvmtiError = 14;
pub const JVMTI_ERROR_THREAD_NOT_ALIVE: jvmtiError = 15;
pub const JVMTI_ERROR_INVALID_OBJECT: jvmtiError = 20;
pub const JVMTI_ERROR_INVALID_CLASS: jvmtiError = 21;
pub const JVMTI_ERROR_CLASS_NOT_PREPARED: jvmtiError = 22;
pub const JVMTI_ERROR_INVALID_METHODID: jvmtiError = 23;
pub const JVMTI_ERROR_INVALID_LOCATION: jvmtiError = 24;
pub const JVMTI_ERROR_INVALID_FIELDID: jvmtiError = 25;
pub const JVMTI_ERROR_INVALID_MODULE: jvmtiError = 26;
pub const JVMTI_ERROR_NO_MORE_FRAMES: jvmtiError = 31;
pub const JVMTI_ERROR_OPAQUE_FRAME: jvmtiError = 32;
pub const JVMTI_ERROR_TYPE_MISMATCH: jvmtiError = 34;
pub const JVMTI_ERROR_INVALID_SLOT: jvmtiError = 35;
pub const JVMTI_ERROR_DUPLICATE: jvmtiError = 40;
pub const JVMTI_ERROR_NOT_FOUND: jvmtiError = 41;
pub const JVMTI_ERROR_INVALID_MONITOR: jvmtiError = 50;
pub const JVMTI_ERROR_NOT_MONITOR_OWNER: jvmtiError = 51;
pub const JVMTI_ERROR_INTERRUPT: jvmtiError = 52;
pub const JVMTI_ERROR_INVALID_CLASS_FORMAT: jvmtiError = 60;
pub const JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION: jvmtiError = 61;
pub const JVMTI_ERROR_FAILS_VERIFICATION: jvmtiError = 62;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED: jvmtiError = 63;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED: jvmtiError = 64;
pub const JVMTI_ERROR_INVALID_TYPESTATE: jvmtiError = 65;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED: jvmtiError = 66;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED: jvmtiError = 67;
pub const JVMTI_ERROR_UNSUPPORTED_VERSION: jvmtiError = 68;
pub const JVMTI_ERROR_NAMES_DONT_MATCH: jvmtiError = 69;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED: jvmtiError = 70;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED: jvmtiError = 71;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_ATTRIBUTE_CHANGED: jvmtiError = 72;
pub const JVMTI_ERROR_UNSUPPORTED_OPERATION: jvmtiError = 73;
pub const JVMTI_ERROR_UNMODIFIABLE_CLASS: jvmtiError = 79;
pub const JVMTI_ERROR_UNMODIFIABLE_MODULE: jvmtiError = 80;
pub const JVMTI_ERROR_NOT_AVAILABLE: jvmtiError = 98;
pub const JVMTI_ERROR_MUST_POSSESS_CAPABILITY: jvmtiError = 99;
pub const JVMTI_ERROR_NULL_POINTER: jvmtiError = 100;
pub const JVMTI_ERROR_ABSENT_INFORMATION: jvmtiError = 101;
pub const JVMTI_ERROR_INVALID_EVENT_TYPE: jvmtiError = 102;
pub const JVMTI_ERROR_ILLEGAL_ARGUMENT: jvmtiError = 103;
pub const JVMTI_ERROR_NATIVE_METHOD: jvmtiError = 104;
pub const JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED: jvmtiError = 106;
pub const JVMTI_ERROR_OUT_OF_MEMORY: jvmtiError = 110;
pub const JVMTI_ERROR_ACCESS_DENIED: jvmtiError = 111;
pub const JVMTI_ERROR_WRONG_PHASE: jvmtiError = 112;
pub const JVMTI_ERROR_INTERNAL: jvmtiError = 113;
pub const JVMTI_ERROR_UNATTACHED_THREAD: jvmtiError = 115;
pub const JVMTI_ERROR_INVALID_ENVIRONMENT: jvmtiError = 116;
pub const JVMTI_ERROR_MAX: jvmtiError = 116;

pub type jvmtiEvent = c_int;
pub const JVMTI_MIN_EVENT_TYPE_VAL: jvmtiEvent = 50;
pub const JVMTI_EVENT_VM_INIT: jvmtiEvent = 50;
pub const JVMTI_EVENT_VM_DEATH: jvmtiEvent = 51;
pub const JVMTI_EVENT_THREAD_START: jvmtiEvent = 52;
pub const JVMTI_EVENT_THREAD_END: jvmtiEvent = 53;
pub const JVMTI_EVENT_CLASS_FILE_LOAD_HOOK: jvmtiEvent = 54;
pub const JVMTI_EVENT_CLASS_LOAD: jvmtiEvent = 55;
pub const JVMTI_EVENT_CLASS_PREPARE: jvmtiEvent = 56;
pub const JVMTI_EVENT_VM_START: jvmtiEvent = 57;
pub const JVMTI_EVENT_EXCEPTION: jvmtiEvent = 58;
pub const JVMTI_EVENT_EXCEPTION_CATCH: jvmtiEvent = 59;
pub const JVMTI_EVENT_SINGLE_STEP: jvmtiEvent = 60;
pub const JVMTI_EVENT_FRAME_POP: jvmtiEvent = 61;
pub const JVMTI_EVENT_BREAKPOINT: jvmtiEvent = 62;
pub const JVMTI_EVENT_FIELD_ACCESS: jvmtiEvent = 63;
pub const JVMTI_EVENT_FIELD_MODIFICATION: jvmtiEvent = 64;
pub const JVMTI_EVENT_METHOD_ENTRY: jvmtiEvent = 65;
pub const JVMTI_EVENT_METHOD_EXIT: jvmtiEvent = 66;
pub const JVMTI_EVENT_NATIVE_METHOD_BIND: jvmtiEvent = 67;
pub const JVMTI_EVENT_COMPILED_METHOD_LOAD: jvmtiEvent = 68;
pub const JVMTI_EVENT_COMPILED_METHOD_UNLOAD: jvmtiEvent = 69;
pub const JVMTI_EVENT_DYNAMIC_CODE_GENERATED: jvmtiEvent = 70;
pub const JVMTI_EVENT_DATA_DUMP_REQUEST: jvmtiEvent = 71;
pub const JVMTI_EVENT_MONITOR_WAIT: jvmtiEvent = 73;
pub const JVMTI_EVENT_MONITOR_WAITED: jvmtiEvent = 74;
pub const JVMTI_EVENT_MONITOR_CONTENDED_ENTER: jvmtiEvent = 75;
pub const JVMTI_EVENT_MONITOR_CONTENDED_ENTERED: jvmtiEvent = 76;
pub const JVMTI_EVENT_RESOURCE_EXHAUSTED: jvmtiEvent = 80;
pub const JVMTI_EVENT_GARBAGE_COLLECTION_START: jvmtiEvent = 81;
pub const JVMTI_EVENT_GARBAGE_COLLECTION_FINISH: jvmtiEvent = 82;
pub const JVMTI_EVENT_OBJECT_FREE: jvmtiEvent = 83;
pub const JVMTI_EVENT_VM_OBJECT_ALLOC: jvmtiEvent = 84;
pub const JVMTI_EVENT_SAMPLED_OBJECT_ALLOC: jvmtiEvent = 86;
pub const JVMTI_EVENT_VIRTUAL_THREAD_START: jvmtiEvent = 87;
pub const JVMTI_EVENT_VIRTUAL_THREAD_END: jvmtiEvent = 88;
pub const JVMTI_MAX_EVENT_TYPE_VAL: jvmtiEvent = 88;

pub type jvmtiStartFunction =
	unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, arg: *mut c_void);

pub type jvmtiHeapIterationCallback = unsafe extern "system" fn(
	class_tag: jlong,
	size: jlong,
	tag_ptr: *mut jlong,
	length: jint,
	user_data: *mut c_void,
) -> jint;

pub type jvmtiHeapReferenceCallback = unsafe extern "system" fn(
	reference_kind: jvmtiHeapReferenceKind,
	reference_info: *const jvmtiHeapReferenceInfo,
	class_tag: jlong,
	referrer_class_tag: jlong,
	size: jlong,
	tag_ptr: *mut jlong,
	referrer_tag_ptr: *mut jlong,
	length: jint,
	user_data: *mut c_void,
) -> jint;

pub type jvmtiPrimitiveFieldCallback = unsafe extern "system" fn(
	kind: jvmtiHeapReferenceKind,
	info: *const jvmtiHeapReferenceInfo,
	object_class_tag: jlong,
	object_tag_ptr: *mut jlong,
	value: jvalue,
	value_type: jvmtiPrimitiveType,
	user_data: *mut c_void,
) -> jint;

pub type jvmtiArrayPrimitiveValueCallback = unsafe extern "system" fn(
	class_tag: jlong,
	size: jlong,
	tag_ptr: *mut jlong,
	element_count: jint,
	element_type: jvmtiPrimitiveType,
	element: *const c_void,
	user_data: *mut c_void,
) -> jint;

pub type jvmtiStringPrimitiveValueCallback = unsafe extern "system" fn(
	class_tag: jlong,
	size: jlong,
	tag_ptr: *mut jlong,
	value: *const jchar,
	value_length: jint,
	user_data: *mut c_void,
) -> jint;

pub type jvmtiReservedCallback = unsafe extern "system" fn() -> jint;

pub type jvmtiHeapObjectCallback = unsafe extern "system" fn(
	class_tag: jlong,
	size: jlong,
	tag_ptr: *mut jlong,
	user_data: *mut c_void,
) -> jvmtiIterationControl;

pub type jvmtiHeapRootCallback = unsafe extern "system" fn(
	root_kind: jvmtiHeapRootKind,
	class_tag: jlong,
	size: jlong,
	tag_ptr: *mut jlong,
	user_data: *mut c_void,
) -> jvmtiIterationControl;

pub type jvmtiStackReferenceCallback = unsafe extern "system" fn(
	root_kind: jvmtiHeapRootKind,
	class_tag: jlong,
	size: jlong,
	tag_ptr: *mut jlong,
	thread_tag: jlong,
	depth: jint,
	method: jmethodID,
	slot: jint,
	user_data: *mut c_void,
) -> jvmtiIterationControl;

pub type jvmtiObjectReferenceCallback = unsafe extern "system" fn(
	reference_kind: jvmtiObjectReferenceKind,
	class_tag: jlong,
	size: jlong,
	tag_ptr: *mut jlong,
	referrer_tag: jlong,
	referrer_index: jint,
	user_data: *mut c_void,
) -> jvmtiIterationControl;

pub type jvmtiExtensionFunction = unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, ...) -> jvmtiError;

pub type jvmtiExtensionEvent = unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, ...);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiThreadInfo {
	pub name: *mut c_char,
	pub priority: jint,
	pub is_daemon: jboolean,
	pub thread_group: jthreadGroup,
	pub context_class_loader: jobject,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiMonitorStackDepthInfo {
	pub monitor: jobject,
	pub stack_depth: jint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiThreadGroupInfo {
	pub parent: jthreadGroup,
	pub name: *mut c_char,
	pub max_priority: jint,
	pub is_daemon: jboolean,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiFrameInfo {
	pub method: jmethodID,
	pub location: jlocation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiStackInfo {
	pub thread: jthread,
	pub state: jint,
	pub frame_buffer: *const jvmtiFrameInfo,
	pub frame_count: jint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiHeapReferenceInfoField {
	pub index: jint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiHeapReferenceInfoArray {
	pub index: jint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiHeapReferenceInfoConstantPool {
	pub index: jint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiHeapReferenceInfoStackLocal {
	pub thread_tag: jlong,
	pub thread_id: jlong,
	pub depth: jint,
	pub method: jmethodID,
	pub location: jlocation,
	pub slot: jint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiHeapReferenceInfoJniLocal {
	pub thread_tag: jlong,
	pub thread_id: jlong,
	pub depth: jint,
	pub method: jmethodID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiHeapReferenceInfoReserved {
	pub reserved1: jlong,
	pub reserved2: jlong,
	pub reserved3: jlong,
	pub reserved4: jlong,
	pub reserved5: jlong,
	pub reserved6: jlong,
	pub reserved7: jlong,
	pub reserved8: jlong,
}

pub union jvmtiHeapReferenceInfo {
	pub field: jvmtiHeapReferenceInfoField,
	pub array: jvmtiHeapReferenceInfoArray,
	pub constant_pool: jvmtiHeapReferenceInfoConstantPool,
	pub stack_local: jvmtiHeapReferenceInfoStackLocal,
	pub jni_local: jvmtiHeapReferenceInfoJniLocal,
	pub other: jvmtiHeapReferenceInfoReserved,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiHeapCallbacks {
	pub heap_iteration_callback: jvmtiHeapIterationCallback,
	pub heap_reference_callback: jvmtiHeapReferenceCallback,
	pub primitive_field_callback: jvmtiPrimitiveFieldCallback,
	pub array_primitive_value_callback: jvmtiArrayPrimitiveValueCallback,
	pub string_primitive_value_callback: jvmtiStringPrimitiveValueCallback,
	pub reserved5: jvmtiReservedCallback,
	pub reserved6: jvmtiReservedCallback,
	pub reserved7: jvmtiReservedCallback,
	pub reserved8: jvmtiReservedCallback,
	pub reserved9: jvmtiReservedCallback,
	pub reserved10: jvmtiReservedCallback,
	pub reserved11: jvmtiReservedCallback,
	pub reserved12: jvmtiReservedCallback,
	pub reserved13: jvmtiReservedCallback,
	pub reserved14: jvmtiReservedCallback,
	pub reserved15: jvmtiReservedCallback,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiClassDefinition {
	pub klass: jclass,
	pub class_byte_count: jint,
	pub class_bytes: *const c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiMonitorUsage {
	pub owner: jthread,
	pub entry_count: jint,
	pub waiter_count: jint,
	pub waiters: *mut jthread,
	pub notify_waiter_count: jint,
	pub notify_waiters: *mut jthread,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiLineNumberEntry {
	pub start_location: jlocation,
	pub line_number: jint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiLocalVariableEntry {
	pub start_location: jlocation,
	pub length: jint,
	pub name: *mut c_char,
	pub signature: *mut c_char,
	pub generic_signature: *mut c_char,
	pub slot: jint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiParamInfo {
	pub name: *mut c_char,
	pub kind: jvmtiParamKind,
	pub base_type: jvmtiParamTypes,
	pub null_ok: jboolean,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiExtensionFunctionInfo {
	pub func: jvmtiExtensionFunction,
	pub id: *mut c_char,
	pub short_description: *mut c_char,
	pub param_count: jint,
	pub params: *mut jvmtiParamInfo,
	pub error_count: jint,
	pub errors: *mut jvmtiError,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiExtensionEventInfo {
	pub extension_event_index: jint,
	pub id: *mut c_char,
	pub short_description: *mut c_char,
	pub param_count: jint,
	pub params: *mut jvmtiParamInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiTimerInfo {
	pub max_value: jlong,
	pub may_skip_forward: jboolean,
	pub may_skip_backward: jboolean,
	pub kind: jvmtiTimerKind,
	pub reserved1: jlong,
	pub reserved2: jlong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiAddrLocationMap {
	pub start_address: *const c_void,
	pub location: jlocation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiCapabilities {
	// TODO: getters/setters
	bits: [c_uint; 4],
}

pub type jvmtiEventReserved = unsafe extern "system" fn();

pub type jvmtiEventBreakpoint = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	method: jmethodID,
	location: jlocation,
);

pub type jvmtiEventClassFileLoadHook = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	class_being_redefined: jclass,
	loader: jobject,
	name: *const c_char,
	protection_domain: jobject,
	class_data_len: jint,
	class_data: *const c_uchar,
	new_class_data_len: jint,
	*mut *mut c_uchar,
);

pub type jvmtiEventClassLoad = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	klass: jclass,
);

pub type jvmtiEventClassPrepare = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	klass: jclass,
);

pub type jvmtiEventCompiledMethodLoad = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	method: jmethodID,
	code_size: jint,
	code_addr: *const c_void,
	map_length: jint,
	map: *const jvmtiAddrLocationMap,
	compile_info: *const c_void,
);

pub type jvmtiEventCompiledMethodUnload = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	method: jmethodID,
	code_addr: *const c_void,
);

pub type jvmtiEventDataDumpRequest = unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv);

pub type jvmtiEventDynamicCodeGenerated = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	name: *const c_char,
	address: *const c_void,
	length: jint,
);

pub type jvmtiEventException = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	method: jmethodID,
	location: jlocation,
	exception: jobject,
	catch_method: jmethodID,
	catch_location: jlocation,
);

pub type jvmtiEventExceptionCatch = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	method: jmethodID,
	location: jlocation,
	exception: jobject,
);

pub type jvmtiEventFieldAccess = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	method: jmethodID,
	location: jlocation,
	field_klass: jclass,
	object: jobject,
	field: jfieldID,
);

pub type jvmtiEventFieldModification = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	method: jmethodID,
	location: jlocation,
	field_klass: jclass,
	object: jobject,
	field: jfieldID,
	signature_type: c_char,
	new_value: jvalue,
);

pub type jvmtiEventFramePop = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	method: jmethodID,
	was_popped_by_execution: jboolean,
);

pub type jvmtiEventGarbageCollectionFinish = unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv);

pub type jvmtiEventGarbageCollectionStart = unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv);

pub type jvmtiEventMethodEntry = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	method: jmethodID,
);

pub type jvmtiEventMethodExit = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	method: jmethodID,
	was_popped_by_execution: jboolean,
	return_value: jvalue,
);

pub type jvmtiEventMonitorContendedEnter = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	object: jobject,
);

pub type jvmtiEventMonitorContendedEntered = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	object: jobject,
);

pub type jvmtiEventMonitorWait = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	object: jobject,
	timeout: jlong,
);

pub type jvmtiEventMonitorWaited = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	object: jobject,
	timed_out: jboolean,
);

pub type jvmtiEventNativeMethodBind = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	method: jmethodID,
	address: *mut c_void,
	new_address_ptr: *mut *mut c_void,
);

pub type jvmtiEventObjectFree = unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, tag: jlong);

pub type jvmtiEventResourceExhausted = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	flags: jint,
	reserved: *const c_void,
	description: *const c_char,
);

pub type jvmtiEventSampledObjectAlloc = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	object: jobject,
	object_klass: jclass,
	size: jlong,
);

pub type jvmtiEventSingleStep = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	method: jmethodID,
	location: jlocation,
);

pub type jvmtiEventThreadEnd =
	unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread);

pub type jvmtiEventThreadStart =
	unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread);

pub type jvmtiEventVirtualThreadEnd = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	virtual_thread: jthread,
);

pub type jvmtiEventVirtualThreadStart = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	virtual_thread: jthread,
);

pub type jvmtiEventVMDeath =
	unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv);

pub type jvmtiEventVMInit =
	unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread);

pub type jvmtiEventVMObjectAlloc = unsafe extern "system" fn(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut JNIEnv,
	thread: jthread,
	object: jobject,
	object_klass: jclass,
	size: jlong,
);

pub type jvmtiEventVMStart =
	unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiEventCallbacks {
	pub VMInit: jvmtiEventVMInit,
	pub VMDeath: jvmtiEventVMDeath,
	pub ThreadStart: jvmtiEventThreadStart,
	pub ThreadEnd: jvmtiEventThreadEnd,
	pub ClassFileLoadHook: jvmtiEventClassFileLoadHook,
	pub ClassLoad: jvmtiEventClassLoad,
	pub ClassPrepare: jvmtiEventClassPrepare,
	pub VMStart: jvmtiEventVMStart,
	pub Exception: jvmtiEventException,
	pub ExceptionCatch: jvmtiEventExceptionCatch,
	pub SingleStep: jvmtiEventSingleStep,
	pub FramePop: jvmtiEventFramePop,
	pub Breakpoint: jvmtiEventBreakpoint,
	pub FieldAccess: jvmtiEventFieldAccess,
	pub FieldModification: jvmtiEventFieldModification,
	pub MethodEntry: jvmtiEventMethodEntry,
	pub MethodExit: jvmtiEventMethodExit,
	pub NativeMethodBind: jvmtiEventNativeMethodBind,
	pub CompiledMethodLoad: jvmtiEventCompiledMethodLoad,
	pub CompiledMethodUnload: jvmtiEventCompiledMethodUnload,
	pub DynamicCodeGenerated: jvmtiEventDynamicCodeGenerated,
	pub DataDumpRequest: jvmtiEventDataDumpRequest,
	pub reserved72: jvmtiEventReserved,
	pub MonitorWait: jvmtiEventMonitorWait,
	pub MonitorWaited: jvmtiEventMonitorWaited,
	pub MonitorContendedEnter: jvmtiEventMonitorContendedEnter,
	pub MonitorContendedEntered: jvmtiEventMonitorContendedEntered,
	pub reserved77: jvmtiEventReserved,
	pub reserved78: jvmtiEventReserved,
	pub reserved79: jvmtiEventReserved,
	pub ResourceExhausted: jvmtiEventResourceExhausted,
	pub GarbageCollectionStart: jvmtiEventGarbageCollectionStart,
	pub GarbageCollectionFinish: jvmtiEventGarbageCollectionFinish,
	pub ObjectFree: jvmtiEventObjectFree,
	pub VMObjectAlloc: jvmtiEventVMObjectAlloc,
	pub reserved85: jvmtiEventReserved,
	pub SampledObjectAlloc: jvmtiEventSampledObjectAlloc,
	pub VirtualThreadStart: jvmtiEventVirtualThreadStart,
	pub VirtualThreadEnd: jvmtiEventVirtualThreadEnd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvmtiInterface_1_ {
	pub reserved1: *const c_void,

	pub SetEventNotificationMode: unsafe extern "C" fn(
		env: *mut jvmtiEnv,
		mode: jvmtiEventMode,
		event_type: jvmtiEvent,
		event_thread: jthread,
		...
	) -> jvmtiError,
	pub GetAllModules: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		module_count_ptr: *mut jint,
		modules_ptr: *mut *mut jobject,
	) -> jvmtiError,
	pub GetAllThreads: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		threads_count_ptr: *mut jint,
		threads_ptr: *mut *mut jthread,
	) -> jvmtiError,
	pub SuspendThread: unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
	pub ResumeThread: unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
	pub StopThread: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		exception: jobject,
	) -> jvmtiError,
	pub InterruptThread:
		unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
	pub GetThreadInfo: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		info_ptr: *mut jvmtiThreadInfo,
	) -> jvmtiError,
	pub GetOwnedMonitorInfo: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		owned_monitor_count_ptr: *mut jint,
		owned_monitors_ptr: *mut *mut jobject,
	) -> jvmtiError,
	pub GetCurrentContendedMonitor: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		monitor_ptr: *mut jobject,
	) -> jvmtiError,
	pub RunAgentThread: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		proc: jvmtiStartFunction,
		arg: *const c_void,
		priority: jint,
	) -> jvmtiError,
	pub GetTopThreadGroups: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		group_count_ptr: *mut jint,
		groups_ptr: *mut *mut jthreadGroup,
	) -> jvmtiError,
	pub GetThreadGroupInfo: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		group: jthreadGroup,
		info_ptr: *mut jvmtiThreadGroupInfo,
	) -> jvmtiError,
	pub GetThreadGroupChildren: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		group: jthreadGroup,
		thread_count_ptr: *mut jint,
		threads_ptr: *mut *mut jthread,
		group_count_ptr: *mut jint,
		groups_ptr: *mut *mut jthreadGroup,
	) -> jvmtiError,
	pub GetFrameCount: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		count_ptr: *mut jint,
	) -> jvmtiError,
	pub GetThreadState: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		thread_state_ptr: *mut jint,
	) -> jvmtiError,
	pub GetCurrentThread:
		unsafe extern "system" fn(env: *mut jvmtiEnv, thread_ptr: *mut jthread) -> jvmtiError,
	pub GetFrameLocation: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		depth: jint,
		method_ptr: *mut jmethodID,
		location_ptr: *mut jlocation,
	) -> jvmtiError,
	pub NotifyFramePop:
		unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint) -> jvmtiError,
	pub GetLocalObject: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		depth: jint,
		slot: jint,
		value_ptr: *mut jobject,
	) -> jvmtiError,
	pub GetLocalInt: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		depth: jint,
		slot: jint,
		value_ptr: *mut jint,
	) -> jvmtiError,
	pub GetLocalLong: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		depth: jint,
		slot: jint,
		value_ptr: *mut jlong,
	) -> jvmtiError,
	pub GetLocalFloat: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		depth: jint,
		slot: jint,
		value_ptr: *mut jfloat,
	) -> jvmtiError,
	pub GetLocalDouble: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		depth: jint,
		slot: jint,
		value_ptr: *mut jdouble,
	) -> jvmtiError,
	pub SetLocalObject: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		depth: jint,
		slot: jint,
		value: jobject,
	) -> jvmtiError,
	pub SetLocalInt: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		depth: jint,
		slot: jint,
		value: jint,
	) -> jvmtiError,
	pub SetLocalLong: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		depth: jint,
		slot: jint,
		value: jlong,
	) -> jvmtiError,
	pub SetLocalFloat: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		depth: jint,
		slot: jint,
		value: jfloat,
	) -> jvmtiError,
	pub SetLocalDouble: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		depth: jint,
		slot: jint,
		value: jdouble,
	) -> jvmtiError,
	pub CreateRawMonitor: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		name: *const c_char,
		monitor_ptr: *mut jrawMonitorID,
	) -> jvmtiError,
	pub DestroyRawMonitor:
		unsafe extern "system" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
	pub RawMonitorEnter:
		unsafe extern "system" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
	pub RawMonitorExit:
		unsafe extern "system" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
	pub RawMonitorWait: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		monitor: jrawMonitorID,
		millis: jlong,
	) -> jvmtiError,
	pub RawMonitorNotify:
		unsafe extern "system" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
	pub RawMonitorNotifyAll:
		unsafe extern "system" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
	pub SetBreakpoint: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		method: jmethodID,
		location: jlocation,
	) -> jvmtiError,
	pub ClearBreakpoint: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		method: jmethodID,
		location: jlocation,
	) -> jvmtiError,
	pub GetNamedModule: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		class_loader: jobject,
		package_name: *const c_char,
		module_ptr: *mut jobject,
	) -> jvmtiError,
	pub SetFieldAccessWatch:
		unsafe extern "system" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
	pub ClearFieldAccessWatch:
		unsafe extern "system" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
	pub SetFieldModificationWatch:
		unsafe extern "system" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
	pub ClearFieldModificationWatch:
		unsafe extern "system" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
	pub IsModifiableClass: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		is_modifiable_class_ptr: *mut jboolean,
	) -> jvmtiError,
	pub Allocate: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		size: jlong,
		mem_ptr: *mut *mut c_uchar,
	) -> jvmtiError,
	pub Deallocate: unsafe extern "system" fn(env: *mut jvmtiEnv, mem: *mut c_uchar) -> jvmtiError,
	pub GetClassSignature: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		signature_ptr: *mut *mut c_char,
		generic_ptr: *mut *mut c_char,
	) -> jvmtiError,
	pub GetClassStatus: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		status_ptr: *mut jint,
	) -> jvmtiError,
	pub GetSourceFileName: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		source_name_ptr: *mut *mut c_char,
	) -> jvmtiError,
	pub GetClassModifiers: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		modifiers_ptr: *mut jint,
	) -> jvmtiError,
	pub GetClassMethods: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		method_count_ptr: *mut jint,
		methods_ptr: *mut *mut jmethodID,
	) -> jvmtiError,
	pub GetClassFields: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		field_count_ptr: *mut jint,
		fields_ptr: *mut *mut jfieldID,
	) -> jvmtiError,
	pub GetImplementedInterfaces: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		interface_count_ptr: *mut jint,
		interfaces_ptr: *mut *mut jclass,
	) -> jvmtiError,
	pub IsInterface: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		is_interface_ptr: *mut jboolean,
	) -> jvmtiError,
	pub IsArrayClass: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		is_array_class_ptr: *mut jboolean,
	) -> jvmtiError,
	pub GetClassLoader: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		classloader_ptr: *mut jobject,
	) -> jvmtiError,
	pub GetObjectHashCode: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		object: jobject,
		hash_code_ptr: *mut jint,
	) -> jvmtiError,
	pub GetObjectMonitorUsage: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		object: jobject,
		info_ptr: *mut jvmtiMonitorUsage,
	) -> jvmtiError,
	pub GetFieldName: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		field: jfieldID,
		name_ptr: *mut *mut c_char,
		signature_ptr: *mut *mut c_char,
		generic_ptr: *mut *mut c_char,
	) -> jvmtiError,
	pub GetFieldDeclaringClass: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		field: jfieldID,
		declaring_class_ptr: *mut jclass,
	) -> jvmtiError,
	pub GetFieldModifiers: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		field: jfieldID,
		modifiers_ptr: *mut jint,
	) -> jvmtiError,
	pub IsFieldSynthetic: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		field: jfieldID,
		is_synthetic_ptr: *mut jboolean,
	) -> jvmtiError,
	pub GetMethodName: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		method: jmethodID,
		name_ptr: *mut *mut c_char,
		signature_ptr: *mut *mut c_char,
		generic_ptr: *mut *mut c_char,
	) -> jvmtiError,
	pub GetMethodDeclaringClass: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		method: jmethodID,
		declaring_class_ptr: *mut jclass,
	) -> jvmtiError,
	pub GetMethodModifiers: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		method: jmethodID,
		modifiers_ptr: *mut jint,
	) -> jvmtiError,
	pub ClearAllFramePops:
		unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
	pub GetMaxLocals: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		method: jmethodID,
		max_ptr: *mut jint,
	) -> jvmtiError,
	pub GetArgumentSize: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		method: jmethodID,
		size_ptr: *mut jint,
	) -> jvmtiError,
	pub GetLineNumberTable: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		method: jmethodID,
		entry_count_ptr: *mut jint,
		table_ptr: *mut *mut jvmtiLineNumberEntry,
	) -> jvmtiError,
	pub GetMethodLocation: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		method: jmethodID,
		start_location_ptr: *mut jlocation,
		end_location_ptr: *mut jlocation,
	) -> jvmtiError,
	pub GetLocalVariableTable: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		method: jmethodID,
		entry_count_ptr: *mut jint,
		table_ptr: *mut *mut jvmtiLocalVariableEntry,
	) -> jvmtiError,
	pub SetNativeMethodPrefix:
		unsafe extern "system" fn(env: *mut jvmtiEnv, prefix: *const c_char) -> jvmtiError,
	pub SetNativeMethodPrefixes: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		prefix_count: jint,
		prefixes: *mut *mut c_char,
	) -> jvmtiError,
	pub GetBytecodes: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		method: jmethodID,
		bytecode_count_ptr: *mut jint,
		bytecodes_ptr: *mut *mut c_uchar,
	) -> jvmtiError,
	pub IsMethodNative: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		method: jmethodID,
		is_native_ptr: *mut jboolean,
	) -> jvmtiError,
	pub IsMethodSynthetic: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		method: jmethodID,
		is_synthetic_ptr: *mut jboolean,
	) -> jvmtiError,
	pub GetLoadedClasses: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		class_count_ptr: *mut jint,
		classes_ptr: *mut *mut jclass,
	) -> jvmtiError,
	pub GetClassLoaderClasses: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		initiating_loader: jobject,
		class_count_ptr: *mut jint,
		classes_ptr: *mut *mut jclass,
	) -> jvmtiError,
	pub PopFrame: unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
	pub ForceEarlyReturnObject: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		value: jobject,
	) -> jvmtiError,
	pub ForceEarlyReturnInt:
		unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread, value: jint) -> jvmtiError,
	pub ForceEarlyReturnLong:
		unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread, value: jlong) -> jvmtiError,
	pub ForceEarlyReturnFloat:
		unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread, value: jfloat) -> jvmtiError,
	pub ForceEarlyReturnDouble: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		value: jdouble,
	) -> jvmtiError,
	pub ForceEarlyReturnVoid:
		unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
	pub RedefineClasses: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		class_count: jint,
		class_definitions: *const jvmtiClassDefinition,
	) -> jvmtiError,
	pub GetVersionNumber:
		unsafe extern "system" fn(env: *mut jvmtiEnv, version_ptr: *mut jint) -> jvmtiError,
	pub GetCapabilities: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		capabilities_ptr: *mut jvmtiCapabilities,
	) -> jvmtiError,
	pub GetSourceDebugExtension: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		source_debug_extension_ptr: *mut *mut c_char,
	) -> jvmtiError,
	pub IsMethodObsolete: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		method: jmethodID,
		is_obsolete_ptr: *mut jboolean,
	) -> jvmtiError,
	pub SuspendThreadList: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		request_count: jint,
		request_list: *const jthread,
		results: *mut jvmtiError,
	) -> jvmtiError,
	pub ResumeThreadList: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		request_count: jint,
		request_list: *const jthread,
		results: *mut jvmtiError,
	) -> jvmtiError,
	pub AddModuleReads: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		module: jobject,
		to_module: jobject,
	) -> jvmtiError,
	pub AddModuleExports: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		module: jobject,
		pkg_name: *const c_char,
		to_module: jobject,
	) -> jvmtiError,
	pub AddModuleOpens: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		module: jobject,
		pkg_name: *const c_char,
		to_module: jobject,
	) -> jvmtiError,
	pub AddModuleUses: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		module: jobject,
		service: jclass,
	) -> jvmtiError,
	pub AddModuleProvides: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		module: jobject,
		service: jclass,
		impl_class: jclass,
	) -> jvmtiError,
	pub IsModifiableModule: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		module: jobject,
		is_modifiable_module_ptr: *mut jboolean,
	) -> jvmtiError,
	pub GetAllStackTraces: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		max_frame_count: jint,
		stack_info_ptr: *mut *mut jvmtiStackInfo,
		thread_count_ptr: *mut jint,
	) -> jvmtiError,
	pub GetThreadListStackTraces: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread_count: jint,
		thread_list: *const jthread,
		max_frame_count: jint,
		stack_info_ptr: *mut *mut jvmtiStackInfo,
	) -> jvmtiError,
	pub GetThreadLocalStorage: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		data_ptr: *mut *mut c_void,
	) -> jvmtiError,
	pub SetThreadLocalStorage: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		data: *const c_void,
	) -> jvmtiError,
	pub GetStackTrace: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		start_depth: jint,
		max_frame_count: jint,
		frame_buffer: *mut jvmtiFrameInfo,
		count_ptr: *mut jint,
	) -> jvmtiError,
	pub reserved105: *mut c_void,
	pub GetTag: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		object: jobject,
		tag_ptr: *mut jlong,
	) -> jvmtiError,
	pub SetTag:
		unsafe extern "system" fn(env: *mut jvmtiEnv, object: jobject, tag: jlong) -> jvmtiError,
	pub ForceGarbageCollection: unsafe extern "system" fn(env: *mut jvmtiEnv) -> jvmtiError,
	pub IterateOverObjectsReachableFromObject: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		object: jobject,
		object_reference_callback: jvmtiObjectReferenceCallback,
		user_data: *const c_void,
	) -> jvmtiError,
	pub IterateOverReachableObjects: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		heap_root_callback: jvmtiHeapRootCallback,
		stack_ref_callback: jvmtiStackReferenceCallback,
		object_ref_callback: jvmtiObjectReferenceCallback,
		user_data: *const c_void,
	) -> jvmtiError,
	pub IterateOverHeap: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		object_filter: jvmtiHeapObjectFilter,
		heap_object_callback: jvmtiHeapObjectCallback,
		user_data: *const c_void,
	) -> jvmtiError,
	pub IterateOverInstancesOfClass: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		object_filter: jvmtiHeapObjectFilter,
		heap_object_callback: jvmtiHeapObjectCallback,
		user_data: *const c_void,
	) -> jvmtiError,
	pub reserved113: *mut c_void,
	pub GetObjectsWithTags: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		tag_count: jint,
		tags: *const jlong,
		count_ptr: *mut jint,
		object_result_ptr: *mut *mut jobject,
		tag_result_ptr: *mut *mut jlong,
	) -> jvmtiError,
	pub FollowReferences: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		heap_filter: jint,
		klass: jclass,
		initial_object: jobject,
		callback: *const jvmtiHeapCallbacks,
		user_data: *const c_void,
	) -> jvmtiError,
	pub IterateThroughHeap: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		heap_filter: jint,
		klass: jclass,
		callbacks: *const jvmtiHeapCallbacks,
		user_data: *const c_void,
	) -> jvmtiError,
	pub reserved117: *mut c_void,
	pub SuspendAllVirtualThreads: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		except_count: jint,
		except_list: *const jthread,
	) -> jvmtiError,
	pub ResumeAllVirtualThreads: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		except_count: jint,
		except_list: *const jthread,
	) -> jvmtiError,
	pub SetJNIFunctionTable: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		function_table: *const jniNativeInterface,
	) -> jvmtiError,
	pub GetJNIFunctionTable: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		function_table: *mut *mut jniNativeInterface,
	) -> jvmtiError,
	pub SetEventCallbacks: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		callbacks: *const jvmtiEventCallbacks,
		size_of_callbacks: jint,
	) -> jvmtiError,
	pub GenerateEvents:
		unsafe extern "system" fn(env: *mut jvmtiEnv, event_type: jvmtiEvent) -> jvmtiError,
	pub GetExtensionFunctions: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		extension_count_ptr: *mut jint,
		extensions: *mut *mut jvmtiExtensionEventInfo,
	) -> jvmtiError,
	pub GetExtensionEvents: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		extension_count_ptr: *mut jint,
		extensions: *mut *mut jvmtiExtensionEventInfo,
	) -> jvmtiError,
	pub SetExtensionEventCallback: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		extension_event_index: jint,
		callback: jvmtiExtensionEvent,
	) -> jvmtiError,
	pub DisposeEnvironment: unsafe extern "system" fn(env: *mut jvmtiEnv) -> jvmtiError,
	pub GetErrorName: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		error: jvmtiError,
		name_ptr: *mut *mut c_char,
	) -> jvmtiError,
	pub GetJLocationFormat: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		format_ptr: *mut jvmtiJlocationFormat,
	) -> jvmtiError,
	pub GetSystemProperties: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		count_ptr: *mut jint,
		value_ptr: *mut *mut *mut c_char,
	) -> jvmtiError,
	pub GetSystemProperty: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		property: *const c_char,
		value_ptr: *mut *mut c_char,
	) -> jvmtiError,
	pub SetSystemProperty: unsafe extern "system" fn(
		env: jvmtiEnv,
		property: *const c_char,
		value_ptr: *const c_char,
	) -> jvmtiError,
	pub GetPhase:
		unsafe extern "system" fn(env: jvmtiEnv, phase_ptr: *mut jvmtiPhase) -> jvmtiError,
	pub GetCurrentThreadCpuTimerInfo:
		unsafe extern "system" fn(env: jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
	pub GetCurrentThreadCpuTime: unsafe extern "system" fn(
		env: jvmtiEnv,
		thread: jthread,
		nanos_ptr: *mut jlong,
	) -> jvmtiError,
	pub GetThreadCpuTimerInfo:
		unsafe extern "system" fn(env: jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
	pub GetThreadCpuTime: unsafe extern "system" fn(
		env: jvmtiEnv,
		thread: jthread,
		nanos_ptr: *mut jlong,
	) -> jvmtiError,
	pub GetTimerInfo:
		unsafe extern "system" fn(env: jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
	pub GetTime: unsafe extern "system" fn(env: jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError,
	pub GetPotentialCapabilities: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		capabilities_ptr: *const jvmtiCapabilities,
	) -> jvmtiError,
	pub reserved141: *mut c_void,
	pub AddCapabilities: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		capabilities_ptr: *const jvmtiCapabilities,
	) -> jvmtiError,
	pub RelinquishCapabilities: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		capabilities_ptr: *const jvmtiCapabilities,
	) -> jvmtiError,
	pub GetAvailableProcessors:
		unsafe extern "system" fn(env: *mut jvmtiEnv, processor_count_ptr: *mut jint) -> jvmtiError,
	pub GetClassVersionNumbers: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		jklass: jclass,
		minor_version_ptr: *mut jint,
		major_version_ptr: *mut jint,
	) -> jvmtiError,
	pub GetConstantPool: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		klass: jclass,
		constant_pool_count_ptr: *mut jint,
		constant_pool_byte_count_ptr: *mut jint,
		constant_pool_bytes_ptr: *mut *mut c_uchar,
	) -> jvmtiError,
	pub GetEnvironmentLocalStorage:
		unsafe extern "system" fn(env: *mut jvmtiEnv, data_ptr: *mut *mut c_void) -> jvmtiError,
	pub SetEnvironmentLocalStorage:
		unsafe extern "system" fn(env: *mut jvmtiEnv, data: *const c_void) -> jvmtiError,
	pub AddToBootstrapClassLoaderSearch:
		unsafe extern "system" fn(env: *mut jvmtiEnv, segment: *const c_char) -> jvmtiError,
	pub SetVerboseFlag: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		flag: jvmtiVerboseFlag,
		value: jboolean,
	) -> jvmtiError,
	pub AddToSystemClassLoaderSearch:
		unsafe extern "system" fn(env: *mut jvmtiEnv, segment: *const c_char) -> jvmtiError,
	pub RetransformClasses: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		class_count: jint,
		classes: *const jclass,
	) -> jvmtiError,
	pub GetOwnedMonitorStackDepthInfo: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		monitor_info_count_ptr: *mut jint,
		monitor_info_ptr: *mut *mut jvmtiMonitorStackDepthInfo,
	) -> jvmtiError,
	pub GetObjectSize: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		object: jobject,
		size_ptr: *mut jlong,
	) -> jvmtiError,
	pub GetLocalInstance: unsafe extern "system" fn(
		env: *mut jvmtiEnv,
		thread: jthread,
		depth: jint,
		value_ptr: *mut jobject,
	) -> jvmtiError,
	pub SetHeapSamplingInterval:
		unsafe extern "system" fn(jvmtiEnv: *mut jvmtiEnv, sampling_interval: jint) -> jvmtiError,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _jvmtiEnv {
	pub functions: *const jvmtiInterface_1_,
}

pub type jvmtiEnv = _jvmtiEnv;
