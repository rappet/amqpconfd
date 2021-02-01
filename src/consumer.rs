use anyhow::Context;
use futures_util::StreamExt;
use lapin::options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{Channel, Connection, ConnectionProperties, Consumer, Queue};
use log::{error, info};
use serde_json::Value;
use std::future::Future;
use tokio_amqp::LapinTokioExt;

pub struct ConfigChangeConsumer {
    /// AQMP connection
    connection: Connection,
    /// AQMP channel
    channel: Channel,
    /// AQMP queue
    queue: Queue,
    /// AQMP consumer
    consumer: Consumer,
}

impl ConfigChangeConsumer {
    /// Opens a connection to the RabbitMQ server, and creates a channel
    pub async fn connect(uri: &str, topic: &str) -> anyhow::Result<ConfigChangeConsumer> {
        let connection =
            Connection::connect(uri, ConnectionProperties::default().with_tokio()).await?;
        info!("amqp connected");

        let channel = connection.create_channel().await?;
        info!("connection state: {:?}", connection.status().state());

        let queue = channel
            .queue_declare(topic, QueueDeclareOptions::default(), FieldTable::default())
            .await?;
        info!("Declared queue {:?}", queue);

        let consumer = channel
            .basic_consume(
                topic,
                "foo",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        Ok(ConfigChangeConsumer {
            connection,
            channel,
            queue,
            consumer,
        })
    }

    async fn handle_message<F, Fut>(msg: &[u8], handler: &F) -> anyhow::Result<()>
    where
        F: Fn(Value) -> Fut,
        Fut: Future<Output = anyhow::Result<()>>,
    {
        let value = serde_json::from_slice(msg)?;
        handler(value).await?;
        Ok(())
    }

    /// consumes messages
    pub async fn consume<F, Fut>(&mut self, handler: F) -> anyhow::Result<()>
    where
        F: Fn(Value) -> Fut,
        Fut: Future<Output = anyhow::Result<()>>,
    {
        while let Some(delivery) = self.consumer.next().await {
            let (_, delivery) = delivery?;

            match Self::handle_message(delivery.data.as_slice(), &handler).await {
                Ok(()) => {
                    delivery.ack(BasicAckOptions::default()).await?;
                }
                Err(e) => {
                    error!("{:?}", e);
                    delivery.nack(BasicNackOptions::default()).await?;
                }
            }
        }

        Ok(())
    }
}
