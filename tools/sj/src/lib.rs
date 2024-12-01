mod cli;
mod error;
mod native;

use crate::error::{Error, Result};

use std::path::Path;

use clap::Parser;
use jvm_runtime::classpath::{add_classpath_entry, jar, jimage, ClassPathEntry};

#[tracing::instrument]
pub fn launch() -> Result<()> {
	let args = cli::Args::parse();

	if let Some(classpath) = args.options.classpath.as_deref() {
		for path in classpath.split(':') {
			add_classpath_entry(ClassPathEntry::new(path));
		}
	}

	if let Some(_vm_options) = jimage::lookup_vm_options() {
		// TODO: Actually parse the options, for now this is just here to load the JImage
		// https://github.com/openjdk/jdk/blob/03a9a88efbb68537e24b7de28c5b81d6cd8fdb04/src/hotspot/share/runtime/arguments.cpp#L3322
	}

	let main_class = match args.main_class {
		Some(main_class) => main_class,
		None => match jar::main_class_from_jar_manifest(Path::new(&args.jar.unwrap())) {
			Some(main_class) => main_class,
			None => return Err(Error::NoJarMain),
		},
	};

	let (vm, env) = native::init_java_vm(args.options.system_properties.unwrap_or_default())?;

	// Load the main class
	let main_class = env.find_class(main_class)?;

	if args.options.dry_run {
		return Ok(());
	}

	native::invoke_main_method(env, main_class, args.args)?;

	Ok(())
}
