use crate::parse::access_flags::access_flags;
use crate::parse::types::{ty, Type};
use crate::parse::{lex, word1};

use combine::parser::char::char;
use combine::parser::repeat::take_until;
use combine::{struct_parser, ParseError, Parser, Stream};

#[derive(Clone, Debug)]
pub struct Field {
	pub name: String,
	pub ty: Type,
	pub expr: String,
}

pub(crate) fn field<Input>(annotation: Option<&str>) -> impl Parser<Input, Output = Field>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	assert!(
		matches!(annotation, Some("@Native") | None),
		"Annotation should be @Native or None, found {:?}",
		annotation
	);

	struct_parser!(
		Field {
			_: access_flags(),
			ty: lex(ty()),
			name: lex(word1()),
			_: lex(char('=')),
			expr: lex(take_until(char(';'))),
			_: lex(char(';')),
		}
	)
}
