use anyhow::{anyhow, Error};
use std::fs;
use tinyjson::JsonValue;

enum Perks {
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
}

enum Actions {
	Kick,
	SessionBan,
	BanIp,
	BanId,
}

pub struct ServerConfig {
	basicAuthorization: String,
	intervalCheck: usize,
	action: Actions,
	minLevel: usize,
	warnings: bool,
	warningMessage: String,
	warningPeriod: usize,
	removePerks: Vec<Perks>,
	log: bool,
}

pub type Config = Vec<ServerConfig>;

pub fn get_config(args: &[String]) -> Result<Config, Error> {
	if args.len() != 2 {
		return Err(anyhow!("You must only provide a path to the config file"));
	}

	let path = &args[1];

	let parsed: JsonValue = fs::read_to_string(path)?
		.parse()
		.or_else(|e: tinyjson::JsonParseError| Err(anyhow!(e)))?;

	// print!("{:?}", parsed);

	Ok(vec![])
}
