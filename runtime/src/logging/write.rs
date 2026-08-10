use crate::logging::{LogDecorator, LogDecoratorContext, LogLevel, TagSet};
use crate::options::logging::{LogOptions, LogOutput, LogOutputName};

use std::io::Write;
use std::sync::atomic::Ordering;

struct DecoratorWriter<'a> {
	level: LogLevel,
	/// All tags enabled for the output
	all_tags: TagSet,
	/// The tags applicable to this log event
	tags: TagSet,
	decorator_ctx: &'a LogDecoratorContext,
}

impl DecoratorWriter<'_> {
	/// Write the decorators to the `writer`
	///
	/// This returns the total number of bytes written
	fn write<W>(self, mut writer: &mut W) -> std::io::Result<usize>
	where
		W: Write,
	{
		let mut written = 0;
		for decorator in self.decorator_ctx.decorators.iter() {
			let mut tracking_writer = TrackingWriter { written: 0, writer };
			let padding =
				self.decorator_ctx.widths[decorator as usize].load(Ordering::Relaxed) as usize;

			match decorator {
				// TODO: uptime tracking
				LogDecorator::Uptime => write!(tracking_writer, "[]")?,
				LogDecorator::Level => write!(tracking_writer, "[{:padding$}]", self.level)?,
				LogDecorator::Tags => {
					write!(tracking_writer, "[{:padding$}]", self.tags)?;
				},
				LogDecorator::All => unreachable!("marker variant, should never be constructed"),
			}

			written += tracking_writer.written;

			// - 2 for the brackets already written
			let decorator_width = tracking_writer.written - 2;
			assert!(u8::try_from(decorator_width).is_ok());

			// Decorators grow as needed during execution.
			//
			// For example, initial `info` level decorators can appear unpadded as `[info]`, but once
			// a larger level appears (e.g., warning), then all future events will have their level
			// decorator padded, like so:
			//
			// [info]
			// [warning]
			// [info   ]
			if decorator_width > padding {
				self.decorator_ctx.widths[decorator as usize]
					.store(decorator_width as u8, Ordering::SeqCst);
			}

			writer = tracking_writer.writer;
		}

		Ok(written)
	}
}

#[doc(hidden)]
pub fn __write(level: LogLevel, tags: TagSet, message: &str) {
	let options = LogOptions::get();

	// TODO: Actually handle write failures
	for output in options.applicable_outputs(tags, level) {
		match &output.name {
			LogOutputName::Stdout => {
				let writer =
					WriteImpl::new(std::io::stdout(), output, level, tags, message, options);
				writer.write().unwrap();
			},
			LogOutputName::Stderr => {
				let writer =
					WriteImpl::new(std::io::stderr(), output, level, tags, message, options);
				writer.write().unwrap();
			},
			LogOutputName::File(_file) => {
				todo!("file log outputs")
			},
		}
	}
}

/// A writer that tracks the number of bytes written to it.
struct TrackingWriter<W> {
	written: usize,
	writer: W,
}

impl<W: Write> Write for TrackingWriter<W> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		match self.writer.write(buf) {
			Ok(written) => {
				self.written += written;
				Ok(written)
			},
			Err(err) => Err(err),
		}
	}

	fn flush(&mut self) -> std::io::Result<()> {
		self.writer.flush()
	}
}

struct WriteImpl<'a, W> {
	writer: W,
	output: &'a LogOutput,
	decorator_writer: DecoratorWriter<'a>,
	message: &'a str,
	options: &'static LogOptions,
}

impl<'a, W> WriteImpl<'a, W> {
	fn new(
		writer: W,
		output: &'a LogOutput,
		level: LogLevel,
		tags: TagSet,
		message: &'a str,
		options: &'static LogOptions,
	) -> Self {
		let decorator_writer = DecoratorWriter {
			level,
			all_tags: output.enabled_tags(),
			tags,
			decorator_ctx: &output.decorator_ctx,
		};

		Self {
			writer,
			output,
			decorator_writer,
			message,
			options,
		}
	}
}

impl<W: Write> WriteImpl<'_, W> {
	fn write(mut self) -> std::io::Result<()> {
		let decorator_width = self.decorator_writer.write(&mut self.writer)?;

		// For multiline outputs, the decorators are only output once, with the remaining lines
		// getting padded brackets, like so:
		//
		// [error][exceptions] <some long message
		// [                 ] that extends multiple lines>
		let mut decoration_padding = decorator_width.saturating_sub(2); // - 2 for the brackets already written
		if decoration_padding > 0 {
			write!(self.writer, " ")?;
		}

		for (idx, line) in self.message.lines().enumerate() {
			if self.output.output_options.fold_multilines {
				// TODO: This leaves a trailing newline
				write!(self.writer, "{}\\n", line)?;
			} else if decoration_padding > 0 && idx > 0 {
				writeln!(self.writer, "[{}] {line}", " ".repeat(decoration_padding))?;
			} else {
				writeln!(self.writer, "{line}")?;
			}
		}

		if self.output.output_options.fold_multilines {
			writeln!(self.writer)?;
		}

		Ok(())
	}
}
