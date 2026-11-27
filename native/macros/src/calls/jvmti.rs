use super::{CallType, Error};
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{ItemFn, ReturnType, Type, TypePath};

enum ParameterType {
	Jni(super::jni::ParameterType),
	Env,
	JvmtiError,
	JThread,
	JThreadGroup,
	JRawMonitorId,
}

impl super::ParameterType for ParameterType {
	fn env() -> Self {
		Self::Env
	}

	fn from_str_strict(ty: &str, optional: bool) -> Option<Self>
	where
		Self: Sized,
	{
		super::jni::ParameterType::from_str_strict(ty, optional).map(Self::Jni)
	}

	fn from_loose_ty(ty: Box<Type>) -> Self
	where
		Self: Sized,
	{
		Self::Jni(super::jni::ParameterType::from_loose_ty(ty))
	}
}

pub struct JvmtiFn {
	pub rust_fn: proc_macro2::TokenStream,
	pub errors: Vec<(Error, Span)>,
}

pub fn generate(input: &ItemFn) -> JvmtiFn {
	let mut errors = Vec::new();
	super::validate_abi(&mut errors, input);
	super::validate_params::<ParameterType>(&mut errors, input, false, true, CallType::Jvmti);
	validate_return(&mut errors, input);

	let rust_fn = quote! {
		#[allow(non_snake_case)]
		#[allow(unused)] // Only used via export
		#input
	};

	JvmtiFn { rust_fn, errors }
}

fn validate_return(errors: &mut Vec<(Error, Span)>, fun: &ItemFn) {
	let ReturnType::Type(_, ty) = &fun.sig.output else {
		errors.push((
			Error::BadReturnType {
				ty: String::from("()"),
				call_type: CallType::Jvmti,
			},
			fun.span(),
		));
		return;
	};

	let Type::Path(TypePath { path, .. }) = &**ty else {
		errors.push((
			Error::BadReturnType {
				ty: ty.to_token_stream().to_string(),
				call_type: CallType::Jvmti,
			},
			ty.span(),
		));
		return;
	};

	let Some((path_str, false)) = super::path_str(path) else {
		errors.push((
			Error::BadReturnType {
				ty: ty.to_token_stream().to_string(),
				call_type: CallType::Jvmti,
			},
			ty.span(),
		));
		return;
	};

	if !["jvmti::error::JvmtiError", "JvmtiError"].contains(&&*path_str) {
		errors.push((
			Error::BadReturnType {
				ty: ty.to_token_stream().to_string(),
				call_type: CallType::Jvmti,
			},
			ty.span(),
		));
	}
}
