// @NOTE (sinewyk): the piece that I was missing, you declare mod once (the first time needed, here in main),
// and then the "crate" is aware of it, so, start from the crate (or super) =)
use crate::config::ServerConfig;
use reqwest::{Client, Result};

pub async fn fetch_infos(config: &ServerConfig) -> Result<String> {
	let client = Client::new();

	let mut req = client.get(&format!("{}/ServerAdmin/current/info", &config.address));

	if let Some(basic_auth) = &config.basic_authorization {
		req = req.basic_auth(&basic_auth.0, Some(&basic_auth.1));
	}

	req.send().await?.text().await
}
