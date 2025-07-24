use crate::globals;
use crate::objects::instance::Instance;
use crate::objects::instance::class::ClassInstanceRef;
use crate::objects::instance::object::Object;
use crate::objects::method::MethodEntryPoint;
use crate::symbols::sym;

use classfile::FieldType;

/// `java.lang.invoke.MethodHandle#form` field
pub fn form(instance: ClassInstanceRef) -> ClassInstanceRef {
	assert!(
		instance
			.class()
			.is_subclass_of(globals::classes::java_lang_invoke_MethodHandle())
	);
	instance
		.get_field_value0(form_field_index())
		.expect_reference()
		.extract_class()
}

crate::classes::field_module! {
	@CLASS java_lang_invoke_MethodHandle;

	@FIELDSTART
	/// `java.lang.invoke.MethodHandle#form` field offset
	///
	/// Expected field type: `Reference` to `java.lang.invoke.LambdaForm`
	@FIELD form: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/invoke/LambdaForm"),
}

/// Initializes the entry points for `java.lang.invoke.MethodHandle` natives
pub fn init_entry_points() {
	let class = globals::classes::java_lang_invoke_MethodHandle();

	// Don't need to verify this was only called once, `Method::set_entry_point` will panic
	// if the entry point is already set.
	for method in class.vtable().iter_local() {
		if method.name == sym!(invoke_name) {
			method.set_entry_point(MethodEntryPoint::MethodHandleInvoker(_dynamic::invoke));
			continue;
		}

		if method.name == sym!(invokeBasic_name) {
			method.set_entry_point(MethodEntryPoint::MethodHandleInvoker(
				_dynamic::invoke_basic,
			));
			continue;
		}

		if method.name == sym!(invokeExact_name) {
			method.set_entry_point(MethodEntryPoint::MethodHandleInvoker(
				_dynamic::invoke_exact,
			));
			continue;
		}

		if method.name == sym!(linkToStatic_name) {
			method.set_entry_point(MethodEntryPoint::MethodHandleLinker(
				_dynamic::link_to_static,
			));
			continue;
		}

		if method.name == sym!(linkToSpecial_name) {
			method.set_entry_point(MethodEntryPoint::MethodHandleLinker(
				_dynamic::link_to_special,
			));
			continue;
		}

		if method.name == sym!(linkToInterface_name) {
			method.set_entry_point(MethodEntryPoint::MethodHandleLinker(
				_dynamic::link_to_interface,
			));
			continue;
		}

		if method.name == sym!(linkToVirtual_name) {
			method.set_entry_point(MethodEntryPoint::MethodHandleLinker(
				_dynamic::link_to_virtual,
			));
			continue;
		}
	}
}

mod _dynamic {
	use crate::method_invoker::MethodInvoker;
	use crate::objects::boxing::Boxable;
	use crate::objects::constant_pool::cp_types::MethodEntry;
	use crate::objects::instance::class::ClassInstanceRef;
	use crate::objects::method::Method;
	use crate::objects::reference::Reference;
	use crate::stack::local_stack::LocalStack;
	use crate::thread::exceptions::{Throws, throw};
	use crate::thread::frame::Frame;
	use crate::{classes, java_call};

	use instructions::{Operand, StackLike};

	fn get_target_method(frame: &Frame, handle: ClassInstanceRef) -> Option<&'static Method> {
		let form = classes::java::lang::invoke::MethodHandle::form(handle);
		let vmentry = classes::java::lang::invoke::LambdaForm::vmentry(form);
		match classes::java::lang::invoke::MemberName::target_method(vmentry) {
			Throws::Ok(method) => Some(method),
			Throws::Exception(e) => {
				e.throw(frame.thread());
				None
			},
		}
	}

	fn morph_return_value(frame: &mut Frame, ret: Option<Operand<Reference>>) {
		if frame.thread().has_pending_exception() {
			return;
		}

		// A void return from a method handle target gets converted to `null`
		let Some(ret) = ret else {
			frame.stack_mut().push_reference(Reference::null());
			return;
		};

		match ret.into_box(frame.thread()) {
			Throws::Ok(ret) => frame.stack_mut().push_reference(ret),
			Throws::Exception(e) => {
				e.throw(frame.thread());
				return;
			},
		}
	}

	pub fn invoke_basic(frame: &mut Frame, entry: MethodEntry) {
		// Add 1 to the parameters size, since it doesn't account for `this`
		let parameters_count = (entry.parameters_stack_size as usize) + 1;

		let receiver = frame.stack().at(parameters_count).expect_reference();
		if receiver.is_null() {
			throw!(frame.thread(), NullPointerException);
		}

		let Some(target_method) = get_target_method(frame, receiver.extract_class()) else {
			return;
		};

		let call_args = frame.stack_mut().popn(parameters_count);
		let call_args =
			unsafe { LocalStack::new_with_args(call_args, target_method.code.max_locals as usize) };
		let ret = java_call!(@WITH_ARGS_LIST frame.thread(), target_method, call_args);
		morph_return_value(frame, ret);
	}

	pub fn invoke_exact(frame: &mut Frame, entry: MethodEntry) {
		invoke_basic(frame, entry);
	}

	pub fn invoke(frame: &mut Frame, entry: MethodEntry) {
		invoke_basic(frame, entry);
	}

	fn appendix_and_target_method(
		frame: &mut Frame,
	) -> Option<(ClassInstanceRef, &'static Method)> {
		let appendix = frame.stack_mut().pop_reference().extract_class();
		match classes::java::lang::invoke::MemberName::target_method(appendix) {
			Throws::Ok(method) => Some((appendix, method)),
			Throws::Exception(e) => {
				e.throw(frame.thread());
				None
			},
		}
	}

	pub fn link_to_static(frame: &mut Frame) {
		let Some((_appendix, target_method)) = appendix_and_target_method(frame) else {
			return;
		};
		MethodInvoker::invoke_virtual(frame, target_method);
	}

	pub fn link_to_special(frame: &mut Frame) {
		link_to_static(frame);
	}

	pub fn link_to_interface(frame: &mut Frame) {
		let Some((_appendix, _target_method)) = appendix_and_target_method(frame) else {
			return;
		};
		unimplemented!("link_to_interface");
	}

	pub fn link_to_virtual(frame: &mut Frame) {
		let Some((appendix, target_method)) = appendix_and_target_method(frame) else {
			return;
		};

		let receiver = frame
			.stack()
			.at(target_method.parameter_stack_size())
			.expect_reference();
		if receiver.is_null() {
			throw!(frame.thread(), NullPointerException);
		}

		let vmindex = classes::java::lang::invoke::MemberName::vmindex(appendix);
		let target_method = &receiver.extract_target_class().vtable()[vmindex as usize];
		MethodInvoker::invoke_virtual(frame, target_method);
	}
}
