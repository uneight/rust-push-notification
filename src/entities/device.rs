use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Device {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_token: String,
    pub os: String,
}