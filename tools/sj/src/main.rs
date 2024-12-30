fn main() {
	init_logger();

	if let Err(e) = sj_lib::launch() {
		eprintln!("{}", e);
	}
}

// TODO: Make the format nicer
fn init_logger() {
	use tracing_subscriber::prelude::*;
	use tracing_subscriber::{fmt, EnvFilter};

	tracing_subscriber::registry()
		.with(fmt::layer().compact().with_thread_names(true))
		.with(
			EnvFilter::try_from_default_env()
				.or_else(|_| EnvFilter::try_new("info"))
				.unwrap(),
		)
		.init();
}
