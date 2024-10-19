use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
}