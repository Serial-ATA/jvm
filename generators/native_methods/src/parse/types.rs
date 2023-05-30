use crate::parse::class::IMPORTS;
use crate::parse::{lex, path1, word1};

use std::fmt::Write;

use combine::parser::char::{char, string};
use combine::parser::combinator::no_partial;
use combine::{choice, many1, opaque, optional, token, value, ParseError, Parser, Stream};

#[derive(Clone, Debug)]
pub enum Type {
	Boolean,
	Byte,
	Char,
	Double,
	Float,
	Int,
	Long,
	Short,
	Void,
	Class(String),
	Array(Box<Type>),
}

impl Type {
	pub(crate) fn write_to(&self, string: &mut String) {
		match self {
			Type::Boolean => write!(string, "Z"),
			Type::Byte => write!(string, "B"),
			Type::Char => write!(string, "C"),
			Type::Double => write!(string, "D"),
			Type::Float => write!(string, "F"),
			Type::Int => write!(string, "I"),
			Type::Long => write!(string, "J"),
			Type::Short => write!(string, "S"),
			Type::Void => write!(string, "V"),
			Type::Class(name) => {
				if name.contains('.') {
					// Trust we were provided a full path
					write!(string, "L{};", name)
				} else {
					write!(
						string,
						"L{};",
						IMPORTS.lock().unwrap()[name].replace('.', "/")
					)
				}
			},
			Type::Array(ty) => {
				write!(string, "[").unwrap();
				return ty.write_to(string);
			},
		}
		.unwrap();
	}

	#[allow(clippy::match_same_arms)]
	pub(crate) fn map_to_rust_ty(&self) -> String {
		match self {
			Type::Boolean => String::from("bool"),
			Type::Byte => String::from("::common::int_types::s1"),
			Type::Char => String::from("::common::int_types::s4"),
			Type::Double => String::from("f64"),
			Type::Float => String::from("f32"),
			Type::Int => String::from("::common::int_types::s4"),
			Type::Long => String::from("::common::int_types::s8"),
			Type::Short => String::from("::common::int_types::s2"),
			Type::Array(arr_ty) => String::from("&[") + &*arr_ty.map_to_rust_ty() + "]",
			ty => unimplemented!("{ty:?} as rust type"),
		}
	}
}

pub fn ty<Input>() -> impl Parser<Input, Output = Type>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		choice((
			(
				lex(token('b')),
				choice((
					lex(string("oolean")).then(|_| value(Type::Boolean)),
					lex(string("yte")).then(|_| value(Type::Byte)),
				)),
			)
				.map(|(_, ty)| ty),
			lex(string("char")).then(|_| value(Type::Char)),
			lex(string("double")).then(|_| value(Type::Double)),
			lex(string("float")).then(|_| value(Type::Float)),
			lex(string("int")).then(|_| value(Type::Int)),
			lex(string("long")).then(|_| value(Type::Long)),
			lex(string("short")).then(|_| value(Type::Short)),
			lex(string("void")).then(|_| value(Type::Void)),
			class_ty(),
		)),
		optional(many1::<Vec<&str>, _, _>(lex(string("[]")))),
	)
		.map(|(ty, arr)| match arr {
			Some(arr) => {
				let mut ty = Type::Array(Box::new(ty));
				for _ in 0..arr.len() - 1 {
					ty = Type::Array(Box::new(ty));
				}

				ty
			},
			None => ty,
		})
}

fn class_ty<Input>() -> impl Parser<Input, Output = Type>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(lex(path1()), optional(generics())).map(|(class_name, _)| Type::Class(class_name))
}

fn generics<Input>() -> impl Parser<Input, Output = ()>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	opaque!(no_partial((
		lex(char('<')),
		choice((wildcard_generics(), class_name_with_generics())),
		lex(char('>')),
	)))
	.map(|_| ())
}

// Class<?>
// Class<? extends Qux>
// etc...
fn wildcard_generics<Input>() -> impl Parser<Input, Output = ()>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	opaque!(no_partial((
		lex(char('?')),
		optional((
			choice((lex(string("extends")), lex(string("super")))),
			class_name_with_generics(),
		)),
	)))
	.map(|_| ())
}

// Class<Foo>
// Class<Foo<Bar>
// Class<Foo<? extends Qux>>
// etc...
fn class_name_with_generics<Input>() -> impl Parser<Input, Output = ()>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(lex(word1()), optional(generics())).map(|_| ())
}
