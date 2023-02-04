use crate::native::{JNIEnv, NativeReturn};
use crate::stack::local_stack::LocalStack;

use std::time::{SystemTime, UNIX_EPOCH};

use common::int_types::s8;
use common::traits::PtrType;
use instructions::Operand;

include!("def/System.registerNatives");

pub fn setIn0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("System#setIn0")
}
pub fn setOut0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("System#setOut0")
}
pub fn setErr0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("System#setErr0")
}

pub fn currentTimeMillis(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("System#currentTimeMillis")
}
pub fn nanoTime(_: JNIEnv, _: LocalStack) -> NativeReturn {
	let time_nanos = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("current system time should not be before the UNIX epoch")
		.as_nanos();

	Some(Operand::Long(time_nanos as s8))
}

pub fn arraycopy(_: JNIEnv, locals: LocalStack) -> NativeReturn {
	let src = locals[0].expect_reference();
	let src_pos = locals[1].expect_int();
	let dest = locals[2].expect_reference();
	let dest_pos = locals[3].expect_int();
	let length = locals[4].expect_int();

	if src.is_null() || dest.is_null() {
		// TODO
		panic!("NullPointerException")
	}

	let src_array = src.extract_array();
	let dest_array = dest.extract_array();

	if src_pos < 0
		|| dest_pos < 0
		|| length < 0
		|| src_pos + length > src_array.get().elements.element_count() as i32
		|| dest_pos + length > dest_array.get().elements.element_count() as i32
	{
		// TODO
		panic!("IndexOutOfBoundsException")
	}

	if src_array.as_raw() == dest_array.as_raw() {
		unimplemented!("arraycopy on same instance")
	}

	src_array.get().elements.copy_into(
		src_pos as usize,
		&mut dest_array.get_mut().elements,
		dest_pos as usize,
		length as usize,
	);

	None
}

pub fn identityHashCode(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("System#identityHashCode")
}

pub fn mapLibraryName(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("System#mapLibraryName")
}
