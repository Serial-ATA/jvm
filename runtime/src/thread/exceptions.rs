use super::JavaThread;
use crate::classpath::loader::ClassLoader;
use crate::java_call;
use crate::native::java::lang::String::StringInterner;
use crate::objects::class::ClassPtr;
use crate::objects::instance::class::ClassInstance;
use crate::objects::reference::Reference;
use crate::symbols::{Symbol, sym};

use std::ops::{ControlFlow, FromResidual, Try};

use classfile::accessflags::MethodAccessFlags;
use instructions::Operand;

#[must_use]
#[derive(Debug)]
pub enum Throws<T> {
	Ok(T),
	Exception(Exception),
}

impl<T> Throws<T> {
	/// Used to indicate that an exception is pending on the current thread. For when we want to use
	/// our exception control flow, but exceptions occur that are out of our control.
	pub const PENDING_EXCEPTION: Self = Self::Exception(Exception {
		kind: ExceptionKind::PendingException,
		message: None,
	});

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

impl<T> FromIterator<Throws<T>> for Throws<Vec<T>> {
	fn from_iter<I: IntoIterator<Item = Throws<T>>>(iter: I) -> Self {
		let mut vec = Vec::new();
		for item in iter {
			match item {
				Throws::Ok(val) => vec.push(val),
				Throws::Exception(e) => {
					return Throws::Exception(e);
				},
			}
		}

		Throws::Ok(vec)
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ExceptionKind {
	/// java.lang.ClassFormatError
	ClassFormatError,
	/// java.lang.UnsupportedClassVersionError
	UnsupportedClassVersionError,
	/// java.lang.NoClassDefFoundError
	NoClassDefFoundError,
	/// java.lang.ClassNotFoundException
	ClassNotFoundException,
	/// java.lang.ClassCastException
	ClassCastException,
	/// java.lang.InstantiationError
	InstantiationError,

	/// java.lang.LinkageError
	LinkageError,
	/// java.lang.UnsatisfiedLinkError
	UnsatisfiedLinkError,
	/// java.lang.IncompatibleClassChangeError
	IncompatibleClassChangeError,
	/// java.lang.NoSuchFieldError
	NoSuchFieldError,
	/// java.lang.NoSuchMethodError
	NoSuchMethodError,
	/// java.lang.AbstractMethodError
	AbstractMethodError,

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
	/// java.lang.IllegalMonitorStateException
	IllegalMonitorStateException,

	/// java.io.IOException
	IOException,

	/// java.lang.InternalError
	InternalError,

	/// Used to indicate an exception is pending on the current thread. For when we want to use our
	/// exception control flow, but exceptions occur that are out of our control.
	PendingException,
}

impl ExceptionKind {
	fn class_name(&self) -> Symbol {
		match self {
			ExceptionKind::ClassFormatError => sym!(java_lang_ClassFormatError),
			ExceptionKind::UnsupportedClassVersionError => {
				sym!(java_lang_UnsupportedClassVersionError)
			},
			ExceptionKind::NoClassDefFoundError => sym!(java_lang_NoClassDefFoundError),
			ExceptionKind::ClassNotFoundException => sym!(java_lang_ClassNotFoundException),
			ExceptionKind::ClassCastException => sym!(java_lang_ClassCastException),
			ExceptionKind::InstantiationError => sym!(java_lang_InstantiationError),

			ExceptionKind::LinkageError => sym!(java_lang_LinkageError),
			ExceptionKind::UnsatisfiedLinkError => sym!(java_lang_UnsatisfiedLinkError),
			ExceptionKind::IncompatibleClassChangeError => {
				sym!(java_lang_IncompatibleClassChangeError)
			},
			ExceptionKind::NoSuchFieldError => sym!(java_lang_NoSuchFieldError),
			ExceptionKind::NoSuchMethodError => sym!(java_lang_NoSuchMethodError),
			ExceptionKind::AbstractMethodError => sym!(java_lang_AbstractMethodError),

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
			ExceptionKind::IllegalMonitorStateException => {
				sym!(java_lang_IllegalMonitorStateException)
			},

			ExceptionKind::IOException => sym!(java_io_IOException),

			ExceptionKind::InternalError => sym!(java_lang_InternalError),

			ExceptionKind::PendingException => unreachable!(),
		}
	}

	pub fn class(&self) -> ClassPtr {
		if *self == ExceptionKind::PendingException {
			let Some(exception) = JavaThread::current().pending_exception() else {
				panic!("Thread has no pending exception");
			};

			return exception.extract_target_class();
		}

		let class_name = self.class_name();
		ClassLoader::bootstrap()
			.load(class_name)
			.expect("exception class should exist")
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

	pub fn kind(&self) -> ExceptionKind {
		self.kind
	}

	pub fn throw(self, thread: &'static JavaThread) {
		if self.kind == ExceptionKind::PendingException {
			// The exception is already constructed and pending, will be handled eventually.
			assert!(thread.has_pending_exception());
			return;
		}

		let this = Reference::class(ClassInstance::new(self.kind.class()));

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
	($thread:expr, $($tt:tt)*) => {{
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
			crate::thread::exceptions::ExceptionKind::$exception_variant, format!($message)
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
	($thread:expr, $throwsy_expr:expr) => {{ crate::thread::exceptions::handle_exception!((), $thread, $throwsy_expr) }};
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

pub fn class_cast_exception_message(from: ClassPtr, to: ClassPtr) -> String {
	let from_class_description;
	let to_class_description;
	let class_separator;
	if from.module() == to.module() {
		let to_description = to.in_module_of_loader(true, false);
		from_class_description = format!("{} and {to_description}", from.external_name());
		to_class_description = String::new();
		class_separator = "";
	} else {
		from_class_description = from.in_module_of_loader(false, false);
		to_class_description = to.in_module_of_loader(false, false);
		class_separator = "; "
	}

	format!(
		"class {} cannot be cast to class {} \
		 ({from_class_description}{class_separator}{to_class_description})",
		from.name(),
		to.name(),
	)
}
