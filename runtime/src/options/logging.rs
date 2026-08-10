//! All the craziness for the `-Xlog` option.

use crate::logging::{LogDecorator, LogDecoratorContext, LogDecoratorSet, LogLevel, Tag, TagSet};

use std::fmt::Display;
use std::str::FromStr;
use std::sync::OnceLock;

/// A single `tag[=level]` selection
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selection {
	pub tag: Tag,
	pub level: LogLevel,
}

#[derive(Debug)]
pub enum SelectionsParseError {
	InvalidTag(String),
	InvalidLevel(String),
}

impl Display for SelectionsParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SelectionsParseError::InvalidTag(tag) => {
				write!(f, "Invalid tag '{tag}' in log selection")
			},
			SelectionsParseError::InvalidLevel(level) => {
				write!(f, "Invalid level '{level}' in log selection")
			},
		}
	}
}

/// A collection of tag selections
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Selections(pub Vec<Selection>);

impl FromStr for Selections {
	type Err = SelectionsParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut selections = Vec::new();
		for selection_str in s.split(',').map(str::trim) {
			if selection_str.is_empty() {
				continue;
			}

			// TODO: wildcard selections
			match selection_str.split_once('=') {
				Some((tag, level)) => {
					let tag = Tag::from_str(tag)
						.map_err(|_| SelectionsParseError::InvalidTag(tag.to_string()))?;
					let level = LogLevel::from_str(level)
						.map_err(|_| SelectionsParseError::InvalidLevel(level.to_string()))?;
					selections.push(Selection { tag, level });
				},
				// No level implies `LogLevel::Info`
				None => {
					let tag = Tag::from_str(selection_str)
						.map_err(|_| SelectionsParseError::InvalidTag(selection_str.to_string()))?;
					selections.push(Selection {
						tag,
						level: LogLevel::Info,
					});
				},
			}
		}

		Ok(Selections(selections))
	}
}

/// The log output name
///
/// There are two special cases: `stdout` and `stderr`. Anything else
/// is a file name pattern, optionally prefixed with `file=`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogOutputName {
	Stdout,
	Stderr,
	File(Box<str>),
}

#[derive(Debug)]
pub enum OutputNameParseError {
	// TODO: Actually implement the error cases
}

impl Display for OutputNameParseError {
	fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl FromStr for LogOutputName {
	type Err = OutputNameParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.trim() {
			"stdout" | "" => Ok(LogOutputName::Stdout),
			"stderr" => Ok(LogOutputName::Stderr),
			_ => {
				let file_pattern = s.strip_prefix("file=").unwrap_or(s);
				let unquoted = file_pattern.trim_matches('"');

				Ok(LogOutputName::File(unquoted.into()))
			},
		}
	}
}

/// Options to control the behavior of a log output.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct LogOutputOptions {
	/// Whether multiline log events should be folded into a single line.
	///
	/// This replaces newlines in the message with a literal `['\', '\n']`.
	pub fold_multilines: bool,
	/// Target byte size for log rotation
	pub file_size: usize,
	/// Number of files to keep in rotation (not counting the active file)
	pub file_count: usize,
}

#[derive(Debug)]
pub enum LogOutputOptionsParseError {
	UnknownOption(String),
	BadBool(&'static str),
	BadInt(&'static str),
}

impl Display for LogOutputOptionsParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			LogOutputOptionsParseError::UnknownOption(option) => {
				write!(f, "Invalid option '{}' for log output", option)
			},
			LogOutputOptionsParseError::BadBool(field) => {
				write!(f, "{field} must be 'true' or 'false'")
			},
			LogOutputOptionsParseError::BadInt(field) => write!(
				f,
				"{field} must be in range [{}, {}]",
				usize::MIN,
				usize::MAX
			),
		}
	}
}

impl FromStr for LogOutputOptions {
	type Err = LogOutputOptionsParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		fn parse_bool(
			field: &'static str,
			value: &str,
		) -> Result<bool, LogOutputOptionsParseError> {
			match value.trim() {
				"true" => Ok(true),
				"false" => Ok(false),
				_ => Err(LogOutputOptionsParseError::BadBool(field)),
			}
		}

		let mut options = LogOutputOptions::default();
		for option in s.split(',') {
			let trimmed = option.trim();
			if trimmed.is_empty() {
				continue;
			}

			let Some((key, value)) = option.split_once('=') else {
				return Err(LogOutputOptionsParseError::UnknownOption(
					option.to_string(),
				));
			};

			match key.trim() {
				"foldmultilines" => options.fold_multilines = parse_bool("foldmultilines", value)?,
				"filesize" => {
					options.file_size = value
						.parse()
						.map_err(|_| LogOutputOptionsParseError::BadInt("filesize"))?
				},
				"filecount" => {
					options.file_count = value
						.parse()
						.map_err(|_| LogOutputOptionsParseError::BadInt("filecount"))?
				},
				key => return Err(LogOutputOptionsParseError::UnknownOption(key.to_string())),
			}
		}
		Ok(options)
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct LogOutput {
	pub name: LogOutputName,
	pub levels: [LogLevel; Tag::VARIANTS as usize],
	pub decorator_ctx: LogDecoratorContext,
	pub output_options: LogOutputOptions,
}

impl LogOutput {
	/// Default stdout output config
	fn default_stdout() -> Self {
		let mut output = Self::new(LogOutputName::Stdout);
		output.levels.fill(LogLevel::Warning);
		output
	}

	/// Default stderr output config
	fn default_stderr() -> Self {
		Self::new(LogOutputName::Stderr)
	}

	fn new(name: LogOutputName) -> Self {
		Self {
			name,
			levels: [LogLevel::Off; Tag::VARIANTS as usize],
			decorator_ctx: LogDecoratorContext::new(LogDecoratorSet::DEFAULT),
			output_options: LogOutputOptions::default(),
		}
	}

	fn apply_option(&mut self, opt: LogOption) {
		if opt.selections.0.is_empty() {
			// No selections implies `all=info`
			self.levels.fill(LogLevel::Info);
		} else {
			for selection in opt.selections.0 {
				self.levels[selection.tag as usize] =
					self.levels[selection.tag as usize].max(selection.level);
			}
		}

		self.decorator_ctx.decorators = self
			.decorator_ctx
			.decorators
			.union(opt.decorator_ctx.decorators);
		self.output_options = opt.output_options;
	}

	pub fn enabled_tags(&self) -> TagSet {
		self.levels
			.into_iter()
			.enumerate()
			.filter(|(_, level)| *level != LogLevel::Off)
			.map(|(tag, _)| {
				// SAFETY: `self.levels` is constructed with `Tag::VARIANTS` elements
				unsafe { std::mem::transmute::<u8, Tag>(tag as u8) }
			})
			.collect()
	}
}

#[derive(Debug)]
pub enum LogParseError {
	Selections(SelectionsParseError),
	OutputName(OutputNameParseError),
	BadDecorator(String),
	OutputOptions(LogOutputOptionsParseError),
}

impl Display for LogParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			LogParseError::Selections(err) => err.fmt(f),
			LogParseError::OutputName(err) => err.fmt(f),
			LogParseError::BadDecorator(decorator) => write!(f, "invalid decorator '{decorator}'"),
			LogParseError::OutputOptions(err) => err.fmt(f),
		}
	}
}

/// A parsed `-Xlog` CLI option.
#[derive(Debug, PartialEq, Eq)]
pub struct LogOption {
	pub selections: Selections,
	pub output: LogOutputName,
	pub decorator_ctx: LogDecoratorContext,
	pub output_options: LogOutputOptions,
}

// From Hotspot, bare "-Xlog" is equivalent to "-Xlog:all=info:stdout:uptime,levels,tags"
impl Default for LogOption {
	fn default() -> Self {
		Self {
			selections: Selections::default(),
			output: LogOutputName::Stdout,
			decorator_ctx: LogDecoratorContext::new(LogDecoratorSet::DEFAULT),
			output_options: LogOutputOptions::default(),
		}
	}
}

impl FromStr for LogOption {
	type Err = LogParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut sections = s.split(':');
		let _empty = sections
			.next()
			.expect("split should always return something");
		if !_empty.is_empty() {
			// There shouldn't be anything between "-Xlog" and the first colon
			todo!("Some error")
		}

		let mut option = LogOption::default();
		let Some(selections_str) = sections.next() else {
			return Ok(option);
		};

		option.selections =
			Selections::from_str(selections_str).map_err(LogParseError::Selections)?;

		let output_str = sections.next().unwrap_or("");
		option.output = LogOutputName::from_str(output_str).map_err(LogParseError::OutputName)?;

		let decorators_str = sections.next().unwrap_or("");
		for decorator in decorators_str.split(',') {
			let trimmed = decorator.trim();
			if trimmed.is_empty() {
				continue;
			}

			let decorator = LogDecorator::from_str(trimmed)
				.map_err(|_| LogParseError::BadDecorator(decorator.to_string()))?;
			option.decorator_ctx.decorators.insert(decorator);
		}

		option.output_options = LogOutputOptions::from_str(sections.next().unwrap_or(""))
			.map_err(LogParseError::OutputOptions)?;

		Ok(option)
	}
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct LogOptionsBuilder {
	outputs: Vec<LogOutput>,
}

impl Default for LogOptionsBuilder {
	fn default() -> Self {
		Self {
			outputs: vec![LogOutput::default_stdout(), LogOutput::default_stderr()],
		}
	}
}

impl LogOptionsBuilder {
	pub(super) fn apply_option(&mut self, opt: LogOption) {
		let index = match self
			.outputs
			.iter()
			.position(|output| output.name == opt.output)
		{
			Some(index) => index,
			None => {
				self.outputs.push(LogOutput::new(opt.output.clone()));
				self.outputs.len() - 1
			},
		};

		self.outputs
			.get_mut(index)
			.expect("output index is valid")
			.apply_option(opt);
	}

	pub(super) fn build(self) -> LogOptions {
		LogOptions {
			outputs: self.outputs.into_boxed_slice(),
		}
	}
}

/// Options for the `-Xlog` CLI option.
#[derive(Debug, PartialEq, Eq)]
pub struct LogOptions {
	outputs: Box<[LogOutput]>,
}

#[cfg(test)]
impl From<LogOption> for LogOptions {
	fn from(opt: LogOption) -> Self {
		let mut builder = LogOptionsBuilder::default();
		builder.apply_option(opt);
		builder.build()
	}
}

static OPTIONS: OnceLock<LogOptions> = OnceLock::new();

impl LogOptions {
	/// Apply the log options globally.
	pub(crate) fn apply(self) {
		OPTIONS
			.set(self)
			.expect("log options should not be initialized yet");
	}

	pub fn get() -> &'static Self {
		OPTIONS.get().expect("log options should be initialized")
	}

	/// Whether all of the given tags in the set are enabled at the given [`LogLevel`] in at least one output.
	pub fn are_tags_enabled_at(&self, tags: TagSet, level: LogLevel) -> bool {
		self.applicable_outputs(tags, level).next().is_some()
	}

	/// Gets all of the log outputs that are enabled for the given tags and level.
	pub fn applicable_outputs(
		&self,
		tags: TagSet,
		level: LogLevel,
	) -> impl Iterator<Item = &LogOutput> {
		self.outputs
			.iter()
			.filter(move |output| tags.iter().all(|t| output.levels[t as usize] >= level))
	}
}
