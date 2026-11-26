#![native_macros::jni_fn_module]

use crate::classpath::loader::{ClassLoader, ClassLoaderSet};
use crate::native::RawSymbolExt;
use crate::native::java::lang::String::StringInterner;
use crate::native::jni::references::JObjectExt;
use crate::objects::class::ClassPtr;
use crate::objects::instance::array::{Array, ObjectArrayInstance, ObjectArrayInstanceRef};
use crate::objects::instance::class::ClassInstanceRef;
use crate::objects::instance::mirror::MirrorInstanceRef;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::symbols::{Symbol, sym};
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, handle_exception, throw, throw_with_ret};
use crate::{classes, globals};

use std::ffi::{CStr, c_char, c_int};

use ::jni::env::JniEnv;
use ::jni::objects::{JByteArray, JClass, JObject, JObjectArray, JString};
use ::jni::sys::{jboolean, jbyte, jint, jsize};
use common::int_types::s4;
use common::unicode;
use native_macros::jni_call;

const MN_NESTMATE_CLASS: s4 = 0x0000_0001;
const MN_HIDDEN_CLASS: s4 = 0x0000_0002;
const MN_STRONG_LOADER_LINK: s4 = 0x0000_0004;
const MN_ACCESS_VM_ANNOTATIONS: s4 = 0x0000_0008;

/// Ensure that the mirror instance is not primitive or an array
pub(super) fn ensure_class_mirror(this: Reference) -> Throws<Option<MirrorInstanceRef>> {
	if this.is_null() {
		throw!(@DEFER NullPointerException);
	}

	let mirror = this.extract_mirror();
	if mirror.is_primitive() || mirror.is_array() {
		return Throws::Ok(None);
	}

	Throws::Ok(Some(mirror))
}

#[jni_call]
pub extern "C" fn JVM_GetCallerClass(env: JniEnv) -> JClass {
	let current_thread = unsafe { &*JavaThread::for_env(env.raw().cast_const()) };

	// The call stack at this point looks something like this:
	//
	// [0] [ @CallerSensitive public jdk.internal.reflect.Reflection.getCallerClass ]
	// [1] [ @CallerSensitive API.method                                            ]
	// [.] [ (skipped intermediate frames)                                          ]
	// [n] [ caller                                                                 ]
	for (n, frame) in current_thread.frame_stack().iter().enumerate() {
		let method = frame.method();

		if n == 0 || n == 1 {
			if n == 0 {
				if !(method.class().name() == sym!(jdk_internal_reflect_Reflection)
					&& method.name == sym!(getCallerClass))
				{
					throw_with_ret!(
						JClass::null(),
						current_thread,
						InternalError,
						"JVM_GetCallerClass must only be called from Reflection.getCallerClass"
					);
				}
			}

			if !method.is_caller_sensitive() {
				throw_with_ret!(
					JClass::null(),
					current_thread,
					InternalError,
					"`getCallerClass` is not called from a @CallerSensitive method"
				);
			}

			continue;
		}

		if !method.is_stack_walk_ignored() {
			return current_thread.jni_refs().allocate_wrapped(method.class());
		}
	}

	JClass::null()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_FindPrimitiveClass(env: JniEnv, utf: *const c_char) -> JClass {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let name_c = unsafe { CStr::from_ptr(utf) };
	for (name, ty) in crate::globals::PRIMITIVE_TYPE_NAMES_TO_FIELD_TYPES {
		if name_c.to_bytes() == name.as_bytes() {
			let ret = crate::globals::mirrors::primitive_mirror_for(ty);
			return thread.jni_refs().allocate_wrapped(ret.extract_mirror());
		}
	}

	throw_with_ret!(
		JClass::null(),
		thread,
		ClassNotFoundException,
		"{}",
		name_c.to_string_lossy()
	);
}

/// Despite the name, this will actually *load* a class if necessary. We also discard any exceptions
/// and just return `null`.
#[jni_call(no_strict_types)]
pub extern "C" fn JVM_FindClassFromBootLoader(env: JniEnv, name: *const c_char) -> JClass {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	let name_c = unsafe { CStr::from_ptr(name) };
	let name = unicode::decode(name_c.to_bytes()).unwrap();

	match ClassLoader::bootstrap().load(Symbol::intern(name)) {
		Throws::Ok(class) => thread.jni_refs().allocate_wrapped(class),
		Throws::Exception(_) => JClass::null(),
	}
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_FindClassFromLoader(
	env: JniEnv,
	name: *const c_char,
	init: jboolean,
	loader: JObject,
) -> JClass {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let loader = unsafe { loader.to_reference_maybe_null() };
	if name.is_null() {
		throw_with_ret!(JClass::null(), thread, NullPointerException);
	}

	let name_c = unsafe { CStr::from_ptr(name) };
	let Ok(binary_name) = unicode::decode(name_c.to_bytes()) else {
		throw_with_ret!(JClass::null(), thread, IllegalArgumentException);
	};

	let internal_name = binary_name.replace('.', "/");
	let internal_name_sym = Symbol::intern(internal_name);

	let loader = ClassLoaderSet::find_or_add(loader, false);
	let class: ClassPtr = handle_exception!(JClass::null(), thread, loader.load(internal_name_sym));

	if init {
		handle_exception!(JClass::null(), thread, class.initialize(thread));
	}

	thread.jni_refs().allocate_wrapped(class)
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_FindClassFromClass(
	_env: JniEnv,
	_name: *const c_char,
	_init: jboolean,
	_from: JClass,
) -> JClass {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_DefineClass(
	_env: JniEnv,
	_name: *const c_char,
	_loader: JObject,
	_buf: *const jbyte,
	_len: jsize,
	_protection_domain: JObject,
) -> JClass {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_LookupDefineClass(
	env: JniEnv,
	lookup: JClass,
	name: *const c_char,
	buf: *const jbyte,
	len: jsize,
	_protection_domain: JObject,
	initialize: jboolean,
	flags: c_int,
	class_data: JObject,
) -> JClass {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	let lookup = unsafe { lookup.to_reference_maybe_null() };
	if lookup.is_null() {
		throw_with_ret!(
			JClass::null(),
			thread,
			IllegalArgumentException,
			"Lookup class is null"
		);
	}

	let lookup = lookup.extract_target_class();

	let buf = unsafe {
		// SAFETY: `i8` and `u8` have the same size and alignment
		let ptr = buf.cast::<u8>();
		std::slice::from_raw_parts(ptr, len as usize)
	};

	let is_nestmate = (flags & MN_NESTMATE_CLASS) == MN_NESTMATE_CLASS;
	let is_hidden = (flags & MN_HIDDEN_CLASS) == MN_HIDDEN_CLASS;
	let is_strong = (flags & MN_STRONG_LOADER_LINK) == MN_STRONG_LOADER_LINK;
	let vm_annotations = (flags & MN_ACCESS_VM_ANNOTATIONS) == MN_ACCESS_VM_ANNOTATIONS;

	let nest_host_class;
	if is_nestmate {
		let host = handle_exception!(JClass::null(), thread, lookup.nest_host(thread));
		nest_host_class = Some(host);
	} else {
		nest_host_class = None;
	}

	if !is_hidden {
		if !class_data.is_null() {
			throw_with_ret!(
				JClass::null(),
				thread,
				IllegalArgumentException,
				"classData is only applicable for hidden classes"
			);
		}

		if is_nestmate {
			throw_with_ret!(
				JClass::null(),
				thread,
				IllegalArgumentException,
				"dynamic nestmate is only applicable for hidden classes"
			);
		}

		if !is_strong {
			throw_with_ret!(
				JClass::null(),
				thread,
				IllegalArgumentException,
				"an ordinary class must be strongly referenced by its defining loader"
			);
		}

		if vm_annotations {
			throw_with_ret!(
				JClass::null(),
				thread,
				IllegalArgumentException,
				"vm annotations only allowed for hidden classes"
			);
		}

		if flags != MN_STRONG_LOADER_LINK {
			throw_with_ret!(
				JClass::null(),
				thread,
				IllegalArgumentException,
				"invalid flag 0x{flags}"
			);
		}
	}

	if name.is_null() {
		// TODO: In this case, the name is just to be derived from the class file itself.
		//       Will need to train the loader to be able to do that.
		todo!()
	}

	let name = unsafe { Symbol::intern_mutf_c_str(name) };

	let class = match lookup.loader().derive_class(name, Some(buf), is_hidden) {
		Throws::Ok(class) => class,
		Throws::Exception(e) => {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			e.throw(thread);
			return JClass::null();
		},
	};

	if let Some(host) = nest_host_class {
		unsafe {
			class.set_nest_host(host);
		}
	}

	if is_hidden {
		let class_data = unsafe { class_data.to_reference_maybe_null() };
		class.mirror().set_class_data(class_data);
	}

	if initialize && let Throws::Exception(e) = class.initialize(thread) {
		e.throw(thread);
		return JClass::null();
	}

	// TODO: Parallel class loaders
	thread.jni_refs().allocate_wrapped(class)
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_DefineClassWithSource(
	env: JniEnv,
	name: *const c_char,
	loader: JObject,
	buf: *const jbyte,
	len: jsize,
	_protection_domain: JObject,
	source: *const c_char,
) -> JClass {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	let buf = unsafe {
		// SAFETY: `i8` and `u8` have the same size and alignment
		let ptr = buf.cast::<u8>();
		std::slice::from_raw_parts(ptr, len as usize)
	};

	let name_sym;
	if name.is_null() {
		// TODO: In this case, the name is just to be derived from the class file itself.
		//       Will need to train the loader to be able to do that.
		todo!()
	} else {
		let external_name_c = unsafe { CStr::from_ptr(name) };
		let external_name = unicode::decode(external_name_c.to_bytes()).unwrap();
		let internal_name = external_name.replace('.', "/");
		name_sym = Symbol::intern(internal_name);
	}

	let loader = unsafe { loader.to_reference_maybe_null() };

	let source_str;
	if source.is_null() {
		source_str = None;
	} else {
		let source_c = unsafe { CStr::from_ptr(source) };
		source_str = unicode::decode(source_c.to_bytes()).ok(); // TODO: maybe panic?
	}

	// Not a thing in defineClass1
	let is_hidden = false;

	let loader = ClassLoaderSet::find_or_add(loader, is_hidden);
	let class = match loader.derive_class(name_sym, Some(buf), is_hidden) {
		Throws::Ok(class) => class,
		Throws::Exception(e) => {
			e.throw(thread);
			return JClass::null();
		},
	};

	thread.jni_refs().allocate_wrapped(class)
}

#[jni_call]
pub extern "C" fn JVM_FindLoadedClass(env: JniEnv, loader: JObject, name: JString) -> JClass {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	let name = unsafe { name.to_reference_maybe_null() };
	if name.is_null() {
		return JClass::null();
	}

	let name_str = classes::java::lang::String::extract(name.extract_class());
	let internal_name = name_str.replace('.', "/");

	let internal_name_sym = Symbol::intern(internal_name);

	let loader_obj = unsafe { loader.to_reference_maybe_null() };
	let Some(loader) = ClassLoaderSet::find(loader_obj, false) else {
		// Unknown loader
		return JClass::null();
	};

	match loader.lookup_class(internal_name_sym) {
		None => JClass::null(),
		Some(class) => thread.jni_refs().allocate_wrapped(class),
	}
}

#[jni_call]
pub extern "C" fn JVM_InitClassName(env: JniEnv, class: JClass) -> JString {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let this = unsafe { class.to_reference() }.expect("invalid class reference");
	let this_mirror = this.extract_mirror();
	let this_mirror_target = this_mirror.target_class();

	let this_name = this_mirror_target.name();
	let this_binary_name = this_name.as_str().replace('/', ".");
	let name_string = StringInterner::intern(&*this_binary_name);

	classes::java::lang::Class::set_name(this_mirror, Reference::class(name_string));

	let ptr = thread
		.jni_refs()
		.allocate_wrapped(Reference::class(name_string));
	unsafe { JString::from_raw(ptr.raw()) }
}

#[jni_call]
pub extern "C" fn JVM_GetClassInterfaces(env: JniEnv, class: JClass) -> JObjectArray {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let class = unsafe { class.to_reference() }.expect("invalid class reference");
	let mirror = class.extract_mirror();

	let target_class = mirror.target_class();
	let interfaces = target_class.interfaces();

	let interfaces_array =
		ObjectArrayInstance::new(interfaces.len() as s4, globals::classes::java_lang_Class());
	let interfaces_array: ObjectArrayInstanceRef =
		handle_exception!(JObjectArray::null(), thread, interfaces_array);

	for (index, interface) in interfaces.iter().enumerate() {
		// SAFETY: interfaces_array.len() is guaranteed to equal interfaces.len()
		unsafe {
			interfaces_array.store_unchecked(index, Reference::mirror(interface.mirror()));
		}
	}

	thread.jni_refs().allocate_wrapped(interfaces_array)
}

#[jni_call]
pub extern "C" fn JVM_IsInterface(_env: JniEnv, class: JClass) -> jboolean {
	let this = unsafe { class.to_reference() }.expect("invalid class reference");
	this.extract_target_class().is_interface()
}

#[jni_call]
pub extern "C" fn JVM_IsHiddenClass(_env: JniEnv, class: JClass) -> jboolean {
	let class = unsafe { class.to_reference() }.expect("invalid class reference");

	let mirror = class.extract_mirror();
	mirror.target_class().is_hidden()
}

#[jni_call]
pub extern "C" fn JVM_FindScopedValueBindings(_env: JniEnv, _class: JClass) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetDeclaredClasses(_env: JniEnv, _class: JClass) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetDeclaringClass(env: JniEnv, class: JClass) -> JClass {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let this = unsafe { class.to_reference() }.expect("invalid class reference");
	let Some(mirror) = handle_exception!(JClass::null(), thread, ensure_class_mirror(this)) else {
		return JClass::null();
	};

	let target_class = mirror.target_class();
	let target_class_descriptor = target_class.unwrap_class_instance();
	let Some(inner_classes) = target_class_descriptor.inner_classes() else {
		// No InnerClasses attribute
		return JClass::null();
	};

	let mut declaring_class = Reference::null();
	for inner_class in inner_classes {
		if inner_class.inner_class != target_class.name() {
			continue;
		}

		match inner_class.outer_class {
			Some(outer) => {
				let outer: ClassPtr =
					handle_exception!(JClass::null(), thread, target_class.loader().load(outer));

				if outer.is_array() {
					throw_with_ret!(JClass::null(), thread, IncompatibleClassChangeError);
				}

				declaring_class = Reference::mirror(outer.mirror());
			},
			None => {
				let Some(enclosing_method) = target_class_descriptor.enclosing_method else {
					// Class has no immediate enclosing method/class information
					return JClass::null();
				};

				declaring_class = Reference::mirror(enclosing_method.class.mirror());
			},
		}

		break;
	}

	if declaring_class.is_null() {
		return JClass::null();
	}

	// TODO: need to verify that outer class actually declared the inner class
	thread
		.jni_refs()
		.allocate_wrapped(declaring_class.extract_mirror())
}

#[jni_call]
pub extern "C" fn JVM_GetSimpleBinaryName(env: JniEnv, class: JClass) -> JString {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let this = unsafe { class.to_reference() }.expect("invalid class reference");
	let Some(mirror) = handle_exception!(JString::null(), thread, ensure_class_mirror(this)) else {
		return JString::null();
	};

	let target_class = mirror.target_class();
	let target_class_descriptor = target_class.unwrap_class_instance();
	let Some(inner_classes) = target_class_descriptor.inner_classes() else {
		// No InnerClasses attribute
		return JString::null();
	};

	for inner_class in inner_classes {
		if inner_class.inner_class != target_class.name() {
			continue;
		}

		if let Some(name) = inner_class.inner_class_name {
			let ptr = thread
				.jni_refs()
				.allocate_wrapped(Reference::class(StringInterner::intern(name)));

			return unsafe { JString::from_raw(ptr.raw()) };
		}

		break;
	}

	JString::null()
}

#[jni_call]
pub extern "C" fn JVM_GetClassSignature(_env: JniEnv, _class: JClass) -> JString {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassAnnotations(_env: JniEnv, _class: JClass) -> JByteArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassTypeAnnotations(_env: JniEnv, _class: JClass) -> JByteArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetMethodTypeAnnotations(_env: JniEnv, _method: JObject) -> JByteArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetFieldTypeAnnotations(_env: JniEnv, _field: JObject) -> JByteArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetMethodParameters(_env: JniEnv, _method: JObject) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassDeclaredFields(
	env: JniEnv,
	class: JClass,
	public_only: jboolean,
) -> JObjectArray {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let this = unsafe { class.to_reference() }.expect("invalid class reference");
	let mirror = this.extract_mirror();

	// Primitive and array mirrors are excluded
	if mirror.is_array() || mirror.is_primitive() {
		let empty_array =
			match ObjectArrayInstance::new(0, globals::classes::java_lang_reflect_Field()) {
				Throws::Ok(array) => array,
				Throws::Exception(e) => {
					e.throw(thread);
					return JObjectArray::null();
				},
			};

		return thread.jni_refs().allocate_wrapped(empty_array);
	}

	let target = mirror.target_class();

	let fields = target
		.fields()
		.filter(|field| !public_only || field.access_flags.is_public())
		.collect::<Vec<_>>();

	let ret = match ObjectArrayInstance::new(
		fields.len() as s4,
		globals::classes::java_lang_reflect_Field(),
	) {
		Throws::Ok(array) => array,
		Throws::Exception(e) => {
			e.throw(thread);
			return JObjectArray::null();
		},
	};

	for (index, field) in fields.iter().enumerate() {
		let reflect_field = match classes::java::lang::reflect::Field::new(field) {
			Throws::Ok(field) => field,
			Throws::Exception(e) => {
				e.throw(thread);
				return JObjectArray::null();
			},
		};

		// SAFETY: We can't go outside the bounds, `ret` was initialized to the right size
		unsafe {
			ret.store_unchecked(index, Reference::class(reflect_field));
		}
	}

	thread.jni_refs().allocate_wrapped(ret)
}

#[jni_call]
pub extern "C" fn JVM_IsRecord(_env: JniEnv, _class: JClass) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetRecordComponents(_env: JniEnv, _class: JClass) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassDeclaredMethods(
	env: JniEnv,
	class: JClass,
	public_only: jboolean,
) -> JObjectArray {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let this = unsafe { class.to_reference() }.expect("invalid class reference");
	let mirror = this.extract_mirror();

	// Primitive and array mirrors are excluded
	if mirror.is_array() || mirror.is_primitive() {
		let empty_array =
			match ObjectArrayInstance::new(0, globals::classes::java_lang_reflect_Method()) {
				Throws::Ok(array) => array,
				Throws::Exception(e) => {
					e.throw(thread);
					return JObjectArray::null();
				},
			};

		return thread.jni_refs().allocate_wrapped(empty_array);
	}

	let target = mirror.target_class();

	let methods = target
		.vtable()
		.iter_local()
		.filter(|method| {
			let matches_visibility = if public_only {
				method.is_public()
			} else {
				true
			};
			!method.is_clinit() && !method.is_constructor() && matches_visibility
		})
		.collect::<Vec<_>>();

	let ret = match ObjectArrayInstance::new(
		methods.len() as s4,
		globals::classes::java_lang_reflect_Method(),
	) {
		Throws::Ok(array) => array,
		Throws::Exception(e) => {
			e.throw(thread);
			return JObjectArray::null();
		},
	};

	for (index, method) in methods.iter().enumerate() {
		let reflect_method = match classes::java::lang::reflect::Method::new(method) {
			Throws::Ok(method) => method,
			Throws::Exception(e) => {
				e.throw(thread);
				return JObjectArray::null();
			},
		};

		// SAFETY: We can't go outside the bounds, `ret` was initialized to the right size
		unsafe {
			ret.store_unchecked(index, Reference::class(reflect_method));
		}
	}

	thread.jni_refs().allocate_wrapped(ret)
}

#[jni_call]
pub extern "C" fn JVM_GetClassDeclaredConstructors(
	env: JniEnv,
	class: JClass,
	public_only: jboolean,
) -> JObjectArray {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let this = unsafe { class.to_reference() }.expect("invalid class reference");
	let this_mirror = this.extract_mirror();

	// Not applicable for primitive and array mirrors
	if this_mirror.is_primitive() || this_mirror.is_array() {
		let ret = handle_exception!(
			JObjectArray::null(),
			thread,
			ObjectArrayInstance::new(0, globals::classes::java_lang_reflect_Constructor())
		);
		return thread.jni_refs().allocate_wrapped(ret);
	}

	let target = this_mirror.target_class();
	let constructors = target
		.vtable()
		.iter_local()
		.filter(|method| {
			if !method.is_public() && public_only {
				return false;
			}

			method.name == sym!(object_initializer_name)
		})
		.collect::<Vec<&'static Method>>();

	let ret: ObjectArrayInstanceRef = handle_exception!(
		JObjectArray::null(),
		thread,
		ObjectArrayInstance::new(
			constructors.len() as jint,
			globals::classes::java_lang_reflect_Constructor()
		)
	);

	for (i, constructor) in constructors.into_iter().enumerate() {
		let constructor: ClassInstanceRef = handle_exception!(
			JObjectArray::null(),
			thread,
			classes::java::lang::reflect::Constructor::new(constructor)
		);

		// SAFETY: The array is known to have the correct length
		unsafe { ret.store_unchecked(i, Reference::class(constructor)) };
	}

	thread.jni_refs().allocate_wrapped(ret)
}

#[jni_call]
pub extern "C" fn JVM_AreNestMates(env: JniEnv, current: JClass, member: JClass) -> jboolean {
	let thread = unsafe { &*JavaThread::for_env(env.raw().cast_const()) };

	let Some(current) = (unsafe { current.to_reference() }) else {
		panic!("JVM_AreNestMates called with null `current` class");
	};

	let Some(member) = (unsafe { member.to_reference() }) else {
		panic!("JVM_AreNestMates called with null `member` class");
	};

	let current_class = current.extract_target_class();
	let member_class = member.extract_target_class();
	match current_class.is_nestmate_of(member_class, thread) {
		Throws::Ok(ret) => ret,
		Throws::Exception(e) => {
			e.throw(thread);
			false
		},
	}
}

#[jni_call]
pub extern "C" fn JVM_GetNestHost(_env: JniEnv, _current: JClass) -> JClass {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetNestMembers(_env: JniEnv, _current: JClass) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetPermittedSubclasses(_env: JniEnv, _current: JClass) -> JObjectArray {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetClassNameUTF(_env: JniEnv, _cb: JClass) -> *const c_char {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassFieldsCount(_env: JniEnv, _cb: JClass) -> jint {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassMethodsCount(_env: JniEnv, _cb: JClass) -> jint {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassFileVersion(_env: JniEnv, _current: JClass) -> jint {
	todo!()
}
