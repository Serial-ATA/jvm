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
		pub struct SafeJniWrapperType;

		impl SafeJniWrapperType {
            pub fn is_valid(s: &str) -> bool {
				match s {
					$(stringify!($sys_ty) | stringify!(jni::sys::$sys_ty) | stringify!(::jni::sys::$sys_ty) => true,)+
					$(stringify!($obj_sys_ty) | stringify!(jni::objects::$obj_sys_ty) | stringify!(::jni::objects::$obj_sys_ty)
					| stringify!($obj_ty) | stringify!(jni::objects::$obj_ty) | stringify!(::jni::objects::$obj_ty) => {
                        true
                    },)+
					_ => false,
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

pub struct ParameterType;

impl super::ParameterType for ParameterType {
	fn from_str_strict(ty: &str) -> bool
	where
		Self: Sized,
	{
		SafeJniWrapperType::is_valid(ty)
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

	let Some(path_str) = super::path_str(path) else {
		errors.push((
			Error::BadReturnType {
				ty: ty.to_token_stream().to_string(),
				call_type: CallType::Jni,
			},
			ty.span(),
		));
		return;
	};

	if !SafeJniWrapperType::is_valid(&path_str) {
		errors.push((
			Error::BadReturnType {
				ty: path_str,
				call_type: CallType::Jni,
			},
			ty.span(),
		));
	}
}
