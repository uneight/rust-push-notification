mod config;
mod providers;
mod rpc;
mod rabbit;
mod listeners;
mod repositories;
mod entities;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio;
use tonic::transport::Server;
use crate::config::app::AppConfig;
use crate::rpc::service::Service;
use crate::rpc::service::api::device_service_server::{DeviceServiceServer};
use crate::rabbit::client::RabbitClient;
use crate::listeners::notification::Notification;
use crate::providers::fcm::Fcm;
use crate::providers::apns::Apns;
use crate::repositories::device::{DeviceRepository, Repository};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = AppConfig::load()?;

    let fcm = Fcm::new(cfg.firebase);
    let apns = Apns::new(cfg.apns);

    let rabbitmq = RabbitClient::new(cfg.rabbitmq).await?;
    let notification_listener = Notification::new(rabbitmq.conn).await?;

    let db_conn = sqlx::postgres::PgPoolOptions::new().max_connections(10).connect(&cfg.database.dsn).await?;
    let device_repository = DeviceRepository::new(db_conn);
    let service = Service::new(fcm, apns, notification_listener, device_repository).await?;


    let ip_addr: IpAddr = cfg.rpc.host.parse()?;
    let addr = SocketAddr::new(ip_addr, cfg.rpc.port);

    println!("Start server listening on {}", addr);

    Server::builder()
        .add_service(DeviceServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}