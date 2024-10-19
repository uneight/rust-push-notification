use std::error::Error;
use futures::StreamExt;
use lapin::{options::*, Channel, Connection, ExchangeKind};
use lapin::message::Delivery;
use lapin::types::FieldTable;
use crate::entities::message::{Message};

const NOTIFICATION_CONSUMER_NAME: &str = "notification_consumer";
const NOTIFICATION_EXCHANGE_NAME: &str = "notification-service";

pub struct Notification {
    channel: Channel,
}

impl Notification {
    pub async fn new(conn: Connection) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = conn.create_channel().await?;

        Ok(Notification { channel })
    }

    pub async fn listen(&self, topic: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut exchange_options = ExchangeDeclareOptions::default();
        exchange_options.durable = false;

        self.channel.exchange_declare(
            NOTIFICATION_EXCHANGE_NAME,
            ExchangeKind::Fanout,
            exchange_options,
            FieldTable::default(),
        ).await?;

        let queue = self.channel.queue_declare(
            topic,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        ).await?;

        self.channel.queue_bind(
            queue.name().as_str(),
            NOTIFICATION_EXCHANGE_NAME,
            "notifications",
            QueueBindOptions::default(),
            FieldTable::default(),
        ).await?;

        let mut consumer = self.channel.basic_consume(
            queue.name().as_str(),
            NOTIFICATION_CONSUMER_NAME,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        ).await?;

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                let result = self.handle(delivery).await;

                match result {
                    Ok(res) => {
                        println!("{:?}", res);
                    }

                    Err(e) => {
                        eprintln!("can not parse notification message: {}", e);
                    }
                }
            }
        }

        Ok(())
    }

    async fn handle(&self, delivery: Delivery) -> Result<Message, Box<dyn Error>> {
        let message: Message = serde_json::from_slice(&delivery.data.as_slice())?;

        delivery.ack(BasicAckOptions::default()).await?;

        Ok(message)
    }
}