use std::collections::HashSet;
use std::hash::Hash;
use std::path::PathBuf;
use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, LitByteStr, LitStr, Token};

/// A symbol, as defined in runtime/src/symbols
#[derive(Clone)]
struct SymbolDefinition {
	/// The enum variant name
	variant_name: Ident,
	/// The value that the symbol maps to, otherwise the variant name will be used
	value: Option<LitStr>,
}

impl SymbolDefinition {
	fn bstr(&self) -> LitByteStr {
		let value_str = self.str();

		match &self.value {
			Some(value) => LitByteStr::new(value_str.as_bytes(), value.span()),
			None => LitByteStr::new(value_str.as_bytes(), self.variant_name.span()),
		}
	}

	fn str(&self) -> String {
		match &self.value {
			Some(value) => value.value(),
			None => self.variant_name.to_string(),
		}
	}
}

impl Hash for SymbolDefinition {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match &self.value {
			Some(value) => value.hash(state),
			None => self.variant_name.hash(state),
		}
	}
}

impl PartialEq for SymbolDefinition {
	fn eq(&self, other: &Self) -> bool {
		match (&self.value, &other.value) {
			(Some(value), Some(other_value)) => value.value().eq(&other_value.value()),
			(Some(value), None) => value.value().eq(&other.variant_name.to_string()),
			(None, Some(other_value)) => other_value.value().eq(&self.variant_name.to_string()),
			(None, None) => self
				.variant_name
				.to_string()
				.eq(&other.variant_name.to_string()),
		}
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
		return Err(syn::Error::new(
			Span::call_site(),
			"Failed to find workspace root",
		));
	};

	Ok(workspace_root.join("generated").join("native"))
}

fn collect_symbols_from_files() -> syn::Result<TokenStream> {
	let generated_dir = get_generated_dir()?;
	if !generated_dir.exists() {
		return Err(syn::Error::new(
			Span::call_site(),
			format!(
				"Generated directory `{}` does not exist",
				generated_dir.display()
			),
		));
	}

	let entries = match std::fs::read_dir(&generated_dir) {
		Ok(entries) => entries,
		Err(e) => {
			return Err(syn::Error::new(
				Span::call_site(),
				format!(
					"Unable to read symbols from `{}`: {e}",
					generated_dir.display()
				),
			));
		},
	};

	let symbols_files = entries.into_iter().map(Result::unwrap).filter(|entry| {
		entry.file_type().unwrap().is_file()
			&& entry.path().extension().map(std::ffi::OsStr::to_str) == Some(Some("symbols"))
	});

	let mut tokenstream = TokenStream::new();
	for file in symbols_files {
		let content = match std::fs::read_to_string(file.path()) {
			Ok(content) => content,
			Err(e) => {
				return Err(syn::Error::new(
					Span::call_site(),
					format!(
						"Unable to read symbols from `{}`: {e}",
						file.path().display()
					),
				));
			},
		};

		tokenstream.extend(TokenStream::from_str(&content))
	}

	Ok(tokenstream)
}

fn collect_merged_symbols(predefined_symbols: TokenStream) -> syn::Result<Symbols> {
	let mut symbols: Symbols = syn::parse2(predefined_symbols.into())?;

	let generated_symbols_stream = collect_symbols_from_files()?;
	let generated_symbols: Symbols = syn::parse2(generated_symbols_stream.into())?;

	// TODO: Ideally, just create an alias so that:
	//
	// collides: "collides",
	// otherCollides: "collides",
	//
	// become:
	//
	// const collides: Symbol = Symbol(0);
	// const otherCollides: Symbol = collides;
	for generated_symbol in generated_symbols.0 {
		assert!(
			symbols.0.insert(generated_symbol.clone()),
			"Unable to insert generated symbol (name: `{}`, value: `{:?}`), collides with a \
			 predefined symbol",
			generated_symbol.variant_name,
			generated_symbol.value.map(|v| v.value())
		);
	}

	Ok(symbols)
}

#[proc_macro]
pub fn define_symbols(input: TokenStream) -> TokenStream {
	let symbols;
	match collect_merged_symbols(input) {
		Ok(s) => symbols = s,
		Err(e) => return e.to_compile_error().into(),
	}

	let mut index = 0u32;

	let mut symbol_value_stream = quote! {};
	let mut symbol_const_stream = quote! {};
	for symbol in symbols.0 {
		let name = symbol.variant_name.clone();
		let value_bstr = symbol.bstr();
		let value_str = symbol.str();

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
