use anyhow::{anyhow, Error};
use serde::Deserialize;
use std::fs;

// enum Perks {
// 	Berserker,
// 	Survivalist,
// 	Commando,
// 	Support,
// 	FieldMedic,
// 	Demolitionist,
// 	Firebug,
// 	Gunslinger,
// 	Sharpshooter,
// 	SWAT,
// }

// enum Actions {
// 	Kick,
// 	SessionBan,
// 	BanIp,
// 	BanId,
// }

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
	address: String,
	basic_authorization: Option<String>,
	interval_check: Option<usize>,
	action: Option<String>,
	minimum_level: Option<usize>,
	warnings: Option<bool>,
	warning_message: Option<String>,
	warning_period: Option<usize>,
	remove_perks: Option<Vec<String>>,
	log: Option<bool>,
}

pub fn get_config(args: &[String]) -> Result<ServerConfig, Error> {
	if args.len() != 2 {
		return Err(anyhow!("You must only provide a path to the config file"));
	}

	let path = &args[1];

	let parsed = serde_json::from_str(&fs::read_to_string(path)?)?;

	Ok(parsed)
}
