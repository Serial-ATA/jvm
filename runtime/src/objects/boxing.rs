use crate::calls::jcall::JavaCallResult;
use crate::java_call;
use crate::objects::class::Class;
use crate::objects::reference::Reference;
use crate::symbols::{sym, Symbol};
use crate::thread::exceptions::Throws;
use crate::thread::JavaThread;

use instructions::Operand;
use jni::sys::{jboolean, jdouble, jfloat, jint, jlong};

/// Box primitives into their object form (ex. `jint` -> `java.lang.Integer`)
pub trait Boxable: Sized
where
	Operand<Reference>: From<Self>,
{
	const VALUE_OF_SIGNATURE: Symbol;

	fn class() -> &'static Class;

	fn into_box(self, thread: &JavaThread) -> Throws<Reference> {
		let value_of_method =
			Self::class().resolve_method(sym!(valueOf_name), Self::VALUE_OF_SIGNATURE)?;
		let result = java_call!(thread, value_of_method, self);

		match result {
			JavaCallResult::Ok(res) => {
				Throws::Ok(res.expect("method should return").expect_reference())
			},
			JavaCallResult::PendingException => {
				todo!()
			},
		}
	}
}

impl Boxable for jint {
	const VALUE_OF_SIGNATURE: Symbol = sym!(Integer_valueOf_signature);

	fn class() -> &'static Class {
		crate::globals::classes::java_lang_Integer()
	}
}

impl Boxable for jboolean {
	const VALUE_OF_SIGNATURE: Symbol = sym!(Boolean_valueOf_signature);

	fn class() -> &'static Class {
		crate::globals::classes::java_lang_Boolean()
	}
}

impl Boxable for jlong {
	const VALUE_OF_SIGNATURE: Symbol = sym!(Long_valueOf_signature);

	fn class() -> &'static Class {
		crate::globals::classes::java_lang_Long()
	}
}

impl Boxable for jdouble {
	const VALUE_OF_SIGNATURE: Symbol = sym!(Double_valueOf_signature);

	fn class() -> &'static Class {
		crate::globals::classes::java_lang_Double()
	}
}

impl Boxable for jfloat {
	const VALUE_OF_SIGNATURE: Symbol = sym!(Float_valueOf_signature);

	fn class() -> &'static Class {
		crate::globals::classes::java_lang_Float()
	}
}
