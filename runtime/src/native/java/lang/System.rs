use crate::native::NativeReturn;
use crate::stack::local_stack::LocalStack;

use std::time::{SystemTime, UNIX_EPOCH};

use common::int_types::s8;
use instructions::Operand;

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
	let time_nanos = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("current system time should not be before the UNIX epoch")
		.as_nanos();

	Some(Operand::Long(time_nanos as s8))
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
