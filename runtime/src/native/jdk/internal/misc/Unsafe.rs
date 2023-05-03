use crate::native::{JNIEnv, NativeReturn};
use crate::reference::Reference;
use crate::stack::local_stack::LocalStack;
use crate::string_interner::StringInterner;

use common::int_types::{s4, s8};
use common::traits::PtrType;
use instructions::Operand;

include!("def/Unsafe.registerNatives");

pub fn getInt(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getInt")
}
pub fn putInt(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putInt")
}
pub fn getReference(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getReference")
}
pub fn putReference(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putReference")
}
pub fn getBoolean(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getBoolean")
}
pub fn putBoolean(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putBoolean")
}
pub fn getByte(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getByte")
}
pub fn putByte(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putByte")
}
pub fn getShort(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#  getShort")
}
pub fn putShort(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putShort")
}
pub fn getChar(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getChar")
}
pub fn putChar(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putChar")
}
pub fn getLong(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getLong")
}
pub fn putLong(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putLong")
}
pub fn getFloat(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#  getFloat")
}
pub fn putFloat(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putFloat")
}
pub fn getDouble(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe# getDouble")
}
pub fn putDouble(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putDouble")
}

pub fn getUncompressedObject(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getUncompressedObject")
}

pub fn writeback0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#writeback0")
}
pub fn writebackPreSync0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#writebackPreSync0")
}
pub fn writebackPostSync0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#writebackPostSync0")
}

pub fn defineClass0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#defineClass0")
}

pub fn throwException(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#throwException")
}

pub fn compareAndSetReference(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndSetReference")
}
pub fn compareAndExchangeReference(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndExchangeReference")
}

pub fn compareAndSetInt(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndSetInt")
}
pub fn compareAndExchangeInt(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndExchangeInt")
}

pub fn compareAndSetLong(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndSetLong")
}
pub fn compareAndExchangeLong(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndExchangeLong")
}

pub fn getReferenceVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getReferenceVolatile")
}
pub fn putReferenceVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putReferenceVolatile")
}
pub fn getIntVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe# getIntVolatile")
}
pub fn putIntVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putIntVolatile")
}
pub fn getBooleanVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getBooleanVolatile")
}
pub fn putBooleanVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putBooleanVolatile")
}
pub fn getByteVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getByteVolatile")
}
pub fn putByteVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putByteVolatile")
}
pub fn getShortVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#  getShortVolatile")
}
pub fn putShortVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putShortVolatile")
}
pub fn getCharVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getCharVolatile")
}
pub fn putCharVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putCharVolatile")
}
pub fn getLongVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getLongVolatile")
}
pub fn putLongVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putLongVolatile")
}
pub fn getFloatVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#  getFloatVolatile")
}
pub fn putFloatVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putFloatVolatile")
}
pub fn getDoubleVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe# getDoubleVolatile")
}
pub fn putDoubleVolatile(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putDoubleVolatile")
}

pub fn unpark(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#unpark")
}
pub fn park(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#park")
}

pub fn fullFence(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#fullFence")
}

pub fn allocateMemory0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#allocateMemory0")
}
pub fn reallocateMemory0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#reallocateMemory0")
}
pub fn freeMemory0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#freeMemory0")
}
pub fn setMemory0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#setMemory0")
}
pub fn copyMemory0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#copyMemory0")
}
pub fn copySwapMemory0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#copySwapMemory0")
}
pub fn objectFieldOffset0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#objectFieldOffset0")
}
pub fn objectFieldOffset1(_: JNIEnv, locals: LocalStack) -> NativeReturn {
	let class = locals[1].expect_reference().extract_mirror();
	let name = locals[2].expect_reference();

	let name_str = StringInterner::rust_string_from_java_string(name.extract_class());
	let classref = class.get().expect_class();
	for (offset, field) in classref.unwrap_class_instance().fields.iter().enumerate() {
		if field.name == name_str.as_bytes() {
			return Some(Operand::Long(offset as s8));
		}
	}

	// TODO
	panic!("InternalError")
}
pub fn staticFieldOffset0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#staticFieldOffset0")
}
pub fn staticFieldBase0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#staticFieldBase0")
}
pub fn shouldBeInitialized0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#shouldBeInitialized0")
}
pub fn ensureClassInitialized0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#ensureClassInitialized0")
}
pub fn arrayBaseOffset0(_: JNIEnv, locals: LocalStack) -> NativeReturn {
	let reference = locals[1].expect_reference();
	let mirror = reference.extract_mirror();
	// TODO: InvalidClassException
	let _array = mirror.get().expect_class().unwrap_array_instance();

	// TODO: We don't do byte packing like Hotspot
	Some(Operand::Int(0))
}
pub fn arrayIndexScale0(_: JNIEnv, locals: LocalStack) -> NativeReturn {
	let reference = locals[1].expect_reference();
	let mirror = reference.extract_mirror();
	// TODO: InvalidClassException
	let _array = mirror.get().expect_class().unwrap_array_instance();

	// TODO: We don't do byte packing like Hotspot
	Some(Operand::Int(
		core::mem::size_of::<Operand<Reference>>() as s4
	))
}
pub fn getLoadAverage0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getLoadAverage0")
}
