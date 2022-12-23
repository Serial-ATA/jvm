use clap::Parser;

#[derive(Parser)]
#[command(
	name = "jimage",
	bin_name = "jimage",
	author = "Serial-ATA",
	version,
	long_about = None,
)]
struct Command {
	#[arg(
		long,
		help = "Pattern list for filtering entries",
		long_help = "<INCLUDE> will be a comma separated list of elements each using one the \
		             following forms:
<glob-pattern>
glob:<glob-pattern>
regex:<regex-pattern>"
	)]
	include: Option<Vec<String>>,

	#[command(subcommand)]
	command: Option<SubCommand>,
	#[arg(required = true, help = "The path to the jimage file")]
	jimage: String,
}

#[derive(Debug, clap::Subcommand)]
enum SubCommand {
	/// Extract all jimage entries and place in a directory specified.
	Extract {
		#[arg(long, default_value = ".")]
		dir: String,
	},
	/// Prints detailed information contained in the jimage header.
	Info,
	/// Prints the names of all the entries in the jimage.
	///
	/// When used with --verbose, list will also print entry size and offset attributes.
	List {
		#[arg(long)]
		verbose: bool,
	},
	/// Reports on any .class entries that dont verify as classes.
	Verify,
}

fn main() {
	let _args = Command::parse();

	// TODO: glob includes
	// TODO: regex includes
	// TODO: extract subcommand
	// TODO: info subcommand
	// TODO: list subcommand
	// TODO: verify subcommand
}
