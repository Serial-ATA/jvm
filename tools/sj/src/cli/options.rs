use super::{Args, HelpFlag, LaunchTarget, VersionFlag};
use crate::error::Error;

use common::size::ByteSize;
use std::iter::Peekable;
use std::str::FromStr;
use std::sync::LazyLock;

pub trait CliOption: Send + Sync + 'static {
	/// Whether the option's name can contain multiple parts
	///
	/// Hotspot-style extra options can contain multiple parts (e.g., `-XshowSettings`/`-XshowSettings:all`/...).
	///
	/// If this is true, we'll just treat the [`variants`](Self::variants()) as prefixes, rather than
	/// absolute strings.
	fn multipart(&self) -> bool {
		false
	}

	/// All possible variants of this flag (long and short)
	fn variants(&self) -> &'static [&'static str];

	/// Returns the variable name describing the type of value this flag accepts.
	///
	/// For example, the `--class-path` options has its variable name set to "<class search path of directories and zip/jar files>".
	///
	/// By default this returns `None`.
	fn doc_variable(&self) -> Option<&'static str> {
		None
	}

	/// A short documentation string describing what this flag does
	fn doc_short(&self) -> &'static str;

	/// A longer documentation string describing in full detail what this flag does. By default, this
	/// is the same as [`Self::doc_short()`].
	fn doc_long(&self) -> &'static str {
		self.doc_short()
	}

	/// Signals that the argument parsing can terminate early. Only used for the help flags.
	fn should_exit(&self) -> bool {
		false
	}

	/// Update the args based on the parsed flag
	///
	/// This has access to the remaining args as well as the variant from [`Self::variants()`] that was used.
	fn act(
		&self,
		variant: String,
		cli_args: &mut Peekable<std::vec::IntoIter<String>>,
		args: &mut Args,
	) -> Result<(), Error>;
}

pub struct Options {
	options: Vec<Box<dyn CliOption>>,
}

impl Options {
	pub fn find(&self, opt: &str) -> Option<&dyn CliOption> {
		self.options
			.iter()
			.find(|o| {
				if o.multipart() {
					o.variants().iter().any(|v| opt.starts_with(v))
				} else {
					o.variants().contains(&opt)
				}
			})
			.map(|v| &**v)
	}
}

pub static OPTIONS: LazyLock<Options> = LazyLock::new(|| Options {
	options: vec![
		Box::new(HelpStdout),
		Box::new(HelpStderr),
		Box::new(VersionStdout),
		Box::new(VersionStderr),
		Box::new(ShowVersionStdout),
		Box::new(ShowVersionStderr),
		Box::new(ClassPath),
		Box::new(DryRun),
		Box::new(Jar),
		// Hotspot-compat stuff
		Box::new(ShowSettings),
		Box::new(StackSize),
		Box::new(InitialHeap),
		Box::new(MaxHeap),
	],
});

pub struct HelpStdout;

impl CliOption for HelpStdout {
	fn variants(&self) -> &'static [&'static str] {
		&["--help"]
	}

	fn doc_short(&self) -> &'static str {
		"Print this help message to the output stream"
	}

	fn should_exit(&self) -> bool {
		true
	}

	fn act(
		&self,
		_variant: String,
		_cli_args: &mut Peekable<std::vec::IntoIter<String>>,
		args: &mut Args,
	) -> Result<(), Error> {
		args.help = Some(HelpFlag::HelpStdout);
		Ok(())
	}
}

pub struct HelpStderr;

impl CliOption for HelpStderr {
	fn variants(&self) -> &'static [&'static str] {
		&["-help", "-h", "-?"]
	}

	fn doc_short(&self) -> &'static str {
		"Print this help message to the error stream"
	}

	fn should_exit(&self) -> bool {
		true
	}

	fn act(
		&self,
		_variant: String,
		_cli_args: &mut Peekable<std::vec::IntoIter<String>>,
		args: &mut Args,
	) -> Result<(), Error> {
		args.help = Some(HelpFlag::HelpStderr);
		Ok(())
	}
}

pub struct ClassPath;

impl CliOption for ClassPath {
	fn variants(&self) -> &'static [&'static str] {
		&["--class-path", "-classpath", "-cp"]
	}

	fn doc_variable(&self) -> Option<&'static str> {
		Some("<class search path of directories and zip/jar files>")
	}

	fn doc_short(&self) -> &'static str {
		"The class search path(s)"
	}

	fn doc_long(&self) -> &'static str {
		"The class search path(s) of directories and zip/jar files, semicolon separated"
	}

	fn act(
		&self,
		variant: String,
		cli_args: &mut Peekable<std::vec::IntoIter<String>>,
		args: &mut Args,
	) -> Result<(), Error> {
		let Some(cp) = cli_args.next() else {
			return Err(Error::MissingClasspath(variant));
		};

		args.options.set_classpath(&cp);
		Ok(())
	}
}

pub struct VersionStdout;

impl CliOption for VersionStdout {
	fn variants(&self) -> &'static [&'static str] {
		&["--version"]
	}

	fn doc_short(&self) -> &'static str {
		"Print product version to the output stream and exit"
	}

	fn act(
		&self,
		_variant: String,
		_cli_args: &mut Peekable<std::vec::IntoIter<String>>,
		args: &mut Args,
	) -> Result<(), Error> {
		args.version = Some(VersionFlag::PrintStdout);
		Ok(())
	}
}

pub struct VersionStderr;

impl CliOption for VersionStderr {
	fn variants(&self) -> &'static [&'static str] {
		&["-version"]
	}

	fn doc_short(&self) -> &'static str {
		"Print product version to the error stream and exit"
	}

	fn act(
		&self,
		_variant: String,
		_cli_args: &mut Peekable<std::vec::IntoIter<String>>,
		args: &mut Args,
	) -> Result<(), Error> {
		args.version = Some(VersionFlag::PrintStderr);
		Ok(())
	}
}

pub struct ShowVersionStdout;

impl CliOption for ShowVersionStdout {
	fn variants(&self) -> &'static [&'static str] {
		&["--showversion"]
	}

	fn doc_short(&self) -> &'static str {
		"Print product version to the output stream and continue"
	}

	fn act(
		&self,
		_variant: String,
		_cli_args: &mut Peekable<std::vec::IntoIter<String>>,
		args: &mut Args,
	) -> Result<(), Error> {
		args.version = Some(VersionFlag::ShowStdout);
		Ok(())
	}
}

pub struct ShowVersionStderr;

impl CliOption for ShowVersionStderr {
	fn variants(&self) -> &'static [&'static str] {
		&["-showversion"]
	}

	fn doc_short(&self) -> &'static str {
		"Print product version to the error stream and continue"
	}

	fn act(
		&self,
		_variant: String,
		_cli_args: &mut Peekable<std::vec::IntoIter<String>>,
		args: &mut Args,
	) -> Result<(), Error> {
		args.version = Some(VersionFlag::ShowStderr);
		Ok(())
	}
}

pub struct DryRun;

impl CliOption for DryRun {
	fn variants(&self) -> &'static [&'static str] {
		&["--dry-run"]
	}

	fn doc_short(&self) -> &'static str {
		"Create VM and load main class but do not execute main method"
	}

	fn doc_long(&self) -> &'static str {
		r"Create VM and load main class but do not execute main method.
		This is useful for validating the command-line options such as the module system configuration."
	}

	fn act(
		&self,
		_variant: String,
		_cli_args: &mut Peekable<std::vec::IntoIter<String>>,
		args: &mut Args,
	) -> Result<(), Error> {
		args.dry_run = true;
		Ok(())
	}
}

pub struct Jar;

impl CliOption for Jar {
	fn variants(&self) -> &'static [&'static str] {
		&["-jar"]
	}

	fn doc_short(&self) -> &'static str {
		"Print product version to the error stream and continue"
	}

	fn act(
		&self,
		variant: String,
		cli_args: &mut Peekable<std::vec::IntoIter<String>>,
		args: &mut Args,
	) -> Result<(), Error> {
		let Some(jar_file) = cli_args.next() else {
			return Err(Error::MissingJar(variant));
		};

		args.launch_target = Some(LaunchTarget::Jar(jar_file));
		Ok(())
	}
}

pub struct ShowSettings;

impl CliOption for ShowSettings {
	fn multipart(&self) -> bool {
		true
	}

	fn variants(&self) -> &'static [&'static str] {
		&["-XshowSettings"]
	}

	fn doc_short(&self) -> &'static str {
		"Show settings and continue"
	}

	fn act(
		&self,
		variant: String,
		_: &mut Peekable<std::vec::IntoIter<String>>,
		args: &mut Args,
	) -> Result<(), Error> {
		args.hotspot_options.show_settings = Some(variant);
		Ok(())
	}
}

pub struct StackSize;

impl CliOption for StackSize {
	fn multipart(&self) -> bool {
		true
	}

	fn variants(&self) -> &'static [&'static str] {
		&["-Xss"]
	}

	fn doc_short(&self) -> &'static str {
		"Set the Java thread stack size"
	}

	fn act(
		&self,
		variant: String,
		_: &mut Peekable<std::vec::IntoIter<String>>,
		args: &mut Args,
	) -> Result<(), Error> {
		args.hotspot_options.stack_size = parse_size(variant.strip_prefix("-Xss").unwrap());
		args.options.extra_args.push(variant); // Need to pass it along to the VM...
		Ok(())
	}
}

pub struct InitialHeap;

impl CliOption for InitialHeap {
	fn multipart(&self) -> bool {
		true
	}

	fn variants(&self) -> &'static [&'static str] {
		&["-Xms"]
	}

	fn doc_short(&self) -> &'static str {
		"Set the minimum and initial Java heap size"
	}

	fn act(
		&self,
		variant: String,
		_: &mut Peekable<std::vec::IntoIter<String>>,
		args: &mut Args,
	) -> Result<(), Error> {
		args.hotspot_options.initial_heap = parse_size(variant.strip_prefix("-Xms").unwrap());
		args.options.extra_args.push(variant); // Need to pass it along to the VM...
		Ok(())
	}
}

pub struct MaxHeap;

impl CliOption for MaxHeap {
	fn multipart(&self) -> bool {
		true
	}

	fn variants(&self) -> &'static [&'static str] {
		&["-Xmx"]
	}

	fn doc_short(&self) -> &'static str {
		"Set the maximum Java heap size"
	}

	fn act(
		&self,
		variant: String,
		_: &mut Peekable<std::vec::IntoIter<String>>,
		args: &mut Args,
	) -> Result<(), Error> {
		args.hotspot_options.max_heap = parse_size(variant.strip_prefix("-Xmx").unwrap());
		args.options.extra_args.push(variant); // Need to pass it along to the VM...
		Ok(())
	}
}

fn parse_size(s: &str) -> Option<ByteSize> {
	// Don't actually need to handle errors here. We just need the value as a hint for the launcher.
	// The VM will yell at us later if the value ends up being bad.
	ByteSize::from_str(s).ok()
}
