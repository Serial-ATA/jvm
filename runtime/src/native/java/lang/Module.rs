use crate::objects::class::Class;
use crate::objects::reference::Reference;

use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::jint;
use common::int_types::{s4, s8};
use common::traits::PtrType;
use instructions::Operand;

include_generated!("native/java/lang/def/Module.definitions.rs");

pub fn defineModule0(
	_: NonNull<JniEnv>,
	_class: &'static Class,
	_module: Reference, // java.lang.Module
	_is_open: bool,
	_version: Reference,  // java.lang.String
	_location: Reference, // java.lang.String
	_pns: Reference,      // java.lang.Object[]
) {
	unimplemented!("java.lang.Module#defineModule0");
}

pub fn addReads0(
	_: NonNull<JniEnv>,
	_class: &'static Class,
	_from: Reference, // java.lang.Module
	_to: Reference,   // java.lang.Module
) {
	unimplemented!("java.lang.Module#addReads0");
}

pub fn addExports0(
	_: NonNull<JniEnv>,
	_class: &'static Class,
	_from: Reference, // java.lang.Module
	_pn: Reference,   // java.lang.String
	_to: Reference,   // java.lang.Module
) {
	unimplemented!("java.lang.Module#addExports0");
}

pub fn addExportsToAll0(
	_: NonNull<JniEnv>,
	_class: &'static Class,
	_from: Reference, // java.lang.Module
	_pn: Reference,   // java.lang.String
) {
	unimplemented!("java.lang.Module#addExportsToAll0");
}

pub fn addExportsToAllUnnamed0(
	_: NonNull<JniEnv>,
	_class: &'static Class,
	_from: Reference, // java.lang.Module
	_pn: Reference,   // java.lang.String
) {
	unimplemented!("java.lang.Module#addExportsToAllUnnamed0");
}
