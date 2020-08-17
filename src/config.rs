use anyhow::{anyhow, Error};
use serde::Deserialize;
use std::fs;

// enum Actions {
// 	Kick,
// 	SessionBan,
// 	BanIp,
// 	BanId,
// }

#[derive(Deserialize)]
pub struct ServerConfig {
	pub address: String,
	pub basic_authorization: Option<(String, String)>,
	pub interval_check: Option<usize>,
	pub action: Option<String>,
	pub minimum_level: Option<usize>,
	pub warnings: Option<bool>,
	pub warning_message: Option<String>,
	pub warning_period: Option<usize>,
	pub remove_perks: Option<Vec<String>>,
	pub log: Option<bool>,
}

pub fn get_config(args: &[String]) -> Result<ServerConfig, Error> {
	if args.len() != 2 {
		return Err(anyhow!("You must only provide a path to the config file"));
	}

	let path = &args[1];

	Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}
