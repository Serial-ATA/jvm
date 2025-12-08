use crate::native::method::NativeMethodPtr;
use crate::objects::method::MethodEntryPoint;
use crate::symbols::sym;

use std::ffi::c_void;
use std::sync::Once;

/// Unlike other classes, `java.lang.Object` has a native implementation of `registerNatives`.
///
/// This is called very early in init, prior to any Java code being executed.
pub fn register_natives() {
	fn do_register_natives() {
		for method in crate::globals::classes::java_lang_Object().vtable() {
			if !method.is_native() {
				continue;
			}

			if method.name == sym!(getClass) {
				// Special case, defined in libjava to be linked later
				continue;
			}

			if method.name == sym!(hashCode) {
				method.set_entry_point(MethodEntryPoint::NativeMethod(NativeMethodPtr::External(
					crate::native::jvm::object::JVM_IHashCode as *const c_void,
				)));
				continue;
			}

			if method.name == sym!(wait0) {
				method.set_entry_point(MethodEntryPoint::NativeMethod(NativeMethodPtr::External(
					crate::native::jvm::object::JVM_MonitorWait as *const c_void,
				)));
				continue;
			}

			if method.name == sym!(notify) {
				method.set_entry_point(MethodEntryPoint::NativeMethod(NativeMethodPtr::External(
					crate::native::jvm::object::JVM_MonitorNotify as *const c_void,
				)));
				continue;
			}

			if method.name == sym!(notifyAll) {
				method.set_entry_point(MethodEntryPoint::NativeMethod(NativeMethodPtr::External(
					crate::native::jvm::object::JVM_MonitorNotifyAll as *const c_void,
				)));
				continue;
			}

			if method.name == sym!(clone) {
				method.set_entry_point(MethodEntryPoint::NativeMethod(NativeMethodPtr::External(
					crate::native::jvm::object::JVM_Clone as *const c_void,
				)));
				continue;
			}

			panic!(
				"Unhandled native method in java.lang.Object: {}",
				method.name
			);
		}
	}

	static ONCE: Once = Once::new();

	ONCE.call_once(do_register_natives);
}
