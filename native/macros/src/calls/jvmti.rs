use super::{CallType, Error};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{Abi, ItemFn, ReturnType, Type, TypePath};

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

	fn to_raw(&self) -> TokenStream {
		match self {
			ParameterType::Jni(ty) => ty.to_raw(),
			ParameterType::Env => quote! { *mut ::jvmti::sys::jvmtiEnv },
			ParameterType::JvmtiError => quote! { ::jvmti::sys::jvmtiError },
			ParameterType::JThread => quote! { ::jvmti::sys::jthread },
			ParameterType::JThreadGroup => quote! { ::jvmti::sys::jthreadGroup },
			ParameterType::JRawMonitorId => quote! { ::jvmti::sys::jrawMonitorID },
		}
	}

	fn to_safe(&self) -> TokenStream {
		match self {
			ParameterType::Jni(ty) => ty.to_safe(),
			ParameterType::Env => quote! { ::jvmti::env::JvmtiEnv },
			ParameterType::JvmtiError => quote! { ::jvmti::error::JvmtiError },
			ParameterType::JThread => quote! { ::jvmti::objects::JThread },
			ParameterType::JThreadGroup => quote! { ::jvmti::objects::JThreadGroup },
			ParameterType::JRawMonitorId => quote! { ::jvmti::objects::JRawMonitorId },
		}
	}

	fn raw_conversion_fn(&self, param_name: Ident) -> TokenStream {
		match self {
			ParameterType::Jni(ty) => ty.raw_conversion_fn(param_name),
			ParameterType::Env
			| ParameterType::JvmtiError
			| ParameterType::JThread
			| ParameterType::JThreadGroup
			| ParameterType::JRawMonitorId => quote! { #param_name.raw() },
		}
	}

	fn safe_conversion_fn(&self, param_name: Ident) -> TokenStream {
		match self {
			ParameterType::Jni(ty) => ty.safe_conversion_fn(param_name),
			ParameterType::Env => {
				quote! { unsafe { ::jvmti::env::JvmtiEnv::from_raw(#param_name) } }
			},
			ParameterType::JvmtiError => {
				quote! { unsafe { ::jvmti::error::JvmtiError::from_raw(#param_name) } }
			},
			ParameterType::JThread => {
				quote! { unsafe { ::jvmti::objects::JThread::from_raw(#param_name) } }
			},
			ParameterType::JThreadGroup => {
				quote! { unsafe { ::jvmti::objects::JThreadGroup::from_raw(#param_name) } }
			},
			ParameterType::JRawMonitorId => {
				quote! { unsafe { ::jvmti::objects::JRawMonitorId::from_raw(#param_name) } }
			},
		}
	}
}

pub struct JvmtiFn {
	pub rust_fn: proc_macro2::TokenStream,
	pub extern_fn: proc_macro2::TokenStream,
	pub errors: Vec<(Error, Span)>,
}

pub fn generate(input: &ItemFn) -> JvmtiFn {
	let mut errors = Vec::new();
	super::validate_abi(&mut errors, input);
	let params = super::validate_params(&mut errors, input, false, true, CallType::Jvmti);
	let return_ty = validate_return(&mut errors, input);

	let fn_name = &input.sig.ident;
	let raw_mod_name = Ident::new(&format!("raw_{}", fn_name), Span::call_site());

	let extern_fn_def = super::generate_extern_fn(input, &params, return_ty);
	let extern_fn = quote! {
		#[allow(unused_imports)]
		mod #raw_mod_name {
			use super::*;

			#extern_fn_def
		}
	};

	let rust_fn = quote! {
		#[allow(non_snake_case)]
		#input
	};

	JvmtiFn {
		rust_fn,
		extern_fn,
		errors,
	}
}

fn validate_return(errors: &mut Vec<(Error, Span)>, fun: &ItemFn) -> Option<ParameterType> {
	let ReturnType::Type(_, ty) = &fun.sig.output else {
		errors.push((
			Error::BadReturnType {
				ty: String::from("()"),
				call_type: CallType::Jvmti,
			},
			fun.span(),
		));
		return None;
	};

	let Type::Path(TypePath { path, .. }) = &**ty else {
		errors.push((
			Error::BadReturnType {
				ty: ty.to_token_stream().to_string(),
				call_type: CallType::Jvmti,
			},
			ty.span(),
		));
		return None;
	};

	let Some((path_str, false)) = super::path_str(path) else {
		errors.push((
			Error::BadReturnType {
				ty: ty.to_token_stream().to_string(),
				call_type: CallType::Jvmti,
			},
			ty.span(),
		));
		return None;
	};

	if !["jvmti::error::JvmtiError", "JvmtiError"].contains(&&*path_str) {
		errors.push((
			Error::BadReturnType {
				ty: ty.to_token_stream().to_string(),
				call_type: CallType::Jvmti,
			},
			ty.span(),
		));
		return None;
	}

	Some(ParameterType::JvmtiError)
}
