use anyhow::Error;
use reqwest::Client;
use std::env;
use std::time::Instant;

mod config;

#[tokio::main]
async fn main() -> Result<(), Error> {
	let args: Vec<String> = env::args().collect();

	let config = config::get_config(&args)?;

	// print!("Config: {:?}\n", config);

	let client = Client::new();

	let mut req = client.get(&format!("{}/ServerAdmin/current/players", &config.address));

	if let Some(basic_auth) = config.basic_authorization {
		req = req.basic_auth(&basic_auth.0, Some(&basic_auth.1));
	}

	let now = Instant::now();
	let resp = req.send().await?.text().await?;

	println!("{}", now.elapsed().as_millis());
	Ok(())
}
