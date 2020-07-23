use std::env;

mod config;

use config::Config;

fn main() {
	let args: Vec<String> = env::args().collect();
	let config = Config::new(&args).unwrap();
}
