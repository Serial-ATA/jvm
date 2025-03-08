mod access_flags;
mod class;
mod field;
mod method;
mod types;

pub use access_flags::AccessFlags;
pub use class::{Class, Member};
pub use method::Method;
pub use types::Type;

use combine::parser::char::{space, string};
use combine::{
	attempt, many1, not_followed_by, satisfy, sep_by, skip_many, skip_many1, token, ParseError,
	Parser, Stream, StreamOnce,
};

fn lex<Input, P>(p: P) -> impl Parser<Input, Output = P::Output>
where
	P: Parser<Input>,
	Input: Stream<Token = char>,
	<Input as StreamOnce>::Error: ParseError<
		<Input as StreamOnce>::Token,
		<Input as StreamOnce>::Range,
		<Input as StreamOnce>::Position,
	>,
{
	p.skip(whitespace_or_comment())
}

fn whitespace_or_comment<Input>() -> impl Parser<Input>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	let comment = ((token('/'), token('/')), skip_many(satisfy(|c| c != '\n'))).map(|_| ());

	skip_many(skip_many1(space()).or(comment))
}

fn word1<Input>() -> impl Parser<Input, Output = String>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	many1::<String, _, _>(satisfy(|c: char| c.is_alphanumeric() || c == '_'))
}

fn path1<Input>() -> impl Parser<Input, Output = String>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	sep_by::<Vec<_>, _, _, _>(
		lex(word1()),
		attempt(token('.').skip(not_followed_by(lex(string(".."))))),
	)
	.map(|parts| parts.into_iter().intersperse(String::from(".")).collect())
}
