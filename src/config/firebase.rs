use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub server_key: String,
    pub url: String,
}
