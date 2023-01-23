#![feature(let_chains)]

use std::fs;
use std::path::PathBuf;

use clap::Parser;
use jimage::JImage;

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
	command: SubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum SubCommand {
	/// Extract all jimage entries and place in a directory specified.
	Extract {
		#[arg(long, default_value = ".")]
		dir: String,
		#[arg(required = true, help = "The path to the jimage file")]
		jimage: PathBuf,
	},
	/// Prints detailed information contained in the jimage header.
	Info {
		#[arg(required = true, help = "The path to the jimage file")]
		jimage: PathBuf,
	},
	/// Prints the names of all the entries in the jimage.
	///
	/// When used with --verbose, list will also print entry size and offset attributes.
	List {
		#[arg(long)]
		verbose: bool,
		#[arg(required = true, help = "The path to the jimage file")]
		jimage: PathBuf,
	},
	/// Reports on any .class entries that dont verify as classes.
	Verify {
		#[arg(required = true, help = "The path to the jimage file")]
		jimage: PathBuf,
	},
}

fn main() {
	let args = Command::parse();

	match args.command {
		SubCommand::List { verbose, jimage } => list(jimage, verbose),
		c => unimplemented!("{:?}", c),
	}
	// TODO: glob includes
	// TODO: regex includes
	// TODO: extract subcommand
	// TODO: info subcommand
	// TODO: verify subcommand
}

fn list(path: PathBuf, verbose: bool) {
	let jimage = jimage_parser::parse(&mut fs::File::open(&path).unwrap());

	println!("jimage: {:?}", fs::canonicalize(path).unwrap());

	let mut old_module = String::new();
	for name in jimage.get_entry_names().unwrap() {
		if !JImage::is_tree_info_resource(&name) {
			let new_module = module_name(&name);

			if new_module != old_module {
				list_module(new_module, verbose);
				old_module = new_module.to_string();
			}

			print(&name, verbose.then_some(&jimage));
		}
	}
}

fn module_name(path: &str) -> &str {
	let offset = path[1..].find('/');
	match offset {
		Some(offset) => &path[1..=offset],
		None => "<unknown>",
	}
}

fn trim_module(name: &str) -> &str {
	let offset = name[1..].find('/').map(|offset| offset + 1);
	if let Some(offset) = offset && offset + 1 < name.len() {
		return &name[offset+1..];
	}

	name
}

fn print(name: &str, image: Option<&JImage>) {
	if let Some(image) = image {
		let Some(location) = image.find_location(name) else {
			return
		};

		print!("{:>12} ", location.get_content_offset());
		print!("{:>10} ", location.get_uncompressed_size());
		print!("{:>10} ", location.get_compressed_size());
	}

	println!("{}", trim_module(name));
}

fn list_module(module: &str, verbose: bool) {
	println!("\nModule: {}", module);

	if verbose {
		print!("Offset       ");
		print!("Size       ");
		print!("Compressed ");
		println!("Entry")
	}
}
