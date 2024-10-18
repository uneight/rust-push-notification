use lapin::{Connection, ConnectionProperties};
use lapin::uri::{AMQPAuthority, AMQPUri, AMQPUserInfo};
use crate::config::rabbitmq::Config;

pub struct RabbitClient {
    pub conn: Connection,
}

impl RabbitClient {
    pub async fn new(cfg: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let mut uri = AMQPUri::default();

        uri.authority = AMQPAuthority{
            userinfo: AMQPUserInfo {
                username: cfg.username,
                password: cfg.password,
            },
            host: cfg.host,
            port: cfg.port,
        };

        let conn = Connection::connect_uri(uri, ConnectionProperties::default()).await?;

        Ok(RabbitClient { conn })
    }
}