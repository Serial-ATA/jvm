use super::convert_operand;
use super::references::{JObjectExt, field_ref_from_jfieldid};
use crate::objects::class::ClassPtr;
use crate::objects::instance::Instance;
use crate::symbols::Symbol;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw};

use core::ffi::c_char;
use std::ffi::CStr;

use ::jni::sys::{
	JNIEnv, jboolean, jbyte, jchar, jclass, jdouble, jfieldID, jfloat, jint, jlong, jobject, jshort,
};
use common::unicode;
use instructions::Operand;

fn find_field(
	thread: &'static JavaThread,
	class: ClassPtr,
	name: *const c_char,
	sig: *const c_char,
	is_static: bool,
) -> Throws<jfieldID> {
	let name_c = unsafe { CStr::from_ptr(name) };
	let sig_c = unsafe { CStr::from_ptr(sig) };

	let Ok(name) = unicode::decode(name_c.to_bytes()) else {
		return Throws::Ok(std::ptr::null_mut());
	};
	let Ok(sig) = unicode::decode(sig_c.to_bytes()) else {
		return Throws::Ok(std::ptr::null_mut());
	};

	let name_sym = Symbol::intern(name);
	let sig_sym = Symbol::intern(sig);

	let ret = class.resolve_field(name_sym, sig_sym);
	if let Throws::Ok(ret) = &ret {
		if ret.is_static() != is_static {
			throw!(@DEFER NoSuchFieldError, "{name_sym}");
		}
	}

	ret.map(|field| thread.jni_refs().allocate(field))
}

// --------------
//   NON-STATIC
// --------------

pub extern "system" fn GetFieldID(
	env: *mut JNIEnv,
	clazz: jclass,
	name: *const c_char,
	sig: *const c_char,
) -> jfieldID {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let Some(class) = (unsafe { clazz.to_reference() }) else {
		panic!("Invalid arguments to `GetFieldID`");
	};

	match find_field(thread, class.extract_target_class(), name, sig, false) {
		Throws::Ok(f) => f,
		Throws::Exception(e) => {
			e.throw(thread);
			std::ptr::null_mut()
		},
	}
}

macro_rules! impl_get_field {
    ($($name:ident |$value:ident| $transformer:block => $ret_ty:ty),* $(,)?) => {
        $(
            #[allow(trivial_numeric_casts)]
            pub extern "system" fn $name(
                env: *mut JNIEnv,
                obj: jobject,
                fieldID: jfieldID,
            ) -> $ret_ty {
                let thread = JavaThread::current();
                assert_eq!(thread.env().raw(), env);

                let Some(obj) = (unsafe { obj.to_reference() }) else {
                    panic!("null object passed to `{}`", stringify!($name));
                };

                let Some(field) = (unsafe { field_ref_from_jfieldid(fieldID) }) else {
                    panic!("bad field ID");
                };

                let $value = convert_operand(thread, obj.get_field_value(field));
                $transformer
            }
        )*
    }
}

#[rustfmt::skip]
impl_get_field!(
	GetObjectField  |value| { unsafe { value.l           } } => jobject,
	GetBooleanField |value| { unsafe { value.i != 0      } } => jboolean,
	GetByteField    |value| { unsafe { value.i as jbyte  } } => jbyte,
	GetCharField    |value| { unsafe { value.i as jchar  } } => jchar,
	GetShortField   |value| { unsafe { value.i as jshort } } => jshort,
	GetIntField     |value| { unsafe { value.i           } } => jint,
	GetLongField    |value| { unsafe { value.j           } } => jlong,
	GetFloatField   |value| { unsafe { value.f           } } => jfloat,
	GetDoubleField  |value| { unsafe { value.d           } } => jdouble,
);

macro_rules! impl_set_field {
    (
        $($name:ident($jni_ty:ty) |$value:ident| $transformer:block),* $(,)?
    ) => {
        $(
            #[allow(trivial_numeric_casts)]
            pub extern "system" fn $name(
                env: *mut JNIEnv,
                obj: jobject,
                fieldID: jfieldID,
                $value: $jni_ty,
            ) {
                let thread = JavaThread::current();
                assert_eq!(thread.env().raw(), env);

                let Some(obj) = (unsafe { obj.to_reference() }) else {
                    panic!("null object passed to `{}`", stringify!($name));
                };

                let Some(field) = (unsafe { field_ref_from_jfieldid(fieldID) }) else {
                    panic!("bad field ID");
                };

                obj.put_field_value(field, $transformer)
            }
        )*
    }
}

#[rustfmt::skip]
impl_set_field!(
	SetObjectField(jobject)   |val| { Operand::Reference(unsafe { val.to_reference_maybe_null() }) },
	SetBooleanField(jboolean) |val| { Operand::Int(val as jint)                                    },
	SetByteField(jbyte)       |val| { Operand::Int(val as jint)                                    },
	SetCharField(jchar)       |val| { Operand::Int(val as jint)                                    },
	SetShortField(jshort)     |val| { Operand::Int(val as jint)                                    },
	SetIntField(jint)         |val| { Operand::Int(val)                                            },
	SetLongField(jlong)       |val| { Operand::Long(val)                                           },
	SetFloatField(jfloat)     |val| { Operand::Float(val)                                          },
	SetDoubleField(jdouble)   |val| { Operand::Double(val)                                         },
);

// --------------
//     STATIC
// --------------

pub extern "system" fn GetStaticFieldID(
	env: *mut JNIEnv,
	clazz: jclass,
	name: *const c_char,
	sig: *const c_char,
) -> jfieldID {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let Some(class) = (unsafe { clazz.to_reference() }) else {
		panic!("Invalid arguments to `GetStaticFieldID`");
	};

	match find_field(thread, class.extract_target_class(), name, sig, true) {
		Throws::Ok(f) => f,
		Throws::Exception(e) => {
			e.throw(thread);
			std::ptr::null_mut()
		},
	}
}

pub extern "system" fn GetStaticObjectField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jobject {
	unimplemented!("jni::GetStaticObjectField");
}

pub extern "system" fn GetStaticBooleanField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jboolean {
	unimplemented!("jni::GetStaticBooleanField");
}

pub extern "system" fn GetStaticByteField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jbyte {
	unimplemented!("jni::GetStaticByteField");
}

pub extern "system" fn GetStaticCharField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jchar {
	unimplemented!("jni::GetStaticCharField");
}

pub extern "system" fn GetStaticShortField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jshort {
	unimplemented!("jni::GetStaticShortField");
}

pub extern "system" fn GetStaticIntField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jint {
	unimplemented!("jni::GetStaticIntField");
}

pub extern "system" fn GetStaticLongField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jlong {
	unimplemented!("jni::GetStaticLongField");
}

pub extern "system" fn GetStaticFloatField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jfloat {
	unimplemented!("jni::GetStaticFloatField");
}

pub extern "system" fn GetStaticDoubleField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jdouble {
	unimplemented!("jni::GetStaticDoubleField");
}

pub unsafe extern "system" fn SetStaticObjectField(
	env: *mut JNIEnv,
	_clazz: jclass,
	fieldID: jfieldID,
	value: jobject,
) {
	let Some(field) = (unsafe { field_ref_from_jfieldid(fieldID) }) else {
		panic!("Invalid field ID");
	};

	let value = unsafe { value.to_reference_maybe_null() };
	let mirror = field.class.mirror();

	// SAFETY: Assuming that `fieldID` points to a valid field, then its index is guaranteed to be valid
	//         by the class loader.
	unsafe { mirror.put_field_value(field, Operand::Reference(value)) }
}

pub extern "system" fn SetStaticBooleanField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jboolean,
) {
	unimplemented!("jni::SetStaticBooleanField")
}

pub extern "system" fn SetStaticByteField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jbyte,
) {
	unimplemented!("jni::SetStaticByteField");
}

pub extern "system" fn SetStaticCharField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jchar,
) {
	unimplemented!("jni::SetStaticCharField");
}

pub extern "system" fn SetStaticShortField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jshort,
) {
	unimplemented!("jni::SetStaticShortField")
}

pub extern "system" fn SetStaticIntField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jint,
) {
	unimplemented!("jni::SetStaticIntField");
}

pub extern "system" fn SetStaticLongField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jlong,
) {
	unimplemented!("jni::SetStaticLongField");
}

pub extern "system" fn SetStaticFloatField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jfloat,
) {
	unimplemented!("jni::SetStaticFloatField")
}

pub extern "system" fn SetStaticDoubleField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jdouble,
) {
	unimplemented!("jni::SetStaticDoubleField")
}
