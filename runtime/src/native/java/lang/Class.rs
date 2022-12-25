use crate::native::NativeReturn;
use crate::stack::local_stack::LocalStack;

use common::traits::PtrType;
use instructions::Operand;

include!("def/Class.registerNatives");

pub fn forName0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#forName0");
}

pub fn isInstance(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#isInstance");
}
pub fn isAssignableFrom(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#isAssignableFrom");
}
pub fn isInterface(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#isInterface");
}
pub fn isArray(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#isArray");
}
pub fn isPrimitive(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#isPrimitive");
}

pub fn initClassName(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#initClassName");
}

pub fn getSuperclass(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getSuperclass");
}
pub fn getInterfaces0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getInterfaces0");
}
pub fn getModifiers(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getModifiers");
}
pub fn getSigners(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getSigners");
}
pub fn setSigners(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#setSigners");
}
pub fn getEnclosingMethod0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getEnclosingMethod0");
}
pub fn getDeclaringClass0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getDeclaringClass0");
}
pub fn getSimpleBinaryName0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getSimpleBinaryName0");
}
pub fn getProtectionDomain0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getProtectionDomain0");
}
pub fn getPrimitiveClass(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getPrimitiveClass");
}
pub fn getGenericSignature0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getGenericSignature0");
}
pub fn getRawAnnotations(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getRawAnnotations");
}
pub fn getRawTypeAnnotations(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getRawTypeAnnotations");
}
pub fn getConstantPool(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getConstantPool");
}
pub fn getDeclaredFields0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getDeclaredFields0");
}
pub fn getDeclaredMethods0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getDeclaredMethods0");
}
pub fn getDeclaredConstructors0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getDeclaredConstructors0");
}
pub fn getDeclaredClasses0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getDeclaredClasses0");
}
pub fn getRecordComponents0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getRecordComponents0");
}

pub fn isRecord0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#isRecord0");
}

// TODO: https://github.com/openjdk/jdk/blob/19373b2ff0cd795afa262c17dcb3388fd6a5be59/src/hotspot/share/classfile/javaAssertions.cpp#L195
#[allow(clippy::unnecessary_wraps, clippy::no_effect_underscore_binding)]
pub fn desiredAssertionStatus0(locals: LocalStack) -> NativeReturn {
	let operand = locals[0].clone();
	let reference = operand.expect_reference();
	let class_instance = reference.extract_class();

	let class = &class_instance.get().class.get();
	let _name = &class.name;

	Some(Operand::Int(i32::from(false)))
}

pub fn getNestHost0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getNestHost0");
}

pub fn getNestMembers0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getNestMembers0");
}

pub fn isHidden(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#isHidden");
}

pub fn getPermittedSubclasses0(_: LocalStack) -> NativeReturn {
	unimplemented!("Class#getPermittedSubclasses0");
}
