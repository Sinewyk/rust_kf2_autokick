use anyhow::{anyhow, Error};
use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Debug, Deserialize)]
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

// @NOTE (sinewyk): the deref is supposed to be automatic
// so why is this block necessary ?
impl From<&String> for Perk {
	fn from(perk: &String) -> Self {
		let perk: &str = perk;
		Self::from(perk)
	}
}

impl From<&str> for Perk {
	fn from(perk_string: &str) -> Self {
		match perk_string {
			"KFPerk_Berserker" => Self::Berserker,
			"KFPerk_Commando" => Self::Commando,
			"KFPerk_Demolitionist" => Self::Demolitionist,
			"KFPerk_FieldMedic" => Self::FieldMedic,
			"KFPerk_Firebug" => Self::Firebug,
			"KFPerk_Gunslinger" => Self::Gunslinger,
			"KFPerk_Sharpshooter" => Self::Sharpshooter,
			"KFPerk_Support" => Self::Support,
			"KFPerk_Survivalist" => Self::Survivalist,
			"KFPerk_SWAT" => Self::SWAT,
			_ => Self::Unknown, // This should mean that the user didn't finish loading yet
		}
	}
}

// enum Actions {
// 	Kick,
// 	SessionBan,
// 	BanIp,
// 	BanId,
// }

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
	pub address: String,
	pub basic_authorization: Option<(String, String)>,
	pub interval_check: Option<usize>,
	pub action: Option<String>,
	pub minimum_level: Option<usize>,
	pub warnings: Option<bool>,
	pub warning_message: Option<String>,
	pub warning_period: Option<usize>,
	pub remove_perks: Option<Vec<Perk>>,
	pub log: Option<bool>,
}

const NO_RULES_ERR: &str =
	"You didn't actually provide any rules (neither level nor forbidden perks)";

impl ServerConfig {
	pub fn new() -> Result<ServerConfig, Error> {
		let args: Vec<String> = env::args().collect();

		if args.len() != 2 {
			return Err(anyhow!("You must only provide a path to the config file"));
		}

		let path = &args[1];

		let config: ServerConfig = serde_json::from_str(&fs::read_to_string(path)?)?;

		if let None = &config.minimum_level {
			if let Some(perks) = &config.remove_perks {
				if perks.len() == 0 {
					return Err(anyhow!(NO_RULES_ERR));
				}
			}
			if let None = &config.remove_perks {
				return Err(anyhow!(NO_RULES_ERR));
			}
		}

		Ok(config)
	}
}
