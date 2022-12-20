use runtime::classpath::{add_classpath_entry, ClassPathEntry};
use runtime::Thread;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(
	name = "jvm",
	author = "Serial-ATA",
	version,
	about = "Serial's JVM - An implementation of the Java SE 19 Virtual Machine",
	long_about = None
)]
struct Args {
	#[arg(
		long,
		alias = "cp",
		help = "The class search path(s) of directories and zip/jar files, semicolon separated"
	)]
	classpath: Option<String>,
	// TODO: --module-path (alias: p): <module path>...
	//                   A : separated list of directories, each directory
	//                   is a directory of modules.
	// TODO: --add-modules: <module name>[,<module name>...]
	//                   root modules to resolve in addition to the initial module.
	//                   <module name> can also be ALL-DEFAULT, ALL-SYSTEM,
	//                   ALL-MODULE-PATH.
	// TODO: --list-modules:
	//                   list observable modules and exit
	// TODO: --describe-module (alias: d): <module name>
	//                   describe a module and exit
	// TODO: --dry-run: create VM and load main class but do not execute main method.
	// TODO: --validate-modules:
	//                   validate all modules and exit
	// TODO: -D<name>=<value>
	//                   set a system property
	// TODO: --show-version (alias: -showversion): print product version to the error stream and continue
	#[arg(
		required = true,
		help = "The name of the main class with the `.class` extension omitted"
	)]
	main_class: String,
	#[arg(required = false, help = "Arguments passed to the main class")]
	args: Vec<String>,
}

fn main() {
	let args = Args::parse();

	if let Some(classpath) = args.classpath.as_deref() {
		for path in classpath.split(':') {
			add_classpath_entry(ClassPathEntry::Dir(PathBuf::from(path)));
		}
	}

	let thread = Thread::new_main(args.main_class.as_bytes(), args.args);
	Thread::run(&thread);
}
