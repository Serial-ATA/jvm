use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(
	name = "jmod",
	bin_name = "jmod",
	author = "Serial-ATA",
	version,
	long_about = None,
	disable_help_subcommand = true,
)]
struct Command {
	#[command(subcommand)]
	command: SubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum SubCommand {
	/// Creates a new jmod archive
	Create {
		#[arg(long, help = "Application jar file|dir containing classes")]
		class_path: Option<String>,
		#[arg(long, help = "Location of native commands")]
		cmds: Option<String>,
		#[arg(long, help = "Location of user-editable config files")]
		config: Option<String>,
		/// Date and time for the timestamps of entries
		///
		/// Specified in ISO-8601 extended offset date-time with optional time-zone format, e.g. \"2022-02-12T12:30:00-05:00\"
		#[arg(long)]
		date: Option<String>,
		#[arg(long)]
		dry_run: bool,
		/// Exclude files matching the supplied comma separated pattern list
		///
		/// Each element using one the following forms: <glob-pattern>, glob:<glob-pattern> or regex:<regex-pattern>
		#[arg(long)]
		exclude: Option<String>,
		/// Compute and record hashes to tie a packaged module
		///
		/// This will work with modules matching the given <regex-pattern> and depending upon it directly or indirectly.
		#[arg(long)]
		hash_modules: Option<String>,
		#[arg(long, help = "Location of header files")]
		header_files: Option<PathBuf>,
		#[arg(long, help = "Location of legal notices")]
		legal_notices: Option<PathBuf>,
		#[arg(long, help = "Location of native libraries")]
		libs: Option<PathBuf>,
		#[arg(long, help = "Main class")]
		main_class: Option<String>,
		#[arg(long, help = "Location of man pages")]
		man_pages: Option<PathBuf>,
		#[arg(long, help = "Module version")]
		modules_version: Option<String>,
		#[arg(long, short = 'p', help = "Module path")]
		module_path: Option<PathBuf>,
		#[arg(long, help = "Target platform")]
		target_platform: Option<String>,
		#[arg(help = "Output path of the jmod file")]
		jmod_file: PathBuf,
	},
	/// Extracts all the files from the archive
	Extract {
		/// Exclude files matching the supplied comma separated pattern list
		///
		/// Each element using one the following forms: <glob-pattern>, glob:<glob-pattern> or regex:<regex-pattern>
		#[arg(long)]
		exclude: Option<String>,
		#[arg(long, help = "Target directory for extract")]
		dir: Option<PathBuf>,
		#[arg(help = "Path to the jmod file to operate on")]
		jmod_file: PathBuf,
	},
	/// Prints the names of all the entries
	List {
		#[arg(help = "Path to the jmod file to operate on")]
		jmod_file: PathBuf,
	},
	/// Prints the module details
	Describe {
		#[arg(help = "Path to the jmod file to operate on")]
		jmod_file: PathBuf,
	},
	/// Records hashes of tied modules.
	Hash {
		/// Date and time for the timestamps of entries
		///
		/// Specified in ISO-8601 extended offset date-time with optional time-zone format, e.g. \"2022-02-12T12:30:00-05:00\"
		#[arg(long)]
		date: Option<String>,
		#[arg(long)]
		dry_run: bool,
		/// Exclude files matching the supplied comma separated pattern list
		///
		/// Each element using one the following forms: <glob-pattern>, glob:<glob-pattern> or regex:<regex-pattern>
		#[arg(long)]
		exclude: Option<String>,
		/// Compute and record hashes to tie a packaged module
		///
		/// This will work with modules matching the given <regex-pattern> and depending upon it directly or indirectly.
		#[arg(long)]
		hash_modules: Option<String>,
		#[arg(long, short = 'p', help = "Module path")]
		module_path: Option<PathBuf>,
		#[arg(help = "Path to the jmod file to operate on")]
		jmod_file: PathBuf,
	},
}

fn main() {
	let args = Command::parse();

	match args.command {
		c => unimplemented!("{:?}", c),
	}
	// TODO: Create subcommand
	// TODO: Extract subcommand
	// TODO: List subcommand
	// TODO: Describe subcommand
	// TODO: Hash subcommand
}
