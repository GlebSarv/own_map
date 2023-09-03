use crate::config::Config;
use crate::message::Message;
use log::info;
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

pub async fn produce(messages: Vec<Message>) -> Result<(), rdkafka::error::KafkaError> {
    let config = Config::from_env();
    let config = config.unwrap();
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", config.kafka.bootstrapserver)
        .set("message.timeout.ms", format!("{}", config.kafka.timeout))
        .create()
        .expect("Producer creation error");

    for message in messages {
        let producing_message = producer
            .send(
                FutureRecord::to(&config.kafka.topics)
                    .payload(&format!("{}", message.value))
                    .key(&format!("{}", message.key))
                    .headers(OwnedHeaders::new().add(&message.key, "exif_data")),
                Duration::from_secs(0),
            )
            .await;

        match producing_message {
            Ok((int_value, long_value)) => {
                info!("{:?} {} {}", producing_message, int_value, long_value);
            }
            Err((kafka_error, _)) => {
                return Err(kafka_error);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::message::{Message, PhotoData};
    use crate::producer::produce;

    #[tokio::test]
    async fn test_produce() {
        let mut photo_data: HashMap<String, PhotoData> = HashMap::new();
        let mut messages: Vec<Message> = Vec::new();

        photo_data
            .entry("title".to_string())
            .or_insert(PhotoData::default());
        let message = Message::new(photo_data);
        messages.push(message);

        let p = produce(messages).await;

        assert!(matches!(p, Ok(_)))
    }
}
