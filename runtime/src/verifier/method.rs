use super::error::{Error, Result};
use super::type_system::{IsAssignable, VerificationType, types};
use crate::classpath::loader::ClassLoader;
use crate::objects::class::{Class, ClassPtr};
use crate::objects::constant_pool::cp_types;
use crate::objects::method::Method;
use crate::symbols::sym;
use crate::verifier::frame::{Frame, method_inital_stack_frame};

use classfile::attribute::{Attribute, CodeException, StackMapTable};
use common::int_types::{u1, u2};

pub(super) trait MethodTypeCheckExt {
	fn is_type_safe(&'static self) -> Result<()>;
	fn does_not_override_final_method(&self) -> bool;
	fn does_not_override_final_method_of_superclass(&self) -> bool;
}

impl MethodTypeCheckExt for Method {
	fn is_type_safe(&'static self) -> Result<()> {
		tracing::trace!("Verifying type safety of method {:?}", self);

		// abstract methods and native methods are considered to be type safe if they do not override a final method.
		//
		// methodIsTypeSafe(Class, Method) :-
		//     doesNotOverrideFinalMethod(Class, Method),
		//     methodAccessFlags(Method, AccessFlags),
		//     member(abstract, AccessFlags).
		//
		// methodIsTypeSafe(Class, Method) :-
		//     doesNotOverrideFinalMethod(Class, Method),
		//     methodAccessFlags(Method, AccessFlags),
		//     member(native, AccessFlags).
		if self.is_abstract() || self.is_native() {
			if !self.does_not_override_final_method() {
				return Err(Error::FinalMethodOverridden);
			}

			return Ok(());
		}

		// Non-abstract, non-native methods are type correct if they have code and the code is type correct.
		//
		// methodIsTypeSafe(Class, Method) :-
		//     doesNotOverrideFinalMethod(Class, Method),
		//     methodAccessFlags(Method, AccessFlags),
		//     methodAttributes(Method, Attributes),
		//     notMember(native, AccessFlags),
		//     notMember(abstract, AccessFlags),
		//     member(attribute('Code', _), Attributes),
		//     methodWithCodeIsTypeSafe(Class, Method).
		if !self.does_not_override_final_method() {
			return Err(Error::FinalMethodOverridden);
		}

		code_is_type_safe(self.class(), self)
	}

	fn does_not_override_final_method(&self) -> bool {
		todo!()
	}

	fn does_not_override_final_method_of_superclass(&self) -> bool {
		todo!()
	}
}

#[derive(Copy, Clone)]
struct MergedCode<'a> {
	code: &'a [u1],
	stack_map: Option<&'a StackMapTable>,
}

// When type checking a method's body, it is convenient to access information about the method.
// For this purpose, we define an environment, a six-tuple consisting of:
struct Environment<'a> {
	// a class
	class: ClassPtr,
	// a method
	method: &'static Method,
	// the declared return type of the method
	return_type: Option<VerificationType>,
	// the instructions in a method
	instructions: MergedCode<'a>,
	// the maximal size of the operand stack
	max_stack: u2,
	// a list of exception handlers
	handlers: &'a [CodeException],
}

impl Environment<'_> {
	// An exception handler is *legal* if its start (`Start`) is less than its end (`End`), there exists
	// an instruction whose offset is equal to `Start`, there exists an instruction whose offset
	// equals `End`, and the handler's exception class is assignable to the class Throwable.
	// The exception class of a handler is Throwable if the handler's class entry is 0, otherwise
	// it is the class named in the handler.
	//
	// handlersAreLegal(Environment) :-
	//     exceptionHandlers(Environment, Handlers),
	//     checklist(handlerIsLegal(Environment), Handlers).
	//
	// handlerIsLegal(Environment, Handler) :-
	//     Handler = handler(Start, End, Target, _),
	//     Start < End,
	//     allInstructions(Environment, Instructions),
	//     member(instruction(Start, _), Instructions),
	//     offsetStackFrame(Environment, Target, _),
	//     instructionsIncludeEnd(Instructions, End),
	//     currentClassLoader(Environment, CurrentLoader),
	//     handlerExceptionClass(Handler, ExceptionClass, CurrentLoader),
	//     isBootstrapLoader(BL),
	//     isAssignable(ExceptionClass, class('java/lang/Throwable', BL)).
	//
	// instructionsIncludeEnd(Instructions, End) :-
	//     member(instruction(End, _), Instructions).
	// instructionsIncludeEnd(Instructions, End) :-
	//     member(endOfCode(End), Instructions).
	//
	// handlerExceptionClass(handler(_, _, _, 0),
	//                       class('java/lang/Throwable', BL), _) :-
	//     isBootstrapLoader(BL).
	//
	// handlerExceptionClass(handler(_, _, _, Name),
	//                       class(Name, L), L) :-
	//     Name \= 0.
	fn handlers_are_legal(&self) -> Result<()> {
		fn handler_is_legal(environment: &Environment<'_>, handler: &CodeException) -> Result<()> {
			let start = handler.start_pc as usize;
			let end = handler.end_pc as usize;
			if start >= end {
				return Err(Error::BadExceptionHandlerRange(
					handler.start_pc,
					handler.end_pc,
				));
			}

			if !(0..environment.instructions.code.len()).contains(&start) {
				return Err(Error::InstructionOutOfBounds(
					handler.start_pc,
					environment.instructions.code.len(),
				));
			}

			if !(0..environment.instructions.code.len()).contains(&end) {
				return Err(Error::InstructionOutOfBounds(
					handler.end_pc,
					environment.instructions.code.len(),
				));
			}

			if handler.catch_type == 0 {
				return Ok(());
			}

			let exception_class_name = environment
				.class
				.constant_pool()
				.unwrap()
				.get::<cp_types::ClassName>(handler.catch_type)
				.expect("class name should always resolve");

			let exception_class = types::Class(exception_class_name);
			if !exception_class.is_assignable(types::Class(sym!(java_lang_Throwable))) {
				return Err(Error::HandlerNotThrowable);
			}

			Ok(())
		}

		for handler in self.handlers {
			handler_is_legal(self, handler)?;
		}

		Ok(())
	}

	// mergedCodeIsTypeSafe(Environment, [stackMap(Offset, MapFrame) | MoreCode],
	//                      frame(Locals, OperandStack, Flags)) :-
	//     frameIsAssignable(frame(Locals, OperandStack, Flags), MapFrame),
	//     mergedCodeIsTypeSafe(Environment, MoreCode, MapFrame).
	//
	// mergedCodeIsTypeSafe(Environment, [instruction(Offset, Parse) | MoreCode],
	//                      frame(Locals, OperandStack, Flags)) :-
	//     instructionIsTypeSafe(Parse, Environment, Offset,
	//                           frame(Locals, OperandStack, Flags),
	//                           NextStackFrame, ExceptionStackFrame),
	//     instructionSatisfiesHandlers(Environment, Offset, ExceptionStackFrame),
	//     mergedCodeIsTypeSafe(Environment, MoreCode, NextStackFrame).
	//
	// mergedCodeIsTypeSafe(Environment, [stackMap(Offset, MapFrame) | MoreCode],
	//                      afterGoto) :-
	//     mergedCodeIsTypeSafe(Environment, MoreCode, MapFrame).
	//
	// mergedCodeIsTypeSafe(_Environment, [instruction(_, _) | _MoreCode],
	//                      afterGoto) :-
	//     write_ln('No stack frame after unconditional branch'),
	//     fail.
	//
	// mergedCodeIsTypeSafe(_Environment, [endOfCode(Offset)],
	//                      afterGoto).
	fn merged_code_is_type_safe(
		&self,
		_merged_code: MergedCode<'_>,
		_stack_frame: Frame,
	) -> Result<()> {
		todo!()
	}

	/// ```prolog
	/// allInstructions(Environment, Instructions) :-
	///     Environment = environment(_Class, _Method, _ReturnType,
	///                               Instructions, _, _).
	/// ```
	fn all_instructions(&self) -> MergedCode<'_> {
		self.instructions
	}

	/// ```prolog
	/// exceptionHandlers(Environment, Handlers) :-
	///     Environment = environment(_Class, _Method, _ReturnType,
	///                               _Instructions, _, Handlers).
	/// ```
	fn exception_handlers(&self) -> &'_ [CodeException] {
		self.handlers
	}

	/// ```prolog
	/// maxOperandStackLength(Environment, MaxStack) :-
	///     Environment = environment(_Class, _Method, _ReturnType,
	///                               _Instructions, MaxStack, _Handlers).
	/// ```
	#[inline]
	fn max_operand_stack_length(&self) -> u2 {
		self.max_stack
	}

	/// ```prolog
	/// thisClass(Environment, class(ClassName, L)) :-
	///     Environment = environment(Class, _Method, _ReturnType,
	///                               _Instructions, _, _),
	///     classDefiningLoader(Class, L),
	///     classClassName(Class, ClassName).
	/// ```
	#[inline]
	fn this_class(&self) -> ClassPtr {
		self.class
	}

	/// ```prolog
	/// thisMethodReturnType(Environment, ReturnType) :-
	///     Environment = environment(_Class, _Method, ReturnType,
	///                               _Instructions, _, _).
	/// ```
	#[inline]
	fn this_method_return_type(&self) -> Option<VerificationType> {
		self.return_type
	}

	/// ```prolog
	/// currentClassLoader(Environment, Loader) :-
	///     thisClass(Environment, class(_, Loader)).
	/// ```
	#[inline]
	fn current_class_loader(&self) -> &'static ClassLoader {
		self.this_class().loader()
	}
}

/// A method with code is type safe if it is possible to merge the code and the stack map frames into
/// a single stream such that each stack map frame precedes the instruction it corresponds to, and the
/// merged stream is type correct. The method's exception handlers, if any, must also be legal.
///
/// ```prolog
/// methodWithCodeIsTypeSafe(Class, Method) :-
///     parseCodeAttribute(Class, Method, FrameSize, MaxStack,
///                        ParsedCode, Handlers, StackMap),
///     mergeStackMapAndCode(StackMap, ParsedCode, MergedCode),
///     methodInitialStackFrame(Class, Method, FrameSize, StackFrame, ReturnType),
///     Environment = environment(Class, Method, ReturnType, MergedCode,
///                               MaxStack, Handlers),
///     handlersAreLegal(Environment),
///     mergedCodeIsTypeSafe(Environment, MergedCode, StackFrame).
/// ```
fn code_is_type_safe(class: ClassPtr, method: &'static Method) -> Result<()> {
	let code = &method.code;
	let frame_size = code.max_locals;
	let merged_code = MergedCode {
		code: &code.code,
		stack_map: code.attributes.iter().find_map(Attribute::stack_map_table),
	};
	let stack_frame = method_inital_stack_frame(class, method, frame_size);
	let environment = Environment {
		class,
		method,
		return_type: None,
		instructions: merged_code,
		max_stack: code.max_stack,
		handlers: &code.exception_table,
	};
	environment.handlers_are_legal()?;
	environment.merged_code_is_type_safe(merged_code, stack_frame)?;
	Ok(())
}
