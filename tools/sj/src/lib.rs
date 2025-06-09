mod cli;
mod error;
mod native;

use crate::cli::{HelpFlag, VersionFlag};
use crate::error::{Error, Result};
use std::path::PathBuf;

use jni::error::JniError;

const USAGE: &str = r"java [OPTIONS] <MAIN_CLASS> [ARGS]...
   or  java [OPTIONS] -jar <JARFILE> [ARGS]...
   or  java [OPTIONS] [--module|-m] <module>[/<mainclass>] [ARGS]...
   or  java [OPTIONS] <sourcefile> [ARGS]...";

const ABOUT: &str = "Serial's JVM - An implementation of the Java SE 23 Virtual Machine";

const ARGS_NOTICE: &str = r" Arguments following the main class, source file, -jar <jarfile>,
 -m or --module <module>/<mainclass> are passed as the arguments to
 main class.";

const AUTHOR: &str = env!("SYSTEM_PROPS_VM_VENDOR");

const USAGE_STRING: &str = const_format::formatcp!("{ABOUT}\n\nUsage: {USAGE}\n\n{ARGS_NOTICE}");

/// Launch a Java application
///
/// This will create a Java VM and invoke the `static void main(String[])` method in the main class.
///
/// # Returns
///
/// An `Err` indicates something went wrong in the process of setting up the VM, before any Java code
/// actually runs. An `Ok` return does **not** always indicate success.
///
/// The `Ok` value is the exit code for the process. If there is an uncaught exception, it is handled
/// and the exit code is set to 1.
pub fn launch() -> Result<i32> {
	let args = cli::Args::parse()?;

	if let Some(help) = args.help {
		match help {
			HelpFlag::HelpStdout => println!("{USAGE_STRING}"),
			HelpFlag::HelpStderr => eprintln!("{USAGE_STRING}"),
		}

		return Ok(0);
	}

	std::thread::spawn(move || main(args))
		.join()
		.unwrap_or_else(|_| {
			eprintln!("Main thread panicked");
			Ok(1)
		})
}

fn main(args: cli::Args) -> Result<i32> {
	macro_rules! handle_error {
		(($vm:expr, $env:expr) $e:expr) => {
			match $e {
				Err(Error::Jni(JniError::ExceptionThrown)) => {
					$env.exception_describe();
					// TODO: Detach thread
					$vm.destroy()?;
					return Ok(1);
				},
				Err(e) => return Err(e),
				Ok(val) => val,
			}
		};
	}

	let (vm, env) = native::init_java_vm(args.options.system_properties.unwrap_or_default())?;

	if let Some(version) = args.version {
		let use_stderr;
		let exit;

		match version {
			VersionFlag::PrintStdout => {
				use_stderr = false;
				exit = true;
			},
			VersionFlag::PrintStderr => {
				use_stderr = true;
				exit = true;
			},
			VersionFlag::ShowStdout => {
				use_stderr = false;
				exit = false;
			},
			VersionFlag::ShowStderr => {
				use_stderr = true;
				exit = false;
			},
		}

		handle_error!((vm, env) native::print_version(env, use_stderr));
		if exit {
			return Ok(0);
		}
	}

	// Load the main class
	let launcher_helper = handle_error!((vm, env) native::LauncherHelper::new(env));

	let launch_target = args.launch_target.as_ref().expect("launch_target");
	let launch_mode = launch_target.mode();

	let main_class = handle_error!((vm, env) launcher_helper.check_and_load_main(
		env,
		true,
		launch_mode,
		launch_target.target().to_string(),
	));

	if args.dry_run {
		return Ok(0);
	}

	handle_error!((vm, env) native::invoke_main_method(env, main_class, args.args).map(Ok))
}
