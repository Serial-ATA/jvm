use crate::objects::class::Class;
use crate::objects::reference::Reference;

use jni::env::JniEnv;

include_generated!("native/java/security/def/AccessController.definitions.rs");

pub fn getProtectionDomain(
	_env: JniEnv,
	_this_class: &'static Class,
	_class: Reference, // java.lang.Class
) -> Reference /* java.security.ProtectionDomain */ {
	unimplemented!("java.security.AccessController#getProtectionDomain");
}

pub fn ensureMaterializedForStackWalk(
	_env: JniEnv,
	_this_class: &'static Class,
	_class: Reference, // java.lang.Object
) {
	// Does nothing
}

pub fn getStackAccessControlContext(_env: JniEnv, _class: &'static Class) -> Reference /* java.security.AccessControlContext */
{
	// TODO: Actually implement this
	tracing::warn!(target: "java.security.AccessController#getStackAccessContext", "Assuming no privileged stack");
	Reference::null()
}

pub fn getInheritedAccessControlContext(_env: JniEnv, _class: &'static Class) -> Reference /* java.security.AccessControlContext */
{
	unimplemented!("java.security.AccessController#getInheritedAccessControlContext");
}
