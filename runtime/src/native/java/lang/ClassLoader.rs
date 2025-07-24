use crate::classes;
use crate::classpath::loader::{ClassLoader, ClassLoaderSet};
use crate::objects::class::ClassPtr;
use crate::objects::reference::Reference;
use crate::symbols::Symbol;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, handle_exception, throw_and_return_null};

use ::jni::env::JniEnv;
use ::jni::sys::jint;
use common::int_types::s4;
use jni::sys::jbyte;

include_generated!("native/java/lang/def/ClassLoader.registerNatives.rs");
include_generated!("native/java/lang/def/ClassLoader.definitions.rs");

const MN_NESTMATE_CLASS: s4 = 0x00000001;
const MN_HIDDEN_CLASS: s4 = 0x00000002;
const MN_STRONG_LOADER_LINK: s4 = 0x00000004;
const MN_ACCESS_VM_ANNOTATIONS: s4 = 0x00000008;

pub fn defineClass1(
	env: JniEnv,
	_class: ClassPtr,
	loader: Reference, // java.lang.ClassLoader
	name: Reference,   // java.lang.String
	b: Reference,      // byte[],
	off: jint,
	len: jint,
	_pd: Reference,    // ProtectionDomain
	source: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	if b.is_null() {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw_and_return_null!(thread, NullPointerException);
	}

	let bytes = b.extract_primitive_array();
	let bytes_slice = bytes.as_slice::<jbyte>();

	let off = off as usize;
	let len = len as usize;

	let bytes_window_signed = &bytes_slice[off..off + len];

	// SAFETY: `i8` and `u8` have the same size and alignment
	let bytes_window: &[u8] = unsafe {
		std::slice::from_raw_parts(
			bytes_window_signed.as_ptr() as *const u8,
			bytes_window_signed.len(),
		)
	};

	let name_sym;
	if name.is_null() {
		// TODO: In this case, the name is just to be derived from the class file itself.
		//       Will need to train the loader to be able to do that.
		todo!()
	} else {
		let external_name = classes::java::lang::String::extract(name.extract_class());
		let internal_name = external_name.replace('.', "/");
		name_sym = Symbol::intern(internal_name);
	}

	let source_str;
	if source.is_null() {
		source_str = None;
	} else {
		source_str = Some(classes::java::lang::String::extract(source.extract_class()));
	}

	// Not a thing in defineClass1
	let is_hidden = false;

	let loader = ClassLoaderSet::find_or_add(loader, is_hidden);
	let class = match loader.derive_class(name_sym, Some(bytes_window), is_hidden) {
		Throws::Ok(class) => class,
		Throws::Exception(e) => {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			e.throw(thread);
			return Reference::null();
		},
	};

	Reference::mirror(class.mirror())
}

pub fn defineClass2(
	_env: JniEnv,
	_class: ClassPtr,
	_loader: Reference, // java.lang.ClassLoader
	_name: Reference,   // java.lang.String
	_b: Reference,      // java.nio.ByteBuffer,
	_off: jint,
	_len: jint,
	_pd: Reference,     // ProtectionDomain
	_source: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	unimplemented!("java.lang.ClassLoader#defineClass2")
}

pub fn defineClass0(
	env: JniEnv,
	_class: ClassPtr,
	loader: Reference, // java.lang.ClassLoader
	lookup: Reference, // java.lang.Class
	name: Reference,   // java.lang.String
	b: Reference,      // byte[],
	off: jint,
	len: jint,
	_pd: Reference, // ProtectionDomain
	initialize: bool,
	flags: jint,
	classData: Reference, // java.lang.Object
) -> Reference // java.lang.Class
{
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	if lookup.is_null() {
		throw_and_return_null!(thread, IllegalArgumentException, "Lookup class is null");
	}

	let lookup = lookup.extract_target_class();

	let is_nestmate = (flags & MN_NESTMATE_CLASS) == MN_NESTMATE_CLASS;
	let is_hidden = (flags & MN_HIDDEN_CLASS) == MN_HIDDEN_CLASS;
	let is_strong = (flags & MN_STRONG_LOADER_LINK) == MN_STRONG_LOADER_LINK;
	let vm_annotations = (flags & MN_ACCESS_VM_ANNOTATIONS) == MN_ACCESS_VM_ANNOTATIONS;

	let nest_host_class;
	if is_nestmate {
		let host = handle_exception!(Reference::null(), thread, lookup.nest_host(thread));
		nest_host_class = Some(host);
	} else {
		nest_host_class = None;
	}

	if !is_hidden {
		if !classData.is_null() {
			throw_and_return_null!(
				thread,
				IllegalArgumentException,
				"classData is only applicable for hidden classes"
			);
		}

		if is_nestmate {
			throw_and_return_null!(
				thread,
				IllegalArgumentException,
				"dynamic nestmate is only applicable for hidden classes"
			);
		}

		if vm_annotations {
			throw_and_return_null!(
				thread,
				IllegalArgumentException,
				"vm annotations only allowed for hidden classes"
			);
		}

		if flags != MN_STRONG_LOADER_LINK {
			throw_and_return_null!(thread, IllegalArgumentException, "invalid flag 0x{flags}");
		}
	}

	let loader = ClassLoaderSet::find_or_add(loader, is_hidden && !is_strong);
	if name.is_null() {
		// TODO: In this case, the name is just to be derived from the class file itself.
		//       Will need to train the loader to be able to do that.
		todo!()
	}

	let name = Symbol::intern(classes::java::lang::String::extract(name.extract_class()));

	let bytes = b.extract_primitive_array();
	let bytes_slice = bytes.as_slice::<jbyte>();

	let off = off as usize;
	let len = len as usize;

	let bytes_window_signed = &bytes_slice[off..off + len];

	// SAFETY: `i8` and `u8` have the same size and alignment
	let bytes_window: &[u8] = unsafe {
		std::slice::from_raw_parts(
			bytes_window_signed.as_ptr() as *const u8,
			bytes_window_signed.len(),
		)
	};

	let class = match loader.derive_class(name, Some(bytes_window), is_hidden) {
		Throws::Ok(class) => class,
		Throws::Exception(e) => {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			e.throw(thread);
			return Reference::null();
		},
	};

	if let Some(host) = nest_host_class {
		unsafe {
			class.set_nest_host(host);
		}
	}

	if is_hidden {
		class.mirror().set_class_data(classData);
	}

	if initialize {
		if let Throws::Exception(e) = class.initialize(thread) {
			e.throw(thread);
			return Reference::null();
		}
	}

	// TODO: Parallel class loaders
	Reference::mirror(class.mirror())
}

pub fn findBootstrapClass(
	_env: JniEnv,
	_class: ClassPtr,
	name: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	let name = classes::java::lang::String::extract(name.extract_class());
	if let Some(class) = ClassLoader::bootstrap().lookup_class(Symbol::intern(name)) {
		return Reference::mirror(class.mirror());
	}

	Reference::null()
}

pub fn findLoadedClass0(
	_env: JniEnv,
	this: Reference, // java.lang.ClassLoader
	name: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	if name.is_null() {
		return Reference::null();
	}

	let name_str = classes::java::lang::String::extract(name.extract_class());
	let internal_name = name_str.replace('.', "/");

	let internal_name_sym = Symbol::intern(internal_name);

	let Some(loader) = ClassLoaderSet::find(this, false) else {
		// Unknown loader
		return Reference::null();
	};

	match loader.lookup_class(internal_name_sym) {
		None => Reference::null(),
		Some(class) => Reference::mirror(class.mirror()),
	}
}

pub fn retrieveDirectives(_env: JniEnv, _class: ClassPtr) -> Reference // AssertionStatusDirectives
{
	unimplemented!("java.lang.ClassLoader#retrieveDirectives")
}
