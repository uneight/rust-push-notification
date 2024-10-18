use config::{Config, ConfigError, File};
use serde::Deserialize;
use crate::config::apns::{Config as ApnsConfig};
use crate::config::firebase::{Config as FirebaseConfig};
use crate::config::rabbitmq::{Config as RabbitmqConfig};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub firebase: FirebaseConfig,
    pub apns: ApnsConfig,
    pub rabbitmq: RabbitmqConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name("config.toml"))
            .build()?
            .try_deserialize()
    }
}