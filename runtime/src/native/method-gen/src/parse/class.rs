use crate::parse::method::{AccessFlags, Method};
use crate::parse::{lex, word1};

use combine::parser::char::{char, string};
use combine::stream::position::Stream as PositionStream;
use combine::{choice, many, many1, EasyParser, ParseError, Parser, Stream};

#[derive(Clone, Debug)]
pub struct Class {
	pub class_name: String,
	pub methods: Vec<Method>,
}

impl Class {
	pub fn parse(text: String) -> Self {
		let class = class().easy_parse(PositionStream::new(&*text)).unwrap().0;

		for method in &class.methods {
			assert!(
				method.modifiers.contains(AccessFlags::ACC_NATIVE),
				"Method `{}#{}` is not declared as native!",
				class.class_name,
				method.name
			);
		}

		class
	}
}

fn class<Input>() -> impl Parser<Input, Output = Class>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		lex(class_def()),
		many(super::method::method()),
		lex(char('}')),
	)
		.map(|(class_name, methods, _)| Class {
			class_name,
			methods,
		})
}

fn class_def<Input>() -> impl Parser<Input, Output = String>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		many1::<Vec<&str>, _, _>(choice((
			lex(string("public")),
			lex(string("final")),
			lex(string("abstract")),
			lex(string("sealed")),
		))),
		choice((
			lex(string("class")),
			lex(string("interface")),
			lex(string("enum")),
		)),
		lex(word1()),
		char('{'),
	)
		.map(|(_, _, name, ..)| name)
}
