use tonic::{Request, Response, Status};

pub mod api {
    tonic::include_proto!("api");
}

use api::device_service_server::{DeviceService};
use api::{RegisterDeviceRequest, RegisterDeviceResponse};
use providers::fcm::Fcm;
use providers::apns::Apns;
use crate::listeners::notification::Notification;
use crate::providers;

pub struct Service {
    fcm: Fcm,
    apns: Apns,
    notification_listener: Notification,
}

impl Service {
    pub async fn new(fcm: Fcm, apns: Apns, notification_listener: Notification) -> Result<Self, Box<dyn std::error::Error>> {
        notification_listener.listen("notifications").await?;

        Ok(Service {fcm, apns, notification_listener})
    }
}

#[tonic::async_trait]
impl DeviceService for Service {
    async fn register_device(&self, request: Request<RegisterDeviceRequest>) -> Result<Response<RegisterDeviceResponse>, Status> {
        let req = request.into_inner();
        let device_token = req.device_token;
        let user_id = req.user_id;
        let os = req.os;

        let resp = RegisterDeviceResponse {
          message: format!("Device {device_token} received for user {user_id}, os: {os}")
        };

        Ok(Response::new(resp))
    }
}