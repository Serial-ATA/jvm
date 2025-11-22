use crate::classpath::loader::ClassLoaderSet;
use crate::native::java::lang::String::StringInterner;
use crate::native::jni::{IntoJni, reference_from_jobject};
use crate::objects::class::ClassPtr;
use crate::objects::instance::array::{Array, ObjectArrayInstance, ObjectArrayInstanceRef};
use crate::objects::instance::class::ClassInstanceRef;
use crate::objects::instance::mirror::MirrorInstanceRef;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::symbols::{Symbol, sym};
use crate::thread::JavaThread;
use crate::thread::exceptions::{
	Throws, handle_exception, throw, throw_and_return_null, throw_with_ret,
};
use crate::{classes, globals, include_generated};

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint};
use classfile::accessflags::ClassAccessFlags;
use common::int_types::s4;

include_generated!("native/java/lang/def/Class.registerNatives.rs");
include_generated!("native/java/lang/def/Class.definitions.rs");

/// Ensure that the mirror instance is not primitive or an array
fn ensure_class_mirror(this: Reference) -> Throws<Option<MirrorInstanceRef>> {
	if this.is_null() {
		throw!(@DEFER NullPointerException);
	}

	let mirror = this.extract_mirror();
	if mirror.is_primitive() || mirror.is_array() {
		return Throws::Ok(None);
	}

	Throws::Ok(Some(mirror))
}

// throws ClassNotFoundException
pub fn forName0(
	env: JniEnv,
	_class: ClassPtr,
	name: Reference, // java.lang.String
	initialize: jboolean,
	loader: Reference,  // java.lang.ClassLoader
	_caller: Reference, // java.lang.Class
) -> Reference /* java.lang.Class */ {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	if name.is_null() {
		throw_and_return_null!(thread, NullPointerException);
	}

	let binary_name = classes::java::lang::String::extract(name.extract_class());
	let internal_name = binary_name.replace('.', "/");
	let internal_name_sym = Symbol::intern(internal_name);

	let loader = ClassLoaderSet::find_or_add(loader, false);
	let class: ClassPtr =
		handle_exception!(Reference::null(), thread, loader.load(internal_name_sym));

	if initialize {
		handle_exception!(Reference::null(), thread, class.initialize(thread));
	}

	Reference::mirror(class.mirror())
}

pub fn isInstance(
	_env: JniEnv,
	this: Reference, // java.lang.Class
	obj: Reference,  // java.lang.Object
) -> jboolean {
	if this.is_null() {
		return false;
	}

	let target_class = this.extract_target_class();
	let obj_class = obj.extract_instance_class();

	obj_class.can_cast_to(target_class)
}
pub fn isAssignableFrom(
	env: JniEnv,
	this: Reference, // java.lang.Class
	cls: Reference,  // java.lang.Class
) -> jboolean {
	if cls.is_null() {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw_with_ret!(false, thread, NullPointerException);
	}

	// For clarity
	let sub = cls.extract_target_class();
	let super_ = this.extract_target_class();

	env.is_assignable_from(sub.into_jni_safe(), super_.into_jni_safe())
}
pub fn isInterface(_env: JniEnv, this: Reference /* java.lang.Class */) -> jboolean {
	this.extract_target_class().is_interface()
}
pub fn isArray(_env: JniEnv, this: Reference /* java.lang.Class */) -> jboolean {
	this.extract_mirror().is_array()
}
pub fn isPrimitive(_env: JniEnv, this: Reference /* java.lang.Class */) -> jboolean {
	this.extract_mirror().is_primitive()
}

pub fn initClassName(
	_env: JniEnv,
	this: Reference, // java.lang.Class
) -> Reference {
	let this_mirror = this.extract_mirror();
	let this_mirror_target = this_mirror.target_class();
	let this_name = this_mirror_target.name();
	let this_binary_name = this_name.as_str().replace('/', ".");
	let name_string = StringInterner::intern(&*this_binary_name);

	classes::java::lang::Class::set_name(this_mirror, Reference::class(name_string));

	Reference::class(name_string)
}

pub fn getSuperclass(
	env: JniEnv,
	this: Reference, // java.lang.Class
) -> Reference /* Class<? super T> */
{
	let target_class = this.extract_target_class();
	let Some(super_class_jni) = env.get_super_class(target_class.into_jni_safe()) else {
		return Reference::null();
	};

	let super_class_obj = unsafe { reference_from_jobject(super_class_jni.raw() as _) }
		.expect("should never be null");
	let super_class = super_class_obj.extract_target_class();

	Reference::mirror(super_class.mirror())
}
pub fn getInterfaces0(
	env: JniEnv,
	this: Reference, // java.lang.Class
) -> Reference /* Class<?>[] */
{
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let mirror = this.extract_mirror();
	let target_class = mirror.target_class();
	let interfaces = target_class.interfaces();

	let interfaces_array =
		ObjectArrayInstance::new(interfaces.len() as s4, globals::classes::java_lang_Class());
	let interfaces_array: ObjectArrayInstanceRef =
		handle_exception!(Reference::null(), thread, interfaces_array);

	for (index, interface) in interfaces.iter().enumerate() {
		// SAFETY: interfaces_array.len() is guaranteed to equal interfaces.len()
		unsafe {
			interfaces_array.store_unchecked(index, Reference::mirror(interface.mirror()));
		}
	}

	Reference::object_array(interfaces_array)
}
pub fn getModifiers(_env: JniEnv, this: Reference /* java.lang.Class */) -> jint {
	let mirror = this.extract_mirror();
	if mirror.is_primitive() {
		return jint::from(
			(ClassAccessFlags::ACC_ABSTRACT
				| ClassAccessFlags::ACC_FINAL
				| ClassAccessFlags::ACC_PUBLIC)
				.as_u2(),
		);
	}

	let class = mirror.target_class();
	jint::from(class.modifier_flags())
}
pub fn getSigners(_env: JniEnv, _this: Reference /* java.lang.Class */) -> Reference /* Object[] */
{
	unimplemented!("Class#getSigners");
}
pub fn setSigners(
	_env: JniEnv,
	_this: Reference,    // java.lang.Class
	_signers: Reference, // Object[]
) {
	unimplemented!("Class#setSigners");
}

// Returns Object[3] where:
//
// [0]: The class holding the enclosing method (not null, java.lang.Class<?>)
// [1]: The enclosing method's name (nullable, java.lang.String)
// [2]: The enclosing method's descriptor (nullable. java.lang.String)
pub fn getEnclosingMethod0(
	env: JniEnv,
	this: Reference, // java.lang.Class
) -> Reference /* Object[] */
{
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	let Some(mirror) = handle_exception!(Reference::null(), thread, ensure_class_mirror(this))
	else {
		return Reference::null();
	};

	let array = ObjectArrayInstance::new(3, crate::globals::classes::java_lang_Object());
	let array_instance: ObjectArrayInstanceRef =
		handle_exception!(Reference::null(), thread, array);

	let target_class = mirror.target_class();

	let Some(enclosing_method) = target_class.unwrap_class_instance().enclosing_method else {
		// Class has no immediate enclosing method/class information
		return Reference::null();
	};

	let enclosing_class = enclosing_method.class;

	// SAFETY: We know that the array has a length of 3
	unsafe {
		array_instance.store_unchecked(0, Reference::mirror(enclosing_class.mirror()));
	}

	let Some(enclosing_method) = enclosing_method.method else {
		// There is no immediate enclosing method
		return Reference::object_array(array_instance);
	};

	unsafe {
		array_instance.store_unchecked(
			1,
			Reference::class(StringInterner::intern(enclosing_method.name)),
		);

		array_instance.store_unchecked(
			2,
			Reference::class(StringInterner::intern(enclosing_method.descriptor_sym())),
		);
	}

	Reference::object_array(array_instance)
}
pub fn getDeclaringClass0(
	env: JniEnv,
	this: Reference, // java.lang.Class
) -> Reference /* Class<?> */
{
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	let Some(mirror) = handle_exception!(Reference::null(), thread, ensure_class_mirror(this))
	else {
		return Reference::null();
	};

	let target_class = mirror.target_class();
	let target_class_descriptor = target_class.unwrap_class_instance();
	let Some(inner_classes) = target_class_descriptor.inner_classes() else {
		// No InnerClasses attribute
		return Reference::null();
	};

	let mut declaring_class = Reference::null();
	for inner_class in inner_classes {
		if inner_class.inner_class != target_class.name() {
			continue;
		}

		match inner_class.outer_class {
			Some(outer) => {
				let outer: ClassPtr =
					handle_exception!(Reference::null(), thread, target_class.loader().load(outer));

				if outer.is_array() {
					throw_and_return_null!(thread, IncompatibleClassChangeError);
				}

				declaring_class = Reference::mirror(outer.mirror());
			},
			None => {
				let Some(enclosing_method) = target_class_descriptor.enclosing_method else {
					// Class has no immediate enclosing method/class information
					return Reference::null();
				};

				declaring_class = Reference::mirror(enclosing_method.class.mirror());
			},
		}

		break;
	}

	// TODO: need to verify that outer class actually declared the inner class
	declaring_class
}

pub fn getSimpleBinaryName0(
	env: JniEnv,
	this: Reference, // java.lang.Class
) -> Reference /* String */
{
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	let Some(mirror) = handle_exception!(Reference::null(), thread, ensure_class_mirror(this))
	else {
		return Reference::null();
	};

	let target_class = mirror.target_class();
	let target_class_descriptor = target_class.unwrap_class_instance();
	let Some(inner_classes) = target_class_descriptor.inner_classes() else {
		// No InnerClasses attribute
		return Reference::null();
	};

	for inner_class in inner_classes {
		if inner_class.inner_class != target_class.name() {
			continue;
		}

		if let Some(name) = inner_class.inner_class_name {
			return Reference::class(StringInterner::intern(name));
		}

		break;
	}

	Reference::null()
}
pub fn getProtectionDomain0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
) -> Reference /* java.security.ProtectionDomain */
{
	unimplemented!("Class#getProtectionDomain0");
}
pub fn getPrimitiveClass(
	env: JniEnv,
	_class: ClassPtr,
	name: Reference, // String
) -> Reference /* Class */
{
	let string_class = name.extract_class();
	let name_string = classes::java::lang::String::extract(string_class);

	for (name, ty) in crate::globals::PRIMITIVE_TYPE_NAMES_TO_FIELD_TYPES {
		if &name_string == name {
			return crate::globals::mirrors::primitive_mirror_for(ty);
		}
	}

	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	throw_and_return_null!(thread, ClassNotFoundException, "{name_string}");
}
pub fn getGenericSignature0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
) -> Reference /* String */
{
	unimplemented!("Class#getGenericSignature0");
}
pub fn getRawAnnotations(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
) -> Reference /* byte[] */
{
	unimplemented!("Class#getRawAnnotations");
}
pub fn getRawTypeAnnotations(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
) -> Reference /* byte[] */ {
	unimplemented!("Class#getRawTypeAnnotations");
}
pub fn getConstantPool(
	_env: JniEnv,
	this: Reference, // java.lang.Class
) -> Reference /* ConstantPool */
{
	let thread = unsafe { &*JavaThread::for_env(_env.raw()) };

	let this = this.extract_mirror();

	// not applicable for primitives or arrays
	if this.is_primitive() || this.is_array() {
		return Reference::null();
	}

	let constant_pool =
		match classes::jdk::internal::reflect::ConstantPool::new(this.target_class(), thread) {
			Throws::Ok(cp) => cp,
			Throws::Exception(e) => {
				e.throw(thread);
				return Reference::null();
			},
		};

	Reference::class(constant_pool)
}
pub fn getDeclaredFields0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
	_public_only: jboolean,
) -> Reference /* Field[] */ {
	unimplemented!("Class#getDeclaredFields0");
}
pub fn getDeclaredMethods0(
	env: JniEnv,
	this: Reference, // java.lang.Class
	public_only: jboolean,
) -> Reference /* Method[] */ {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let mirror = this.extract_mirror();

	// Primitive and array mirrors are excluded
	if mirror.is_array() || mirror.is_primitive() {
		let empty_array =
			match ObjectArrayInstance::new(0, globals::classes::java_lang_reflect_Method()) {
				Throws::Ok(array) => array,
				Throws::Exception(e) => {
					e.throw(thread);
					return Reference::null();
				},
			};

		return Reference::object_array(empty_array);
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
			return Reference::null();
		},
	};

	for (index, method) in methods.iter().enumerate() {
		let reflect_method = match classes::java::lang::reflect::Method::new(method) {
			Throws::Ok(method) => method,
			Throws::Exception(e) => {
				e.throw(thread);
				return Reference::null();
			},
		};

		// SAFETY: We can't go outside the bounds, `ret` was initialized to the right size
		unsafe {
			ret.store_unchecked(index, Reference::class(reflect_method));
		}
	}

	Reference::object_array(ret)
}
pub fn getDeclaredConstructors0(
	env: JniEnv,
	this: Reference, // java.lang.Class
	public_only: jboolean,
) -> Reference /* Constructor<T>[] */ {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let this_mirror = this.extract_mirror();

	// Not applicable for primitive and array mirrors
	if this_mirror.is_primitive() || this_mirror.is_array() {
		let ret = handle_exception!(
			Reference::null(),
			thread,
			ObjectArrayInstance::new(0, globals::classes::java_lang_reflect_Constructor())
		);
		return Reference::object_array(ret);
	}

	let target = this_mirror.target_class();
	let constructors = target
		.vtable()
		.iter_local()
		.filter_map(|method| {
			if !method.is_public() && public_only {
				return None;
			}

			if method.name == sym!(object_initializer_name) {
				Some(method)
			} else {
				None
			}
		})
		.collect::<Vec<&'static Method>>();

	let ret: ObjectArrayInstanceRef = handle_exception!(
		Reference::null(),
		thread,
		ObjectArrayInstance::new(
			constructors.len() as jint,
			globals::classes::java_lang_reflect_Constructor()
		)
	);

	for (i, constructor) in constructors.into_iter().enumerate() {
		let constructor: ClassInstanceRef = handle_exception!(
			Reference::null(),
			thread,
			classes::java::lang::reflect::Constructor::new(constructor)
		);

		// SAFETY: The array is known to have the correct length
		unsafe { ret.store_unchecked(i, Reference::class(constructor)) };
	}

	Reference::object_array(ret)
}
pub fn getDeclaredClasses0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
) -> Reference /* Class<?>[] */
{
	unimplemented!("Class#getDeclaredClasses0");
}

pub fn getRecordComponents0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
) -> Reference /* RecordComponent[] */
{
	unimplemented!("Class#getRecordComponents0");
}
pub fn isRecord0(_env: JniEnv, _this: Reference /* java.lang.Class */) -> jboolean {
	unimplemented!("Class#isRecord0");
}

// TODO: https://github.com/openjdk/jdk/blob/19373b2ff0cd795afa262c17dcb3388fd6a5be59/src/hotspot/share/classfile/javaAssertions.cpp#L195
#[allow(clippy::unnecessary_wraps, clippy::no_effect_underscore_binding)]
pub fn desiredAssertionStatus0(
	_env: JniEnv,
	_class: ClassPtr,
	clazz: Reference, // java/lang/Class
) -> jboolean {
	let mirror = clazz.extract_mirror();
	let _name = &mirror.target_class().name();

	false
}

pub fn getNestHost0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
) -> Reference /* java/lang/Class */
{
	unimplemented!("Class#getNestHost0");
}

pub fn getNestMembers0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
) -> Reference /* Class<?>[] */
{
	unimplemented!("Class#getNestMembers0");
}

pub fn isHidden(_env: JniEnv, this: Reference /* java.lang.Class */) -> jboolean {
	let mirror = this.extract_mirror();
	mirror.target_class().is_hidden()
}

pub fn getPermittedSubclasses0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
) -> Reference /* Class<?>[] */ {
	unimplemented!("Class#getPermittedSubclasses0");
}

pub fn getClassFileVersion0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
) -> jint {
	unimplemented!("Class#getClassFileVersion0");
}
