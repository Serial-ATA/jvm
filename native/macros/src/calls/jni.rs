use super::{CallType, Error};
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{ItemFn, ReturnType, Type, TypePath};

// jni_sys types
macro_rules! parameter_types {
	(
		sys => [
			$($sys_ty:ident),+ $(,)?
		]
		obj => [
			$($obj_ty:ident => $obj_sys_ty:ident),+ $(,)?
		]
	) => {
		#[derive(Copy, Clone)]
		#[allow(non_camel_case_types)]
		enum Primitive {
			$($sys_ty),+
		}

		#[derive(Clone)]
		pub enum SafeJniWrapperType {
			Primitive(Primitive),
            Optional(Box<SafeJniWrapperType>),
			$($obj_ty),+
		}

		impl SafeJniWrapperType {
            pub fn from_str(s: &str, optional: bool) -> Option<Self> {
				match s {
					$(stringify!($sys_ty) | stringify!(jni::sys::$sys_ty) | stringify!(::jni::sys::$sys_ty) => Some(SafeJniWrapperType::Primitive(Primitive::$sys_ty)),)+
					$(stringify!($obj_sys_ty) | stringify!(jni::objects::$obj_sys_ty) | stringify!(::jni::objects::$obj_sys_ty)
					| stringify!($obj_ty) | stringify!(jni::objects::$obj_ty) | stringify!(::jni::objects::$obj_ty) => {
                        if optional {
                            Some(SafeJniWrapperType::Optional(Box::new(SafeJniWrapperType::$obj_ty)))
                        } else {
                            Some(SafeJniWrapperType::$obj_ty)
                        }
                    },)+
					_ => None,
				}
			}
		}
	}
}
parameter_types!(
	sys => [
		jint,
		jlong,
		jbyte,
		jboolean,
		jchar,
		jshort,
		jfloat,
		jdouble,
		jsize,
	]
	obj => [
		JObject       => jobject,
		JClass        => jclass,
		JThrowable    => jthrowable,
		JString       => jstring,
		JArray        => jarray,
		JBooleanArray => jbooleanArray,
		JByteArray    => jbyteArray,
		JCharArray    => jcharArray,
		JShortArray   => jshortArray,
		JIntArray     => jintArray,
		JLongArray    => jlongArray,
		JFloatArray   => jfloatArray,
		JDoubleArray  => jdoubleArray,
		JObjectArray  => jobjectArray,
		JWeak         => jweak,
	]
);

pub enum ParameterType {
	Env,
	Strict(SafeJniWrapperType),
	Loose(Box<Type>),
}

impl super::ParameterType for ParameterType {
	fn env() -> Self {
		Self::Env
	}

	fn from_str_strict(ty: &str, optional: bool) -> Option<Self>
	where
		Self: Sized,
	{
		SafeJniWrapperType::from_str(ty, optional).map(Self::Strict)
	}

	fn from_loose_ty(ty: Box<Type>) -> Self
	where
		Self: Sized,
	{
		Self::Loose(ty)
	}
}

pub struct JniFn {
	pub rust_fn: proc_macro2::TokenStream,
	pub errors: Vec<(Error, Span)>,
}

pub fn generate(input: &ItemFn, no_env: bool, no_strict_types: bool) -> JniFn {
	let mut errors = Vec::new();
	super::validate_abi(&mut errors, input);
	super::validate_params::<ParameterType>(
		&mut errors,
		input,
		no_env,
		no_strict_types,
		CallType::Jni,
	);
	validate_return(&mut errors, input, no_strict_types);

	let rust_fn = quote! {
		#[unsafe(no_mangle)]
		#[allow(non_snake_case)]
		#input
	};

	JniFn { rust_fn, errors }
}

fn validate_return(errors: &mut Vec<(Error, Span)>, fun: &ItemFn, no_strict_types: bool) {
	let ReturnType::Type(_, ty) = &fun.sig.output else {
		return;
	};

	if no_strict_types {
		return;
	}

	let Type::Path(TypePath { path, .. }) = &**ty else {
		errors.push((
			Error::BadReturnType {
				ty: ty.to_token_stream().to_string(),
				call_type: CallType::Jni,
			},
			ty.span(),
		));
		return;
	};

	let Some((path_str, optional)) = super::path_str(path) else {
		errors.push((
			Error::BadReturnType {
				ty: ty.to_token_stream().to_string(),
				call_type: CallType::Jni,
			},
			ty.span(),
		));
		return;
	};

	if SafeJniWrapperType::from_str(&path_str, optional).is_none() {
		errors.push((
			Error::BadReturnType {
				ty: path_str,
				call_type: CallType::Jni,
			},
			ty.span(),
		));
	}
}
