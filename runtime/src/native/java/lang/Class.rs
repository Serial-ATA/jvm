use crate::include_generated;
use crate::native::jni::{safe_classref_from_jclass, IntoJni};
use crate::objects::array::ArrayInstance;
use crate::objects::class::Class;
use crate::objects::instance::Instance;
use crate::objects::reference::{ArrayInstanceRef, MirrorInstanceRef, Reference};
use crate::string_interner::StringInterner;
use crate::thread::exceptions::{
	handle_exception, throw, throw_and_return_null, throw_with_ret, Throws,
};
use crate::thread::JavaThread;

use std::sync::Arc;

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint};
use common::traits::PtrType;
use instructions::Operand;
use symbols::sym;

include_generated!("native/java/lang/def/Class.registerNatives.rs");
include_generated!("native/java/lang/def/Class.definitions.rs");

/// Ensure that the mirror instance is not primitive or an array
fn ensure_class_mirror(this: Reference) -> Throws<Option<MirrorInstanceRef>> {
	if this.is_null() {
		throw!(@DEFER NullPointerException);
	}

	let mirror_ref = this.extract_mirror();
	let mirror = mirror_ref.get();
	if mirror.is_primitive() || mirror.is_array() {
		return Throws::Ok(None);
	}

	Throws::Ok(Some(mirror_ref))
}

// throws ClassNotFoundException
pub fn forName0(
	_env: JniEnv,
	_class: &'static Class,
	_name: Reference, // java.lang.String
	_initialize: jboolean,
	_loader: Reference, // java.lang.ClassLoader
	_caller: Reference, // java.lang.Class
) -> Reference /* java.lang.Class */ {
	unimplemented!("Class#forName0");
}

pub fn isInstance(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
	_obj: Reference,  // java.lang.Object
) -> jboolean {
	unimplemented!("Class#isInstance");
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
	this.extract_mirror().get().is_array()
}
pub fn isPrimitive(_env: JniEnv, this: Reference /* java.lang.Class */) -> jboolean {
	this.extract_mirror().get().is_primitive()
}

pub fn initClassName(
	_env: JniEnv,
	this: Reference, // java.lang.Class
) -> Reference {
	let this_mirror = this.extract_mirror();
	let this_mirror_target = this_mirror.get().target_class();
	let this_name = this_mirror_target.name;
	let name_string = StringInterner::intern_symbol(this_name);

	this_mirror.get_mut().put_field_value0(
		crate::globals::fields::java_lang_Class::name_field_offset(),
		Operand::Reference(Reference::class(Arc::clone(&name_string))),
	);

	Reference::class(name_string)
}

pub fn getSuperclass(
	env: JniEnv,
	this: Reference, // java.lang.Class
) -> Reference /* Class<? super T> */
{
	let target_class = this.extract_target_class();
	let Some(super_class_raw) = env.get_super_class(target_class.into_jni_safe()) else {
		return Reference::null();
	};

	let super_class = safe_classref_from_jclass(super_class_raw);
	Reference::mirror(super_class.mirror())
}
pub fn getInterfaces0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
) -> Reference /* Class<?>[] */
{
	unimplemented!("Class#getInterfaces0");
}
pub fn getModifiers(_env: JniEnv, _this: Reference /* java.lang.Class */) -> jint {
	unimplemented!("Class#getModifiers");
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
	let Some(mirror) =
		handle_exception!(Reference::null(), thread, ensure_class_mirror(this.clone()))
	else {
		return Reference::null();
	};

	let array = ArrayInstance::new_reference(3, crate::globals::classes::java_lang_Object());
	let array_instance: ArrayInstanceRef = handle_exception!(Reference::null(), thread, array);

	let target_class = mirror.get().target_class();

	let Some(enclosing_method) = target_class.unwrap_class_instance().enclosing_method else {
		// Class has no immediate enclosing method/class information
		return Reference::null();
	};

	let enclosing_class = enclosing_method.class;

	// SAFETY: We know that the array has a length of 3
	unsafe {
		array_instance.get_mut().store_unchecked(
			0,
			Operand::Reference(Reference::mirror(enclosing_class.mirror())),
		);
	}

	let Some(enclosing_method) = enclosing_method.method else {
		// There is no immediate enclosing method
		return Reference::array(array_instance);
	};

	unsafe {
		array_instance.get_mut().store_unchecked(
			1,
			Operand::Reference(Reference::class(StringInterner::intern_symbol(
				enclosing_method.name,
			))),
		);

		array_instance.get_mut().store_unchecked(
			2,
			Operand::Reference(Reference::class(StringInterner::intern_symbol(
				enclosing_method.descriptor_sym,
			))),
		);
	}

	Reference::array(array_instance)
}
pub fn getDeclaringClass0(
	env: JniEnv,
	this: Reference, // java.lang.Class
) -> Reference /* Class<?> */
{
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	let Some(mirror) =
		handle_exception!(Reference::null(), thread, ensure_class_mirror(this.clone()))
	else {
		return Reference::null();
	};

	let target_class = mirror.get().target_class();
	let target_class_descriptor = target_class.unwrap_class_instance();
	let Some(inner_classes) = target_class_descriptor.inner_classes() else {
		// No InnerClasses attribute
		return Reference::null();
	};

	let mut declaring_class = Reference::null();
	for inner_class in inner_classes {
		if inner_class.inner_class != target_class.name {
			continue;
		}

		match inner_class.outer_class {
			Some(outer) => {
				let outer: &'static Class =
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
	let Some(mirror) =
		handle_exception!(Reference::null(), thread, ensure_class_mirror(this.clone()))
	else {
		return Reference::null();
	};

	let target_class = mirror.get().target_class();
	let target_class_descriptor = target_class.unwrap_class_instance();
	let Some(inner_classes) = target_class_descriptor.inner_classes() else {
		// No InnerClasses attribute
		return Reference::null();
	};

	for inner_class in inner_classes {
		if inner_class.inner_class != target_class.name {
			continue;
		}

		if let Some(name) = inner_class.inner_class_name {
			return Reference::class(StringInterner::intern_symbol(name));
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
	_env: JniEnv,
	_class: &'static Class,
	name: Reference, // String
) -> Reference /* Class */
{
	let string_class = name.extract_class();
	let name_string = StringInterner::rust_string_from_java_string(string_class);

	for (name, ty) in crate::globals::TYPES {
		if &name_string == name {
			return crate::globals::mirrors::primitive_mirror_for(ty);
		}
	}

	// TODO
	panic!("ClassNotFoundException")
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
	_this: Reference, // java.lang.Class
) -> Reference /* ConstantPool */
{
	unimplemented!("Class#getConstantPool");
}
pub fn getDeclaredFields0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
	_public_only: jboolean,
) -> Reference /* Field[] */ {
	unimplemented!("Class#getDeclaredFields0");
}
pub fn getDeclaredMethods0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
	_public_only: jboolean,
) -> Reference /* Method[] */ {
	unimplemented!("Class#getDeclaredMethods0");
}
pub fn getDeclaredConstructors0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
	_public_only: jboolean,
) -> Reference /* Constructor<T>[] */ {
	unimplemented!("Class#getDeclaredConstructors0");
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
	_class: &'static Class,
	clazz: Reference, // java/lang/Class
) -> jboolean {
	let mirror = clazz.extract_mirror();
	let _name = &mirror.get().target_class().name;

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

pub fn isHidden(_env: JniEnv, _this: Reference /* java.lang.Class */) -> jboolean {
	unimplemented!("Class#isHidden");
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

pub fn getClassAccessFlagsRaw0(
	_env: JniEnv,
	_this: Reference, // java.lang.Class
) -> jint {
	unimplemented!("Class#getClassAccessFlagsRaw0");
}
