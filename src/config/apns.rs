use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub team_id: String,
    pub key_id: String,
    pub url: String,
    pub bundle_id: String,
}

