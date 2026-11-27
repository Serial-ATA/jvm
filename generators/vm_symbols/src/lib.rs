use std::collections::HashSet;
use std::hash::Hash;

use proc_macro::TokenStream;
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

#[proc_macro]
pub fn define_symbols(input: TokenStream) -> TokenStream {
	let symbols: Symbols = match syn::parse2(input.into()) {
		Ok(symbols) => symbols,
		Err(e) => return e.to_compile_error().into(),
	};

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
