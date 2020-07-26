use std::env;
use std::process::exit;

mod config;

fn main() {
	let args: Vec<String> = env::args().collect();

	let config = match config::get_config(&args) {
		Ok(config) => config,
		Err(e) => {
			eprint!("Error: {}\n", e);
			exit(1);
		}
	};

	print!("ok: {:?}", config);
}
