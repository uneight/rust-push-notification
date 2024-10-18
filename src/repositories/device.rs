use sqlx::{Executor};
use crate::entities::device::Device;

pub struct DeviceRepository {
    conn: sqlx::PgPool,
}

pub trait Repository<T> {
    fn new(conn: sqlx::PgPool) -> Self;
    async fn create(&self, entity: T) -> Result<T, Box<dyn std::error::Error>>;
}

impl Repository<Device> for DeviceRepository {
    fn new(conn: sqlx::PgPool) -> Self {
        DeviceRepository { conn }
    }

    async fn create(&self, entity: Device) -> Result<Device, Box<dyn std::error::Error>> {
        let query = sqlx::query("insert into user_devices (id, user_id, device_token, os) values ($1, $2, $3, $4);")
            .bind(entity.id.clone())
            .bind(entity.user_id.clone())
            .bind(entity.device_token.clone())
            .bind(entity.os.clone());

        let result = self.conn.execute(query).await;

        match result {
            Ok(res) => {
                Ok(entity)
            }
            Err(e) => {
                Err(e.into())
            }
        }
    }
}