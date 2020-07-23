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

struct ServerConfig {
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

pub struct Config {
	servers: Vec<ServerConfig>,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Self, &'static str> {
		if args.len() != 2 {
			return Err("You must only provide a path to the config file");
		}

		Ok(Config { servers: vec![] })
	}
}
