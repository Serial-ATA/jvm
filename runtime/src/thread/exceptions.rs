use super::JavaThread;
use crate::classpath::loader::ClassLoader;
use crate::java_call;
use crate::native::java::lang::String::StringInterner;
use crate::objects::class_instance::ClassInstance;
use crate::objects::reference::Reference;
use crate::stack::local_stack::LocalStack;
use crate::symbols::sym;

use std::ops::{ControlFlow, FromResidual, Try};
use std::sync::atomic::Ordering;

use classfile::accessflags::MethodAccessFlags;
use common::traits::PtrType;
use instructions::{Operand, StackLike};

#[must_use]
#[derive(Debug)]
pub enum Throws<T> {
	Ok(T),
	Exception(Exception),
}

impl<T> Throws<T> {
	pub fn threw(&self) -> bool {
		matches!(self, Throws::Exception(_))
	}

	pub fn map<U, F>(self, f: F) -> Throws<U>
	where
		F: FnOnce(T) -> U,
	{
		match self {
			Throws::Ok(x) => Throws::Ok(f(x)),
			Throws::Exception(e) => Throws::Exception(e),
		}
	}

	pub fn expect(self, msg: &str) -> T {
		match self {
			Throws::Ok(t) => t,
			Throws::Exception(e) => panic!(
				"{msg}: thread threw {:?} with message: {:?}",
				e.kind, e.message
			),
		}
	}

	pub fn unwrap(self) -> T {
		match self {
			Throws::Ok(t) => t,
			Throws::Exception(e) => panic!("unwrapped exception: {:?}", e),
		}
	}
}

impl<T> Try for Throws<T> {
	type Output = T;
	type Residual = Exception;

	fn from_output(output: Self::Output) -> Self {
		Self::Ok(output)
	}

	fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
		match self {
			Throws::Ok(val) => ControlFlow::Continue(val),
			Throws::Exception(val) => ControlFlow::Break(val),
		}
	}
}

impl<T> FromResidual<Exception> for Throws<T> {
	fn from_residual(residual: Exception) -> Self {
		Self::Exception(residual)
	}
}

#[derive(Copy, Clone, Debug)]
pub enum ExceptionKind {
	/// java.lang.ClassFormatError
	ClassFormatError,
	/// java.lang.UnsupportedClassVersionError
	UnsupportedClassVersionError,
	/// java.lang.NoClassDefFoundError
	NoClassDefFoundError,

	/// java.lang.LinkageError
	LinkageError,
	/// java.lang.IncompatibleClassChangeError
	IncompatibleClassChangeError,
	/// java.lang.NoSuchFieldError
	NoSuchFieldError,
	/// java.lang.NoSuchMethodError
	NoSuchMethodError,

	/// java.lang.NegativeArraySizeException
	NegativeArraySizeException,
	/// java.lang.ArrayIndexOutOfBoundsException
	ArrayIndexOutOfBoundsException,

	/// java.lang.CloneNotSupportedException
	CloneNotSupportedException,

	/// java.lang.NullPointerException
	NullPointerException,
	/// java.lang.IllegalArgumentException
	IllegalArgumentException,
	/// java.lang.IllegalStateException
	IllegalStateException,
	/// java.lang.IllegalAccessError
	IllegalAccessError,
	/// java.lang.IndexOutOfBoundsException
	IndexOutOfBoundsException,
	/// java.lang.IllegalThreadStateException
	IllegalThreadStateException,

	/// java.lang.InternalError
	InternalError,
}

impl ExceptionKind {
	fn obj(&self) -> Reference {
		let class_name = match self {
			ExceptionKind::ClassFormatError => sym!(java_lang_ClassFormatError),
			ExceptionKind::UnsupportedClassVersionError => {
				sym!(java_lang_UnsupportedClassVersionError)
			},
			ExceptionKind::NoClassDefFoundError => sym!(java_lang_NoClassDefFoundError),

			ExceptionKind::LinkageError => sym!(java_lang_LinkageError),
			ExceptionKind::IncompatibleClassChangeError => {
				sym!(java_lang_IncompatibleClassChangeError)
			},
			ExceptionKind::NoSuchFieldError => sym!(java_lang_NoSuchFieldError),
			ExceptionKind::NoSuchMethodError => sym!(java_lang_NoSuchMethodError),

			ExceptionKind::NegativeArraySizeException => sym!(java_lang_NegativeArraySizeException),
			ExceptionKind::ArrayIndexOutOfBoundsException => {
				sym!(java_lang_ArrayIndexOutOfBoundsException)
			},

			ExceptionKind::CloneNotSupportedException => sym!(java_lang_CloneNotSupportedException),

			ExceptionKind::NullPointerException => sym!(java_lang_NullPointerException),
			ExceptionKind::IllegalArgumentException => sym!(java_lang_IllegalArgumentException),
			ExceptionKind::IllegalStateException => sym!(java_lang_IllegalStateException),
			ExceptionKind::IllegalAccessError => sym!(java_lang_IllegalAccessError),
			ExceptionKind::IndexOutOfBoundsException => sym!(java_lang_IndexOutOfBoundsException),
			ExceptionKind::IllegalThreadStateException => {
				sym!(java_lang_IllegalThreadStateException)
			},
			ExceptionKind::InternalError => sym!(java_lang_InternalError),
		};

		let class = ClassLoader::bootstrap()
			.load(class_name)
			.expect("exception class should exist");
		Reference::class(ClassInstance::new(class))
	}
}

#[derive(Debug)]
pub struct Exception {
	kind: ExceptionKind,
	message: Option<String>,
}

impl Exception {
	pub fn new(kind: ExceptionKind) -> Self {
		Exception {
			kind,
			message: None,
		}
	}

	pub fn with_message<T>(kind: ExceptionKind, message: T) -> Self
	where
		T: Into<String>,
	{
		Exception {
			kind,
			message: Some(message.into()),
		}
	}

	pub fn throw(self, thread: &JavaThread) {
		let this = self.kind.obj();

		match self.message {
			Some(message) => {
				let init_method = crate::globals::classes::java_lang_Throwable()
					.vtable()
					.find(
						sym!(object_initializer_name),
						sym!(String_void_signature),
						MethodAccessFlags::NONE,
					)
					.expect("method should exist");

				let string_object = StringInterner::intern(message.as_str());
				java_call!(
					thread,
					init_method,
					Operand::Reference(this.clone()),
					Operand::Reference(Reference::class(string_object))
				);
			},
			None => {
				let init_method = crate::globals::classes::java_lang_Throwable()
					.vtable()
					.find(
						sym!(object_initializer_name),
						sym!(void_method_signature),
						MethodAccessFlags::NONE,
					)
					.expect("method should exist");

				java_call!(thread, init_method, Operand::Reference(this.clone()));
			},
		}

		thread.set_pending_exception(this);
	}
}

// TODO: Document, maybe also have a second private macro to hide construction patterns
macro_rules! throw {
	($thread:ident, $($tt:tt)*) => {{
		crate::thread::exceptions::throw_with_ret!((), $thread, $($tt)*);
	}};
	(@DEFER $($tt:tt)*) => {{
		return crate::thread::exceptions::Throws::Exception(throw!(@CONSTRUCT $($tt)*));
	}};
	(@CONSTRUCT $exception_variant:ident) => {{
		crate::thread::exceptions::Exception::new(
			crate::thread::exceptions::ExceptionKind::$exception_variant
		)
	}};
	(@CONSTRUCT $exception_variant:ident, $message:expr) => {{
		crate::thread::exceptions::Exception::with_message(
			crate::thread::exceptions::ExceptionKind::$exception_variant, $message
		)
	}};
	(@CONSTRUCT $exception_variant:ident, $message:expr, $($arg:expr),+ $(,)?) => {{
		crate::thread::exceptions::Exception::with_message(
			crate::thread::exceptions::ExceptionKind::$exception_variant, format!($message, $($arg),+)
		)
	}};
}

macro_rules! throw_with_ret {
	($ret:expr, $thread:expr, $($tt:tt)*) => {
		let __ex = crate::thread::exceptions::throw!(@CONSTRUCT $($tt)*);
		__ex.throw(&$thread);
		return $ret;
	};
}

macro_rules! throw_and_return_null {
	($thread:expr, $($tt:tt)*) => {
		crate::thread::exceptions::throw_with_ret!(
			 $crate::objects::reference::Reference::null(),
			 $thread,
			 $($tt)*
		);
	};
}

macro_rules! handle_exception {
	($thread:expr, $throwsy_expr:expr) => {{
		crate::thread::exceptions::handle_exception!((), $thread, $throwsy_expr)
	}};
	($ret:expr, $thread:expr, $throwsy_expr:expr) => {{
		match $throwsy_expr {
			crate::thread::exceptions::Throws::Ok(__val) => __val,
			crate::thread::exceptions::Throws::Exception(__exception) => {
				__exception.throw(&$thread);
				return $ret;
			},
		}
	}};
}

pub(crate) use {handle_exception, throw, throw_and_return_null, throw_with_ret};

/// See [`JavaThread::throw_exception`]
#[must_use = "must know whether the exception was thrown"]
pub(super) fn handle_throw(thread: &JavaThread, object_ref: Reference) -> bool {
	// https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-6.html#jvms-6.5.athrow
	// The objectref must be of type reference and must refer to an object that is an instance of class Throwable or of a subclass of Throwable.

	let class_instance = object_ref.extract_class();

	let throwable_class = crate::globals::classes::java_lang_Throwable();
	assert!(
		class_instance.get().class() == throwable_class
			|| class_instance.get().is_subclass_of(&throwable_class)
	);

	// Search each frame for an exception handler
	thread.stash_and_reset_pc();
	while let Some(current_frame) = thread.frame_stack.current() {
		let current_frame_pc = current_frame.stashed_pc();

		// If an exception handler that matches objectref is found, it contains the location of the code intended to handle this exception.
		if let Some(handler_pc) = current_frame
			.method()
			.find_exception_handler(class_instance.get().class(), current_frame_pc)
		{
			// The pc register is reset to that location, the operand stack of the current frame is cleared, objectref
			// is pushed back onto the operand stack, and execution continues.
			thread.pc.store(handler_pc, Ordering::Relaxed);

			let stack = current_frame.stack_mut();
			stack.clear();
			stack.push_reference(object_ref);

			return false;
		}

		let _ = thread.frame_stack.pop();
	}

	// No handler found, we have to print the stack trace and exit
	thread.set_exiting();
	print_stack_trace(thread, object_ref);

	false
}

fn print_stack_trace(thread: &JavaThread, object_ref: Reference) {
	let print_stack_trace = object_ref
		.extract_class()
		.get()
		.class()
		.vtable()
		.find(
			sym!(printStackTrace_name),
			sym!(void_method_signature),
			MethodAccessFlags::NONE,
		)
		.expect("java/lang/Throwable#printStackTrace should exist");

	let mut locals = LocalStack::new(1);
	locals[0] = Operand::Reference(object_ref);

	thread.invoke_method_with_local_stack(print_stack_trace, locals);
}
