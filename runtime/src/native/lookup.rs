use crate::classpath::classloader::ClassLoader;
use crate::reference::{MethodRef, Reference};
use crate::string_interner::StringInterner;
use crate::thread::{Thread, ThreadRef};

use std::ffi::{c_void, CStr};
use std::fmt::Write;
use std::os::raw::c_int;
use std::sync::Arc;

use ::jni::sys::{jclass, JNIEnv, JNINativeMethod};
use classfile::accessflags::MethodAccessFlags;
use instructions::Operand;
use symbols::sym;

// The JNI specification defines the mapping from a Java native method name to
// a C native library implementation function name as follows:
//
// The mapping produces a native method name by concatenating the following components
// derived from a `native` method declaration:
//
// 1. the prefix Java_
// 2. given the binary name, in internal form, of the class which declares the native method:
// the result of escaping the name.
// 3. an underscore ("_")
// 4. the escaped method name
// 5. if the native method declaration is overloaded: two underscores ("__") followed by the
// escaped parameter descriptor (JVMS 4.3.3) of the method declaration.
//
// Escaping leaves every alphanumeric ASCII character (A-Za-z0-9) unchanged, and replaces each
// UTF-16 code unit n the table below with the corresponding escape sequence. If the name to be
// escaped contains a surrogate pair, then the high-surrogate code unit and the low-surrogate code
// unit are escaped separately. The result of escaping is a string consisting only of the ASCII
// characters A-Za-z0-9 and underscore.
//
// ------------------------------                  ------------------------------------
// UTF-16 code unit                                Escape sequence
// ------------------------------                  ------------------------------------
// Forward slash (/, U+002F)                       _
// Underscore (_, U+005F)                          _1
// Semicolon (;, U+003B)                           _2
// Left square bracket ([, U+005B)                 _3
// Any UTF-16 code unit \u_WXYZ_ that does not     _0wxyz where w, x, y, and z are the lower-case
// represent alphanumeric ASCII (A-Za-z0-9),       forms of the hexadecimal digits W, X, Y, and Z.
// forward slash, underscore, semicolon,           (For example, U+ABCD becomes _0abcd.)
// or left square bracket
// ------------------------------                  ------------------------------------
//
// Note that escape sequences can safely begin _0, _1, etc, because class and method
// names in Java source code never begin with a number. However, that is not the case in
// class files that were not generated from Java source code.
//
// To preserve the 1:1 mapping to a native method name, the VM checks the resulting name as
// follows. If the process of escaping any precursor string from the native  method declaration
// (class or method name, or argument type) causes a "0", "1", "2", or "3" character
// from the precursor string to appear unchanged in the result *either* immediately after an
// underscore *or* at the beginning of the escaped string (where it will follow an underscore
// in the fully assembled name), then the escaping process is said to have "failed".
// In such cases, no native library search is performed, and the attempt to link the native
// method invocation will throw UnsatisfiedLinkError.
//
//
// For example:
//
// package/my_class/method
//
// and
//
// package/my/1class/method
//
// both map to
//
// Java_package_my_1class_method
//
// To address this potential conflict we need only check if the character after
// / is a digit 0..3, or if the first character after an injected '_' separator
// is a digit 0..3. If we encounter an invalid identifier we return false. Otherwise the stream
// contains the mapped name and we return true.

struct NativeNameConverter {
	method: MethodRef,
	pure_name: Option<String>,
	long_name: Option<String>,
}

impl NativeNameConverter {
	fn new(method: MethodRef) -> Self {
		Self {
			method,
			pure_name: None,
			long_name: None,
		}
	}

	fn pure_name(&mut self) -> Option<&str> {
		// Start with the prefix
		let mut name = String::from("Java_");

		let class_name = self.0.class.name.as_str();
		if !Self::map_escaped_name_on(&mut name, class_name) {
			return None;
		}

		name.push('_');

		let method_name = self.0.name.as_str();
		if !Self::map_escaped_name_on(&mut name, method_name) {
			return None;
		}

		self.pure_name = Some(name);
		self.pure_name.as_deref()
	}

	fn long_name(&mut self) -> Option<&str> {
		// Start with the prefix
		let mut name = String::from("__");

		let descriptor = self.method.descriptor.as_str();
		let closing_paren_pos = descriptor
			.chars()
			.position(|b| b == ')')
			.expect("Descriptors should be validated before this");

		// Start at 1 to skip the '('
		if !Self::map_escaped_name_on(&mut name, &descriptor[1..closing_paren_pos]) {
			return None;
		}

		self.long_name = Some(name);
		self.long_name.as_deref()
	}

	fn compute_complete_jni_name(
		&self,
		num_args: usize,
		include_long: bool,
		os_style: bool,
	) -> String {
		let mut name = String::new();

		if os_style && cfg!(all(windows, not(target_arch = "x86_64"))) {
			name.push('_');
		}

		if let Some(pure_name) = self.pure_name.as_deref() {
			name.push_str(pure_name);
		}

		if include_long {
			if let Some(long_name) = self.long_name.as_deref() {
				name.push_str(long_name);
			}
		}

		if os_style && cfg!(all(windows, not(target_arch = "x86_64"))) {
			name = format!("{name}@{}", num_args * core::mem::size_of::<c_int>());
		}

		name
	}

	fn map_escaped_name_on(stream: &mut String, name: &str) -> bool {
		// First character follows '_', so this is true initially
		let mut check_escape_char = true;
		for c in name.chars() {
			if c as u32 <= 0x7F && c.is_alphanumeric() {
				if check_escape_char && c >= '0' && c <= '3' {
					return false;
				}

				stream.push(c);
				check_escape_char = false;
				continue;
			}

			check_escape_char = false;

			match c {
				'_' => stream.push_str("_1"),
				'/' => {
					stream.push('_');
					check_escape_char = true;
				},
				';' => stream.push_str("_2"),
				'[' => stream.push_str("_3"),
				c => stream.write_fmt(format_args!("_0{c:05x}")),
			}
		}

		true
	}
}

extern "C" {
	pub fn JVM_RegisterMethodHandleMethods(env: *mut JNIEnv, unsafecls: jclass);
	pub fn JVM_RegisterReferencesMethods(env: *mut JNIEnv, unsafecls: jclass);
	pub fn JVM_RegisterUpcallHandlerMethods(env: *mut JNIEnv, unsafecls: jclass);
	pub fn JVM_RegisterUpcallLinkerMethods(env: *mut JNIEnv, unsafecls: jclass);
	pub fn JVM_RegisterNativeEntryPointMethods(env: *mut JNIEnv, unsafecls: jclass);
	pub fn JVM_RegisterPerfMethods(env: *mut JNIEnv, perfclass: jclass);
	pub fn JVM_RegisterWhiteBoxMethods(env: *mut JNIEnv, wbclass: jclass);
	pub fn JVM_RegisterVectorSupportMethods(env: *mut JNIEnv, vsclass: jclass);
}

macro_rules! cstr {
	( $s:literal ) => {{
		unsafe { std::mem::transmute::<_, &std::ffi::CStr>(concat!($s, "\0")).as_mut_ptr() }
	}};
}

fn lookup_special_native(jni_name: &str) -> Option<*const c_void> {
	let lookup_special_native_methods: &[JNINativeMethod] = &[
		// TODO
		// JNINativeMethod {
		// 	name: cstr!("Java_jdk_internal_misc_Unsafe_registerNatives"),
		// 	signature: std::ptr::null_mut(),
		// 	fnPtr: JVM_RegisterJDKInternalMiscUnsafeMethods as *mut c_void,
		// },
		JNINativeMethod {
			name: cstr!("Java_java_lang_invoke_MethodHandleNatives_registerNatives"),
			signature: std::ptr::null_mut(),
			fnPtr: JVM_RegisterMethodHandleMethods as *mut c_void,
		},
		JNINativeMethod {
			name: cstr!("Java_jdk_internal_foreign_abi_UpcallStubs_registerNatives"),
			signature: std::ptr::null_mut(),
			fnPtr: JVM_RegisterUpcallHandlerMethods as *mut c_void,
		},
		JNINativeMethod {
			name: cstr!("Java_jdk_internal_foreign_abi_UpcallLinker_registerNatives"),
			signature: std::ptr::null_mut(),
			fnPtr: JVM_RegisterUpcallLinkerMethods as *mut c_void,
		},
		JNINativeMethod {
			name: cstr!("Java_jdk_internal_foreign_abi_NativeEntryPoint_registerNatives"),
			signature: std::ptr::null_mut(),
			fnPtr: JVM_RegisterNativeEntryPointMethods as *mut c_void,
		},
		JNINativeMethod {
			name: cstr!("Java_jdk_internal_perf_Perf_registerNatives"),
			signature: std::ptr::null_mut(),
			fnPtr: JVM_RegisterPerfMethods as *mut c_void,
		},
		JNINativeMethod {
			name: cstr!("Java_sun_hotspot_WhiteBox_registerNatives"),
			signature: std::ptr::null_mut(),
			fnPtr: JVM_RegisterWhiteBoxMethods as *mut c_void,
		},
		JNINativeMethod {
			name: cstr!("Java_jdk_test_whitebox_WhiteBox_registerNatives"),
			signature: std::ptr::null_mut(),
			fnPtr: JVM_RegisterWhiteBoxMethods as *mut c_void,
		},
		JNINativeMethod {
			name: cstr!("Java_jdk_internal_vm_vector_VectorSupport_registerNatives"),
			signature: std::ptr::null_mut(),
			fnPtr: JVM_RegisterVectorSupportMethods as *mut c_void,
		},
		// TODO
		// JNINativeMethod {
		// 	name: cstr!("Java_jdk_internal_misc_ScopedMemoryAccess_registerNatives"),
		// 	signature: std::ptr::null_mut(),
		// 	fnPtr: JVM_RegisterJDKInternalMiscScopedMemoryAccessMethods as *mut c_void,
		// },
	];

	let jni_name_c = CStr::new(jni_name);
	for method in lookup_special_native_methods {
		unsafe {
			if jni_name_c == CStr::from_ptr(method.name) {
				return Some(method.fnPtr);
			}
		}
	}

	None
}

fn lookup_style(
	method: MethodRef,
	thread: ThreadRef,
	name_converter: &NativeNameConverter,
	num_args: usize,
	include_long: bool,
	os_style: bool,
) -> Option<*const c_void> {
	let jni_name = name_converter.compute_complete_jni_name(num_args, include_long, os_style);

	let class_loader = method.class.loader;
	if class_loader == ClassLoader::Bootstrap {
		if let Some(entry) = lookup_special_native(&jni_name) {
			return Some(entry);
		}

		// if let Some(entry) = os::dll_lookup(os::native_java_library(), jni_name) {
		// 	return Some(entry);
		// }
	}

	assert_eq!(
		class_loader,
		ClassLoader::Bootstrap,
		"Custom classloaders are not implemented yet"
	);

	let classloader_class = ClassLoader::lookup_class(sym!(java_lang_ClassLoader)).unwrap();
	let name_arg = StringInterner::intern_string(&jni_name);

	let findNative_method = classloader_class
		.get_method(
			sym!(findNative_name),
			sym!(ClassLoader_string_long_signature),
			MethodAccessFlags::NONE,
		)
		.unwrap();

	Thread::pre_main_invoke_method(
		Arc::clone(&thread),
		findNative_method,
		Some(vec![
			Operand::Reference(Reference::Null),
			Operand::Reference(Reference::Class(name_arg)),
		]),
	);

	let address = Thread::pull_remaining_operand(thread)
		.unwrap()
		.expect_long();

	if address == 0 {
		todo!("Agent library search");
	}

	let entry = address as usize as *const c_void;
	Some(entry)
}

fn lookup_entry(method: MethodRef, thread: ThreadRef) -> Option<*const c_void> {
	let mut name_converter = NativeNameConverter::new(Arc::clone(&method));

	// Compute pure name
	let Some(_) = name_converter.pure_name() else {
		return None;
	};

	let num_args = 1 // JNIEnv
    + if method.is_static() { 1 } else { 0 } // Extra argument for class
    + method.parameter_count;

	// 1) Try JNI short style
	if let Some(entry) = lookup_style(
		Arc::clone(&method),
		Arc::clone(&thread),
		&name_converter,
		num_args,
		false,
		true,
	) {
		return Some(entry);
	}

	// Compute long name
	let Some(_) = name_converter.long_name() else {
		return None;
	};

	// 2) Try JNI long style
	if let Some(entry) = lookup_style(
		Arc::clone(&method),
		Arc::clone(&thread),
		&name_converter,
		num_args,
		true,
		true,
	) {
		return Some(entry);
	}

	// 3) Try JNI short style without os prefix/suffix
	if let Some(entry) = lookup_style(
		Arc::clone(&method),
		Arc::clone(&thread),
		&name_converter,
		num_args,
		false,
		false,
	) {
		return Some(entry);
	}

	// 4) Try JNI long style without os prefix/suffix
	if let Some(entry) = lookup_style(method, thread, &name_converter, num_args, true, false) {
		return Some(entry);
	}

	// Not found
	None
}

/// Check if there are any JVM TI prefixes which have been applied to the native method name.
fn lookup_entry_prefixed(_method: MethodRef, _thread: ThreadRef) -> Option<*const c_void> {
	todo!()
}

fn lookup_base(method: MethodRef, thread: ThreadRef) -> *const c_void {
	if let Some(entry) = lookup_entry(Arc::clone(&method), Arc::clone(&thread)) {
		return entry;
	}

	if let Some(entry) = lookup_entry_prefixed(method, thread) {
		return entry;
	}

	// TODO
	panic!("UnsatisfiedLinkError")
}

pub fn lookup_native_method(method: MethodRef, thread: ThreadRef) -> *const c_void {
	if let Some(native_method) = method.native_method {
		return native_method;
	}

	let entry = lookup_base(method, thread);
	method.set_native_method(entry);

	entry
}
