use crate::objects::class::{Class, ClassPtr};
use crate::objects::method::Method;
use crate::symbols::sym;
use crate::verifier::accessors::{ClassAccessorExt, MethodAccessorExt};
use crate::verifier::type_system::VerificationType;

use common::int_types::u2;

/// ```prolog
/// frame(Locals, OperandStack, Flags)
/// ```
///
/// where:
///
/// * `locals` is a list of verification types, such that the i'th element of the list (with 0-based indexing)
///   represents the type of local variable i. Types of size 2 (long and double) are represented by two
///   local variables (§2.6.1), with the first local variable being the type itself and the second local
///   variable being top (§4.10.1.7).
///
/// * `operand_stack` is a list of verification types, such that the first element of the list represents
///    the type of the top of the operand stack, and the types of stack entries below the top follow
///    in the list in the appropriate order. Types of size 2 (long and double) are represented by two
///    stack entries, with the first entry being top and the second entry being the type itself.
///
/// * `flags` is a list which may either be empty or have the single element flagThisUninit. If any local variable
///    in Locals has the type uninitializedThis, then Flags has the single element flagThisUninit,
///    otherwise Flags is an empty list.
///
/// _flagThisUninit is used in constructors to mark type states where initialization of this has not yet been completed. In such type states, it is illegal to return from the method._
pub(super) struct Frame {
	locals: Vec<VerificationType>,
	operand_stack: Vec<VerificationType>,
	flags: bool,
}

/// ```prolog
/// methodInitialStackFrame(Class, Method, FrameSize, frame(Locals, [], Flags),
///                         ReturnType):-
///     methodDescriptor(Method, Descriptor),
///     parseMethodDescriptor(Descriptor, RawArgs, ReturnType),
///     expandTypeList(RawArgs, Args),
///     methodInitialThisType(Class, Method, ThisList),
///     flags(ThisList, Flags),
///     append(ThisList, Args, ThisArgs),
///     expandToLength(ThisArgs, FrameSize, top, Locals).
/// ```
pub(super) fn method_inital_stack_frame(class: ClassPtr, method: &Method, frame_size: u2) -> Frame {
	let descriptor = &method.descriptor;
	method_initial_this_type(class, method);
	let frame = Frame {
		locals: Vec::with_capacity(frame_size as usize),
		operand_stack: Vec::new(),
		flags: true,
	};

	frame
}

/// ```prolog
/// methodInitialThisType(_Class, Method, []) :-
///     methodAccessFlags(Method, AccessFlags),
///     member(static, AccessFlags),
///     methodName(Method, MethodName),
///     MethodName \= '<init>'.
///
/// methodInitialThisType(Class, Method, [This]) :-
///     methodAccessFlags(Method, AccessFlags),
///     notMember(static, AccessFlags),
///     instanceMethodInitialThisType(Class, Method, This).
/// ```
fn method_initial_this_type(class: ClassPtr, method: &Method) -> bool {
	let access_flags = method.access_flags();
	if access_flags.is_static() {
		if method.name() == sym!(object_initializer_name) {
			return false;
		}

		return true;
	}

	instance_method_initial_this_type(class, method)
}

/// ```prolog
/// instanceMethodInitialThisType(Class, Method, class('java/lang/Object', L)) :-
///     methodName(Method, '<init>'),
///     classDefiningLoader(Class, L),
///     isBootstrapLoader(L),
///     classClassName(Class, 'java/lang/Object').
///
/// instanceMethodInitialThisType(Class, Method, uninitializedThis) :-
///     methodName(Method, '<init>'),
///     classClassName(Class, ClassName),
///     classDefiningLoader(Class, CurrentLoader),
///     superclassChain(ClassName, CurrentLoader, Chain),
///     Chain \= [].
///
/// instanceMethodInitialThisType(Class, Method, class(ClassName, L)) :-
///     methodName(Method, MethodName),
///     MethodName \= '<init>',
///     classDefiningLoader(Class, L),
///     classClassName(Class, ClassName).
/// ```
fn instance_method_initial_this_type(class: ClassPtr, method: &Method) -> bool {
	if class.class_name() == sym!(java_lang_Object) {
		if method.name() == sym!(object_initializer_name) && class.is_bootstrap_loader() {
			return true;
		}

		return false;
	}

	if method.name() == sym!(object_initializer_name) {
		if class.super_class.is_some() {
			return true;
		}

		return false;
	}

	true
}
