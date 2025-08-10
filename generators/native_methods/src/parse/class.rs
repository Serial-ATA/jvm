use crate::parse::access_flags::AccessFlags;
use crate::parse::field::Field;
use crate::parse::method::Method;
use crate::parse::{lex, path1, word1};

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use combine::parser::char::{char, string};
use combine::parser::combinator::{no_partial, opaque};
use combine::stream::position::Stream as PositionStream;
use combine::{
	EasyParser, ParseError, Parser, Stream, attempt, choice, dispatch, many, many1, optional,
	token, value,
};

pub(super) static IMPORTS: LazyLock<Mutex<HashMap<String, String>>> =
	LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Member {
	Field(Field),
	Method(Method),
	Class(Class),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Class {
	pub class_name: String,
	pub members: Vec<Member>,
}

impl Class {
	pub fn methods(&self) -> impl Iterator<Item = &Method> {
		self.members.iter().filter_map(|member| match member {
			Member::Method(method) => Some(method),
			_ => None,
		})
	}

	pub fn sanitized_class_name(&self) -> String {
		self.class_name.replace('$', "_")
	}
}

impl Class {
	pub fn parse(text: &str, name: &str, module: &str) -> Self {
		IMPORTS
			.lock()
			.unwrap()
			.insert(name.to_string(), format!("{}{}", module, name));

		let mut class;
		match class_file(module.to_string()).easy_parse(PositionStream::new(text)) {
			Ok((c, _)) => class = c,
			Err(e) => {
				eprintln!("Failed to parse class definition `{}`:\n{}", name, e);
				std::process::exit(1);
			},
		}

		let mut imports = IMPORTS.lock().unwrap();

		for member in &mut class.members {
			if let Member::Method(method) = member {
				assert!(
					method.modifiers.contains(AccessFlags::ACC_NATIVE)
						|| method.is_intrinsic_candidate,
					"Method `{}#{}` is not declared as native or an intrinsic candidate!",
					class.class_name,
					method.name()
				);
			}

			if let Member::Class(subclass) = member {
				let original_subclass_name = subclass.class_name.clone();
				subclass.class_name = format!("{}${}", class.class_name, subclass.class_name);
				imports.insert(
					original_subclass_name,
					format!("{}{}", module, subclass.class_name),
				);
			}
		}

		imports.clear();
		class
	}
}

fn class_file<Input>(module: String) -> impl Parser<Input, Output = Class>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(lex(imports()), lex(class(module))).map(|(_, class)| class)
}

fn class<Input>(module: String) -> impl Parser<Input, Output = Class>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		lex(class_def())
			.message("While parsing class definition")
			.then(move |class_name| {
				(
					value(class_name.clone()),
					lex(many::<Vec<_>, _, _>(member(
						class_name.clone(),
						module.clone(),
					)))
					.message("While parsing fields/methods"),
				)
			}),
		lex(char('}')),
	)
		.message("While parsing class")
		.map(|((class_name, members), _)| Class {
			class_name,
			members,
		})
}

fn member<Input>(class_name: String, module: String) -> impl Parser<Input, Output = Member>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	optional(annotation()).then(move |annotation| {
		dispatch!(
			annotation;
			Some("@IntrinsicCandidate") => choice((
				attempt(super::method::constructor(annotation).map(Member::Method)),
				super::method::method(annotation).map(Member::Method),
			)),
			Some("@Native") => super::field::field(annotation).map(Member::Field),
			_ => choice((
				attempt(super::method::constructor(annotation).map(Member::Method)),
				attempt(super::method::method(annotation).map(Member::Method)),
				attempt(super::field::field(annotation).map(Member::Field)),
				subclass(class_name.clone(), module.clone()).map(Member::Class),
			))
		)
	})
}

fn annotation<Input>() -> impl Parser<Input, Output = &'static str>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	attempt(lex(string("@Native"))).or(attempt(lex(string("@IntrinsicCandidate"))))
}

fn subclass<Input>(parent: String, module: String) -> impl Parser<Input, Output = Class>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	let module1 = module.clone();
	opaque(
		move |f: &mut dyn FnMut(&mut dyn Parser<_, Output = _, PartialState = _>)| {
			let module1 = module.clone();
			f(&mut no_partial(class(module1)))
		},
	)
	.message("While parsing subclasses")
	.map(move |class| {
		IMPORTS.lock().unwrap().insert(
			class.class_name.clone(),
			format!("{}{}${}", module1, parent, class.class_name),
		);
		class
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
	.message("While parsing class imports")
}

fn class_def<Input>() -> impl Parser<Input, Output = String>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		many1::<Vec<&str>, _, _>(choice((
			lex(string("public")),
			lex(string("static")),
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
		optional((lex(string("extends")), lex(word1()))),
		char('{'),
	)
		.map(|(_, _, name, ..)| name)
}
