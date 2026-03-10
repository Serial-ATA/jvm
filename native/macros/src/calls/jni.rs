use super::{CallType, Error};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
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

	fn to_raw(&self) -> TokenStream {
		match self {
			ParameterType::Env => quote! { *mut ::jni::sys::JNIEnv },
			ParameterType::Strict(inner) => inner.to_raw(),
			ParameterType::Loose(inner) => inner.to_token_stream(),
		}
	}

	fn to_safe(&self) -> TokenStream {
		match self {
			ParameterType::Env => quote! { ::jni::env::JniEnv },
			ParameterType::Strict(inner) => inner.to_safe(),
			ParameterType::Loose(inner) => inner.to_token_stream(),
		}
	}

	fn raw_conversion_fn(&self, param_name: Ident) -> TokenStream {
		match self {
			ParameterType::Env => quote! { #param_name.raw() },
			ParameterType::Strict(inner) => inner.raw_conversion_fn(param_name),
			ParameterType::Loose(_inner) => param_name.to_token_stream(),
		}
	}

	fn safe_conversion_fn(&self, param_name: Ident) -> TokenStream {
		match self {
			ParameterType::Env => quote! { unsafe { ::jni::env::JniEnv::from_raw(#param_name) } },
			ParameterType::Strict(inner) => inner.safe_conversion_fn(param_name),
			ParameterType::Loose(_inner) => param_name.to_token_stream(),
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
	super::validate_abi(&mut errors, input);
	let params = super::validate_params(&mut errors, input, no_env, no_strict_types, CallType::Jni);
	let return_ty = validate_return(&mut errors, input, no_strict_types);

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

	JniFn {
		rust_fn,
		extern_fn,
		errors,
	}
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
			Error::BadReturnType {
				ty: ty.to_token_stream().to_string(),
				call_type: CallType::Jni,
			},
			ty.span(),
		));
		return None;
	};

	let Some((path_str, optional)) = super::path_str(path) else {
		errors.push((
			Error::BadReturnType {
				ty: ty.to_token_stream().to_string(),
				call_type: CallType::Jni,
			},
			ty.span(),
		));
		return None;
	};
	let Some(parsed) = SafeJniWrapperType::from_str(&path_str, optional) else {
		errors.push((
			Error::BadReturnType {
				ty: path_str,
				call_type: CallType::Jni,
			},
			ty.span(),
		));
		return None;
	};

	Some(ParameterType::Strict(parsed))
}
