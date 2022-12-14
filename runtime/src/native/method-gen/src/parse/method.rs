use crate::parse::class::IMPORTS;
use crate::parse::{lex, path1, whitespace_or_comment, word1};

use std::fmt::Write;

use combine::parser::char::{char, string};
use combine::parser::combinator::no_partial;
use combine::{
	attempt, choice, many, many1, opaque, optional, sep_by, token, value, ParseError, Parser,
	Stream,
};
use common::int_types::u2;

bitflags::bitflags! {
	pub struct AccessFlags: u2 {
		const ACC_PUBLIC       = 0x0001;
		const ACC_PRIVATE      = 0x0002;
		const ACC_PROTECTED    = 0x0004;
		const ACC_STATIC       = 0x0008;
		const ACC_FINAL        = 0x0010;
		const ACC_SYNCHRONIZED = 0x0020;
		const ACC_BRIDGE       = 0x0040; // Not used (Compiler generated)
		const ACC_VARARGS      = 0x0080;
		const ACC_NATIVE       = 0x0100;
		const ACC_ABSTRACT     = 0x0400;
		const ACC_STRICT       = 0x0800;
		const ACC_SYNTHETIC    = 0x1000; // Not used (Compiler generated)
	}
}

#[derive(Clone, Debug)]
enum Type {
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
	fn write_to(&self, string: &mut String) {
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
}

#[derive(Clone, Debug)]
pub struct Method {
	pub modifiers: AccessFlags,
	pub name: String,
	pub descriptor: String,
}

pub(crate) fn method<Input>() -> impl Parser<Input, Output = Method>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		whitespace_or_comment(),
		method_def(),
		optional(throws()),
		lex(char(';')),
	)
		.map(|(_, (modifiers, return_ty, name, params), ..)| {
			let mut signature = String::from('(');

			for param in params {
				param.write_to(&mut signature);
			}

			signature.push(')');
			return_ty.write_to(&mut signature);

			Method {
				modifiers,
				name,
				descriptor: signature,
			}
		})
}

fn method_def<Input>() -> impl Parser<Input, Output = (AccessFlags, Type, String, Vec<Type>)>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		many1::<Vec<AccessFlags>, _, _>(modifier()),
		ty(),
		word1(),
		method_parameters(),
	)
		.map(|(modifiers, return_ty, name, params)| {
			(
				modifiers.iter().fold(AccessFlags::empty(), |mut acc, x| {
					acc.insert(*x);
					acc
				}),
				return_ty,
				name,
				params,
			)
		})
}

fn method_parameters<Input>() -> impl Parser<Input, Output = Vec<Type>>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		lex(char('(')),
		many::<Vec<Type>, _, _>(
			(ty(), optional(lex(word1())), optional(lex(char(',')))).map(|(ty, ..)| ty),
		),
		lex(char(')')),
	)
		.map(|(_, tys, _)| tys)
}

fn modifier<Input>() -> impl Parser<Input, Output = AccessFlags>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		whitespace_or_comment(),
		choice((
			attempt(lex(string("public"))),
			attempt(lex(string("private"))),
			attempt(lex(string("protected"))),
			attempt(lex(string("static"))),
			attempt(lex(string("final"))),
			attempt(lex(string("synchronized"))),
			attempt(lex(string("native"))),
			attempt(lex(string("abstract"))),
			attempt(lex(string("strict"))),
		)),
	)
		.map(|(_, modifier)| match modifier {
			"public" => AccessFlags::ACC_PUBLIC,
			"private" => AccessFlags::ACC_PRIVATE,
			"protected" => AccessFlags::ACC_PROTECTED,
			"static" => AccessFlags::ACC_STATIC,
			"final" => AccessFlags::ACC_FINAL,
			"synchronized" => AccessFlags::ACC_SYNCHRONIZED,
			"native" => AccessFlags::ACC_NATIVE,
			"abstract" => AccessFlags::ACC_ABSTRACT,
			"strict" => AccessFlags::ACC_STRICT,
			c => unreachable!("{}", c),
		})
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

fn ty<Input>() -> impl Parser<Input, Output = Type>
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
