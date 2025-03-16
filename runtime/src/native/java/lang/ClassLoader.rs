use crate::classes;
use crate::classpath::loader::{ClassLoader, ClassLoaderSet};
use crate::objects::class::Class;
use crate::objects::reference::Reference;
use crate::symbols::Symbol;
use crate::thread::exceptions::{handle_exception, throw_and_return_null, Throws};
use crate::thread::JavaThread;

use ::jni::env::JniEnv;
use ::jni::sys::jint;
use common::int_types::s4;
use common::traits::PtrType;
use jni::sys::jbyte;

include_generated!("native/java/lang/def/ClassLoader.registerNatives.rs");
include_generated!("native/java/lang/def/ClassLoader.definitions.rs");

const MN_NESTMATE_CLASS: s4 = 0x00000001;
const MN_HIDDEN_CLASS: s4 = 0x00000002;
const MN_STRONG_LOADER_LINK: s4 = 0x00000004;
const MN_ACCESS_VM_ANNOTATIONS: s4 = 0x00000008;

pub fn defineClass1(
	_env: JniEnv,
	_class: &'static Class,
	_loader: Reference, // java.lang.ClassLoader
	_name: Reference,   // java.lang.String
	_b: Reference,      // byte[],
	_off: jint,
	_len: jint,
	_pd: Reference,     // ProtectionDomain
	_source: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	unimplemented!("java.lang.ClassLoader#defineClass1")
}

pub fn defineClass2(
	_env: JniEnv,
	_class: &'static Class,
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
	_env: JniEnv,
	_class: &'static Class,
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
	let thread = unsafe { &*JavaThread::for_env(_env.raw()) };

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

	let name = Symbol::intern(classes::java_lang_String::extract(
		name.extract_class().get(),
	));

	let bytes = b.extract_primitive_array();
	let bytes_slice = bytes.get().as_slice::<jbyte>();

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
			let thread = unsafe { &*JavaThread::for_env(_env.raw()) };
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
		class.mirror().get().set_class_data(classData);
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
	_class: &'static Class,
	name: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	let name = classes::java_lang_String::extract(name.extract_class().get());
	if let Some(class) = ClassLoader::bootstrap().lookup_class(Symbol::intern(name)) {
		return Reference::mirror(class.mirror());
	}

	Reference::null()
}

pub fn findLoadedClass0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
	_name: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	unimplemented!("java.lang.ClassLoader#findLoadedClass0")
}

pub fn retrieveDirectives(_env: JniEnv, _class: &'static Class) -> Reference // AssertionStatusDirectives
{
	unimplemented!("java.lang.ClassLoader#retrieveDirectives")
}
