fn main() {
	match sj_lib::launch() {
		Ok(exit_code) => std::process::exit(exit_code),
		Err(e) => {
			eprintln!("{}", e);
		},
	}
}
