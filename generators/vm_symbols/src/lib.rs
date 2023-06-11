use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, LitStr, Token};

/// A symbol, as defined in runtime/src/symbols
struct SymbolDefinition {
	/// The enum variant name
	variant_name: Ident,
	/// The value that the symbol maps to, otherwise the variant name will be used
	value: Option<LitStr>,
}

impl Parse for SymbolDefinition {
	fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
		let variant_name = input.parse()?;
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

struct Symbols(Punctuated<SymbolDefinition, Token![,]>);

impl Parse for Symbols {
	fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
		Ok(Self(Punctuated::parse_terminated(input)?))
	}
}

#[proc_macro]
pub fn define_symbols(input: TokenStream) -> TokenStream {
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
		let name = &symbol.variant_name;
		let value = symbol.value.map_or_else(|| name.to_string(), |value| value.value());

		symbol_value_stream.extend(quote! {
			#value,
		});

		symbol_const_stream.extend(quote! {
			pub const #name: Symbol = Symbol::new(#index);
		});

		index += 1;
	}

	quote! {
		const PREINTERED_SYMBOLS_COUNT: u32 = #index;

		#[allow(non_upper_case_globals)]
		mod generated_symbols {
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
	}.into()
}
