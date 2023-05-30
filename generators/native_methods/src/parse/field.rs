use crate::parse::access_flags::{modifier, AccessFlags};
use crate::parse::types::Type;
use crate::parse::{lex, whitespace_or_comment, word1};

use combine::parser::char::{char, string};
use combine::parser::repeat::take_until;
use combine::{many1, optional, ParseError, Parser, Stream};

#[derive(Clone, Debug)]
pub struct Field {
	pub name: String,
	pub ty: Type,
	pub expr: String,
}

pub(crate) fn field<Input>() -> impl Parser<Input, Output = Field>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		whitespace_or_comment(),
		// Optional since we already check for @Native in the class parser
		// OpenJDK only needs one @Native annotation
		optional(string("@Native")),
		whitespace_or_comment(),
		field_def(),
	)
		.map(|(_, _, _, (ty, name, expr))| Field { name, ty, expr })
}

fn field_def<Input>() -> impl Parser<Input, Output = (Type, String, String)>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		many1::<Vec<AccessFlags>, _, _>(modifier()),
		crate::parse::types::ty(),
		whitespace_or_comment(),
		lex(word1()),
		lex(char('=')),
		lex(take_until(char(';'))),
		lex(char(';')),
	)
		.map(|(_, return_ty, _, name, _, expr, _)| (return_ty, name, expr))
}
