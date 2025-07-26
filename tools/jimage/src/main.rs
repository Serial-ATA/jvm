use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::Parser;
use classfile::ClassFile;
use jimage::error::Result;
use jimage::{JImage, JImageLocation};

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

	let result = match args.command {
		SubCommand::Extract { dir, jimage } => extract(dir, jimage),
		SubCommand::Info { jimage } => info(jimage),
		SubCommand::List { verbose, jimage } => list(jimage, verbose),
		SubCommand::Verify { jimage } => verify(jimage),
	};

	if let Err(err) = result {
		exit(err.to_string());
	}

	// TODO: glob includes
	// TODO: regex includes
}

fn extract(dir: String, path: PathBuf) -> Result<()> {
	let dump_to = Path::new(&dir);
	if !dump_to.exists() && fs::create_dir_all(dump_to).is_err() {
		exit(format!(
			"Cannot create directory '{}'",
			dump_to.to_string_lossy()
		))
	}

	if !dump_to.is_dir() {
		exit(format!(
			"'{}' is not a directory",
			dump_to.to_string_lossy()
		))
	}

	for_each_entry(
		path,
		|_| Ok(()),
		|_| {},
		move |name, location, jimage| {
			let resource_path = Path::new(name);
			let local_resource_path;

			// We have to strip the leading '/', or else `Path::join` won't work properly.
			if resource_path.is_absolute() {
				let resource_path_no_root = &resource_path.to_string_lossy()[1..];
				local_resource_path = dump_to.join(resource_path_no_root);
			} else {
				local_resource_path = dump_to.join(name);
			}

			if let Some(parent) = local_resource_path.parent() {
				if fs::create_dir_all(parent).is_err() {
					exit(format!(
						"Cannot create directory '{}'",
						parent.to_string_lossy()
					))
				}
			}

			let Ok(mut resource) = fs::OpenOptions::new()
				.write(true)
				.create(true)
				.truncate(true)
				.open(&local_resource_path)
			else {
				exit(format!(
					"Cannot create file '{}'",
					local_resource_path.to_string_lossy()
				));
			};

			match jimage.get_resource_from_location(location) {
				Ok(bytes) => {
					if resource.write_all(&bytes).is_err() {
						exit(format!(
							"Cannot write to file '{}'",
							local_resource_path.to_string_lossy()
						));
					}
				},
				Err(e) => {
					exit(format!(
						"Failed to get resource from location {}: {e}",
						location
							.full_name(true)
							.unwrap_or(String::from("(unknown/invalid location)")),
					));
				},
			}

			Ok(())
		},
	)
}

fn info(path: PathBuf) -> Result<()> {
	let mut file = fs::File::open(&path)?;
	let jimage = JImage::read_from(&mut file)?;

	let header = jimage.header();
	println!(" Major Version:  {}", header.major_version());
	println!(" Minor Version:  {}", header.minor_version());
	println!(" Flags:          {}", header.flags());
	println!(" Resource Count: {}", header.resource_count());
	println!(" Table Length:   {}", header.table_length());
	println!(" Offsets Size:   {}", header.offset_table_length());
	println!(" Redirects Size: {}", header.redirect_table_length());
	println!(" Locations Size: {}", header.location_table_length());
	println!(" Strings Size:   {}", header.string_table_length());
	println!(" Index Size:     {}", header.index_length());

	Ok(())
}

fn list(path: PathBuf, verbose: bool) -> Result<()> {
	for_each_entry(
		path,
		|path| {
			println!("jimage: {:?}", fs::canonicalize(path)?);
			Ok(())
		},
		|module| list_module(module, verbose),
		move |name, location, _| {
			print(name, verbose.then_some(location));
			Ok(())
		},
	)
}

fn verify(path: PathBuf) -> Result<()> {
	for_each_entry(
		path,
		|path| {
			println!("jimage: {:?}", fs::canonicalize(path)?);
			Ok(())
		},
		|_| {},
		|name, location, jimage| {
			if name.ends_with(".class") && !name.ends_with("module-info.class") {
				match jimage.get_resource_from_location(location) {
					Ok(resource) => {
						if let Err(err) = ClassFile::read_from(&mut &resource[..]) {
							exit(err.to_string())
						}
					},
					Err(e) => {
						exit(format!(
							"Failed to get resource from location {}: {e}",
							location
								.full_name(true)
								.unwrap_or(String::from("(unknown/invalid location)")),
						));
					},
				}
			}

			Ok(())
		},
	)
}

fn for_each_entry<P, M, L>(
	path: PathBuf,
	on_jimage_parse: P,
	on_new_module: M,
	on_new_resource: L,
) -> Result<()>
where
	P: Fn(&Path) -> Result<()>,
	M: Fn(&str),
	L: Fn(&str, &JImageLocation<'_>, &JImage) -> Result<()>,
{
	let mut file = fs::File::open(&path)?;
	let jimage = JImage::read_from(&mut file)?;
	on_jimage_parse(&path)?;

	let mut old_module = String::new();
	for name in jimage.get_entry_names()? {
		if !JImage::is_tree_info_resource(&name) {
			let new_module = module_name(&name);

			if new_module != old_module {
				on_new_module(new_module);
				old_module = new_module.to_string();
			}

			if let Some(location) = jimage.find_location(&name) {
				on_new_resource(&name, &location, &jimage)?;
			}
		}
	}

	Ok(())
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
	if let Some(offset) = offset
		&& offset + 1 < name.len()
	{
		return &name[offset + 1..];
	}

	name
}

fn print(name: &str, location: Option<&JImageLocation<'_>>) {
	if let Some(location) = location {
		print!("{:>12} ", location.content_offset());
		print!("{:>10} ", location.uncompressed_size());
		print!("{:>10} ", location.compressed_size());
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

fn exit(message: String) -> ! {
	eprintln!("Error: {}", message);
	std::process::exit(1);
}
