//! Java Native Interface
//!
//! # Primitive Types
//!
//! | Java Type | Native Type | Description      |
//! |-----------|-------------|------------------|
//! | boolean   | jboolean    | unsigned 8 bits  |
//! | byte      | jbyte       | signed 8 bits    |
//! | char      | jchar       | unsigned 16 bits |
//! | short     | jshort      | signed 16 bits   |
//! | int       | jint        | signed 32 bits   |
//! | long      | jlong       | signed 64 bits   |
//! | float     | jfloat      | 32 bits          |
//! | double    | jdouble     | 64 bits          |
//! | void      | void        | not applicable   |

#![feature(extern_types)]
#![feature(c_variadic)]
#![no_std]
#![allow(non_snake_case, non_camel_case_types)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use core::ffi::{c_void, c_char};
pub type va_list = *mut c_void;

pub type jint = i32;
pub type jlong = i64;
pub type jbyte = i8;
pub type jboolean = bool;
pub type jchar = u16;
pub type jshort = i16;
pub type jfloat = f32;
pub type jdouble = f64;
pub type jsize = jint;

unsafe extern "C" { pub type _jobject; }
pub type jobject = *mut _jobject;
pub type jclass = jobject;
pub type jthrowable = jobject;
pub type jstring = jobject;
pub type jarray = jobject;
pub type jbooleanArray = jarray;
pub type jbyteArray = jarray;
pub type jcharArray = jarray;
pub type jshortArray = jarray;
pub type jintArray = jarray;
pub type jlongArray = jarray;
pub type jfloatArray = jarray;
pub type jdoubleArray = jarray;
pub type jobjectArray = jarray;
pub type jweak = jobject;

#[repr(C)]
#[derive(Copy, Clone)]
pub union jvalue {
	pub z: jboolean,
	pub b: jbyte,
	pub c: jchar,
	pub s: jshort,
	pub i: jint,
	pub j: jlong,
	pub f: jfloat,
	pub d: jdouble,
	pub l: jobject,
}

unsafe extern "C" { pub type _jfieldID; }
pub type jfieldID = *mut _jfieldID;
unsafe extern "C" { pub type _jmethodID; }
pub type jmethodID = *mut _jmethodID;

/// Return values from `jobjectRefType`
#[derive(Clone, Copy)]
#[repr(C)]
pub enum jobjectRefType {
	JNIInvalidRefType = 0,
	JNILocalRefType = 1,
	JNIGlobalRefType = 2,
	JNIWeakGlobalRefType = 3,
}

/*
 * `jboolean` constants
 */

pub const JNI_FALSE: jboolean = false;
pub const JNI_TRUE : jboolean = true;

/*
 * Possible return values for JNI functions.
 */

/// Success
pub const JNI_OK       : jint = 0;
/// Unknown error
pub const JNI_ERR      : jint = -1;
/// Thread detached from the VM
pub const JNI_EDETACHED: jint = -2;
/// JNI version error
pub const JNI_EVERSION : jint = -3;
/// Not enough memory
pub const JNI_ENOMEM   : jint = -4;
/// VM already created
pub const JNI_EEXIST   : jint = -5;
/// Invalid arguments
pub const JNI_EINVAL   : jint = -6;

/*
 * Used in `ReleaseScalarArrayElements`
 */

pub const JNI_COMMIT: jint = 1;
pub const JNI_ABORT : jint = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JNINativeMethod {
	pub name: *mut c_char,
	pub signature: *mut c_char,
	pub fnPtr: *mut c_void,
}

pub type JNIEnv = *const JNINativeInterface_;
pub type JavaVM = *const JNIInvokeInterface_;

// unsafe extern "system" fn(...) -> ...
macro_rules! jni_system_fn {
	(($($param:tt)*) $(-> $ret:ty)?) => {
		unsafe extern "system" fn($($param)*) $(-> $ret)?
	}
}

// unsafe extern "C" fn(...) -> ...
macro_rules! jni_c_fn {
	(($($param:tt)*) $(-> $ret:ty)?) => {
		unsafe extern "C" fn($($param)*) $(-> $ret)?
	}
}

/// Interface Function Table
///
/// Each function is accessible at a fixed offset through the JNIEnv argument.
/// The JNIEnv type is a pointer to a structure storing all JNI function pointers.
/// It is defined as follows:
///
/// ```c
/// typedef const struct JNINativeInterface *JNIEnv;
/// ```
///
/// Note that the first three entries are reserved for future compatibility with COM.
/// In addition, we reserve a number of additional NULL entries near the beginning of the function table,
/// so that, for example, a future class-related JNI operation can be added after FindClass, rather than at the end of the table.
///
/// Note that the function table can be shared among all JNI interface pointers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JNINativeInterface_ {
	pub reserved0: *mut c_void,
	pub reserved1: *mut c_void,
	pub reserved2: *mut c_void,
	pub reserved3: *mut c_void,

	/// Returns the version of the native method interface.
	///
	/// ## LINKAGE
	///
	/// Index 4 in the `JNIEnv` interface function table.
	///
	/// ## PARAMETERS
	///
	/// `env`: the JNI interface pointer.
	///
	/// ## RETURNS
	///
	/// Returns the major version number in the higher 16 bits and the minor version number in the lower 16 bits.
	///
	/// In JDK/JRE 1.1, GetVersion() returns 0x00010001.
	///
	/// In JDK/JRE 1.2, GetVersion() returns 0x00010002.
	///
	/// In JDK/JRE 1.4, GetVersion() returns 0x00010004.
	///
	/// In JDK/JRE 1.6, GetVersion() returns 0x00010006.
	///
	/// In JDK/JRE 1.8, GetVersion() returns 0x00010008.
	///
	/// In JDK/JRE 9, GetVersion() returns 0x00090000.
	///
	/// In JDK/JRE 10, GetVersion() returns 0x000A0000.
	///
	/// In JDK/JRE 19, GetVersion() returns 0x00130000.
	///
	/// In JDK/JRE 20, GetVersion() returns 0x00140000.
	pub GetVersion: jni_system_fn!((env: *mut JNIEnv) -> jint),

	/// Loads a class from a buffer of raw class data.
	///
	/// The buffer containing the raw class data is not referenced by the VM after the DefineClass call returns, and it may be discarded if desired.
	///
	/// ## LINKAGE
	///
	/// Index 5 in the JNIEnv interface function table.
	///
	/// ## PARAMETERS
	///
	/// `env`: the JNI interface pointer.
	///
	/// `name`: the name of the class or interface to be defined. The string is encoded in modified UTF-8.
	///
	/// `loader`: a class loader assigned to the defined class.
	///
	/// `buf`: buffer containing the .class file data.
	///
	/// `bufLen`: buffer length.
	///
	/// ## RETURNS
	///
	/// Returns a Java class object or NULL if an error occurs.
	///
	/// ## THROWS
	///
	/// `ClassFormatError`: if the class data does not specify a valid class.
	///
	/// `ClassCircularityError`: if a class or interface would be its own superclass or superinterface.
	///
	/// `OutOfMemoryError`: if the system runs out of memory.
	///
	/// `SecurityException`: if the caller attempts to define a class in the "java" package tree.
	pub DefineClass: jni_system_fn!((
			env: *mut JNIEnv,
			name: *const c_char,
			loader: jobject,
			buf: *const jbyte,
			len: jsize,
		) -> jclass),

	/// In JDK release 1.1, this function loads a locally-defined class.
	/// It searches the directories and zip files specified by the `CLASSPATH` environment variable for the class with the specified name.
	///
	/// Since Java 2 SDK release 1.2, the Java security model allows non-system classes to load and call native methods.
	/// `FindClass` locates the class loader associated with the current native method; that is, the class loader of the class that declared the native method.
	/// If the native method belongs to a system class, no class loader will be involved. Otherwise, the proper class loader will be invoked to load and link the named class.
	///
	/// Since Java 2 SDK release 1.2, when `FindClass` is called through the Invocation Interface, there is no current native method or its associated class loader.
	/// In that case, the result of `ClassLoader.getSystemClassLoader` is used. This is the class loader the virtual machine creates for applications, and is able to locate classes
	/// listed in the java.class.path property.
	///
	/// The name argument is a fully-qualified class name or an array type signature.
	///
	/// For example, the fully-qualified class name for the `java.lang.String` class is: "java/lang/String"
	///
	/// The array type signature of the array class java.lang.Object[] is: "[Ljava/lang/Object;"
	///
	/// ## LINKAGE
	///
	/// Index 6 in the `JNIEnv` interface function table.
	///
	/// ## PARAMETERS
	///
	/// `env`: the JNI interface pointer.
	///
	/// `name`: a fully-qualified class name (that is, a package name, delimited by “/”, followed by the class name). If the name begins with “[“ (the array signature character), it returns an array class. The string is encoded in modified UTF-8.
	///
	/// ## RETURNS
	///
	/// Returns a class object from a fully-qualified name, or NULL if the class cannot be found.
	///
	/// ## THROWS
	///
	/// `ClassFormatError`: if the class data does not specify a valid class.
	///
	/// `ClassCircularityError`: if a class or interface would be its own superclass or superinterface.
	///
	/// `NoClassDefFoundError`: if no definition for a requested class or interface can be found.
	///
	/// `OutOfMemoryError`: if the system runs out of memory.
	pub FindClass: jni_system_fn!((env: *mut JNIEnv, name: *const c_char) -> jclass),

	/// Converts a `java.lang.reflect.Method` or `java.lang.reflect.Constructor` object to a method ID.
	///
	/// ## LINKAGE
	///
	/// Index 7 in the `JNIEnv` interface function table.
	///
	/// ## SINCE
	///
	/// JDK/JRE 1.2
	pub FromReflectedMethod: jni_system_fn!((env: *mut JNIEnv, method: jobject) -> jmethodID),

	/// Converts a `java.lang.reflect.Field` to a field ID.
	///
	/// ## LINKAGE
	///
	/// Index 8 in the `JNIEnv` interface function table.
	///
	/// ## SINCE
	///
	/// JDK/JRE 1.2
	pub FromReflectedField: jni_system_fn!((env: *mut JNIEnv, field: jobject) -> jfieldID),

	/// Converts a method ID derived from cls to a `java.lang.reflect.Method` or `java.lang.reflect.Constructor object`.
	///
	/// `isStatic` must be set to `JNI_TRUE` if the method ID refers to a static field, and `JNI_FALSE` otherwise.
	///
	/// Throws `OutOfMemoryError` and returns 0 if fails.
	///
	/// ## LINKAGE
	///
	/// Index 9 in the `JNIEnv` interface function table.
	///
	/// ## SINCE
	///
	/// JDK/JRE 1.2
	pub ToReflectedMethod: jni_system_fn!((
			env: *mut JNIEnv,
			cls: jclass,
			methodID: jmethodID,
			isStatic: jboolean,
		) -> jobject),

	/// If `clazz` represents any class other than the class `Object`, then this function returns the object that represents the superclass of the class specified by `clazz`.
	///
	/// If `clazz` specifies the class `Object`, or clazz represents an interface, this function returns `NULL`.
	///
	/// ## LINKAGE
	///
	/// Index 10 in the `JNIEnv` interface function table.
	///
	/// ## PARAMETERS
	///
	/// `env`: the JNI interface pointer.
	///
	/// `clazz`: a Java class object.
	///
	/// ## RETURNS
	///
	/// Returns the superclass of the class represented by `clazz`, or `NULL`.
	pub GetSuperclass: jni_system_fn!((env: *mut JNIEnv, sub: jclass) -> jclass),

	/// Determines whether an object of `clazz1` can be safely cast to `clazz2`.
	///
	/// ## LINKAGE
	///
	/// Index 11 in the `JNIEnv` interface function table.
	///
	/// ## PARAMETERS
	///
	/// `env`: the JNI interface pointer.
	///
	/// `clazz1`: the first class argument.
	///
	/// `clazz2`: the second class argument.
	///
	/// ## RETURNS
	///
	/// Returns `JNI_TRUE` if either of the following is true:
	///
	/// * The first and second class arguments refer to the same Java class.
	/// * The first class is a subclass of the second class.
	/// * The first class has the second class as one of its interfaces.
	pub IsAssignableFrom: jni_system_fn!((env: *mut JNIEnv, sub: jclass, sup: jclass) -> jboolean),

	/// Converts a field ID derived from cls to a j`ava.lang.reflect.Field` object.
	///
	/// `isStatic` must be set to `JNI_TRUE` if fieldID refers to a static field, and `JNI_FALSE` otherwise.
	///
	/// Throws `OutOfMemoryError` and returns 0 if fails.
	///
	/// ## LINKAGE
	///
	/// Index 12 in the `JNIEnv` interface function table.
	///
	/// ## SINCE
	///
	/// JDK/JRE 1.2
	pub ToReflectedField: jni_system_fn!((
			env: *mut JNIEnv,
			cls: jclass,
			fieldID: jfieldID,
			isStatic: jboolean,
		) -> jobject),

	/// Causes a java.lang.Throwable object to be thrown.
	///
	/// ## LINKAGE
	///
	/// Index 13 in the `JNIEnv` interface function table.
	///
	/// ## PARAMETERS
	///
	/// `env`: the JNI interface pointer.
	///
	/// `obj`: a java.lang.Throwable object.
	///
	/// ## RETURNS
	///
	/// Returns 0 on success; a negative value on failure.
	///
	/// ## THROWS:
	///
	/// the `java.lang.Throwable` object `obj`.
	pub Throw: jni_system_fn!((env: *mut JNIEnv, obj: jthrowable) -> jint),

	/// Constructs an exception object from the specified class with the message specified by message and causes that exception to be thrown.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 14 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `clazz`: a subclass of `java.lang.Throwable`.
	/// 
	/// `message`: the message used to construct the `java.lang.Throwable object`. The string is encoded in modified UTF-8.
	/// 
	/// ## RETURNS
	/// 
	/// Returns 0 on success; a negative value on failure.
	/// 
	/// ## THROWS
	/// 
	/// the newly constructed `java.lang.Throwable` object.
	pub ThrowNew: jni_system_fn!((env: *mut JNIEnv, clazz: jclass, msg: *const c_char) -> jint),
	
	/// Determines if an exception is being thrown.
	/// 
	/// The exception stays being thrown until either the native code calls `ExceptionClear()`, or the Java code handles the exception.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 15 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// ## RETURNS
	/// 
	/// Returns the exception object that is currently in the process of being thrown, or `NULL` if no exception is currently being thrown.
	pub ExceptionOccurred: jni_system_fn!((env: *mut JNIEnv) -> jthrowable),
	
	/// Prints an exception and a backtrace of the stack to a system error-reporting channel, such as stderr. This is a convenience routine provided for debugging.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 16 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	pub ExceptionDescribe: jni_system_fn!((env: *mut JNIEnv)),
	
	/// Clears any exception that is currently being thrown. If no exception is currently being thrown, this routine has no effect.
	/// 
	/// ## LINKAGE:
	/// 
	/// Index 17 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS:
	/// 
	/// `env`: the JNI interface pointer.
	pub ExceptionClear: jni_system_fn!((env: *mut JNIEnv)),
	
	/// Raises a fatal error and does not expect the VM to recover. This function does not return.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 18 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `msg`: an error message. The string is encoded in modified UTF-8.
	pub FatalError: jni_system_fn!((env: *mut JNIEnv, msg: *const c_char) -> !),
	
	/// Creates a new local reference frame, in which at least a given number of local references can be created.
	/// 
	/// Returns 0 on success, a negative number and a pending `OutOfMemoryError` on failure.
	/// 
	/// Note that local references already created in previous local frames are still valid in the current local frame.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 19 in the `JNIEnv` interface function table.
	/// 
	/// ## SINCE
	/// 
	/// JDK/JRE 1.2
	pub PushLocalFrame: jni_system_fn!((env: *mut JNIEnv, capacity: jint) -> jint),
	
	/// Pops off the current local reference frame, frees all the local references, and returns a local reference in the previous local reference frame for the given result object.
	/// 
	/// Pass `NULL` as result if you do not need to return a reference to the previous frame.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 20 in the `JNIEnv` interface function table.
	/// 
	/// ## SINCE
	/// 
	/// JDK/JRE 1.2
	pub PopLocalFrame: jni_system_fn!((env: *mut JNIEnv, result: jobject) -> jobject),
	
	/// Creates a new global reference to the object referred to by the `obj` argument.
	/// 
	/// The `obj` argument may be a global or local reference.
	/// 
	/// Global references must be explicitly disposed of by calling `DeleteGlobalRef()`.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 21 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `obj`: a global or local reference.
	/// 
	/// ## RETURNS
	/// 
	/// Returns a global reference to the given `obj`.
	/// 
	/// May return `NULL` if:
	/// 
	///  * `obj` refers to `null`
	///  * the system has run out of memory
	///  * `obj` was a weak global reference and has already been garbage collected
	pub NewGlobalRef: jni_system_fn!((env: *mut JNIEnv, lobj: jobject) -> jobject),
	
	/// Deletes the global reference pointed to by `globalRef`.
	/// 
	/// LINKAGE:
	/// 
	/// Index 22 in the `JNIEnv` interface function table.
	/// 
	/// PARAMETERS:
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `globalRef`: a global reference.
	pub DeleteGlobalRef: jni_system_fn!((env: *mut JNIEnv, gref: jobject)),
	
	/// Deletes the local reference pointed to by `localRef`.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 23 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `localRef`: a local reference.
	pub DeleteLocalRef: jni_system_fn!((env: *mut JNIEnv, obj: jobject)),
	
	/// Tests whether two references refer to the same Java object.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 24 in the JNIEnv interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `ref1`: a Java object.
	/// 
	/// `ref2`: a Java object.
	/// 
	/// ## RETURNS
	/// 
	/// Returns `JNI_TRUE` if `ref1` and `ref2` refer to the same Java object, or are both `NULL`; otherwise, returns `JNI_FALSE`.
	pub IsSameObject: jni_system_fn!((env: *mut JNIEnv, obj1: jobject, obj2: jobject) -> jboolean),
	
	/// Creates a new local reference that refers to the same object as ref. The given ref may be a global or local reference.
	/// 
	/// Returns `NULL` if ref refers to null.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 25 in the `JNIEnv` interface function table.
	/// 
	/// ## SINCE
	/// 
	/// JDK/JRE 1.2
	pub NewLocalRef: jni_system_fn!((env: *mut JNIEnv, ref_: jobject) -> jobject),
	
	/// Ensures that at least a given number of local references can be created in the current thread.
	/// 
	/// Returns 0 on success; otherwise returns a negative number and throws an `OutOfMemoryError`.
	/// 
	/// Before it enters a native method, the VM automatically ensures that at least 16 local references can be created.
	/// 
	/// For backward compatibility, the VM allocates local references beyond the ensured capacity.
	/// (As a debugging support, the VM may give the user warnings that too many local references are being created.
	/// In the JDK, the programmer can supply the -verbose:jni command line option to turn on these messages.)
	/// The VM calls `FatalError` if no more local references can be created beyond the ensured capacity.
	/// 
	/// ## LINKAGE
	/// Index 26 in the `JNIEnv` interface function table.
	/// 
	/// ## SINCE
	/// 
	/// JDK/JRE 1.2
	pub EnsureLocalCapacity: jni_system_fn!((env: *mut JNIEnv, capacity: jint) -> jint),
	
	/// Allocates a new Java object without invoking any of the constructors for the object.
	/// 
	/// Returns a reference to the object.
	/// 
	/// The `clazz` argument must not refer to an array class.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 27 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `clazz`: a Java class object.
	/// 
	/// ## RETURNS
	/// 
	/// Returns a Java object, or `NULL` if the object cannot be constructed.
	/// 
	/// ## THROWS
	/// 
	/// `InstantiationException`: if the class is an interface or an abstract class.
	/// 
	/// `OutOfMemoryError`: if the system runs out of memory.
	pub AllocObject: jni_system_fn!((env: *mut JNIEnv, clazz: jclass) -> jobject),
	
	/// Constructs a new Java object.
	/// 
	/// The method ID indicates which constructor method to invoke. This ID must be obtained by calling `GetMethodID()` with `<init>` as the method name and void `(V)` as the return type.
	/// 
	/// The clazz argument must not refer to an array class.
	/// 
	/// ## NewObject
	/// 
	/// Programmers place all arguments that are to be passed to the constructor immediately following the methodID argument.
	/// `NewObject()` accepts these arguments and passes them to the Java method that the programmer wishes to invoke.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 28 in the JNIEnv interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `clazz`: a Java class object.
	/// 
	/// `methodID`: the method ID of the constructor.
	/// 
	/// ## Additional Parameter for NewObject
	/// 
	/// arguments to the constructor.
	/// 
	/// ## RETURNS
	/// 
	/// Returns a Java object, or `NULL` if the object cannot be constructed.
	/// 
	/// ## THROWS
	/// 
	/// `InstantiationException`: if the class is an interface or an abstract class.
	/// 
	/// `OutOfMemoryError`: if the system runs out of memory.
	/// 
	/// Any exceptions thrown by the constructor.
	pub NewObject: jni_c_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jobject),

	/// Constructs a new Java object.
	///
	/// The method ID indicates which constructor method to invoke. This ID must be obtained by calling `GetMethodID()` with `<init>` as the method name and void `(V)` as the return type.
	///
	/// The clazz argument must not refer to an array class.
	///
	/// ## NewObjectV
	///
	/// Programmers place all arguments that are to be passed to the constructor in an args argument of type `va_list` that immediately follows the methodID argument.
	/// `NewObjectV()` accepts these arguments, and, in turn, passes them to the Java method that the programmer wishes to invoke.
	///
	/// ## LINKAGE
	///
	/// Index 29 in the JNIEnv interface function table.
	///
	/// ## PARAMETERS
	///
	/// `env`: the JNI interface pointer.
	///
	/// `clazz`: a Java class object.
	///
	/// `methodID`: the method ID of the constructor.
	/// 
	/// ## Additional Parameter for NewObjectV
	/// 
	/// `args`: a `va_list` of arguments to the constructor.
	///
	/// ## RETURNS
	///
	/// Returns a Java object, or `NULL` if the object cannot be constructed.
	///
	/// ## THROWS
	///
	/// `InstantiationException`: if the class is an interface or an abstract class.
	///
	/// `OutOfMemoryError`: if the system runs out of memory.
	///
	/// Any exceptions thrown by the constructor.
	pub NewObjectV: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jobject),

	/// Constructs a new Java object.
	///
	/// The method ID indicates which constructor method to invoke. This ID must be obtained by calling `GetMethodID()` with `<init>` as the method name and void `(V)` as the return type.
	///
	/// The clazz argument must not refer to an array class.
	///
	/// ## NewObjectA
	///
	/// Programmers place all arguments that are to be passed to the constructor in an args array of `jvalue`s that immediately follows the `methodID` argument.
	/// `NewObjectA()` accepts the arguments in this array, and, in turn, passes them to the Java method that the programmer wishes to invoke.
	///
	/// ## LINKAGE
	///
	/// Index 30 in the `JNIEnv` interface function table.
	///
	/// ## PARAMETERS
	///
	/// `env`: the JNI interface pointer.
	///
	/// `clazz`: a Java class object.
	///
	/// `methodID`: the method ID of the constructor.
	///
	/// ## Additional Parameter for NewObjectA
	///
	/// `args`: an array of arguments to the constructor.
	///
	/// ## RETURNS
	///
	/// Returns a Java object, or `NULL` if the object cannot be constructed.
	///
	/// ## THROWS
	///
	/// `InstantiationException`: if the class is an interface or an abstract class.
	///
	/// `OutOfMemoryError`: if the system runs out of memory.
	///
	/// Any exceptions thrown by the constructor.
	pub NewObjectA: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jobject),
	
	/// Returns the class of an object.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 31 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `obj`: a Java object (must not be `NULL`).
	/// 
	/// ## RETURNS
	/// 
	/// Returns a Java class object.
	pub GetObjectClass: jni_system_fn!((env: *mut JNIEnv, obj: jobject) -> jclass),
	
	/// Tests whether an object is an instance of a class.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 32 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `obj`: a Java object.
	/// 
	/// `clazz`: a Java class object.
	/// 
	/// ## RETURNS
	/// 
	/// Returns `JNI_TRUE` if `obj` can be cast to `clazz`; otherwise, returns `JNI_FALSE`. A `NULL` object can be cast to any class.
	pub IsInstanceOf: jni_system_fn!((env: *mut JNIEnv, obj: jobject, clazz: jclass) -> jboolean),
	
	/// Returns the method ID for an instance (nonstatic) method of a class or interface. 
	/// 
	/// The method may be defined in one of the clazz’s superclasses and inherited by clazz. The method is determined by its name and signature.
	/// 
	/// `GetMethodID()` causes an uninitialized class to be initialized.
	/// 
	/// To obtain the method ID of a constructor, supply `<init>` as the method name and void (V) as the return type.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 33 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `clazz`: a Java class object.
	/// 
	/// `name`: the method name in a 0-terminated modified UTF-8 string.
	/// 
	/// `sig`: the method signature in 0-terminated modified UTF-8 string.
	/// 
	/// ## RETURNS
	/// 
	/// Returns a method ID, or `NULL` if the specified method cannot be found.
	/// 
	/// ## THROWS
	/// 
	/// `NoSuchMethodError`: if the specified method cannot be found.
	/// 
	/// `ExceptionInInitializerError`: if the class initializer fails due to an exception.
	/// 
	/// `OutOfMemoryError`: if the system runs out of memory.
	pub GetMethodID: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			name: *const c_char,
			sig: *const c_char,
		) -> jmethodID),
	
	pub CallObjectMethod: jni_c_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jobject),
	
	pub CallObjectMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: va_list,
		) -> jobject),
	
	pub CallObjectMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jobject),
	
	pub CallBooleanMethod: jni_c_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jboolean),
	
	pub CallBooleanMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: va_list,
		) -> jboolean),
	
	pub CallBooleanMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jboolean),
	
	pub CallByteMethod: jni_c_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jbyte),
	
	pub CallByteMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: va_list,
		) -> jbyte),
	
	pub CallByteMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jbyte),
	
	pub CallCharMethod: jni_c_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jchar),
	
	pub CallCharMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: va_list,
		) -> jchar),
	
	pub CallCharMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jchar),
	
	pub CallShortMethod: jni_c_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jshort),
	
	pub CallShortMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: va_list,
		) -> jshort),
	
	pub CallShortMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jshort),
	
	pub CallIntMethod: jni_c_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jint),
	
	pub CallIntMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: va_list,
		) -> jint),
	
	pub CallIntMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jint),
	
	pub CallLongMethod: jni_c_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jlong),
	
	pub CallLongMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: va_list,
		) -> jlong),
	
	pub CallLongMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jlong),
	
	pub CallFloatMethod: jni_c_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jfloat),
	
	pub CallFloatMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: va_list,
		) -> jfloat),
	
	pub CallFloatMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jfloat),
	
	pub CallDoubleMethod: jni_c_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jdouble),
	
	pub CallDoubleMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: va_list,
		) -> jdouble),
	
	pub CallDoubleMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jdouble),
	
	pub CallVoidMethod: jni_c_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...)),
	
	pub CallVoidMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: va_list,
		)),
	
	pub CallVoidMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			methodID: jmethodID,
			args: *const jvalue,
		)),
	
	pub CallNonvirtualObjectMethod: jni_c_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			...
		) -> jobject),
	
	pub CallNonvirtualObjectMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jobject),
	
	pub CallNonvirtualObjectMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jobject),
	
	pub CallNonvirtualBooleanMethod: jni_c_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			...
		) -> jboolean),
	
	pub CallNonvirtualBooleanMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jboolean),
	
	pub CallNonvirtualBooleanMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jboolean),
	
	pub CallNonvirtualByteMethod: jni_c_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			...
		) -> jbyte),
	
	pub CallNonvirtualByteMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jbyte),
	
	pub CallNonvirtualByteMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jbyte),
	
	pub CallNonvirtualCharMethod: jni_c_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			...
		) -> jchar),
	
	pub CallNonvirtualCharMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jchar),
	
	pub CallNonvirtualCharMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jchar),
	
	pub CallNonvirtualShortMethod: jni_c_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			...
		) -> jshort),
	
	pub CallNonvirtualShortMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jshort),
	
	pub CallNonvirtualShortMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jshort),
	
	pub CallNonvirtualIntMethod: jni_c_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			...
		) -> jint),
	
	pub CallNonvirtualIntMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jint),
	
	pub CallNonvirtualIntMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jint),
	
	pub CallNonvirtualLongMethod: jni_c_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			...
		) -> jlong),
	
	pub CallNonvirtualLongMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jlong),
	
	pub CallNonvirtualLongMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jlong),
	
	pub CallNonvirtualFloatMethod: jni_c_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			...
		) -> jfloat),
	
	pub CallNonvirtualFloatMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jfloat),
	
	pub CallNonvirtualFloatMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jfloat),
	
	pub CallNonvirtualDoubleMethod: jni_c_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			...
		) -> jdouble),
	
	pub CallNonvirtualDoubleMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jdouble),
	
	pub CallNonvirtualDoubleMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jdouble),
	
	pub CallNonvirtualVoidMethod: jni_c_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			...
		)),
	
	pub CallNonvirtualVoidMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		)),
	
	pub CallNonvirtualVoidMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			obj: jobject,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		)),
	
	/// Returns the field ID for an instance (nonstatic) field of a class.
	/// 
	/// The field is specified by its name and signature. The Get<type>Field and Set<type>Field families of accessor functions use field IDs to retrieve object fields.
	/// 
	/// `GetFieldID()` causes an uninitialized class to be initialized.
	/// 
	/// `GetFieldID() `cannot be used to obtain the length field of an array. Use `GetArrayLength()` instead.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 94 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `clazz`: a Java class object.
	/// 
	/// `name`: the field name in a 0-terminated modified UTF-8 string.
	/// 
	/// `sig`: the field signature in a 0-terminated modified UTF-8 string.
	/// 
	/// ## RETURNS
	/// 
	/// Returns a field ID, or `NULL` if the operation fails.
	/// 
	/// ## THROWS
	/// 
	/// `NoSuchFieldError`: if the specified field cannot be found.
	/// 
	/// `ExceptionInInitializerError`: if the class initializer fails due to an exception.
	/// 
	/// `OutOfMemoryError`: if the system runs out of memory.
	pub GetFieldID: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			name: *const c_char,
			sig: *const c_char,
		) -> jfieldID),
	
	pub GetObjectField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jobject),
	
	pub GetBooleanField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jboolean),
	
	pub GetByteField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jbyte),
	
	pub GetCharField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jchar),
	
	pub GetShortField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jshort),
	
	pub GetIntField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jint),
	
	pub GetLongField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jlong),
	
	pub GetFloatField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jfloat),
	
	pub GetDoubleField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jdouble),
	
	pub SetObjectField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jobject)),
	
	pub SetBooleanField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jboolean)),
	
	pub SetByteField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jbyte)),
	
	pub SetCharField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jchar)),
	
	pub SetShortField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jshort)),
	
	pub SetIntField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jint)),
	
	pub SetLongField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jlong)),
	
	pub SetFloatField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jfloat)),
	
	pub SetDoubleField: jni_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jdouble)),
	
	pub GetStaticMethodID: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			name: *const c_char,
			sig: *const c_char,
		) -> jmethodID),
	
	pub CallStaticObjectMethod: jni_c_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jobject),
	
	pub CallStaticObjectMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jobject),
	
	pub CallStaticObjectMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jobject),
	
	pub CallStaticBooleanMethod: jni_c_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jboolean),
	
	pub CallStaticBooleanMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jboolean),
	
	pub CallStaticBooleanMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jboolean),
	
	pub CallStaticByteMethod: jni_c_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jbyte),
	
	pub CallStaticByteMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jbyte),
	
	pub CallStaticByteMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jbyte),
	
	pub CallStaticCharMethod: jni_c_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jchar),
	
	pub CallStaticCharMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jchar),
	
	pub CallStaticCharMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jchar),
	
	pub CallStaticShortMethod: jni_c_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jshort),
	
	pub CallStaticShortMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jshort),
	
	pub CallStaticShortMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jshort),
	
	pub CallStaticIntMethod: jni_c_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jint),
	
	pub CallStaticIntMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jint),
	
	pub CallStaticIntMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jint),
	
	pub CallStaticLongMethod: jni_c_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jlong),
	
	pub CallStaticLongMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jlong),
	
	pub CallStaticLongMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jlong),
	
	pub CallStaticFloatMethod: jni_c_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jfloat),
	
	pub CallStaticFloatMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jfloat),
	
	pub CallStaticFloatMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jfloat),
	
	pub CallStaticDoubleMethod: jni_c_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jdouble),
	
	pub CallStaticDoubleMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: va_list,
		) -> jdouble),
	
	pub CallStaticDoubleMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		) -> jdouble),
	
	pub CallStaticVoidMethod: jni_c_fn!((env: *mut JNIEnv, cls: jclass, methodID: jmethodID, ...)),
	
	pub CallStaticVoidMethodV: jni_system_fn!((
			env: *mut JNIEnv,
			cls: jclass,
			methodID: jmethodID,
			args: va_list,
		)),
	
	pub CallStaticVoidMethodA: jni_system_fn!((
			env: *mut JNIEnv,
			cls: jclass,
			methodID: jmethodID,
			args: *const jvalue,
		)),
	
	pub GetStaticFieldID: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			name: *const c_char,
			sig: *const c_char,
		) -> jfieldID),
	
	pub GetStaticObjectField: jni_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jobject),
	
	pub GetStaticBooleanField: jni_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jboolean),
	
	pub GetStaticByteField: jni_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jbyte),
	
	pub GetStaticCharField: jni_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jchar),
	
	pub GetStaticShortField: jni_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jshort),
	
	pub GetStaticIntField: jni_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jint),
	
	pub GetStaticLongField: jni_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jlong),
	
	pub GetStaticFloatField: jni_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jfloat),
	
	pub GetStaticDoubleField: jni_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jdouble),
	
	pub SetStaticObjectField: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			fieldID: jfieldID,
			value: jobject,
		)),
	
	pub SetStaticBooleanField: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			fieldID: jfieldID,
			value: jboolean,
		)),
	
	pub SetStaticByteField: jni_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID, value: jbyte)),
	
	pub SetStaticCharField: jni_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID, value: jchar)),
	
	pub SetStaticShortField: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			fieldID: jfieldID,
			value: jshort,
		)),
	
	pub SetStaticIntField: jni_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID, value: jint)),
	
	pub SetStaticLongField: jni_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID, value: jlong)),
	
	pub SetStaticFloatField: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			fieldID: jfieldID,
			value: jfloat,
		)),
	
	pub SetStaticDoubleField: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			fieldID: jfieldID,
			value: jdouble,
		)),
	
	/// Constructs a new `java.lang.String` object from an array of Unicode characters.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 163 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `unicodeChars`: pointer to a Unicode string.
	/// 
	/// `len`: length of the Unicode string.
	/// 
	/// ## RETURNS
	/// 
	/// Returns a Java string object, or `NULL` if the string cannot be constructed.
	/// 
	/// ## THROWS
	/// 
	/// `OutOfMemoryError`: if the system runs out of memory.
	pub NewString: jni_system_fn!((env: *mut JNIEnv, unicode: *const jchar, len: jsize) -> jstring),
	
	/// Returns the length (the count of Unicode characters) of a Java string.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 164 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `string`: a Java string object.
	/// 
	/// ## RETURNS
	/// 
	/// Returns the length of the Java string.
	pub GetStringLength: jni_system_fn!((env: *mut JNIEnv, str: jstring) -> jsize),
	
	/// Returns a pointer to the array of Unicode characters of the string.
	/// 
	/// This pointer is valid until `ReleaseStringChars()` is called.
	/// 
	/// If `isCopy` is not `NULL`, then *isCopy is set to `JNI_TRUE` if a copy is made; or it is set to `JNI_FALSE` if no copy is made.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 165 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `string`: a Java string object.
	/// 
	/// `isCopy`: a pointer to a boolean.
	/// 
	/// ## RETURNS
	/// 
	/// Returns a pointer to a Unicode string, or `NULL` if the operation fails.
	pub GetStringChars: jni_system_fn!((
			env: *mut JNIEnv,
			str: jstring,
			isCopy: *mut jboolean,
		) -> *const jchar),
	
	/// Informs the VM that the native code no longer needs access to chars.
	/// 
	/// The chars argument is a pointer obtained from string using `GetStringChars()`.
	/// 
	/// ## LINKAGE
	///
	/// Index 166 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `string`: a Java string object.
	/// 
	/// `chars`: a pointer to a Unicode string.
	pub ReleaseStringChars: jni_system_fn!((env: *mut JNIEnv, str: jstring, chars: *const jchar)),
	
	/// Constructs a new `java.lang.String` object from an array of characters in modified UTF-8 encoding.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 167 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `bytes`: the pointer to a modified UTF-8 string.
	/// 
	/// ## RETURNS
	/// 
	/// Returns a Java string object, or `NULL` if the string cannot be constructed.
	/// 
	/// ## THROWS
	/// 
	/// `OutOfMemoryError`: if the system runs out of memory.
	pub NewStringUTF: jni_system_fn!((env: *mut JNIEnv, utf: *const c_char) -> jstring),
	
	/// Returns the length in bytes of the modified UTF-8 representation of a string.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 168 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `string`: a Java string object.
	/// 
	/// ## RETURNS
	/// 
	/// Returns the UTF-8 length of the string.
	pub GetStringUTFLength: jni_system_fn!((env: *mut JNIEnv, str: jstring) -> jsize),
	
	/// Returns a pointer to an array of bytes representing the string in modified UTF-8 encoding.
	/// 
	/// This array is valid until it is released by `ReleaseStringUTFChars()`.
	/// 
	/// If `isCopy` is not `NULL`, then *isCopy is set to `JNI_TRUE` if a copy is made; or it is set to `JNI_FALSE` if no copy is made.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 169 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `string`: a Java string object.
	/// 
	/// `isCopy`: a pointer to a boolean.
	/// 
	/// ## RETURNS
	/// 
	/// Returns a pointer to a modified UTF-8 string, or `NULL` if the operation fails.
	pub GetStringUTFChars: jni_system_fn!((
			env: *mut JNIEnv,
			str: jstring,
			isCopy: *mut jboolean,
		) -> *const c_char),
	
	/// Informs the VM that the native code no longer needs access to `utf`.
	/// 
	/// The `utf` argument is a pointer derived from string using `GetStringUTFChars()`.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 170 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `string`: a Java string object.
	/// 
	/// `utf`: a pointer to a modified UTF-8 string.
	pub ReleaseStringUTFChars: jni_system_fn!((env: *mut JNIEnv, str: jstring, chars: *const c_char)),
	
	/// Returns the number of elements in the array.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 171 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `array`: a Java array object.
	/// 
	/// ## RETURNS
	/// 
	/// Returns the length of the array.
	pub GetArrayLength: jni_system_fn!((env: *mut JNIEnv, array: jarray) -> jsize),
	
	/// Constructs a new array holding objects in class `elementClass`.
	/// 
	/// All elements are initially set to initialElement.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 172 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `length`: array size.
	/// 
	/// `elementClass`: array element class.
	/// 
	/// `initialElement`: initialization value.
	/// 
	/// ## RETURNS
	/// 
	/// Returns a Java array object, or `NULL` if the array cannot be constructed.
	/// 
	/// ## THROWS
	/// 
	/// `OutOfMemoryError`: if the system runs out of memory.
	pub NewObjectArray: jni_system_fn!((
			env: *mut JNIEnv,
			len: jsize,
			clazz: jclass,
			init: jobject,
		) -> jobjectArray),
	
	/// Returns an element of an Object array.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 173 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `array`: a Java array.
	/// 
	/// `index`: array index.
	/// 
	/// ## RETURNS
	/// 
	/// Returns a Java object.
	/// 
	/// ## THROWS
	/// 
	/// `ArrayIndexOutOfBoundsException`: if index does not specify a valid index in the array.
	pub GetObjectArrayElement: jni_system_fn!((env: *mut JNIEnv, array: jobjectArray, index: jsize) -> jobject),
	
	/// Sets an element of an Object array.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 174 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `array`: a Java array.
	/// 
	/// `index`: array index.
	/// 
	/// `value`: the new value.
	/// 
	/// ## THROWS
	/// 
	/// `ArrayIndexOutOfBoundsException`: if index does not specify a valid index in the array.
	/// 
	/// `ArrayStoreException`: if the class of value is not a subclass of the element class of the array.
	pub SetObjectArrayElement: jni_system_fn!((
			env: *mut JNIEnv,
			array: jobjectArray,
			index: jsize,
			val: jobject,
		)),
	
	pub NewBooleanArray: jni_system_fn!((env: *mut JNIEnv, len: jsize) -> jbooleanArray),
	
	pub NewByteArray: jni_system_fn!((env: *mut JNIEnv, len: jsize) -> jbyteArray),
	
	pub NewCharArray: jni_system_fn!((env: *mut JNIEnv, len: jsize) -> jcharArray),
	
	pub NewShortArray: jni_system_fn!((env: *mut JNIEnv, len: jsize) -> jshortArray),
	
	pub NewIntArray: jni_system_fn!((env: *mut JNIEnv, len: jsize) -> jintArray),
	
	pub NewLongArray: jni_system_fn!((env: *mut JNIEnv, len: jsize) -> jlongArray),
	
	pub NewFloatArray: jni_system_fn!((env: *mut JNIEnv, len: jsize) -> jfloatArray),
	
	pub NewDoubleArray: jni_system_fn!((env: *mut JNIEnv, len: jsize) -> jdoubleArray),
	
	pub GetBooleanArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jbooleanArray,
			isCopy: *mut jboolean,
		) -> *mut jboolean),
	
	pub GetByteArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jbyteArray,
			isCopy: *mut jboolean,
		) -> *mut jbyte),
	
	pub GetCharArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jcharArray,
			isCopy: *mut jboolean,
		) -> *mut jchar),
	
	pub GetShortArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jshortArray,
			isCopy: *mut jboolean,
		) -> *mut jshort),
	
	pub GetIntArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jintArray,
			isCopy: *mut jboolean,
		) -> *mut jint),
	
	pub GetLongArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jlongArray,
			isCopy: *mut jboolean,
		) -> *mut jlong),
	
	pub GetFloatArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jfloatArray,
			isCopy: *mut jboolean,
		) -> *mut jfloat),
	
	pub GetDoubleArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jdoubleArray,
			isCopy: *mut jboolean,
		) -> *mut jdouble),
	
	pub ReleaseBooleanArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jbooleanArray,
			elems: *mut jboolean,
			mode: jint,
		)),
	
	pub ReleaseByteArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jbyteArray,
			elems: *mut jbyte,
			mode: jint,
		)),
	
	pub ReleaseCharArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jcharArray,
			elems: *mut jchar,
			mode: jint,
		)),
	
	pub ReleaseShortArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jshortArray,
			elems: *mut jshort,
			mode: jint,
		)),
	
	pub ReleaseIntArrayElements: jni_system_fn!((env: *mut JNIEnv, array: jintArray, elems: *mut jint, mode: jint)),
	
	pub ReleaseLongArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jlongArray,
			elems: *mut jlong,
			mode: jint,
		)),
	
	pub ReleaseFloatArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jfloatArray,
			elems: *mut jfloat,
			mode: jint,
		)),
	
	pub ReleaseDoubleArrayElements: jni_system_fn!((
			env: *mut JNIEnv,
			array: jdoubleArray,
			elems: *mut jdouble,
			mode: jint,
		)),
	
	pub GetBooleanArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jbooleanArray,
			start: jsize,
			l: jsize,
			buf: *mut jboolean,
		)),
	
	pub GetByteArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jbyteArray,
			start: jsize,
			len: jsize,
			buf: *mut jbyte,
		)),
	
	pub GetCharArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jcharArray,
			start: jsize,
			len: jsize,
			buf: *mut jchar,
		)),
	
	pub GetShortArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jshortArray,
			start: jsize,
			len: jsize,
			buf: *mut jshort,
		)),
	
	pub GetIntArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jintArray,
			start: jsize,
			len: jsize,
			buf: *mut jint,
		)),
	
	pub GetLongArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jlongArray,
			start: jsize,
			len: jsize,
			buf: *mut jlong,
		)),
	
	pub GetFloatArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jfloatArray,
			start: jsize,
			len: jsize,
			buf: *mut jfloat,
		)),
	
	pub GetDoubleArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jdoubleArray,
			start: jsize,
			len: jsize,
			buf: *mut jdouble,
		)),
	
	pub SetBooleanArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jbooleanArray,
			start: jsize,
			l: jsize,
			buf: *mut jboolean,
		)),
	
	pub SetByteArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jbyteArray,
			start: jsize,
			len: jsize,
			buf: *mut jbyte,
		)),
	
	pub SetCharArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jcharArray,
			start: jsize,
			len: jsize,
			buf: *mut jchar,
		)),
	
	pub SetShortArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jshortArray,
			start: jsize,
			len: jsize,
			buf: *mut jshort,
		)),
	
	pub SetIntArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jintArray,
			start: jsize,
			len: jsize,
			buf: *mut jint,
		)),
	
	pub SetLongArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jlongArray,
			start: jsize,
			len: jsize,
			buf: *mut jlong,
		)),
	
	pub SetFloatArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jfloatArray,
			start: jsize,
			len: jsize,
			buf: *mut jfloat,
		)),
	
	pub SetDoubleArrayRegion: jni_system_fn!((
			env: *mut JNIEnv,
			array: jdoubleArray,
			start: jsize,
			len: jsize,
			buf: *mut jdouble,
		)),
	
	/// Registers native methods with the class specified by the `clazz` argument.
	/// 
	/// The `methods` parameter specifies an array of `JNINativeMethod` structures that contain the names, signatures, and function pointers of the native methods.
	/// 
	/// The name and signature fields of the `JNINativeMethod` structure are pointers to modified UTF-8 strings.
	/// 
	/// The `nMethods` parameter specifies the number of native methods in the array.
	/// 
	/// The `JNINativeMethod` structure is defined as follows:
	/// 
	/// ```c
	/// typedef struct {
	/// 
	///     char *name;
	/// 
	///     char *signature;
	/// 
	///     void *fnPtr;
	/// 
	/// } JNINativeMethod;
	/// ```
	/// 
	/// The function pointers nominally must have the following signature:
	/// 
	/// ```text
	/// ReturnType (*fnPtr)(JNIEnv *env, jobject objectOrClass, ...);
	/// ```
	/// 
	/// ## LINKAGE
	/// 
	/// Index 215 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `clazz`: a Java class object.
	/// 
	/// `methods`: the native methods in the class.
	/// 
	/// `nMethods`: the number of native methods in the class.
	/// 
	/// ## RETURNS
	/// 
	/// Returns “0” on success; returns a negative value on failure.
	/// 
	/// ## THROWS
	/// 
	/// `NoSuchMethodError`: if a specified method cannot be found or if the method is not native.
	pub RegisterNatives: jni_system_fn!((
			env: *mut JNIEnv,
			clazz: jclass,
			methods: *const JNINativeMethod,
			nMethods: jint,
		) -> jint),
	
	/// Unregisters native methods of a class.
	/// 
	/// The class goes back to the state before it was linked or registered with its native method functions.
	/// 
	/// This function should not be used in normal native code. Instead, it provides special programs a way to reload and relink native libraries.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 216 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `clazz`: a Java class object.
	/// 
	/// ## RETURNS
	/// 
	/// Returns “0” on success; returns a negative value on failure.
	pub UnregisterNatives: jni_system_fn!((env: *mut JNIEnv, clazz: jclass) -> jint),
	
	/// Enters the monitor associated with the underlying Java object referred to by `obj`.
	/// 
	/// Enters the monitor associated with the object referred to by `obj`. The `obj` reference must not be `NULL`.
	/// 
	/// Each Java object has a monitor associated with it. If the current thread already owns the monitor associated with `obj`,
	/// it increments a counter in the monitor indicating the number of times this thread has entered the monitor.
	/// If the monitor associated with `obj` is not owned by any thread, the current thread becomes the owner of the monitor,
	/// setting the entry count of this monitor to 1. If another thread already owns the monitor associated with `obj`, the current thread waits until the monitor is released,
	/// then tries again to gain ownership.
	/// 
	/// A monitor entered through a `MonitorEnter` JNI function call cannot be exited using the `monitorexit` Java virtual machine instruction or a `synchronized` method return.
	/// 
	/// A `MonitorEnter` JNI function call and a `monitorenter` Java virtual machine instruction may race to enter the monitor associated with the same object.
	/// 
	/// To avoid deadlocks, a monitor entered through a `MonitorEnter` JNI function call must be exited using the `MonitorExit` JNI call,
	/// unless the `DetachCurrentThread` call is used to implicitly release JNI monitors.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 217 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `obj`: a normal Java object or class object.
	/// 
	/// ## RETURNS
	/// 
	/// Returns “0” on success; returns a negative value on failure.
	pub MonitorEnter: jni_system_fn!((env: *mut JNIEnv, obj: jobject) -> jint),
	
	/// The current thread must be the owner of the monitor associated with the underlying Java object referred to by `obj`.
	/// 
	/// The thread decrements the counter indicating the number of times it has entered this monitor. If the value of the counter becomes zero, the current thread releases the monitor.
	/// 
	/// Native code must not use `MonitorExit` to exit a monitor entered through a synchronized method or a `monitorenter` Java virtual machine instruction.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 218 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `obj`: a normal Java object or class object.
	/// 
	/// ## RETURNS
	/// 
	/// Returns “0” on success; returns a negative value on failure.
	/// 
	/// ## EXCEPTIONS
	/// 
	/// `IllegalMonitorStateException`: if the current thread does not own the monitor.
	pub MonitorExit: jni_system_fn!((env: *mut JNIEnv, obj: jobject) -> jint),
	
	/// Returns the Java VM interface (used in the Invocation API) associated with the current thread.
	/// 
	/// The result is placed at the location pointed to by the second argument, `vm`.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 219 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `vm`: a pointer to where the result should be placed.
	/// 
	/// ## RETURNS
	/// 
	/// Returns “0” on success; returns a negative value on failure.
	pub GetJavaVM: jni_system_fn!((env: *mut JNIEnv, vm: *mut *mut JavaVM) -> jint),
	
	/// Copies `len` number of Unicode characters beginning at offset `start` to the given buffer `buf`.
	/// 
	/// Throws `StringIndexOutOfBoundsException` on index overflow.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 220 in the `JNIEnv` interface function table.
	/// 
	/// ## SINCE
	/// 
	/// JDK/JRE 1.2
	pub GetStringRegion: jni_system_fn!((
			env: *mut JNIEnv,
			str: jstring,
			start: jsize,
			len: jsize,
			buf: *mut jchar,
		)),
	
	/// Translates `len` number of Unicode characters beginning at offset `start` into modified UTF-8 encoding and place the result in the given buffer `buf`.
	/// 
	/// Throws `StringIndexOutOfBoundsException` on index overflow.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 221 in the `JNIEnv` interface function table.
	/// 
	/// ## SINCE
	/// 
	/// JDK/JRE 1.2
	pub GetStringUTFRegion: jni_system_fn!((
			env: *mut JNIEnv,
			str: jstring,
			start: jsize,
			len: jsize,
			buf: *mut c_char
		)),
	
	pub GetPrimitiveArrayCritical: jni_system_fn!((
			env: *mut JNIEnv,
			array: jarray,
			isCopy: *mut jboolean,
		) -> *mut c_void),
	
	pub ReleasePrimitiveArrayCritical: jni_system_fn!((env: *mut JNIEnv, array: jarray, carray: *mut c_void, mode: jint)),
	
	pub GetStringCritical: jni_system_fn!((
			env: *mut JNIEnv,
			string: jstring,
			isCopy: *mut jboolean,
		) -> *const jchar),
	
	pub ReleaseStringCritical: jni_system_fn!((env: *mut JNIEnv, string: jstring, cstring: *const jchar)),
	
	/// Creates a new weak global reference.
	/// 
	/// Returns `NULL` if `obj` refers to null, or if the VM runs out of memory.
	/// 
	/// If the VM runs out of memory, an `OutOfMemoryError` will be thrown.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 226 in the `JNIEnv` interface function table.
	/// 
	/// ## SINCE
	/// 
	/// JDK/JRE 1.2
	pub NewWeakGlobalRef: jni_system_fn!((env: *mut JNIEnv, obj: jobject) -> jweak),
	
	/// Delete the VM resources needed for the given weak global reference.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 227 in the `JNIEnv` interface function table.
	/// 
	/// ## SINCE
	/// 
	/// JDK/JRE 1.2
	pub DeleteWeakGlobalRef: jni_system_fn!((env: *mut JNIEnv, ref_: jweak)),
	
	/// Returns `JNI_TRUE` when there is a pending exception; otherwise, returns `JNI_FALSE`.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 228 in the `JNIEnv` interface function table.
	/// 
	/// ## SINCE
	/// 
	/// JDK/JRE 1.2
	pub ExceptionCheck: jni_system_fn!((env: *mut JNIEnv) -> jboolean),
	
	/// Allocates and returns a direct `java.nio.ByteBuffer` referring to the block of memory starting at the memory address address and extending capacity bytes.
	/// 
	/// Native code that calls this function and returns the resulting byte-buffer object to Java-level code should ensure that the buffer refers to a valid region of memory that is accessible for reading and,
	/// if appropriate, writing. An attempt to access an invalid memory location from Java code will either return an arbitrary value, have no visible effect, or cause an unspecified exception to be thrown.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 229 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNIEnv interface pointer
	/// 
	/// `address`: the starting address of the memory region (must not be `NULL`)
	/// 
	/// `capacity`: the size in bytes of the memory region (must be positive)
	/// 
	/// ## RETURNS
	/// 
	/// Returns a local reference to the newly-instantiated `java.nio.ByteBuffer` object.
	/// 
	/// Returns `NULL` if an exception occurs, or if JNI access to direct buffers is not supported by this virtual machine.
	/// 
	/// ## EXCEPTIONS
	/// 
	/// `OutOfMemoryError`: if allocation of the ByteBuffer object fails
	/// 
	/// ## SINCE
	/// 
	/// JDK/JRE 1.4
	pub NewDirectByteBuffer: jni_system_fn!((
			env: *mut JNIEnv,
			address: *mut c_void,
			capacity: jlong,
		) -> jobject),
	
	/// Fetches and returns the starting address of the memory region referenced by the given direct `java.nio.Buffer`.
	/// 
	/// This function allows native code to access the same memory region that is accessible to Java code via the buffer object.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 230 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNIEnv interface pointer
	/// 
	/// `buf`: a direct java.nio.Buffer object (must not be `NULL`)
	/// 
	/// ## RETURNS
	/// 
	/// Returns the starting address of the memory region referenced by the buffer.
	/// 
	/// Returns `NULL` if the memory region is undefined, if the given object is not a direct `java.nio.Buffer`, or if JNI access to direct buffers is not supported by this virtual machine.
	/// 
	/// ## SINCE
	/// 
	/// JDK/JRE 1.4
	pub GetDirectBufferAddress: jni_system_fn!((env: *mut JNIEnv, buf: jobject) -> *mut c_void),
	
	/// Fetches and returns the capacity of the memory region referenced by the given direct java.nio.Buffer. The capacity is the number of elements that the memory region contains.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 231 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNIEnv interface pointer
	/// 
	/// `buf`: a direct `java.nio.Buffer` object (must not be `NULL`)
	/// 
	/// ## RETURNS
	/// 
	/// Returns the capacity of the memory region associated with the buffer.
	/// 
	/// Returns -1 if the given object is not a direct `java.nio.Buffer`, if the object is an unaligned view buffer and the processor architecture does not support unaligned access, or if JNI access to direct buffers is not supported by this virtual machine.
	/// 
	/// ## SINCE
	/// 
	/// JDK/JRE 1.4
	pub GetDirectBufferCapacity: jni_system_fn!((env: *mut JNIEnv, buf: jobject) -> jlong),
	
	/// Returns the type of the object referred to by the `obj` argument.
	/// 
	/// The argument `obj` can either be a local, global or weak global reference.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 232 in the `JNIEnv` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `env`: the JNI interface pointer.
	/// 
	/// `obj`: a local, global or weak global reference.
	/// 
	/// `vm`: the virtual machine instance from which the interface will be retrieved.
	/// 
	/// `env`: pointer to the location where the JNI interface pointer for the current thread will be placed.
	/// 
	/// `version`: the requested JNI version.
	/// 
	/// ## RETURNS
	/// 
	/// The function GetObjectRefType returns one of the following enumerated values defined as a `jobjectRefType`:
	/// 
	/// ```text
	/// JNIInvalidRefType = 0,
	/// JNILocalRefType = 1,
	/// JNIGlobalRefType = 2,
	/// JNIWeakGlobalRefType = 3
	/// ```
	/// 
	/// If the argument `obj` is a weak global reference type, the return will be `JNIWeakGlobalRefType`.
	/// 
	/// If the argument `obj` is a global reference type, the return value will be `JNIGlobalRefType`.
	/// 
	/// If the argument `obj` is a local reference type, the return will be `JNILocalRefType`.
	/// 
	/// If the `obj` argument is not a valid reference, the return value for this function will be `JNIInvalidRefType`.
	/// 
	/// An invalid reference is a reference which is not a valid handle. That is, the `obj` pointer address does not point to a location in
	/// memory which has been allocated from one of the Ref creation functions or returned from a JNI function.
	/// 
	/// As such, `NULL` would be an invalid reference and `GetObjectRefType(env,NULL)` would return `JNIInvalidRefType`.
	/// 
	/// On the other hand, a null reference, which is a reference that points to a null, would return the type of reference that the null reference was originally created as.
	/// 
	/// `GetObjectRefType` cannot be used on deleted references.
	/// 
	/// Since references are typically implemented as pointers to memory data structures that can potentially be reused by any of the reference allocation services in the VM,
	/// once deleted, it is not specified what value the `GetObjectRefType` will return.
	/// 
	/// ## SINCE
	/// 
	/// JDK/JRE 1.6
	pub GetObjectRefType: jni_system_fn!((env: *mut JNIEnv, obj: jobject) -> jobjectRefType),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JNIEnv_ {
	pub functions: *const JNINativeInterface_,
}

/// optionString may be any option accepted by the JVM, or one of the
/// following:
///
/// * `-D<name>=<value>` Set a system property.
/// * `-verbose[:class|gc|jni]` Enable verbose output, comma-separated. E.g.
///   "-verbose:class" or "-verbose:gc,class"
///   Standard names include: gc, class, and jni.
///   All nonstandard (VM-specific) names must begin
///   with "X".
/// * `vfprintf` extraInfo is a pointer to the vfprintf hook.
/// * `exit` extraInfo is a pointer to the exit hook.
/// * `abort` extraInfo is a pointer to the abort hook.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JavaVMOption {
	pub optionString: *mut c_char,
	pub extraInfo: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JavaVMInitArgs {
	pub version: jint,
	pub nOptions: jint,
	pub options: *mut JavaVMOption,
	pub ignoreUnrecognized: jboolean,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JavaVMAttachArgs {
	pub version: jint,
	pub name: *mut c_char,
	pub group: jobject,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JNIInvokeInterface_ {
	pub reserved0: *mut c_void,
	pub reserved1: *mut c_void,
	pub reserved2: *mut c_void,
	
	/// Unloads a Java VM and reclaims its resources.
	/// 
	/// Any thread, whether attached or not, can invoke this function.
	/// 
	/// If the current thread is attached, the VM waits until the current thread is the only non-daemon user-level Java thread.
	/// 
	/// If the current thread is not attached, the VM attaches the current thread and then waits until the current thread is the only non-daemon user-level thread.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 3 in the `JavaVM` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `vm`: the Java VM that will be destroyed.
	/// 
	/// ## RETURNS
	/// 
	/// Returns `JNI_OK` on success; returns a suitable JNI error code (a negative number) on failure.
	/// 
	/// Unloading of the VM is not supported.
	pub DestroyJavaVM: jni_system_fn!((vm: *mut JavaVM) -> jint),
	
	/// Attaches the current thread to a Java VM.
	/// 
	/// Returns a JNI interface pointer in the `JNIEnv` argument.
	/// 
	/// Trying to attach a thread that is already attached is a no-op.
	/// 
	/// A native thread cannot be attached simultaneously to two Java VMs.
	/// 
	/// When a thread is attached to the VM, the context class loader is the bootstrap loader.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 4 in the `JavaVM` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `vm`: the VM to which the current thread will be attached.
	/// 
	/// `p_env`: pointer to the location where the JNI interface pointer of the current thread will be placed.
	/// 
	/// `thr_args`: can be `NULL` or a pointer to a JavaVMAttachArgs structure to specify additional information:
	/// 
	/// The second argument to `AttachCurrentThread` is always a pointer to `JNIEnv`. The third argument to `AttachCurrentThread` was reserved, and should be set to `NULL`.
	/// 
	/// You pass a pointer to the following structure to specify additional information:
	/// 
	/// ```c
	/// typedef struct JavaVMAttachArgs {
	///     jint version;
	///     char *name;    /* the name of the thread as a modified UTF-8 string, or NULL */
	///     jobject group; /* global ref of a ThreadGroup object, or NULL */
	/// } JavaVMAttachArgs
	/// ```
	/// 
	/// ## RETURNS
	/// 
	/// Returns `JNI_OK` on success; returns a suitable JNI error code (a negative number) on failure.
	pub AttachCurrentThread: jni_system_fn!((
			vm: *mut JavaVM,
			penv: *mut *mut c_void,
			args: *mut c_void,
		) -> jint),
	
	/// Detaches the current thread from a Java VM.
	/// 
	/// All Java monitors held by this thread are released.
	/// 
	/// All Java threads waiting for this thread to die are notified.
	/// 
	/// The main thread can be detached from the VM.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 5 in the `JavaVM` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `vm`: the VM from which the current thread will be detached.
	/// 
	/// ## RETURNS
	/// 
	/// Returns `JNI_OK` on success; returns a suitable JNI error code (a negative number) on failure.
	pub DetachCurrentThread: jni_system_fn!((vm: *mut JavaVM) -> jint),
	
	/// ## LINKAGE
	/// 
	/// Index 6 in the JavaVM interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `vm`: The virtual machine instance from which the interface will be retrieved.
	/// `env`: pointer to the location where the JNI interface pointer for the current thread will be placed.
	/// `version`: The requested JNI version.
	/// 
	/// ## RETURNS
	/// 
	/// If the current thread is not attached to the VM, sets `*env` to `NULL`, and returns `JNI_EDETACHED`.
	/// 
	/// If the specified version is not supported, sets `*env` to `NULL`, and returns `JNI_EVERSION`.
	/// 
	/// Otherwise, sets `*env` to the appropriate interface, and returns `JNI_OK`.
	pub GetEnv: jni_system_fn!((vm: *mut JavaVM, penv: *mut *mut c_void, version: jint) -> jint),
	
	/// Same semantics as AttachCurrentThread, but the newly-created `java.lang.Thread` instance is a daemon.
	/// 
	/// If the thread has already been attached via either `AttachCurrentThread` or `AttachCurrentThreadAsDaemon`,
	/// this routine simply sets the value pointed to by `penv` to the JNIEnv of the current thread.
	/// 
	/// In this case neither `AttachCurrentThread` nor this routine have any effect on the daemon status of the thread.
	/// 
	/// ## LINKAGE
	/// 
	/// Index 7 in the `JavaVM` interface function table.
	/// 
	/// ## PARAMETERS
	/// 
	/// `vm`: the virtual machine instance to which the current thread will be attached.
	/// 
	/// `penv`: a pointer to the location in which the JNIEnv interface pointer for the current thread will be placed.
	/// 
	/// `args`: a pointer to a `JavaVMAttachArgs` structure.
	/// 
	/// ## RETURNS
	/// 
	/// Returns `JNI_OK` on success; returns a suitable JNI error code (a negative number) on failure.
	/// 
	/// ## EXCEPTIONS
	/// 
	/// None.
	pub AttachCurrentThreadAsDaemon: jni_system_fn!((
			vm: *mut JavaVM,
			penv: *mut *mut c_void,
			args: *mut c_void,
		) -> jint),
}

unsafe extern "system" {
	/// Returns a default configuration for the Java VM.
	/// 
	/// Before calling this function, native code must set the vm_args->version field to the JNI version it expects the VM to support.
	/// 
	/// After this function returns, vm_args->version will be set to the actual JNI version the VM supports.
	/// 
	/// ## LINKAGE
	/// 
	/// Exported from the native library that implements the Java virtual machine.
	/// 
	/// ## PARAMETERS
	/// 
	/// `vm_args`: a pointer to a `JavaVMInitArgs` structure in to which the default arguments are filled.
	/// 
	/// ## RETURNS
	/// 
	/// Returns `JNI_OK` if the requested version is supported; returns a JNI error code (a negative number) if the requested version is not supported.
	pub fn JNI_GetDefaultJavaVMInitArgs(args: *mut c_void) -> jint;
	
	/// Loads and initializes a Java VM.
	/// 
	/// The current thread becomes the main thread.
	/// 
	/// Sets the `env` argument to the JNI interface pointer of the main thread.
	/// 
	/// Creation of multiple VMs in a single process is not supported.
	/// 
	/// The second argument to `JNI_CreateJavaVM` is always a pointer to `JNIEnv *`, while the third argument is a pointer to a `JavaVMInitArgs` structure which
	/// uses option strings to encode arbitrary VM start up options:
	/// 
	/// ```c
	/// typedef struct JavaVMInitArgs {
	///     jint version;
	/// 
	///     jint nOptions;
	///     JavaVMOption *options;
	///     jboolean ignoreUnrecognized;
	/// } JavaVMInitArgs;
	/// ```
	/// 
	/// The options field is an array of the following type:
	/// 
	/// ```c
	/// typedef struct JavaVMOption {
	///     char *optionString;  /* the option as a string in the default platform encoding */
	///     void *extraInfo;
	/// } JavaVMOption;
	/// ```
	/// 
	/// The size of the array is denoted by the `nOptions` field in `JavaVMInitArgs`.
	/// 
	/// If `ignoreUnrecognized` is `JNI_TRUE`, `JNI_CreateJavaVM` ignore all unrecognized option strings that begin with "-X" or "_".
	/// 
	/// If `ignoreUnrecognized` is `JNI_FALSE`, `JNI_CreateJavaVM` returns `JNI_ERR` as soon as it encounters any unrecognized option strings.
	/// 
	/// All Java VMs must recognize the following set of standard options:
	/// 
	/// | optionString              | meaning                                                                                                                                                                                                                                                                                                                                                            |
	/// | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
	/// | `-D<name>=<value>`        | Set a system property                                                                                                                                                                                                                                                                                                                                              |
	/// | `-verbose[:class|gc|jni]` | Enable verbose output. The options can be followed by a comma-separated list of names indicating what kind of messages will be printed by the VM. For example, "`-verbose:gc,class`" instructs the VM to print GC and class loading related messages. Standard names include: `gc`, `class`, and `jni`. All nonstandard (VM-specific) names must begin with "`X`". |
	/// | `vfprintf`                | `extraInfo` is a pointer to the `vfprintf` hook.                                                                                                                                                                                                                                                                                                                   |
	/// | `exit`                    | `extraInfo` is a pointer to the `exit` hook.                                                                                                                                                                                                                                                                                                                       |
	/// | `abort`                   | `extraInfo` is a pointer to the `abort` hook.                                                                                                                                                                                                                                                                                                                      |
	/// 
	/// In addition, each VM implementation may support its own set of non-standard option strings.
	/// 
	/// Non-standard option names must begin with "-X" or an underscore ("_").
	/// 
	/// For example, the JDK/JRE supports -Xms and -Xmx options to allow programmers specify the initial and maximum heap size. Options that begin with "-X" are accessible from the "java" command line.
	/// 
	/// Here is the example code that creates a Java VM in the JDK/JRE:
	/// 
	/// ```c
	/// JavaVMInitArgs vm_args;
	/// JavaVMOption options[4];
	/// 
	/// options[0].optionString = "-Djava.compiler=NONE";           /* disable JIT */
	/// options[1].optionString = "-Djava.class.path=c:\myclasses"; /* user classes */
	/// options[2].optionString = "-Djava.library.path=c:\mylibs";  /* set native library path */
	/// options[3].optionString = "-verbose:jni";                   /* print JNI-related messages */
	/// 
	/// vm_args.version = JNI_VERSION_1_2;
	/// vm_args.options = options;
	/// vm_args.nOptions = 4;
	/// vm_args.ignoreUnrecognized = TRUE;
	/// 
	/// /* Note that in the JDK/JRE, there is no longer any need to call
	///  * JNI_GetDefaultJavaVMInitArgs.
	///  */
	/// res = JNI_CreateJavaVM(&vm, (void **)&env, &vm_args);
	/// if (res < 0) ...
	/// ```
	/// 
	/// ## LINKAGE
	/// 
	/// Exported from the native library that implements the Java virtual machine.
	/// 
	/// ## PARAMETERS
	/// 
	/// `p_vm`: pointer to the location where the resulting VM structure will be placed.
	/// 
	/// `p_env`: pointer to the location where the JNI interface pointer for the main thread will be placed.
	/// 
	/// `vm_args`: Java VM initialization arguments.
	/// 
	/// ## RETURNS
	/// 
	/// Returns `JNI_OK` on success; returns a suitable JNI error code (a negative number) on failure.
	pub fn JNI_CreateJavaVM(
		pvm: *mut *mut JavaVM,
		penv: *mut *mut c_void,
		args: *mut JavaVMInitArgs,
	) -> jint;
	
	/// Returns all Java VMs that have been created.
	/// 
	/// Pointers to VMs are written in the buffer `vmBuf` in the order they are created. At most `bufLen` number of entries will be written.
	/// 
	/// The total number of created VMs is returned in `*nVMs`.
	/// 
	/// Creation of multiple VMs in a single process is not supported.
	/// 
	/// ## LINKAGE
	/// 
	/// Exported from the native library that implements the Java virtual machine.
	/// 
	/// ## PARAMETERS
	/// 
	/// `vmBuf`: pointer to the buffer where the VM structures will be placed.
	/// 
	/// `bufLen`: the length of the buffer.
	/// 
	/// `nVMs`: a pointer to an integer.
	/// 
	/// ## RETURNS
	/// 
	/// Returns `JNI_OK` on success; returns a suitable JNI error code (a negative number) on failure.
	pub fn JNI_GetCreatedJavaVMs(vmBuf: *mut *mut JavaVM, bufLen: jsize, nVMs: *mut jsize) -> jint;
	
	// Defined by native libraries

	pub fn JNI_OnLoad(vm: *mut JavaVM, reserved: *mut c_void) -> jint;
	pub fn JNI_OnUnload(vm: *mut JavaVM, reserved: *mut c_void);
}

pub const JNI_VERSION_1_1: jint = 0x0001_0001;
pub const JNI_VERSION_1_2: jint = 0x0001_0002;
pub const JNI_VERSION_1_4: jint = 0x0001_0004;
pub const JNI_VERSION_1_6: jint = 0x0001_0006;
pub const JNI_VERSION_1_8: jint = 0x0001_0008;
pub const JNI_VERSION_9  : jint = 0x0009_0000;
pub const JNI_VERSION_10 : jint = 0x000A_0000;
pub const JNI_VERSION_19 : jint = 0x0013_0000;
pub const JNI_VERSION_20 : jint = 0x0014_0000;
pub const JNI_VERSION_21 : jint = 0x0015_0000;
pub const JNI_VERSION_24 : jint = 0x0018_0000;