use crate::native::{JNIEnv, NativeReturn};
use crate::reference::Reference;
use crate::stack::local_stack::LocalStack;

use common::int_types::s4;
use common::traits::PtrType;
use instructions::Operand;

pub fn getClass(_: JNIEnv, locals: LocalStack) -> NativeReturn {
	let this = locals[0].expect_reference();
	Some(Operand::Reference(Reference::Mirror(
		this.extract_class_mirror(),
	)))
}

pub fn hashCode(_: JNIEnv, locals: LocalStack) -> NativeReturn {
	let this = locals[0].expect_reference();
	let hash_code = match this {
		Reference::Class(class) => class.as_raw() as s4,
		Reference::Array(array) => array.as_raw() as s4,
		Reference::Mirror(mirror) => mirror.as_raw() as s4,
		Reference::Null => 0,
		_ => unreachable!(),
	};

	Some(Operand::Int(hash_code))
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
