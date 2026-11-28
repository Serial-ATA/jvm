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
		if method.name == sym!(invoke) {
			method.set_entry_point(MethodEntryPoint::MethodHandleInvoker(_dynamic::invoke));
			continue;
		}

		if method.name == sym!(invokeBasic) {
			method.set_entry_point(MethodEntryPoint::MethodHandleInvoker(
				_dynamic::invoke_basic,
			));
			continue;
		}

		if method.name == sym!(invokeExact) {
			method.set_entry_point(MethodEntryPoint::MethodHandleInvoker(
				_dynamic::invoke_exact,
			));
			continue;
		}

		if method.name == sym!(linkToStatic) {
			method.set_entry_point(MethodEntryPoint::MethodHandleLinker(
				_dynamic::link_to_static,
			));
			continue;
		}

		if method.name == sym!(linkToSpecial) {
			method.set_entry_point(MethodEntryPoint::MethodHandleLinker(
				_dynamic::link_to_special,
			));
			continue;
		}

		if method.name == sym!(linkToInterface) {
			method.set_entry_point(MethodEntryPoint::MethodHandleLinker(
				_dynamic::link_to_interface,
			));
			continue;
		}

		if method.name == sym!(linkToVirtual) {
			method.set_entry_point(MethodEntryPoint::MethodHandleLinker(
				_dynamic::link_to_virtual,
			));
			continue;
		}
	}
}

mod _dynamic {
	use crate::classes;
	use crate::objects::constant_pool::cp_types::MethodEntry;
	use crate::objects::instance::class::ClassInstanceRef;
	use crate::objects::method::Method;
	use crate::thread::exceptions::{Throws, throw};
	use crate::thread::frame::Frame;

	use common::int_types::u2;
	use instructions::StackLike;

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

	fn do_invoke(frame: &mut Frame, target_method: &'static Method) {
		let ret = frame.thread().invoke_method_scoped(target_method);
		if frame.thread().has_pending_exception() {
			return;
		}

		if let Some(ret) = ret {
			frame.push_op(ret);
		}
	}

	pub fn invoke_basic(frame: &mut Frame, entry: MethodEntry) {
		let receiver = frame.at(entry.parameters_stack_size).expect_reference();
		if receiver.is_null() {
			throw!(frame.thread(), NullPointerException);
		}

		let Some(target_method) = get_target_method(frame, receiver.extract_class()) else {
			// Exception thrown
			return;
		};

		do_invoke(frame, target_method);
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
		let appendix = frame.pop_reference().extract_class();
		match classes::java::lang::invoke::MemberName::target_method(appendix) {
			Throws::Ok(method) => Some((appendix, method)),
			Throws::Exception(e) => {
				e.throw(frame.thread());
				None
			},
		}
	}

	pub fn link_to_static(frame: &mut Frame, _entry: MethodEntry) {
		let Some((_appendix, target_method)) = appendix_and_target_method(frame) else {
			return;
		};
		do_invoke(frame, target_method)
	}

	pub fn link_to_special(frame: &mut Frame, entry: MethodEntry) {
		link_to_static(frame, entry);
	}

	pub fn link_to_interface(frame: &mut Frame, _entry: MethodEntry) {
		let Some((_appendix, _target_method)) = appendix_and_target_method(frame) else {
			return;
		};
		unimplemented!("link_to_interface");
	}

	pub fn link_to_virtual(frame: &mut Frame, _entry: MethodEntry) {
		let Some((_appendix, target_method)) = appendix_and_target_method(frame) else {
			return;
		};

		let receiver = frame
			// -1 since we popped the appendix
			.at((target_method.parameter_stack_size() - 1) as u2)
			.expect_reference();
		if receiver.is_null() {
			throw!(frame.thread(), NullPointerException);
		}

		let class = receiver.extract_instance_class();
		let target_method = class.select_method(target_method);

		do_invoke(frame, target_method);
	}
}
