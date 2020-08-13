use anyhow::Error;
use std::env;
use std::time::Instant;

mod api;
mod config;

#[tokio::main]
async fn main() -> Result<(), Error> {
	let args: Vec<String> = env::args().collect();

	let config = config::get_config(&args)?;

	let now = Instant::now();
	let last_state = api::fetch_infos(&config).await?;

	println!("{}", now.elapsed().as_millis());
	println!("{:?}", last_state);

	Ok(())
}
