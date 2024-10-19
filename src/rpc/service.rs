use std::error::Error;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod api {
    tonic::include_proto!("api");
}

use api::device_service_server::{DeviceService};
use api::{RegisterDeviceRequest, RegisterDeviceResponse, SubscribeRequest, SubscribeResponse};
use providers::fcm::Fcm;
use providers::apns::Apns;
use crate::entities::device::Device;
use crate::listeners::notification::Notification;
use crate::providers;
use crate::repositories::device::{DeviceRepository, Repository};

pub struct Service {
    fcm: Fcm,
    apns: Apns,
    device_repository: DeviceRepository
}

impl Service {
    pub async fn new(fcm: Fcm, apns: Apns, notification_listener: Notification, device_repository: DeviceRepository) -> Result<Self, Box<dyn std::error::Error>> {
        tokio::spawn(async move {
            if let Err(e) = notification_listener.listen("notifications").await {
                eprintln!("notification_listener error: {:?}", e);
            }
        });

        Ok(Service {fcm, apns, device_repository})
    }
}

#[tonic::async_trait]
impl DeviceService for Service {
    async fn register_device(&self, request: Request<RegisterDeviceRequest>) -> Result<Response<RegisterDeviceResponse>, Status> {
        let req = request.into_inner();

        let os = req.os;
        let topic = req.topic;
        let user_id = Uuid::parse_str(&req.user_id).unwrap();
        let device_token = req.device_token;

        let result = self.device_repository.create(Device{
            user_id,
            device_token,
            os,
        }).await;

        match result {
            Ok(res) => {
                let resp = RegisterDeviceResponse {
                    message: format!("Device {} received for user {}, os: {}", res.device_token, res.user_id, res.os.to_string())
                };

                Ok(Response::new(resp))
            }
            Err(e) => {
                eprintln!("Error creating device: {}", e);

                Err(Status::internal("Failed to create device"))
            }
        }
    }
    async fn subscribe(&self, request: Request<SubscribeRequest>) -> Result<Response<SubscribeResponse>, Status> {
        let request = request.into_inner();

        let resp = SubscribeResponse {
            message: "Subscribed".to_string()
        };

        Ok(Response::new(resp))
    }
}