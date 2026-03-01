use proc_macro2::{Ident, Span};
use quote::{ToTokens, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
	Abi, FnArg, GenericArgument, ItemFn, Pat, PatIdent, PatType, Path, PathArguments, ReturnType,
	Type, TypePath,
};

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

		impl Primitive {
			fn as_ident(&self) -> Ident {
				match self {
					$(
					Primitive::$sys_ty => {
						Ident::new(stringify!($sys_ty), Span::call_site())
					}
					)+
				}
			}
		}

		#[derive(Clone)]
		enum SafeJniWrapperType {
			Primitive(Primitive),
            Optional(Box<SafeJniWrapperType>),
			$($obj_ty),+
		}

		impl SafeJniWrapperType {
			fn from_str(s: &str, optional: bool) -> Option<Self> {
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

			fn to_raw(&self) -> proc_macro2::TokenStream {
				match self {
					SafeJniWrapperType::Primitive(p) => {
						let ident = p.as_ident();
						quote! {
							::jni::sys::#ident
						}
					},
                    SafeJniWrapperType::Optional(inner) => {
                        inner.to_raw()
                    },
					$(
					SafeJniWrapperType::$obj_ty => {
						let ty = Ident::new(stringify!($obj_sys_ty), Span::call_site()).to_token_stream();
						quote! {
							::jni::sys::#ty
						}
					},
					)+
				}
			}

			fn to_safe(&self) -> proc_macro2::TokenStream {
				match self {
					SafeJniWrapperType::Primitive(_) => self.to_raw(),
                    SafeJniWrapperType::Optional(inner) => {
                        let inner_safe = inner.to_safe();
                        quote! {
                            core::option::Option<#inner_safe>
                        }
                    },
					$(
					SafeJniWrapperType::$obj_ty => {
						let ty = Ident::new(stringify!($obj_ty), Span::call_site()).to_token_stream();
						quote! {
							::jni::objects::#ty
						}
					},
					)+
				}
			}

			fn raw_conversion_fn(&self, param_name: Ident) -> proc_macro2::TokenStream {
				match self {
					SafeJniWrapperType::Primitive(_) => {
						quote_spanned!{param_name.span()=>
							::core::convert::identity(#param_name)
						}
					},
                    SafeJniWrapperType::Optional(inner) => {
                        let unwrapped = Ident::new("unwrapped", Span::call_site());
                        let inner_raw = inner.raw_conversion_fn(unwrapped.clone());
						quote_spanned! {param_name.span()=>
							match #param_name {
                                Some(#unwrapped) => #inner_raw,
                                None => ::core::ptr::null_mut() as _,
                            }
						}
                    },
					$(
					SafeJniWrapperType::$obj_ty => {
						quote_spanned! {param_name.span()=>
							#param_name.raw() as _
						}
					},
					)+
				}
			}

			fn safe_conversion_fn(&self, param_name: Ident) -> proc_macro2::TokenStream {
				match self {
					SafeJniWrapperType::Primitive(_) => param_name.to_token_stream(),
                    SafeJniWrapperType::Optional(_inner) => {
                        unimplemented!("Optional object parameters");
                    },
					$(
					SafeJniWrapperType::$obj_ty => {
						let raw = self.to_safe();
						quote_spanned! {param_name.span()=>
							unsafe { #raw::from_raw(#param_name) }
						}
					},
					)+
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

enum ParameterType {
	Strict(SafeJniWrapperType),
	Loose(Box<Type>),
}

impl ParameterType {
	fn to_raw(&self) -> proc_macro2::TokenStream {
		match self {
			ParameterType::Strict(inner) => inner.to_raw(),
			ParameterType::Loose(inner) => inner.to_token_stream(),
		}
	}

	fn safe_conversion_fn(&self, param_name: Ident) -> proc_macro2::TokenStream {
		match self {
			ParameterType::Strict(inner) => inner.safe_conversion_fn(param_name),
			ParameterType::Loose(_inner) => param_name.to_token_stream(),
		}
	}

	fn raw_conversion_fn(&self, param_name: Ident) -> proc_macro2::TokenStream {
		match self {
			ParameterType::Strict(inner) => inner.raw_conversion_fn(param_name),
			ParameterType::Loose(_inner) => param_name.to_token_stream(),
		}
	}
}

pub enum Error {
	MissingAbi,
	BadAbi,
	MissingEnv,
	HasReceiver,
	BadParameterType(String),
	BadReturnType(String),
}

impl Error {
	pub fn into_syn(self, span: Span) -> syn::Error {
		match self {
			Error::MissingAbi => syn::Error::new(span, "Must specify an ABI"),
			Error::BadAbi => syn::Error::new(span, "Must specify valid ABI"),
			Error::MissingEnv => syn::Error::new(span, "First parameter must be a JniEnv"),
			Error::HasReceiver => syn::Error::new(span, "JNI functions must be freestanding"),
			Error::BadParameterType(ty) => syn::Error::new(
				span,
				format!("Bad parameter type `{ty}`, must be one of the `jni_sys` primitives"),
			),
			Error::BadReturnType(ty) => syn::Error::new(
				span,
				format!("Bad return type `{ty}`, must be one of the `jni_sys` primitives"),
			),
		}
	}
}

pub struct JniFn {
	pub rust_fn: proc_macro2::TokenStream,
	pub extern_fn: proc_macro2::TokenStream,
	pub errors: Vec<(Error, Span)>,
}

pub fn generate(input: &ItemFn, no_env: bool, no_strict_types: bool) -> JniFn {
	let mut errors = Vec::new();
	validate_abi(&mut errors, input);
	let params = validate_params(&mut errors, input, no_env, no_strict_types);
	let return_ty = validate_return(&mut errors, input, no_strict_types);

	let fn_name = &input.sig.ident;
	let raw_mod_name = Ident::new(&format!("raw_{}", fn_name), Span::call_site());

	let extern_fn_def = generate_extern_fn(input, &params, return_ty, no_env);
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

	JniFn {
		rust_fn,
		extern_fn,
		errors,
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

fn validate_params(
	errors: &mut Vec<(Error, Span)>,
	fun: &ItemFn,
	no_env: bool,
	no_strict_types: bool,
) -> Vec<(Ident, ParameterType)> {
	fn parse_type(
		fun: &ItemFn,
		no_env: bool,
		index: usize,
		param: &FnArg,
		arg: &PatType,
	) -> Result<Option<SafeJniWrapperType>, (Error, Span)> {
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

			if !JNI_ENV_PATHS.contains(&path_str.as_str()) {
				return Err((Error::MissingEnv, fun.sig.span()));
			}

			// Implicit
			return Ok(None);
		}

		match SafeJniWrapperType::from_str(&path_str, false) {
			Some(parsed) => Ok(Some(parsed)),
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

				match parse_type(fun, no_env, index, param, arg) {
					Ok(Some(parsed)) => params.push((ident.clone(), ParameterType::Strict(parsed))),
					Ok(None) => {},
					Err(e) => {
						if no_strict_types {
							params.push((ident.clone(), ParameterType::Loose(arg.ty.clone())));
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

fn validate_return(
	errors: &mut Vec<(Error, Span)>,
	fun: &ItemFn,
	no_strict_types: bool,
) -> Option<ParameterType> {
	let ReturnType::Type(_, ty) = &fun.sig.output else {
		return None;
	};

	if no_strict_types {
		return Some(ParameterType::Loose(ty.clone()));
	}

	let Type::Path(TypePath { path, .. }) = &**ty else {
		errors.push((
			Error::BadReturnType(ty.to_token_stream().to_string()),
			ty.span(),
		));
		return None;
	};

	let Some((path_str, optional)) = path_str(path) else {
		errors.push((
			Error::BadReturnType(ty.to_token_stream().to_string()),
			ty.span(),
		));
		return None;
	};
	let Some(parsed) = SafeJniWrapperType::from_str(&path_str, optional) else {
		errors.push((Error::BadReturnType(path_str), ty.span()));
		return None;
	};

	Some(ParameterType::Strict(parsed))
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

fn generate_extern_fn(
	fun: &ItemFn,
	params: &[(Ident, ParameterType)],
	return_type: Option<ParameterType>,
	no_env: bool,
) -> proc_macro2::TokenStream {
	let fn_name = &fun.sig.ident;

	let mut sys_params = Vec::new();
	if !no_env {
		sys_params.push(quote! { env: *mut ::jni::sys::JNIEnv });
	}

	sys_params.extend(params.iter().map(|(param, parsed_ty)| {
		let ty = parsed_ty.to_raw();
		quote! { #param: #ty }
	}));

	let mut arg_conversions = Vec::new();
	if !no_env {
		arg_conversions.push(quote! { let env = unsafe { ::jni::env::JniEnv::from_raw(env) }; });
	}

	arg_conversions.extend(params.iter().map(|(param, parsed_ty)| {
		let conversion = parsed_ty.safe_conversion_fn(param.clone());
		quote! { let #param = #conversion; }
	}));

	let mut all_param_names = Vec::new();
	if !no_env {
		all_param_names.push(Ident::new("env", Span::call_site()));
	}

	all_param_names.extend(params.iter().map(|(param, _)| param.clone()));

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
