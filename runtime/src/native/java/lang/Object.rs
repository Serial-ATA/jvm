use crate::native::NativeReturn;
use crate::stack::local_stack::LocalStack;

pub fn getClass(_: LocalStack) -> NativeReturn {
	unimplemented!("Object#getClass")
}

pub fn hashCode(_: LocalStack) -> NativeReturn {
	unimplemented!("Object#hashCode")
}

pub fn clone(_: LocalStack) -> NativeReturn {
	unimplemented!("Object#clone")
}

pub fn notify(_: LocalStack) -> NativeReturn {
	unimplemented!("Object#notify")
}

pub fn notifyAll(_: LocalStack) -> NativeReturn {
	unimplemented!("Object#notifyAll")
}

pub fn wait0(_: LocalStack) -> NativeReturn {
	unimplemented!("Object#wait0")
}
