use runtime::classpath::{add_classpath_entry, ClassPathEntry};
use runtime::Thread;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(long, alias = "cp")]
	classpath: Option<String>,
	#[arg(required = true)]
	main_class: String,
}

fn main() {
	let args = Args::parse();

	if let Some(classpath) = args.classpath.as_deref() {
		for path in classpath.split(':') {
			add_classpath_entry(ClassPathEntry::Dir(PathBuf::from(path)));
		}
	}

	let thread = Thread::new_main(args.main_class.as_bytes());
	Thread::run(&thread);
}
