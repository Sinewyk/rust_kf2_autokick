// @NOTE (sinewyk): the piece that I was missing, you declare mod once (the first time needed, here in main),
// and then the "crate" is aware of it, so, start from the crate (or super) =)
use crate::config::ServerConfig;
use anyhow::Result;
use html_extractor::{html_extractor, HtmlExtractor};
use reqwest::Client;
use std::time::Instant;

#[derive(Debug)]
pub enum Perk {
	Berserker,
	Survivalist,
	Commando,
	Support,
	FieldMedic,
	Demolitionist,
	Firebug,
	Gunslinger,
	Sharpshooter,
	SWAT,
	Unknown,
}

#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub key: String,
	pub perk: Perk,
	pub level: usize,
	pub health: usize,
	pub health_max: usize,
	pub dosh: usize,
	pub kills: usize,
	pub admin: bool,
	pub spectator: bool,
	pub ping: usize,
	pub packetloss: String,
	pub starttime: usize,
	pub deaths: usize,
}

#[derive(Debug)]
pub struct ServerState {
	pub timestamp: Instant,
	pub map: String,
	pub time_elapsed: usize,
	pub time_remaining: usize,
	pub wave: usize,
	pub wave_max: usize,
	pub monsters_total: usize,
	pub monsters_dead: isize,
	pub monsters_pending: isize,
	pub players: Vec<Player>,
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
				perk: match raw_player.perk.as_ref() {
					"KFPerk_Berserker" => Perk::Berserker,
					"KFPerk_Commando" => Perk::Commando,
					"KFPerk_Demolitionist" => Perk::Demolitionist,
					"KFPerk_FieldMedic" => Perk::FieldMedic,
					"KFPerk_Firebug" => Perk::Firebug,
					"KFPerk_Gunslinger" => Perk::Gunslinger,
					"KFPerk_Sharpshooter" => Perk::Sharpshooter,
					"KFPerk_Support" => Perk::Support,
					"KFPerk_Survivalist" => Perk::Survivalist,
					"KFPerk_SWAT" => Perk::SWAT,
					_ => Perk::Unknown, // This should mean that the user didn't finish loading yet
				},
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
