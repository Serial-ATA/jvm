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

	/// java.lang.InvalidClassException
	InvalidClassException,

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

			ExceptionKind::InvalidClassException => sym!(java_lang_InvalidClassException),

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
	($ret:expr, $thread:expr, $($tt:tt)*) => {{
		let __ex = crate::thread::exceptions::throw!(@CONSTRUCT $($tt)*);
		__ex.throw(&$thread);
		return $ret;
	}};
}

macro_rules! throw_and_return_null {
	($thread:expr, $($tt:tt)*) => {{
		crate::thread::exceptions::throw_with_ret!(
			 $crate::objects::reference::Reference::null(),
			 $thread,
			 $($tt)*
		);
	}};
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
