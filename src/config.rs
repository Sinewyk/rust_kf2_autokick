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

pub fn get_config(args: &[String]) -> Result<Config, &'static str> {
	if args.len() != 2 {
		return Err("You must only provide a path to the config file");
	}

	let path = &args[1];

	let parsed: JsonValue = fs::read_to_string(path).unwrap().parse().unwrap();

	print!("{:?}", parsed);

	Ok(vec![])
}
