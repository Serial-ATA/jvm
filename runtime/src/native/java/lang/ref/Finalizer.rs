use crate::objects::class::Class;
use crate::objects::reference::Reference;

use jni::env::JniEnv;
use jni::sys::jboolean;

include_generated!("native/java/lang/ref/def/Finalizer.definitions.rs");

pub fn isFinalizationEnabled(_: JniEnv, _class: &'static Class) -> jboolean {
	false // finalization is deprecated anyway
}

pub fn reportComplete(
	_: JniEnv,
	_class: &'static Class,
	_finalizee: Reference, // java.lang.Object
) {
	unimplemented!("java.lang.ref.Finalizer#reportComplete")
}
