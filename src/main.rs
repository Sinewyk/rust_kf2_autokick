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
	let resp = api::fetch_infos(&config).await?;

	println!("{}", now.elapsed().as_millis());
	// println!("{}", resp);

	todo!("implement loop, push server state into vector up to #, implement diff to do stuff");

	Ok(())
}
