use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{
	Abi, FnArg, GenericArgument, ItemFn, Pat, PatIdent, PatType, Path, PathArguments, Type,
	TypePath,
};

pub(crate) mod jni;
pub(crate) mod jvmti;

trait ParameterType {
	fn env() -> Self;
	fn from_str_strict(ty: &str, optional: bool) -> Option<Self>
	where
		Self: Sized;
	fn from_loose_ty(ty: Box<Type>) -> Self
	where
		Self: Sized;
	fn to_raw(&self) -> TokenStream;
	fn to_safe(&self) -> TokenStream;
	fn raw_conversion_fn(&self, param_name: Ident) -> TokenStream;
	fn safe_conversion_fn(&self, param_name: Ident) -> TokenStream;
}

#[derive(Copy, Clone, PartialEq)]
pub enum CallType {
	Jni,
	Jvmti,
}

pub enum Error {
	MissingAbi,
	BadAbi,
	MissingEnv { call_type: CallType },
	HasReceiver,
	BadParameterType(String),
	BadReturnType { ty: String, call_type: CallType },
}

impl Error {
	pub fn into_syn(self, span: Span) -> syn::Error {
		match self {
			Error::MissingAbi => syn::Error::new(span, "Must specify an ABI"),
			Error::BadAbi => syn::Error::new(span, "Must specify valid ABI"),
			Error::MissingEnv { call_type } => syn::Error::new(
				span,
				format!(
					"First parameter must be a {}",
					match call_type {
						CallType::Jvmti => "JvmtiEnv",
						CallType::Jni => "JniEnv",
					}
				),
			),
			Error::HasReceiver => syn::Error::new(span, "JNI functions must be freestanding"),
			Error::BadParameterType(ty) => syn::Error::new(
				span,
				format!("Bad parameter type `{ty}`, must be one of the `jni_sys` primitives"),
			),
			Error::BadReturnType { ty, call_type } => syn::Error::new(
				span,
				format!(
					"Bad return type `{ty}`, must be {}",
					match call_type {
						CallType::Jvmti => "JvmtiError",
						CallType::Jni => "one of the `jni_sys` primitives",
					}
				),
			),
		}
	}
}

fn validate_abi(errors: &mut Vec<(Error, Span)>, fun: &ItemFn) {
	let Some(Abi {
		name: Some(name), ..
	}) = &fun.sig.abi
	else {
		errors.push((Error::MissingAbi, fun.sig.span()));
		return;
	};

	if name.value() != "system" && name.value() != "C" {
		errors.push((Error::BadAbi, fun.sig.span()));
	}
}

fn validate_params<P: ParameterType>(
	errors: &mut Vec<(Error, Span)>,
	fun: &ItemFn,
	no_env: bool,
	no_strict_types: bool,
	call_type: CallType,
) -> Vec<(Ident, P)> {
	fn parse_type<P: ParameterType>(
		fun: &ItemFn,
		no_env: bool,
		index: usize,
		param: &FnArg,
		arg: &PatType,
		call_type: CallType,
	) -> Result<P, (Error, Span)> {
		let Type::Path(TypePath { path, .. }) = &*arg.ty else {
			return Err((
				Error::BadParameterType(arg.ty.to_token_stream().to_string()),
				arg.ty.span(),
			));
		};

		let Some((path_str, _optional)) = path_str(path) else {
			return Err((
				Error::BadParameterType(arg.ty.to_token_stream().to_string()),
				param.span(),
			));
		};
		if index == 0 && !no_env {
			const JNI_ENV_PATHS: &[&str] = &["JniEnv", "jni::env::JniEnv"];
			const JVMTI_ENV_PATHS: &[&str] = &["JvmtiEnv", "jvmti::env::JvmtiEnv"];

			let list = match call_type {
				CallType::Jvmti => JVMTI_ENV_PATHS,
				CallType::Jni => JNI_ENV_PATHS,
			};
			if !list.contains(&path_str.as_str()) {
				return Err((Error::MissingEnv { call_type }, fun.sig.span()));
			}

			return Ok(ParameterType::env());
		}

		match P::from_str_strict(&path_str, false) {
			Some(parsed) => Ok(parsed),
			None => Err((Error::BadParameterType(path_str), param.span())),
		}
	}

	let mut params = Vec::new();
	for (index, param) in fun.sig.inputs.iter().enumerate() {
		match param {
			FnArg::Receiver(receiver) => errors.push((Error::HasReceiver, receiver.span())),
			FnArg::Typed(arg) => {
				let Pat::Ident(PatIdent { ident, .. }) = &*arg.pat else {
					errors.push((
						Error::BadParameterType(arg.ty.to_token_stream().to_string()),
						param.span(),
					));
					continue;
				};

				match parse_type(fun, no_env, index, param, arg, call_type) {
					Ok(parsed) => params.push((ident.clone(), parsed)),
					Err(e) => {
						if no_strict_types {
							params.push((ident.clone(), P::from_loose_ty(arg.ty.clone())));
							continue;
						}

						errors.push(e);
					},
				}
			},
		}
	}

	params
}

fn generate_extern_fn<P: ParameterType>(
	fun: &ItemFn,
	params: &[(Ident, P)],
	return_type: Option<P>,
) -> proc_macro2::TokenStream {
	let fn_name = &fun.sig.ident;

	let sys_params = params
		.iter()
		.map(|(param, parsed_ty)| {
			let ty = parsed_ty.to_raw();
			quote! { #param: #ty }
		})
		.collect::<Vec<_>>();

	let arg_conversions = params
		.iter()
		.map(|(param, parsed_ty)| {
			let conversion = parsed_ty.safe_conversion_fn(param.clone());
			quote! { let #param = #conversion; }
		})
		.collect::<Vec<_>>();

	let all_param_names = params
		.iter()
		.map(|(param, _)| param.clone())
		.collect::<Vec<_>>();

	let result_ident = Ident::new("result", Span::call_site());

	let ret;
	let to_raw_conversion;
	if let Some(ty) = return_type {
		let sys_ty = ty.to_raw();
		ret = quote! { -> #sys_ty };
		to_raw_conversion = ty.raw_conversion_fn(result_ident.clone());
	} else {
		ret = quote! {};
		to_raw_conversion = quote! {};
	}

	quote! {
		#[unsafe(no_mangle)]
		#[allow(non_snake_case)]
		pub unsafe extern "system" fn #fn_name(#(#sys_params),*) #ret {
			#(#arg_conversions)*

			let #result_ident = super::#fn_name(#(#all_param_names),*);
			#to_raw_conversion
		}
	}
}

fn path_str(path: &Path) -> Option<(String, bool)> {
	let mut s = String::new();
	let mut optional = false;

	let segments_len = path.segments.len();
	for (index, segment) in path.segments.iter().enumerate() {
		let segment_str = segment.ident.to_string();
		if segment_str == "Option" {
			optional = true;

			let PathArguments::AngleBracketed(args) = &segment.arguments else {
				return None;
			};

			if args.args.len() != 1 {
				return None;
			}

			let GenericArgument::Type(Type::Path(TypePath { path, .. })) = &args.args[0] else {
				return None;
			};

			s.push_str(&path_str(path)?.0);
		} else {
			s.push_str(&segment_str);
		}

		if index != segments_len - 1 {
			s.push_str("::");
		}
	}

	Some((s, optional))
}
