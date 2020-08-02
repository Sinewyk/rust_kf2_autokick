use anyhow::Error;
use std::env;

mod config;

fn main() -> Result<(), Error> {
	let args: Vec<String> = env::args().collect();

	let config = config::get_config(&args)?;

	print!("Config: {:?}", config);

	Ok(())
}
