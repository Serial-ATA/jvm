use crate::native::{JNIEnv, NativeReturn};
use crate::reference::Reference;
use crate::stack::local_stack::LocalStack;

use common::traits::PtrType;
use instructions::Operand;

#[expect(clippy::match_same_arms)]
pub fn getCallerClass(env: JNIEnv, _: LocalStack) -> NativeReturn {
	for (n, frame) in env
		.current_thread
		.get()
		.frame_stack
		.iter()
		.rev()
		.enumerate()
	{
		let method = frame.method();
		match n {
			// TODO:
			//   https://github.com/openjdk/jdk/blob/6a44120a16d0f06b4ed9f0ebf6b0919da7070287/src/hotspot/share/prims/jvm.cpp#L742-L744
			//   https://github.com/openjdk/jdk/blob/6a44120a16d0f06b4ed9f0ebf6b0919da7070287/src/java.base/share/classes/java/lang/invoke/MethodHandleNatives.java#L117
			0 => {},
			// TODO:
			//   https://github.com/openjdk/jdk/blob/6a44120a16d0f06b4ed9f0ebf6b0919da7070287/src/hotspot/share/prims/jvm.cpp#L748-L750
			//   https://github.com/openjdk/jdk/blob/6a44120a16d0f06b4ed9f0ebf6b0919da7070287/src/java.base/share/classes/java/lang/invoke/MethodHandleNatives.java#L117
			1 => {},
			_ => {
				// TODO: https://github.com/openjdk/jdk/blob/6a44120a16d0f06b4ed9f0ebf6b0919da7070287/src/hotspot/share/oops/method.cpp#L1378
				return Some(Operand::Reference(Reference::Mirror(
					method.class.get_mirror(),
				)));
			},
		}
	}

	Some(Operand::Reference(Reference::Null))
}
pub fn getClassAccessFlags(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.reflect.Reflection#getClassAccessFlags")
}

pub fn areNestMates(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.reflect.Reflection#areNestMates")
}
