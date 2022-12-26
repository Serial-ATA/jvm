use runtime::classpath::{add_classpath_entry, jar, ClassPathEntry};
use runtime::Thread;
use std::path::Path;

use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");
static USAGE: &str = r"jvm [OPTIONS] <MAIN_CLASS> [ARGS]...    (to execute a class)
   or  jvm [OPTIONS] --jar <JARFILE> [ARGS]... (to execute a jar file)";

#[derive(Parser)]
#[command(
	name = "jvm",
	author = "Serial-ATA",
	version,
	about = "Serial's JVM - An implementation of the Java SE 19 Virtual Machine",
	long_about = None,
	override_usage = USAGE
)]
struct Args {
	#[clap(flatten)]
	options: JVMOptions,
	#[arg(
		long,
		required_unless_present = "main_class",
		help = "The name of the jar file to execute"
	)]
	jar: Option<String>,
	#[arg(
		required_unless_present = "jar",
		help = "The name of the main class with the `.class` extension omitted"
	)]
	main_class: Option<String>,
	#[arg(required = false, help = "Arguments passed to the main class")]
	args: Vec<String>,
}

#[derive(Debug, clap::Args)]
struct JVMOptions {
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
	#[arg(short = 'D', help = "Sets a system property (format: -Dkey=value)")]
	system_properties: Option<Vec<String>>,
	#[arg(long, help = "Print product version to the error stream and continue")]
	showversion: bool,
	#[arg(long, help = "Print product version to the output stream and continue")]
	show_version: bool,
}

fn main() {
	init_logger();
	let args = Args::parse();

	// TODO: env!("CLASSPATH")
	if let Some(classpath) = args.options.classpath.as_deref() {
		for path in classpath.split(':') {
			add_classpath_entry(ClassPathEntry::new(path));
		}
	}

	// TODO:
	// if let Some(vm_options) = jimage::lookup_vm_options() {
	// 	dbg!(std::str::from_utf8(&vm_options));
	// }

	let main_class = match args.main_class {
		Some(main_class) => main_class,
		None => match jar::main_class_from_jar_manifest(Path::new(&args.jar.unwrap())) {
			Some(main_class) => main_class,
			None => {
				eprintln!("Unable to find main class in jar manifest!");
				std::process::exit(1);
			},
		},
	};

	let thread = Thread::new_main(main_class.as_bytes(), args.args);
	Thread::run(&thread);
}

fn init_logger() {
	env_logger::builder()
		.format_timestamp(None)
		.format_target(false)
		.init();
}
