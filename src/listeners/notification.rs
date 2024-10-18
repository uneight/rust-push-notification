use futures::StreamExt;
use lapin::{options::*, Channel, Connection, ExchangeKind};
use lapin::message::Delivery;
use lapin::protocol::queue;
use lapin::protocol::queue::AMQPMethod;
use lapin::types::FieldTable;

const NOTIFICATION_CONSUMER_NAME: &str = "notification_consumer";

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

        let exchange = self.channel.exchange_declare(
            "notification-service",
            ExchangeKind::Fanout,
            exchange_options,
            FieldTable::default(),
        ).await?;

        let queue = self.channel.queue_declare(
            topic,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        ).await?;

        QueueBindOptions::default();

        let mut consumer = self.channel.basic_consume(
            topic,
            NOTIFICATION_CONSUMER_NAME,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        ).await?;

        self.channel.queue_bind(
            topic,
            "notification-service",
            "notifications",
            QueueBindOptions::default(),
            FieldTable::default(),
        ).await?;

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                self.handle(delivery).await?;
            }
        }

        Ok(())
    }

    async fn handle(&self, delivery: Delivery) -> Result<(), Box<dyn std::error::Error>> {
        let message = std::str::from_utf8(&delivery.data)?;
        println!("received message {}", message);

        delivery.ack(BasicAckOptions::default()).await?;

        Ok(())
    }
}