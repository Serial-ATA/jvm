use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, quote_spanned};
use syn::parse_macro_input;

mod call;

#[proc_macro_attribute]
pub fn jni_call(args: TokenStream, input: TokenStream) -> TokenStream {
	let mut no_env = false;
	let mut no_strict_types = false;
	let meta_parser = syn::meta::parser(|meta| {
		if meta.path.is_ident("no_env") {
			no_env = true;
			Ok(())
		} else if meta.path.is_ident("no_strict_types") {
			no_strict_types = true;
			Ok(())
		} else {
			Err(meta.error("unsupported property"))
		}
	});

	parse_macro_input!(args with meta_parser);
	let input = parse_macro_input!(input as syn::ItemFn);
	let call::JniFn {
		rust_fn,
		extern_fn,
		errors,
	} = call::generate(&input, no_env, no_strict_types);

	let errors = errors
		.into_iter()
		.map(|(err, span)| {
			let syn_err = err.into_syn(span).into_compile_error();
			quote_spanned! {span=>
				#syn_err
			}
		})
		.collect::<Vec<_>>();

	quote! {
		#(#errors)*

		#rust_fn
		#extern_fn
	}
	.into()
}

/// Collects all `#[jni_call]` annotated functions in the module and re-exports them under `self::raw`
#[proc_macro_attribute]
pub fn jni_fn_module(_attr: TokenStream, input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as syn::ItemMod);

	let Some((_, items)) = input.content else {
		return syn::Error::new_spanned(input, "cannot use #[jni_fn_module] on an empty module")
			.into_compile_error()
			.into();
	};

	let mut raw_mod_entries = Vec::new();
	for item in &items {
		let syn::Item::Fn(item_fn) = item else {
			continue;
		};

		if !item_fn
			.attrs
			.iter()
			.any(|a| a.meta.path().is_ident("jni_call"))
		{
			continue;
		}

		let name = item_fn.sig.ident.clone();
		let raw_ident = Ident::new(&format!("raw_{name}"), name.span());
		raw_mod_entries.push(quote_spanned! {raw_ident.span()=>
			pub use super::#raw_ident::*;
		});
	}

	let attrs = &input.attrs;
	let vis = &input.vis;
	let ident = &input.ident;
	quote! {
		#(#attrs)*
		#vis mod #ident {
			#(#items)*

			/// Generated C bindings for sun.io.fs.UnixNativeDispatcher methods
			#[allow(unused_imports)]
			pub mod raw {
				#(#raw_mod_entries)*
			}
		}
	}
	.into()
}
