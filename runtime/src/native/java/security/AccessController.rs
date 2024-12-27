use crate::objects::reference::Reference;

use std::ptr::NonNull;

use jni::env::JniEnv;

include_generated!("native/java/security/def/AccessController.definitions.rs");

pub fn getProtectionDomain(
	_env: NonNull<JniEnv>,
	_class: Reference, // java.lang.Class
) -> Reference /* java.security.ProtectionDomain */ {
	unimplemented!("java.security.AccessController#getProtectionDomain");
}

pub fn ensureMaterializedForStackWalk(
	_env: NonNull<JniEnv>,
	_class: Reference, // java.lang.Object
) {
	unimplemented!("java.security.AccessController#ensureMaterializedForStackWalk")
}

pub fn getStackAccessControlContext(_env: NonNull<JniEnv>) -> Reference /* java.security.AccessControlContext */
{
	// TODO: Actually implement this
	tracing::warn!(target: "java.security.AccessController#getStackAccessContext", "Assuming no privileged stack");
	Reference::null()
}

pub fn getInheritedAccessControlContext(_env: NonNull<JniEnv>) -> Reference /* java.security.AccessControlContext */
{
	unimplemented!("java.security.AccessController#getInheritedAccessControlContext");
}
