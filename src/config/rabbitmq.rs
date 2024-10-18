use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub scheme: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}