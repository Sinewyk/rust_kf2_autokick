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
	health_max: usize,
	dosh: usize,
	kills: usize,
	admin: bool,
	spectator: bool,
	ping: usize,
	packetloss: String,
	starttime: usize,
	deaths: usize,
}

#[derive(Debug)]
pub struct ServerState {
	timestamp: Instant,
	map: String,
	time_elapsed: usize,
	time_remaining: usize,
	wave: usize,
	wave_max: usize,
	monsters_total: usize,
	monsters_dead: isize,
	monsters_pending: isize,
	players: Vec<Player>,
}

html_extractor! {
	RawExtractGlobal {
		map: String = (inner_html of ".game_map"),
		time_elapsed: usize = (inner_html of ".game_time_elapsed"),
		time_remaining: usize = (inner_html of ".game_time_remaining"),
		wave: usize = (inner_html of ".game_wave"),
		wave_max: usize = (inner_html of ".game_wave_max"),
		monsters_dead: isize = (inner_html of ".game_monsters_dead"),
		monsters_pending: isize = (inner_html of ".game_monsters_pending"),
		monsters_total: usize = (inner_html of ".game_monsters_total"),
		players: Vec<RawExtractPlayer> = (elem of ".player_data", collect)
	}
	RawExtractPlayer {
		name: String = (inner_html of ".player_name"),
		key: String = (inner_html of ".player_key"),
		starttime: String = (inner_html of ".player_starttime"),
		perk: String = (inner_html of ".player_perk_class"),
		level: String = (inner_html of ".player_perk_level"),
		health: String = (inner_html of ".player_health"),
		health_max: String = (inner_html of ".player_health_max"),
		dosh: String = (inner_html of ".player_dosh"),
		kills: String = (inner_html of ".player_kills"),
		deaths: String = (inner_html of ".player_deaths"),
		admin: String = (inner_html of ".player_admin"),
		spectator: String = (inner_html of ".player_spectator"),
		ping: String = (inner_html of ".player_ping"),
		packetloss: String = (inner_html of ".player_packetloss"),
	}
}

fn parse(response: String) -> Result<ServerState> {
	let raw_state: RawExtractGlobal = RawExtractGlobal::extract_from_str(&response)?;

	Ok(ServerState {
		timestamp: Instant::now(),
		map: raw_state.map,
		time_elapsed: raw_state.time_elapsed,
		time_remaining: raw_state.time_remaining,
		wave: raw_state.wave,
		wave_max: raw_state.wave_max,
		monsters_total: raw_state.monsters_total,
		monsters_dead: raw_state.monsters_dead,
		monsters_pending: raw_state.monsters_pending,
		players: raw_state
			.players
			.into_iter() // @Note: into_iter *consumes*, and iter() *borrows*
			.map(|raw_player| Player {
				name: raw_player.name,
				key: raw_player.key,
				perk: raw_player.perk,
				level: raw_player.level.parse().unwrap_or(0),
				health: raw_player.health.parse().unwrap_or(0),
				health_max: raw_player.health_max.parse().unwrap_or(0),
				dosh: raw_player.dosh.parse().unwrap_or(0),
				kills: raw_player.kills.parse().unwrap_or(0),
				deaths: raw_player.deaths.parse().unwrap_or(0),
				starttime: raw_player.starttime.parse().unwrap_or(0),
				admin: if raw_player.admin == "Yes" {
					true
				} else {
					false
				},
				spectator: if raw_player.spectator == "Yes" {
					true
				} else {
					false
				},
				ping: raw_player.ping.parse().unwrap_or(0),
				packetloss: raw_player.packetloss,
			})
			.collect::<Vec<Player>>(),
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
