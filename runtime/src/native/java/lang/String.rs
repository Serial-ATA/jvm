use crate::objects::reference::Reference;

use jni::env::JniEnv;

include_generated!("native/java/lang/def/String.definitions.rs");
include_generated!("native/java/lang/def/String.constants.rs");

pub fn intern(_env: JniEnv, _this: Reference /* java.lang.String */) -> Reference /* java.lang.String */
{
	unimplemented!("java.lang.String#intern")
}
