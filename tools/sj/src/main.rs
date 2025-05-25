fn main() {
	init_logger();

	match sj_lib::launch() {
		Ok(exit_code) => std::process::exit(exit_code),
		Err(e) => {
			eprintln!("{}", e);
		},
	}
}

// TODO: Make the format nicer
fn init_logger() {
	use tracing_subscriber::prelude::*;
	use tracing_subscriber::{EnvFilter, fmt};

	tracing_subscriber::registry()
		.with(fmt::layer().compact().with_thread_names(true))
		.with(
			EnvFilter::try_from_default_env()
				.or_else(|_| EnvFilter::try_new("info"))
				.unwrap(),
		)
		.init();
}
