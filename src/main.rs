mod api;
mod config;

use anyhow::Error;
use config::ServerConfig;
use ctrlc;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{convert::TryInto, time::Duration};
use tokio::time;

pub type History = VecDeque<api::ServerState>;

#[tokio::main]
async fn main() -> Result<(), Error> {
	let config = ServerConfig::new()?;

	println!("{:#?}", &config);

	let sleep_duration =
		Duration::from_millis(config.interval_check.unwrap_or(5000).try_into().unwrap());

	let mut history: History = VecDeque::new();

	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();

	ctrlc::set_handler(move || {
		println!("Shutting down ...");
		r.store(false, Ordering::SeqCst);
	})
	.expect("Error setting Ctrl-C handler");

	while running.load(Ordering::SeqCst) {
		let infos = match api::fetch_infos(&config).await {
			Ok(infos) => infos,
			Err(err) => {
				println!("{}", err);
				time::delay_for(sleep_duration).await;
				continue;
			}
		};
		let state = api::parse_infos(infos)?;

		println!("{:#?}", state);

		history.push_front(state);

		if history.len() > 10 {
			history.pop_back();
		}

		let mut need_to_warn = false;

		for state in history.iter() {
			for player in &state.players {
				match player.is_in_infraction(&config, &history) {
					Some(0) => println!("First infraction"),
					Some(x) => println!("Not first infraction"),
					None => println!("Player is clean"),
				}
			}
			break;
		}

		time::delay_for(sleep_duration).await;
	}

	Ok(())
}
