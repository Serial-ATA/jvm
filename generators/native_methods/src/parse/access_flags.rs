use crate::parse::lex;

use combine::parser::char::string;
use combine::{attempt, choice, many1, ParseError, Parser, Stream};

use common::int_types::u2;

bitflags::bitflags! {
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

pub fn access_flags<Input>() -> impl Parser<Input, Output = AccessFlags>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	many1::<Vec<AccessFlags>, _, _>(modifier()).map(|flags| {
		flags
			.iter()
			.fold::<AccessFlags, _>(AccessFlags::empty(), |mut acc, x| {
				acc.insert(*x);
				acc
			})
	})
}

pub fn modifier<Input>() -> impl Parser<Input, Output = AccessFlags>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	lex(choice((
		attempt(lex(string("public"))),
		attempt(lex(string("private"))),
		attempt(lex(string("protected"))),
		attempt(lex(string("static"))),
		attempt(lex(string("final"))),
		attempt(lex(string("synchronized"))),
		attempt(lex(string("native"))),
		attempt(lex(string("abstract"))),
		attempt(lex(string("strict"))),
	)))
	.map(|modifier| match modifier {
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
