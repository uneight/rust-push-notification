mod config;
mod providers;
mod rpc;
mod rabbit;
mod listeners;

use tokio;
use tonic::transport::Server;
use crate::config::app::AppConfig;
use crate::rpc::service::Service;
use crate::rpc::service::api::device_service_server::{DeviceServiceServer};
use crate::rabbit::client::RabbitClient;
use crate::listeners::notification::Notification;
use crate::providers::fcm::Fcm;
use crate::providers::apns::Apns;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Notification service is running!");
    let cfg = AppConfig::load()?;

    let title = "Some test notification local RUST";
    let body = "Something text for something notification";

    let fcm = Fcm::new(cfg.firebase);
    let apns = Apns::new(cfg.apns);

    let rabbitmq = RabbitClient::new(cfg.rabbitmq).await?;
    let notification_listener = Notification::new(rabbitmq.conn).await?;

    let addr = "[::1]:50051".parse().unwrap();
    let service = Service::new(fcm, apns, notification_listener).await?;

    Server::builder()
        .add_service(DeviceServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}