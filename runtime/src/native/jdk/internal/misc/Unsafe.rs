use crate::native::NativeReturn;
use crate::stack::local_stack::LocalStack;

include!("def/Unsafe.registerNatives");

pub fn getInt(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getInt")
}
pub fn putInt(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putInt")
}
pub fn getReference(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getReference")
}
pub fn putReference(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putReference")
}
pub fn getBoolean(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getBoolean")
}
pub fn putBoolean(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putBoolean")
}
pub fn getByte(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getByte")
}
pub fn putByte(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putByte")
}
pub fn getShort(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#  getShort")
}
pub fn putShort(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putShort")
}
pub fn getChar(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getChar")
}
pub fn putChar(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putChar")
}
pub fn getLong(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getLong")
}
pub fn putLong(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putLong")
}
pub fn getFloat(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#  getFloat")
}
pub fn putFloat(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putFloat")
}
pub fn getDouble(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe# getDouble")
}
pub fn putDouble(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putDouble")
}

pub fn getUncompressedObject(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getUncompressedObject")
}

pub fn writeback0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#writeback0")
}
pub fn writebackPreSync0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#writebackPreSync0")
}
pub fn writebackPostSync0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#writebackPostSync0")
}

pub fn defineClass0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#defineClass0")
}

pub fn throwException(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#throwException")
}

pub fn compareAndSetReference(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndSetReference")
}
pub fn compareAndExchangeReference(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndExchangeReference")
}

pub fn compareAndSetInt(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndSetInt")
}
pub fn compareAndExchangeInt(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndExchangeInt")
}

pub fn compareAndSetLong(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndSetLong")
}
pub fn compareAndExchangeLong(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#compareAndExchangeLong")
}

pub fn getReferenceVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getReferenceVolatile")
}
pub fn putReferenceVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putReferenceVolatile")
}
pub fn getIntVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe# getIntVolatile")
}
pub fn putIntVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putIntVolatile")
}
pub fn getBooleanVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getBooleanVolatile")
}
pub fn putBooleanVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putBooleanVolatile")
}
pub fn getByteVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getByteVolatile")
}
pub fn putByteVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putByteVolatile")
}
pub fn getShortVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#  getShortVolatile")
}
pub fn putShortVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putShortVolatile")
}
pub fn getCharVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getCharVolatile")
}
pub fn putCharVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putCharVolatile")
}
pub fn getLongVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getLongVolatile")
}
pub fn putLongVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putLongVolatile")
}
pub fn getFloatVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#  getFloatVolatile")
}
pub fn putFloatVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putFloatVolatile")
}
pub fn getDoubleVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe# getDoubleVolatile")
}
pub fn putDoubleVolatile(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#putDoubleVolatile")
}

pub fn unpark(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#unpark")
}
pub fn park(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#park")
}

pub fn fullFence(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#fullFence")
}

pub fn allocateMemory0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#allocateMemory0")
}
pub fn reallocateMemory0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#reallocateMemory0")
}
pub fn freeMemory0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#freeMemory0")
}
pub fn setMemory0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#setMemory0")
}
pub fn copyMemory0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#copyMemory0")
}
pub fn copySwapMemory0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#copySwapMemory0")
}
pub fn objectFieldOffset0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#objectFieldOffset0")
}
pub fn objectFieldOffset1(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#objectFieldOffset1")
}
pub fn staticFieldOffset0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#staticFieldOffset0")
}
pub fn staticFieldBase0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#staticFieldBase0")
}
pub fn shouldBeInitialized0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#shouldBeInitialized0")
}
pub fn ensureClassInitialized0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#ensureClassInitialized0")
}
pub fn arrayBaseOffset0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#arrayBaseOffset0")
}
pub fn arrayIndexScale0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#arrayIndexScale0")
}
pub fn getLoadAverage0(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.Unsafe#getLoadAverage0")
}
