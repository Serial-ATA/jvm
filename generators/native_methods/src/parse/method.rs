use crate::parse::access_flags::{access_flags, AccessFlags};
use crate::parse::types::{ty, Type};
use crate::parse::{lex, word1, Class};

use std::fmt::Write;

use combine::parser::char::{char, string};
use combine::{optional, sep_by, ParseError, Parser, Stream};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Method {
	pub modifiers: AccessFlags,
	pub is_intrinsic_candidate: bool,
	pub name: Option<String>, // Either a method name or `None` if its a constructor
	pub descriptor: String,
	pub params: Vec<Type>,
	pub return_ty: Type,
}

impl Method {
	pub fn name(&self) -> String {
		match &self.name {
			Some(name) => name.clone(),
			None => String::new(),
		}
	}

	/// The symbol for the class name + method name
	///
	/// For example, `java.lang.Object#hashCode` would become `Object_hashCode`.
	///
	/// If the class makes use of overloading, the parameters will be appended to the name.
	pub fn full_name_symbol(&self, class: &Class) -> String {
		let append_params = class.methods().any(|method| {
			method != self && method.is_intrinsic_candidate && self.name == method.name
		});

		let mut ret = match &self.name {
			Some(name) => format!("{}_{name}", class.class_name.replace('$', "_")),
			None => {
				let mut ret = String::new();
				self.return_ty.write_to(&mut ret, false);
				if self.params.is_empty() && append_params {
					return format!("{ret}_void");
				}

				ret
			},
		};

		if append_params {
			ret = format!("{ret}_");
			for param in &self.params {
				write!(ret, "{}", param.human_readable_name()).unwrap();
			}
		}

		ret
	}

	pub fn generated_name(&self) -> &str {
		match &self.name {
			Some(name) => name,
			None => "<init>",
		}
	}

	pub fn name_symbol(&self) -> String {
		let name = self.generated_name();
		if name == "<init>" {
			return String::from("object_initializer_name");
		}

		format!("{name}_name")
	}

	pub fn signature_symbol_name(&self) -> String {
		let mut signature_symbol = String::new();
		for param in &self.params {
			signature_symbol.push_str(&format!(
				"{}_",
				param.human_readable_name().replace('.', "_")
			));
		}

		signature_symbol.push_str(&self.return_ty.human_readable_name().replace('.', "_"));
		signature_symbol.push_str("_signature");

		// TODO: Hack till SymbolCollector can recognize duplicate values
		if signature_symbol == "void_signature" {
			return String::from("void_method_signature");
		}

		signature_symbol
	}

	pub fn intrinsic_flags(&self) -> &'static str {
		// If a method's access flags do not intersect with this, then it is considered regular
		const STATIC_NATIVE_SYNCHRONIZED: AccessFlags = AccessFlags::from_bits_retain(
			AccessFlags::ACC_STATIC.bits()
				| AccessFlags::ACC_NATIVE.bits()
				| AccessFlags::ACC_SYNCHRONIZED.bits(),
		);

		const STATIC: AccessFlags = AccessFlags::ACC_STATIC;
		const SYNCHRONIZED: AccessFlags = AccessFlags::ACC_SYNCHRONIZED;
		const NATIVE: AccessFlags = AccessFlags::ACC_NATIVE;
		const STATIC_NATIVE: AccessFlags = AccessFlags::from_bits_retain(
			AccessFlags::ACC_STATIC.bits() | AccessFlags::ACC_NATIVE.bits(),
		);

		if !self.modifiers.intersects(STATIC_NATIVE_SYNCHRONIZED) {
			return "IntrinsicFlags::Regular";
		}

		if self.modifiers.contains(STATIC)
			&& !self.modifiers.contains(NATIVE)
			&& !self.modifiers.contains(SYNCHRONIZED)
		{
			return "IntrinsicFlags::Static";
		}

		if self.modifiers.contains(SYNCHRONIZED) && !self.modifiers.intersects(STATIC_NATIVE) {
			return "IntrinsicFlags::Synchronized";
		}

		if self.modifiers.contains(NATIVE)
			&& !self.modifiers.contains(STATIC)
			&& !self.modifiers.contains(SYNCHRONIZED)
		{
			return "IntrinsicFlags::Native";
		}

		if self.modifiers.contains(STATIC_NATIVE) && !self.modifiers.contains(SYNCHRONIZED) {
			return "IntrinsicFlags::Native";
		}

		panic!("Method contains no relevant modifiers, see `IntrinsicFlags`");
	}
}

pub(crate) fn constructor<Input>(annotation: Option<&str>) -> impl Parser<Input, Output = Method>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	assert!(matches!(annotation, Some("@IntrinsicCandidate") | None));

	let is_intrinsic = annotation.is_some();

	(
		access_flags(),
		lex(word1()),
		method_parameters(),
		optional(throws()),
		lex(char(';')),
	)
		.message("While parsing method")
		.map(move |(modifiers, name, params, ..)| {
			let return_ty = Type::Class(name);

			Method {
				modifiers,
				is_intrinsic_candidate: is_intrinsic,
				name: None,
				descriptor: create_signature(params.clone(), return_ty.clone()),
				params,
				return_ty,
			}
		})
}

pub(crate) fn method<Input>(annotation: Option<&str>) -> impl Parser<Input, Output = Method>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	assert!(matches!(annotation, Some("@IntrinsicCandidate") | None));

	let is_intrinsic = annotation.is_some();

	(lex(method_def()), optional(throws()), lex(char(';')))
		.message("While parsing method")
		.map(move |((modifiers, return_ty, name, params), ..)| Method {
			modifiers,
			is_intrinsic_candidate: is_intrinsic,
			name: Some(name),
			descriptor: create_signature(params.clone(), return_ty.clone()),
			params,
			return_ty,
		})
}

fn method_def<Input>() -> impl Parser<Input, Output = (AccessFlags, Type, String, Vec<Type>)>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(access_flags(), lex(ty()), lex(word1()), method_parameters())
}

fn method_parameters<Input>() -> impl Parser<Input, Output = Vec<Type>>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		lex(char('(')),
		sep_by::<Vec<_>, _, _, _>(
			(crate::parse::types::ty(), optional(lex(word1()))).map(|(ty, _)| ty),
			lex(char(',')),
		),
		lex(char(')')),
	)
		.map(|(_, tys, _)| tys)
}

fn throws<Input>() -> impl Parser<Input, Output = ()>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		lex(string("throws")),
		sep_by::<Vec<String>, _, _, _>(lex(word1()), lex(char(','))),
	)
		.map(|_| ())
}

fn create_signature(params: Vec<Type>, return_ty: Type) -> String {
	let mut signature = String::from('(');

	for param in params {
		param.write_to(&mut signature, true);
	}

	signature.push(')');
	return_ty.write_to(&mut signature, true);
	signature
}
