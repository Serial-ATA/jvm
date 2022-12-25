use crate::parse::method::{AccessFlags, Method};
use crate::parse::{lex, path1, word1};

use std::collections::HashMap;
use std::sync::Mutex;

use combine::parser::char::{char, string};
use combine::stream::position::Stream as PositionStream;
use combine::{choice, many, many1, token, EasyParser, ParseError, Parser, Stream};
use once_cell::sync::Lazy;

pub(super) static IMPORTS: Lazy<Mutex<HashMap<String, String>>> =
	Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Debug)]
pub struct Class {
	pub class_name: String,
	pub methods: Vec<Method>,
}

impl Class {
	pub fn parse(text: String, name: &str, module: &str) -> Self {
		IMPORTS
			.lock()
			.unwrap()
			.insert(name.to_string(), format!("{}{}", module, name));

		let class = class().easy_parse(PositionStream::new(&*text)).unwrap().0;

		for method in &class.methods {
			assert!(
				method.modifiers.contains(AccessFlags::ACC_NATIVE),
				"Method `{}#{}` is not declared as native!",
				class.class_name,
				method.name
			);
		}

		IMPORTS.lock().unwrap().clear();
		class
	}
}

fn class<Input>() -> impl Parser<Input, Output = Class>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		lex(imports()),
		lex(class_def()),
		many(super::method::method()),
		lex(char('}')),
	)
		.map(|(_, class_name, methods, _)| Class {
			class_name,
			methods,
		})
}

fn imports<Input>() -> impl Parser<Input, Output = ()>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	lex(many::<Vec<String>, _, _>(
		(lex(string("import")), lex(path1()), lex(token(';'))).map(|(_, import, _)| import),
	))
	.map(|imports| {
		for import in imports {
			let last_dot_pos = import.rfind('.').unwrap();
			let class_name = import.get(last_dot_pos + 1..).unwrap();
			IMPORTS
				.lock()
				.unwrap()
				.insert(class_name.to_string(), import);
		}
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
