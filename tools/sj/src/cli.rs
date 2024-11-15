static USAGE: &str = r"jvm [OPTIONS] <MAIN_CLASS> [ARGS]...    (to execute a class)
   or  jvm [OPTIONS] --jar <JARFILE> [ARGS]... (to execute a jar file)";

#[derive(clap::Parser)]
#[command(
	name = "jvm",
	author = "Serial-ATA",
	version,
	about = "Serial's JVM - An implementation of the Java SE 19 Virtual Machine",
	long_about = None,
	override_usage = USAGE
)]
pub struct Args {
	#[clap(flatten)]
	pub options: JVMOptions,
	#[arg(
		long,
		required_unless_present = "main_class",
		help = "The name of the jar file to execute"
	)]
	pub jar: Option<String>,
	#[arg(
		required_unless_present = "jar",
		help = "The name of the main class with the `.class` extension omitted"
	)]
	pub main_class: Option<String>,
	#[arg(required = false, help = "Arguments passed to the main class")]
	pub args: Vec<String>,
}

#[derive(Debug, clap::Args)]
pub struct JVMOptions {
	#[arg(
		long,
		alias = "cp",
		help = "The class search path(s) of directories and zip/jar files, semicolon separated",
		env
	)]
	pub classpath: Option<String>,
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
	#[arg(
		long,
		help = "Create VM and load main class but do not execute main method"
	)]
	pub dry_run: bool,
	// TODO: --validate-modules:
	//                   validate all modules and exit
	#[arg(short = 'D', help = "Sets a system property (format: -Dkey=value)")]
	pub system_properties: Option<Vec<String>>,
	#[arg(long, help = "Print product version to the error stream and continue")]
	pub showversion: bool,
	#[arg(long, help = "Print product version to the output stream and continue")]
	pub show_version: bool,
}

impl Into<jvm_runtime::JVMOptions> for JVMOptions {
	fn into(self) -> jvm_runtime::JVMOptions {
		jvm_runtime::JVMOptions {
			dry_run: self.dry_run,
			system_properties: self.system_properties,
			showversion: self.showversion,
			show_version: self.show_version,
		}
	}
}
