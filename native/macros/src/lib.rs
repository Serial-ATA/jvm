use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse_macro_input;

mod calls;

fn quote_errors(errors: Vec<(calls::Error, proc_macro2::Span)>) -> Vec<proc_macro2::TokenStream> {
	errors
		.into_iter()
		.map(|(err, span)| {
			let syn_err = err.into_syn(span).into_compile_error();
			quote_spanned! {span=>
				#syn_err
			}
		})
		.collect::<Vec<_>>()
}

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
	let calls::jni::JniFn { rust_fn, errors } =
		calls::jni::generate(&input, no_env, no_strict_types);

	let errors = quote_errors(errors);
	quote! {
		#(#errors)*

		#rust_fn
	}
	.into()
}

#[proc_macro_attribute]
pub fn jvmti_call(_args: TokenStream, input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as syn::ItemFn);
	let calls::jvmti::JvmtiFn { rust_fn, errors } = calls::jvmti::generate(&input);

	let errors = quote_errors(errors);
	quote! {
		#(#errors)*

		#rust_fn
	}
	.into()
}
