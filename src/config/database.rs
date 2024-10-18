use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub dsn: String,
}