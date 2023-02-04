use crate::native::{JNIEnv, NativeReturn};
use crate::reference::Reference;
use crate::stack::local_stack::LocalStack;

use instructions::Operand;

pub fn getClass(_: JNIEnv, locals: LocalStack) -> NativeReturn {
	let this = locals[0].expect_reference();
	Some(Operand::Reference(Reference::Mirror(
		this.extract_class_mirror(),
	)))
}

pub fn hashCode(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Object#hashCode")
}

pub fn clone(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Object#clone")
}

pub fn notify(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Object#notify")
}

pub fn notifyAll(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Object#notifyAll")
}

pub fn wait0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Object#wait0")
}
