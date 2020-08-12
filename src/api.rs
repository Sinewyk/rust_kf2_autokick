// @NOTE (sinewyk): the piece that I was missing, you declare mod once (the first time needed, here in main),
// and then the "crate" is aware of it, so, start from the crate (or super) =)
use crate::config::ServerConfig;
use anyhow::Result;
use html_extractor::{html_extractor, HtmlExtractor};
use reqwest::Client;
use serde::Deserialize;
use std::time::Instant;

#[derive(Debug, Deserialize)]
struct PlayerRead {
	name: String,
	key: String,
	perk: String,
	level: String,
	health: String,
	dosh: String,
	kills: String,
	admin: String,
	spectator: String,
	ping: String,
	packetloss: String,
}

#[derive(Debug)]
struct Player {
	name: String,
	key: String,
	perk: String,
	level: usize,
	health: usize,
	dosh: usize,
	kills: usize,
	admin: bool,
	spectator: bool,
	ping: usize,
	packetloss: String,
}

#[derive(Debug)]
pub struct ServerState {
	timestamp: Instant,
	players: Vec<Player>,
}

#[derive(Debug, Deserialize)]
struct GameStateRead {
	map: String,
	time_elapsed: usize,
	time_limit: usize,
	wave: usize,
	monsters_dead: usize,
	monsters_total: usize,
}

html_extractor! {
	#[derive(Debug, PartialEq)]
	As_Json {
		game: String = (text of ".game_info"),
		players: Vec<String> = (text of ".player", collect)
	}
}

fn parse(response: String) -> Result<ServerState> {
	let foo = As_Json::extract_from_str(&response)?;
	println!("{:?}", foo);
	let ssr: GameStateRead = serde_json::from_str(&foo.game)?;
	println!("{:?}", ssr);
	let psr: Vec<PlayerRead> = foo
		.players
		.iter()
		.map(|x| serde_json::from_str(&x).unwrap())
		.collect();
	println!("{:?}", psr);

	Ok(ServerState {
		timestamp: Instant::now(),
		players: psr
			.iter()
			.map(move |player| Player {
				name: player.name.clone(),
				key: player.key.clone(),
				perk: player.perk.clone(),
				level: player.level.parse::<usize>().unwrap(),
				health: player.health.parse::<usize>().unwrap(),
				dosh: player.dosh.parse::<usize>().unwrap(),
				kills: player.kills.parse::<usize>().unwrap(),
				admin: if player.admin == "No" { false } else { true },
				spectator: if player.spectator == "No" {
					false
				} else {
					true
				},
				ping: player.ping.parse::<usize>().unwrap(),
				packetloss: player.packetloss.clone(),
			})
			.collect(),
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
