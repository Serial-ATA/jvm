use crate::parse::class::IMPORTS;
use crate::parse::{lex, path1, word1};

use std::fmt::Write;

use combine::parser::char::{char, string};
use combine::parser::combinator::no_partial;
use combine::{ParseError, Parser, Stream, choice, many1, opaque, optional, token, value};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
	Variadic(Box<Type>),
}

impl Type {
	pub fn occupies_two_stack_slots(&self) -> bool {
		matches!(self, Type::Long | Type::Double)
	}

	pub(crate) fn human_readable_name(&self) -> String {
		match self {
			Type::Boolean => String::from("bool"),
			Type::Byte => String::from("byte"),
			Type::Char => String::from("char"),
			Type::Double => String::from("double"),
			Type::Float => String::from("float"),
			Type::Int => String::from("int"),
			Type::Long => String::from("long"),
			Type::Short => String::from("short"),
			Type::Void => String::from("void"),
			Type::Class(class) => class.clone(),
			Type::Array(ty) => format!("{}_array", ty.human_readable_name()),
			Type::Variadic(ty) => format!("{}...", ty.human_readable_name()),
		}
	}

	pub(crate) fn write_to(&self, string: &mut String, use_imports: bool) {
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
				} else if use_imports {
					let guard = IMPORTS.lock().unwrap();
					let Some(name) = guard.get(name) else {
						panic!("Failed to find import {}", name);
					};

					write!(string, "L{};", name.replace('.', "/"))
				} else {
					write!(string, "{}", name.replace('.', "_"))
				}
			},
			Type::Array(ty) | Type::Variadic(ty) => {
				write!(string, "[").unwrap();
				return ty.write_to(string, use_imports);
			},
		}
		.unwrap();
	}

	#[allow(clippy::match_same_arms)]
	pub(crate) fn map_to_rust_ty(&self) -> String {
		match self {
			Type::Boolean => String::from("bool"),
			Type::Byte => String::from("::common::int_types::s1"),
			Type::Char => String::from("::common::int_types::u2"),
			Type::Double => String::from("f64"),
			Type::Float => String::from("f32"),
			Type::Int => String::from("::common::int_types::s4"),
			Type::Long => String::from("::common::int_types::s8"),
			Type::Short => String::from("::common::int_types::s2"),
			Type::Array(arr_ty) => String::from("&[") + &*arr_ty.map_to_rust_ty() + "]",
			Type::Class(_) => String::from("crate::objects::reference::Reference"),
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
		optional(lex(string("..."))),
		optional(many1::<Vec<&str>, _, _>(lex(string("[]")))),
	)
		.map(|(ty, varargs, arr)| match (varargs, arr) {
			(Some(_), Some(_)) => panic!("Cannot have both varargs and array"),
			(Some(_), None) => Type::Variadic(Box::new(ty)),
			(None, Some(arr)) => {
				let mut ty = Type::Array(Box::new(ty));
				for _ in 0..arr.len() - 1 {
					ty = Type::Array(Box::new(ty));
				}

				ty
			},
			(None, None) => ty,
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
