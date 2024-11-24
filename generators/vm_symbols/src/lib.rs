use std::path::PathBuf;
use std::str::FromStr;
use std::collections::HashSet;
use std::hash::Hash;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, LitStr, LitByteStr, Token};

/// A symbol, as defined in runtime/src/symbols
struct SymbolDefinition {
	/// The enum variant name
	variant_name: Ident,
	/// The value that the symbol maps to, otherwise the variant name will be used
	value: Option<LitStr>,
}

impl Hash for SymbolDefinition {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.variant_name.hash(state);
	}
}

impl PartialEq for SymbolDefinition {
	fn eq(&self, other: &Self) -> bool {
		self.variant_name == other.variant_name
	}
}

impl Eq for SymbolDefinition {}

impl Parse for SymbolDefinition {
	fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
		let variant_name: Ident = input.parse()?;
		let value = match input.parse::<Token![:]>() {
			Ok(_) => Some(input.parse()?),
			Err(_) => None,
		};

		Ok(Self {
			variant_name,
			value,
		})
	}
}

struct Symbols(HashSet<SymbolDefinition>);

impl Parse for Symbols {
	fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
		let symbols: Punctuated<SymbolDefinition, Token![,]> = Punctuated::parse_terminated(input)?;
		Ok(Self(symbols.into_iter().collect()))
	}
}

const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn get_generated_dir() -> syn::Result<PathBuf> {
	let project_root = PathBuf::from(CRATE_ROOT);
	let Some(workspace_root) = project_root.ancestors().nth(2) else {
		return Err(syn::Error::new(Span::call_site(), "Failed to find workspace root"));
	};

	Ok(workspace_root.join("generated").join("native").to_path_buf())
}

fn collect_symbols_from_files(input: &mut TokenStream) -> syn::Result<()> {
	let generated_dir = get_generated_dir()?;
	if !generated_dir.exists() {
		return Err(syn::Error::new(Span::call_site(), format!("Generated directory `{}` does not exist", generated_dir.display())));
	}

	let entries = match std::fs::read_dir(&generated_dir) {
		Ok(entries) => entries,
		Err(e) => return Err(syn::Error::new(Span::call_site(), format!("Unable to read symbols from `{}`: {e}", generated_dir.display())))
	};

	let symbols_files = entries
		.into_iter()
		.map(Result::unwrap)
		.filter(|entry| {
			entry.file_type().unwrap().is_file()
				&& entry.path().extension().map(std::ffi::OsStr::to_str) == Some(Some("symbols"))
		});

	for file in symbols_files {
		let content = match std::fs::read_to_string(file.path()) {
			Ok(content) => content,
			Err(e) => return Err(syn::Error::new(Span::call_site(), format!("Unable to read symbols from `{}`: {e}", file.path().display()))),
		};

		input.extend(TokenStream::from_str(
			&content,
		))
	}

	Ok(())
}

#[proc_macro]
pub fn define_symbols(mut input: TokenStream) -> TokenStream {
	if let Err(err) = collect_symbols_from_files(&mut input) {
		return err.to_compile_error().into();
	}

	let symbols: Symbols = match syn::parse2(input.into()) {
		Ok(input) => input,
		Err(e) => {
			return e.to_compile_error().into();
		},
	};

	let mut index = 0u32;

	let mut symbol_value_stream = quote! {};
	let mut symbol_const_stream = quote! {};
	for symbol in symbols.0 {
		let name = symbol.variant_name;
		let value_bstr;
		let value_str;
		match symbol.value {
			Some(value) => {
				value_str = value.value();
				value_bstr = LitByteStr::new(value_str.as_bytes(), value.span());
			},
			None => {
				value_str = name.to_string();
				value_bstr = LitByteStr::new(value_str.as_bytes(), name.span())
			}
		}

		symbol_value_stream.extend(quote! {
			&#value_bstr[..],
		});

		symbol_const_stream.extend(quote! {
			#[doc = #value_str]
			pub const #name: Symbol = Symbol::new(#index);
		});

		index += 1;
	}

	quote! {
		const PREINTERED_SYMBOLS_COUNT: u32 = #index;

		#[allow(non_upper_case_globals)]
		#[doc(hidden)]
		pub mod generated_symbols {
			use super::Symbol;
			#symbol_const_stream
		}

		impl SymbolInterner {
			fn preintern(&mut self) {
				self.set.extend(&[
					#symbol_value_stream
				]);
			}
		}
	}
	.into()
}
