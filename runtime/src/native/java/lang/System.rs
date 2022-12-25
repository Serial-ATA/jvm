use crate::native::NativeReturn;
use crate::stack::local_stack::LocalStack;

include!("def/System.registerNatives");

pub fn setIn0(_: LocalStack) -> NativeReturn {
	unimplemented!("System#setIn0")
}
pub fn setOut0(_: LocalStack) -> NativeReturn {
	unimplemented!("System#setOut0")
}
pub fn setErr0(_: LocalStack) -> NativeReturn {
	unimplemented!("System#setErr0")
}

pub fn currentTimeMillis(_: LocalStack) -> NativeReturn {
	unimplemented!("System#currentTimeMillis")
}
pub fn nanoTime(_: LocalStack) -> NativeReturn {
	unimplemented!("System#nanoTime")
}

pub fn arraycopy(_: LocalStack) -> NativeReturn {
	unimplemented!("System#arraycopy")
}

pub fn identityHashCode(_: LocalStack) -> NativeReturn {
	unimplemented!("System#identityHashCode")
}

pub fn mapLibraryName(_: LocalStack) -> NativeReturn {
	unimplemented!("System#mapLibraryName")
}
