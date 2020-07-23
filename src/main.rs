use std::env;

mod config;

fn main() {
	let args: Vec<String> = env::args().collect();

	let config = config::get_config(&args).unwrap();
}
