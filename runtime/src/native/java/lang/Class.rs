use crate::native::{JNIEnv, NativeReturn};
use crate::stack::local_stack::LocalStack;
use crate::string_interner::StringInterner;

use crate::classpath::classloader::ClassLoader;
use crate::heap::mirror::MirrorInstance;
use crate::reference::Reference;

use common::traits::PtrType;
use instructions::Operand;

include!("def/Class.registerNatives");

pub fn forName0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#forName0");
}

pub fn isInstance(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#isInstance");
}
pub fn isAssignableFrom(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#isAssignableFrom");
}
pub fn isInterface(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#isInterface");
}
pub fn isArray(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#isArray");
}
pub fn isPrimitive(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#isPrimitive");
}

pub fn initClassName(env: JNIEnv, locals: LocalStack) -> NativeReturn {
	let this = locals[0].expect_reference().extract_mirror();
	let this_mirror_target = this.get().expect_class(); // TODO: Support primitive mirrors
	let this_name = &this_mirror_target.get().name;
	let name_string = StringInterner::intern_string(this_name, env.current_thread);

	Some(Operand::Reference(Reference::Class(name_string)))
}

pub fn getSuperclass(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getSuperclass");
}
pub fn getInterfaces0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getInterfaces0");
}
pub fn getModifiers(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getModifiers");
}
pub fn getSigners(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getSigners");
}
pub fn setSigners(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#setSigners");
}
pub fn getEnclosingMethod0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getEnclosingMethod0");
}
pub fn getDeclaringClass0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getDeclaringClass0");
}
pub fn getSimpleBinaryName0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getSimpleBinaryName0");
}
pub fn getProtectionDomain0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getProtectionDomain0");
}
pub fn getPrimitiveClass(_: JNIEnv, locals: LocalStack) -> NativeReturn {
	let name = locals[0].expect_reference();
	let string_class = name.extract_class();
	let name_string = StringInterner::rust_string_from_java_string(string_class);

	for (name, ty) in crate::globals::TYPES {
		if &name_string == name {
			let java_lang_class = ClassLoader::lookup_class(b"java/lang/Class")
				.expect("java.lang.Class should be loaded");
			return Some(Operand::Reference(Reference::Mirror(
				MirrorInstance::new_primitive(java_lang_class, ty.clone()),
			)));
		}
	}

	// TODO
	panic!("ClassNotFoundException")
}
pub fn getGenericSignature0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getGenericSignature0");
}
pub fn getRawAnnotations(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getRawAnnotations");
}
pub fn getRawTypeAnnotations(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getRawTypeAnnotations");
}
pub fn getConstantPool(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getConstantPool");
}
pub fn getDeclaredFields0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getDeclaredFields0");
}
pub fn getDeclaredMethods0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getDeclaredMethods0");
}
pub fn getDeclaredConstructors0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getDeclaredConstructors0");
}
pub fn getDeclaredClasses0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getDeclaredClasses0");
}
pub fn getRecordComponents0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getRecordComponents0");
}

pub fn isRecord0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#isRecord0");
}

// TODO: https://github.com/openjdk/jdk/blob/19373b2ff0cd795afa262c17dcb3388fd6a5be59/src/hotspot/share/classfile/javaAssertions.cpp#L195
#[allow(clippy::unnecessary_wraps, clippy::no_effect_underscore_binding)]
pub fn desiredAssertionStatus0(_: JNIEnv, locals: LocalStack) -> NativeReturn {
	let operand = locals[0].clone();
	let reference = operand.expect_reference();
	let mirror = reference.extract_mirror();

	let _name = &mirror.get().expect_class().get().name;

	Some(Operand::Int(i32::from(false)))
}

pub fn getNestHost0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getNestHost0");
}

pub fn getNestMembers0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getNestMembers0");
}

pub fn isHidden(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#isHidden");
}

pub fn getPermittedSubclasses0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("Class#getPermittedSubclasses0");
}
