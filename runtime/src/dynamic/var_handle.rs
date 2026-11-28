use crate::native::jvm::strings::StringInterner;
use crate::objects::instance::array::{Array, ObjectArrayInstance};
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::symbols::{Symbol, sym};
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw};
use crate::{classes, globals, java_call};

use classfile::constant_pool::types::ReferenceKind;
use instructions::Operand;
use jni::sys::jint;

/// Attempts to resolve a signature polymorphic method
///
/// If successful, this returns the resolved method and appendix
pub fn resolve_target(
	method: &'static Method,
	descriptor: Symbol,
) -> Throws<(&'static Method, Reference)> {
	assert!(
		method
			.class()
			.is_subclass_of(globals::classes::java_lang_invoke_VarHandle()),
		"Only VarHandles should make it to this point"
	);

	let caller_class = method.class();

	let caller_class_mirror = caller_class.mirror();
	let defining_class_mirror = method.class().mirror();
	let name_str = StringInterner::intern(method.name);
	let appendix = ObjectArrayInstance::new(1, globals::classes::java_lang_Object())?;
	let method_ty = Method::method_type_for(caller_class, descriptor.as_str())?;

	let link_method = globals::classes::java_lang_invoke_MethodHandleNatives()
		.resolve_method(sym!(linkMethod), sym!(linkMethodSignature))?;

	let result = java_call!(
		JavaThread::current(),
		link_method,
		caller_class_mirror,
		ReferenceKind::InvokeVirtual as jint,
		defining_class_mirror,
		name_str,
		Operand::Reference(method_ty),
		appendix
	)
	.expect("should produce a result")
	.expect_reference();

	if result.is_null() {
		throw!(@DEFER LinkageError);
	}

	let result = result.extract_class();
	let appendix = appendix.array_get(0)?;

	let clazz = classes::java::lang::invoke::MemberName::clazz(result)?;
	let vmindex = classes::java::lang::invoke::MemberName::vmindex(result);

	let Some(method) = clazz.target_class().vtable().iter().nth(vmindex as usize) else {
		throw!(@DEFER LinkageError, "MethodHandleNatives produced an invalid MemberName");
	};

	Throws::Ok((method, appendix))
}
