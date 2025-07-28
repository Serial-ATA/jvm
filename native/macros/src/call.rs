use proc_macro2::{Ident, Span};
use quote::{ToTokens, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Abi, FnArg, ItemFn, Pat, PatIdent, Path, ReturnType, Type, TypePath};

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

		#[derive(Copy, Clone)]
		enum SafeJniWrapperType {
			Primitive(Primitive),
			$($obj_ty),+
		}

		impl SafeJniWrapperType {
			fn from_str(s: &str) -> Option<Self> {
				match s {
					$(stringify!($sys_ty) | stringify!(jni::sys::$sys_ty) | stringify!(::jni::sys::$sys_ty) => Some(SafeJniWrapperType::Primitive(Primitive::$sys_ty)),)+
					$(stringify!($obj_sys_ty) | stringify!(jni::objects::$obj_sys_ty) | stringify!(::jni::objects::$obj_sys_ty)
					| stringify!($obj_ty) | stringify!(jni::objects::$obj_ty) | stringify!(::jni::objects::$obj_ty) => Some(SafeJniWrapperType::$obj_ty),)+
					_ => None,
				}
			}

			fn to_raw(self) -> proc_macro2::TokenStream {
				match self {
					SafeJniWrapperType::Primitive(p) => {
						let ident = p.as_ident();
						quote! {
							::jni::sys::#ident
						}
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

			fn to_safe(self) -> proc_macro2::TokenStream {
				match self {
					SafeJniWrapperType::Primitive(_) => self.to_raw(),
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

			fn raw_conversion_fn(self, param_name: Ident) -> proc_macro2::TokenStream {
				match self {
					SafeJniWrapperType::Primitive(_) => {
						quote_spanned!{param_name.span()=>
							::core::convert::identity(#param_name)
						}
					},
					$(
					SafeJniWrapperType::$obj_ty => {
						let raw = self.to_safe();
						quote_spanned! {param_name.span()=>
							#param_name.raw() as _
						}
					},
					)+
				}
			}

			fn safe_conversion_fn(self, param_name: Ident) -> proc_macro2::TokenStream {
				match self {
					SafeJniWrapperType::Primitive(p) => param_name.to_token_stream(),
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
			Error::BadAbi => syn::Error::new(span, "Must specify \"system\" ABI"),
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

pub fn generate(input: ItemFn) -> JniFn {
	let mut errors = Vec::new();
	validate_abi(&mut errors, &input);
	let params = validate_params(&mut errors, &input);
	let return_ty = validate_return(&mut errors, &input);

	let fn_name = &input.sig.ident;
	let raw_mod_name = Ident::new(&format!("raw_{}", fn_name), Span::call_site());

	let extern_fn_def = generate_extern_fn(&input, params, return_ty);
	let extern_fn = quote! {
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

	if name.value() != "system" {
		errors.push((Error::BadAbi, fun.sig.span()));
	}
}

fn validate_params(
	errors: &mut Vec<(Error, Span)>,
	fun: &ItemFn,
) -> Vec<(Ident, SafeJniWrapperType)> {
	let mut params = Vec::new();
	for (index, param) in fun.sig.inputs.iter().enumerate() {
		match param {
			FnArg::Receiver(receiver) => errors.push((Error::HasReceiver, receiver.span())),
			FnArg::Typed(arg) => {
				let Type::Path(TypePath { path, .. }) = &*arg.ty else {
					errors.push((
						Error::BadParameterType(arg.ty.to_token_stream().to_string()),
						arg.ty.span(),
					));
					continue;
				};

				let Pat::Ident(PatIdent { ident, .. }) = &*arg.pat else {
					errors.push((
						Error::BadParameterType(arg.ty.to_token_stream().to_string()),
						param.span(),
					));
					continue;
				};

				let path_str = path_str(path);
				if index == 0 {
					const JNI_ENV_PATHS: &[&str] = &["JniEnv", "jni::env::JniEnv"];

					if !JNI_ENV_PATHS.contains(&path_str.as_str()) {
						errors.push((Error::MissingEnv, fun.sig.span()));
						continue;
					}

					continue;
				}

				let Some(parsed) = SafeJniWrapperType::from_str(&path_str) else {
					errors.push((Error::BadParameterType(path_str), param.span()));
					continue;
				};

				params.push((ident.clone(), parsed));
			},
		}
	}

	params
}

fn validate_return(errors: &mut Vec<(Error, Span)>, fun: &ItemFn) -> Option<SafeJniWrapperType> {
	let ReturnType::Type(_, ty) = &fun.sig.output else {
		return None;
	};

	let Type::Path(TypePath { path, .. }) = &**ty else {
		errors.push((
			Error::BadReturnType(ty.to_token_stream().to_string()),
			ty.span(),
		));
		return None;
	};

	let path_str = path_str(path);
	let Some(parsed) = SafeJniWrapperType::from_str(&path_str) else {
		errors.push((Error::BadReturnType(path_str), ty.span()));
		return None;
	};

	Some(parsed)
}

fn path_str(path: &Path) -> String {
	let mut path_str = String::new();

	let segments_len = path.segments.len();
	for (index, segment) in path.segments.iter().enumerate() {
		path_str.push_str(&segment.ident.to_string());

		if index != segments_len - 1 {
			path_str.push_str("::");
		}
	}

	path_str
}

fn generate_extern_fn(
	fun: &ItemFn,
	params: Vec<(Ident, SafeJniWrapperType)>,
	return_type: Option<SafeJniWrapperType>,
) -> proc_macro2::TokenStream {
	let fn_name = &fun.sig.ident;

	let sys_params = params.iter().map(|(param, parsed_ty)| {
		let ty = parsed_ty.to_raw();
		quote! { #param: #ty }
	});

	let arg_conversions = params.iter().map(|(param, parsed_ty)| {
		let conversion = parsed_ty.safe_conversion_fn(param.clone());
		quote! { let #param = #conversion; }
	});

	let all_param_names = params.iter().map(|(param, _)| param);

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
		pub extern "system" fn #fn_name(env: *mut ::jni::sys::JNIEnv, #(#sys_params),*) #ret {
			let env = unsafe { ::jni::env::JniEnv::from_raw(env) };
			#(#arg_conversions)*

			let #result_ident = super::#fn_name(env, #(#all_param_names),*);
			#to_raw_conversion
		}
	}
}
