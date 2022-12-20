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
