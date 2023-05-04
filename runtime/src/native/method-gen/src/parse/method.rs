use crate::parse::access_flags::{modifier, AccessFlags};
use crate::parse::types::Type;
use crate::parse::{lex, whitespace_or_comment, word1};

use combine::parser::char::{char, string};
use combine::{many, many1, optional, sep_by, ParseError, Parser, Stream};

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
		crate::parse::types::ty(),
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
			(
				crate::parse::types::ty(),
				optional(lex(word1())),
				optional(lex(char(','))),
			)
				.map(|(ty, ..)| ty),
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
