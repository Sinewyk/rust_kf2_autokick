// @NOTE (sinewyk): the piece that I was missing, you declare mod once (the first time needed, here in main),
// and then the "crate" is aware of it, so, start from the crate (or super) =)
use crate::config::ServerConfig;
use anyhow::Result;
use html_extractor::{html_extractor, HtmlExtractor};
use reqwest::Client;
use std::time::Instant;

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

html_extractor! {
	#[derive(Debug, PartialEq)]
	RawExtractGlobal {
		game_map: String = (inner_html of ".game_map"),
		game_time_elapsed: usize = (inner_html of ".game_time_elapsed"),
		game_time_limit: usize = (inner_html of ".game_time_limit"),
		game_time_remaining: usize = (inner_html of ".game_time_remaining"),
		game_wave: usize = (inner_html of ".game_wave"),
		game_wave_max: usize = (inner_html of ".game_wave_max"),
		game_monsters_dead: usize = (inner_html of ".game_monsters_dead"),
		game_monsters_pending: usize = (inner_html of ".game_monsters_pending"),
		game_monsters_total: usize = (inner_html of ".game_monsters_total"),
		players: Vec<RawExtractPlayer> = (elem of ".player_data", collect)
	}
	#[derive(Debug, PartialEq)]
	RawExtractPlayer {
		player_name: String = (inner_html of ".player_name"),
		player_key: String = (inner_html of ".player_key"),
		player_starttime: String = (inner_html of ".player_starttime"),
		player_perk_class: String = (inner_html of ".player_perk_class"),
		player_perk_level: String = (inner_html of ".player_perk_level"),
		player_health: String = (inner_html of ".player_health"),
		player_health_max: String = (inner_html of ".player_health_max"),
		player_dosh: String = (inner_html of ".player_dosh"),
		player_kills: String = (inner_html of ".player_kills"),
		player_deaths: String = (inner_html of ".player_deaths"),
		player_lives: String = (inner_html of ".player_lives"),
		player_admin: String = (inner_html of ".player_admin"),
		player_spectator: String = (inner_html of ".player_spectator"),
		player_ping: String = (inner_html of ".player_ping"),
		player_packetloss: String = (inner_html of ".player_packetloss"),
	}
}

fn parse(response: String) -> Result<ServerState> {
	let raw_state: RawExtractGlobal = RawExtractGlobal::extract_from_str(&response)?;
	println!("{:#?}", raw_state);

	todo!("finish parsing state")
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
