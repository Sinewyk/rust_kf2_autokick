// @NOTE (sinewyk): the piece that I was missing, you declare mod once (the first time needed, here in main),
// and then the "crate" is aware of it, so, start from the crate (or super) =)
use crate::config::ServerConfig;
use anyhow::Result;
use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Player {
	playerkey: String,
	perk: String,
	level: u8,
}

#[derive(Debug)]
pub struct ServerState {
	timestamp: u128,
	players: Vec<Player>,
}

fn timestamp() -> u128 {
	let start = SystemTime::now();
	let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("wtf date");
	since_the_epoch.as_millis()
}

fn parse(response: String) -> Result<ServerState> {
	println!("{}", response);
	todo!("take a look at html-extractor for probably exactly what we want");
	Ok(ServerState {
		timestamp: timestamp(),
		players: vec![],
	})
}

pub async fn fetch_infos(config: &ServerConfig) -> Result<ServerState> {
	let client = Client::new();

	let mut req = client.get(&format!("{}/ServerAdmin/current/info", &config.address));

	if let Some(basic_auth) = &config.basic_authorization {
		req = req.basic_auth(&basic_auth.0, Some(&basic_auth.1));
	}

	let resp = req.send().await?.text().await?;

	parse(resp)
}
