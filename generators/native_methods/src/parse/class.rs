use crate::parse::access_flags::AccessFlags;
use crate::parse::field::Field;
use crate::parse::method::Method;
use crate::parse::{lex, path1, whitespace_or_comment, word1};

use std::collections::HashMap;
use std::sync::Mutex;

use combine::parser::char::{char, string};
use combine::parser::combinator::no_partial;
use combine::stream::position::Stream as PositionStream;
use combine::{
	attempt, choice, many, many1, opaque, optional, token, EasyParser, ParseError, Parser, Stream,
};
use once_cell::sync::Lazy;

pub(super) static IMPORTS: Lazy<Mutex<HashMap<String, String>>> =
	Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Debug)]
pub enum Member {
	Field(Field),
	Method(Method),
	Class(Class),
}

#[derive(Clone, Debug)]
pub struct Class {
	pub class_name: String,
	pub members: Vec<Member>,
}

impl Class {
	pub fn parse(text: String, name: &str, module: &str) -> Self {
		IMPORTS
			.lock()
			.unwrap()
			.insert(name.to_string(), format!("{}{}", module, name));

		let mut class;
		match class_file().easy_parse(PositionStream::new(&*text)) {
			Ok((c, _)) => class = c,
			Err(e) => {
				eprintln!("Failed to parse class definition `{}`:\n{}", name, e);
				std::process::exit(1);
			},
		}

		for member in &mut class.members {
			if let Member::Method(method) = member {
				assert!(
					method.modifiers.contains(AccessFlags::ACC_NATIVE),
					"Method `{}#{}` is not declared as native!",
					class.class_name,
					method.name
				);
			}

			if let Member::Class(subclass) = member {
				subclass.class_name = format!("{}${}", class.class_name, subclass.class_name)
			}
		}

		IMPORTS.lock().unwrap().clear();
		class
	}
}

fn class_file<Input>() -> impl Parser<Input, Output = Class>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(lex(imports()), lex(class())).map(|(_, class)| class)
}

fn class<Input>() -> impl Parser<Input, Output = Class>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	opaque!(no_partial((
		lex(class_def()),
		whitespace_or_comment(),
		optional(
			attempt(string("@Native")).and(many1::<Vec<_>, _, _>(attempt(
				super::field::field().map(Member::Field)
			)))
		),
		many::<Vec<_>, _, _>(
			attempt(super::method::method().map(Member::Method)).or(class().map(Member::Class))
		),
		lex(char('}')),
	)))
	.map(|(class_name, _, fields, mut members, _)| {
		if let Some((_, mut fields)) = fields {
			members.append(&mut fields);
		}

		Class {
			class_name,
			members,
		}
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
