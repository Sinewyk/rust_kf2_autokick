use anyhow::Error;
use ctrlc;
use std::collections::VecDeque;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{convert::TryInto, time::Duration};
use tokio::time;

mod api;
mod config;

#[tokio::main]
async fn main() -> Result<(), Error> {
	let args: Vec<String> = env::args().collect();

	let config = config::get_config(&args)?;
	let sleep_duration =
		Duration::from_millis(config.interval_check.unwrap_or(5000).try_into().unwrap());

	let mut history: VecDeque<api::ServerState> = VecDeque::new();

	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();

	ctrlc::set_handler(move || {
		println!("Shutting down ...");
		r.store(false, Ordering::SeqCst);
	})
	.expect("Error setting Ctrl-C handler");

	while running.load(Ordering::SeqCst) {
		let state = api::parse_infos(api::fetch_infos(&config).await?)?;

		println!("{:#?}", state);

		history.push_front(state);

		if history.len() > 10 {
			history.pop_back();
		}

		time::delay_for(sleep_duration).await;
	}

	Ok(())
}
