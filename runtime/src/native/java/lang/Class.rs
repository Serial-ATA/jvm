use crate::class_instance::Instance;
use crate::include_generated;
use crate::native::JniEnv;
use crate::objects::mirror::MirrorInstance;
use crate::reference::Reference;
use crate::string_interner::StringInterner;

use std::ptr::NonNull;
use std::sync::Arc;

use common::int_types::s4;
use common::traits::PtrType;
use instructions::Operand;
use symbols::sym;

include_generated!("native/java/lang/def/Class.registerNatives.rs");
include_generated!("native/java/lang/def/Class.definitions.rs");

// throws ClassNotFoundException
pub fn forName0(
	_env: NonNull<JniEnv>,
	_name: Reference, // java.lang.String
	_initialize: bool,
	_loader: Reference, // java.lang.ClassLoader
	_caller: Reference, // java.lang.Class
) -> Reference /* java.lang.Class */ {
	unimplemented!("Class#forName0");
}

pub fn isInstance(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
	_obj: Reference,  // java.lang.Object
) -> bool {
	unimplemented!("Class#isInstance");
}
pub fn isAssignableFrom(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
	_cls: Reference,  // java.lang.Class
) -> bool {
	unimplemented!("Class#isAssignableFrom");
}
pub fn isInterface(_env: NonNull<JniEnv>, _this: Reference /* java.lang.Class */) -> bool {
	unimplemented!("Class#isInterface");
}
pub fn isArray(_env: NonNull<JniEnv>, this: Reference /* java.lang.Class */) -> bool {
	this.extract_mirror().get().is_array()
}
pub fn isPrimitive(_env: NonNull<JniEnv>, this: Reference /* java.lang.Class */) -> bool {
	this.extract_mirror().get().is_primitive()
}

pub fn initClassName(
	_env: NonNull<JniEnv>,
	this: Reference, // java.lang.Class
) -> Reference {
	let this_mirror = this.extract_mirror();
	let this_mirror_target = this_mirror.get().expect_class(); // TODO: Support primitive mirrors
	let this_name = this_mirror_target.get().name;
	let name_string = StringInterner::intern_symbol(this_name);

	this_mirror.get_mut().put_field_value0(
		crate::globals::field_offsets::class_name_field_offset(),
		Operand::Reference(Reference::class(Arc::clone(&name_string))),
	);

	Reference::class(name_string)
}

pub fn getSuperclass(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* Class<? super T> */
{
	unimplemented!("Class#getSuperclass");
}
pub fn getInterfaces0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* Class<?>[] */
{
	unimplemented!("Class#getInterfaces0");
}
pub fn getModifiers(_env: NonNull<JniEnv>, _this: Reference /* java.lang.Class */) -> s4 {
	unimplemented!("Class#getModifiers");
}
pub fn getSigners(_env: NonNull<JniEnv>, _this: Reference /* java.lang.Class */) -> Reference /* Object[] */
{
	unimplemented!("Class#getSigners");
}
pub fn setSigners(
	_env: NonNull<JniEnv>,
	_this: Reference,    // java.lang.Class
	_signers: Reference, // Object[]
) {
	unimplemented!("Class#setSigners");
}
pub fn getEnclosingMethod0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* Object[] */
{
	unimplemented!("Class#getEnclosingMethod0");
}
pub fn getDeclaringClass0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* Class<?> */
{
	unimplemented!("Class#getDeclaringClass0");
}
pub fn getSimpleBinaryName0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* String */
{
	unimplemented!("Class#getSimpleBinaryName0");
}
pub fn getProtectionDomain0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* java.security.ProtectionDomain */
{
	unimplemented!("Class#getProtectionDomain0");
}
pub fn getPrimitiveClass(_env: NonNull<JniEnv>, name: Reference /* String */) -> Reference /* Class */
{
	let string_class = name.extract_class();
	let name_string = StringInterner::rust_string_from_java_string(string_class);

	for (name, ty) in crate::globals::TYPES {
		if &name_string == name {
			let java_lang_class = crate::globals::classes::java_lang_Class();
			return Reference::mirror(MirrorInstance::new_primitive(java_lang_class, ty.clone()));
		}
	}

	// TODO
	panic!("ClassNotFoundException")
}
pub fn getGenericSignature0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* String */
{
	unimplemented!("Class#getGenericSignature0");
}
pub fn getRawAnnotations(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* byte[] */
{
	unimplemented!("Class#getRawAnnotations");
}
pub fn getRawTypeAnnotations(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* byte[] */ {
	unimplemented!("Class#getRawTypeAnnotations");
}
pub fn getConstantPool(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* ConstantPool */
{
	unimplemented!("Class#getConstantPool");
}
pub fn getDeclaredFields0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
	_public_only: bool,
) -> Reference /* Field[] */ {
	unimplemented!("Class#getDeclaredFields0");
}
pub fn getDeclaredMethods0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
	_public_only: bool,
) -> Reference /* Method[] */ {
	unimplemented!("Class#getDeclaredMethods0");
}
pub fn getDeclaredConstructors0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
	_public_only: bool,
) -> Reference /* Constructor<T>[] */ {
	unimplemented!("Class#getDeclaredConstructors0");
}
pub fn getDeclaredClasses0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* Class<?>[] */
{
	unimplemented!("Class#getDeclaredClasses0");
}

pub fn getRecordComponents0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* RecordComponent[] */
{
	unimplemented!("Class#getRecordComponents0");
}
pub fn isRecord0(_env: NonNull<JniEnv>, _this: Reference /* java.lang.Class */) -> bool {
	unimplemented!("Class#isRecord0");
}

// TODO: https://github.com/openjdk/jdk/blob/19373b2ff0cd795afa262c17dcb3388fd6a5be59/src/hotspot/share/classfile/javaAssertions.cpp#L195
#[allow(clippy::unnecessary_wraps, clippy::no_effect_underscore_binding)]
pub fn desiredAssertionStatus0(
	_env: NonNull<JniEnv>,
	clazz: Reference, // java/lang/Class
) -> bool {
	let mirror = clazz.extract_mirror();
	let _name = &mirror.get().expect_class().get().name;

	false
}

pub fn getNestHost0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* java/lang/Class */
{
	unimplemented!("Class#getNestHost0");
}

pub fn getNestMembers0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* Class<?>[] */
{
	unimplemented!("Class#getNestMembers0");
}

pub fn isHidden(_env: NonNull<JniEnv>, _this: Reference /* java.lang.Class */) -> bool {
	unimplemented!("Class#isHidden");
}

pub fn getPermittedSubclasses0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
) -> Reference /* Class<?>[] */ {
	unimplemented!("Class#getPermittedSubclasses0");
}
