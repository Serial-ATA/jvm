mod options;

use crate::error::Error;
use crate::native::LaunchMode;

use std::path::Path;

const JAVA_OPTIONS_ENV: &str = "_JAVA_OPTIONS";

pub enum HelpFlag {
	HelpStdout,
	HelpStderr,
}

pub enum VersionFlag {
	PrintStdout,
	PrintStderr,
	ShowStdout,
	ShowStderr,
}

pub enum LaunchTarget {
	Class(String),
	Jar(String),
	Source(String),
	Module(String),
}

impl LaunchTarget {
	pub fn mode(&self) -> LaunchMode {
		match self {
			LaunchTarget::Class(_) => LaunchMode::Class,
			LaunchTarget::Jar(_) => LaunchMode::Jar,
			LaunchTarget::Module(_) => LaunchMode::Module,
			LaunchTarget::Source(_) => LaunchMode::Source,
		}
	}

	pub fn target(&self) -> &str {
		match self {
			LaunchTarget::Class(target) => target.as_str(),
			LaunchTarget::Jar(target) => target.as_str(),
			LaunchTarget::Module(target) => target.as_str(),
			LaunchTarget::Source(target) => target.as_str(),
		}
	}
}

#[derive(Default)]
pub struct Args {
	pub help: Option<HelpFlag>,
	pub version: Option<VersionFlag>,
	pub dry_run: bool,
	pub options: JVMOptions,
	pub launch_target: Option<LaunchTarget>,
	pub args: Vec<String>,
}

impl Args {
	pub fn parse() -> Result<Self, Error> {
		let cli_args_raw = std::env::args().skip(1).collect::<Vec<_>>();
		let mut cli_args = cli_args_raw.into_iter().peekable();

		if let Ok(_java_options) = std::env::var(JAVA_OPTIONS_ENV) {
			// TODO: Actually load _JAVA_OPTIONS env
		}

		let mut vm_args = Vec::new();

		let mut args = Args::default();
		loop {
			let Some(arg) = cli_args.peek() else {
				break;
			};

			if !arg.starts_with('-') {
				break;
			}

			let arg = cli_args.next().unwrap();

			if arg.starts_with("-D") {
				args.options.add_system_property(arg);
				continue;
			}

			let Some(opt) = options::OPTIONS.find(&arg) else {
				vm_args.push(arg);
				continue;
			};

			opt.act(arg, &mut cli_args, &mut args)?;

			if opt.should_exit() {
				break;
			}
		}

		match &mut args.launch_target {
			Some(LaunchTarget::Jar(jar_name)) => {
				args.options.set_classpath(&jar_name);
			},
			None => {
				if !args.options.specified_classpath {
					args.options.set_classpath(".");
				}

				let Some(target) = cli_args.peek() else {
					if args.help.is_none() && args.version.is_none() {
						args.help = Some(HelpFlag::HelpStdout);
					}

					return Ok(args);
				};

				let target_path = Path::new(target);
				if target_path.exists()
					&& target_path.extension().map_or(false, |ext| ext == "java")
				{
					args.options
						.add_system_property("-Dsun.java.launcher.mode=source");
					args.options
						.add_system_property("--add-modules=ALL-DEFAULT");
					args.launch_target = Some(LaunchTarget::Source(
						target_path.to_str().ok_or(Error::NonUtf8Path)?.to_string(),
					));
				} else {
					// Only consume for class targets, not source targets
					let target = cli_args.next().expect("should exist");
					args.launch_target = Some(LaunchTarget::Class(target));
				}
			},
			_ => {},
		}

		Ok(args)
	}
}

#[derive(Debug, Default)]
pub struct JVMOptions {
	specified_classpath: bool,
	pub system_properties: Option<Vec<String>>,
}

impl JVMOptions {
	fn set_classpath(&mut self, classpath: &str) {
		self.specified_classpath = true;

		let prop = format!("-Djava.class.path={classpath}");
		self.add_system_property(prop);
	}

	fn add_system_property(&mut self, property: impl Into<String>) {
		match self.system_properties.as_mut() {
			Some(system_props) => system_props.push(property.into()),
			None => {
				self.system_properties = Some(vec![property.into()]);
			},
		}
	}
}

impl Into<jni::java_vm::VmInitArgs> for JVMOptions {
	fn into(self) -> jni::java_vm::VmInitArgs {
		jni::java_vm::VmInitArgs::default().options(self.system_properties.unwrap_or_default())
	}
}
