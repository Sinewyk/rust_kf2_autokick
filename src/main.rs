use anyhow::Error;
use std::collections::VecDeque;
use std::env;
use std::thread::sleep;
use std::{convert::TryInto, time::Duration};

mod api;
mod config;

#[tokio::main]
async fn main() -> Result<(), Error> {
	let args: Vec<String> = env::args().collect();

	let config = config::get_config(&args)?;
	let sleep_duration =
		Duration::from_millis(config.interval_check.unwrap_or(5000).try_into().unwrap());

	let mut history: VecDeque<api::ServerState> = VecDeque::new();

	loop {
		let state = api::parse_infos(api::fetch_infos(&config).await?)?;

		history.push_front(state);

		if history.len() > 10 {
			history.pop_back();
		}

		sleep(sleep_duration);
	}
}
