// Import required modules and structs from the project.
use crate::config::Config;
use crate::message::Message;
use crate::logger;

// Import necessary modules from the rdkafka crate.
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

// Asynchronously produce Kafka messages.
pub async fn produce(messages: Vec<Message>) -> Result<(), rdkafka::error::KafkaError> {
    // Load Kafka configuration from environment.
    let config = Config::from_env().unwrap();

    // Create a Kafka producer using the provided configuration.
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", &config.kafka.bootstrapserver)
        .set("message.timeout.ms", format!("{}", config.kafka.timeout))
        .create()
        .expect("Producer creation error");

    // Log the start of message production to Kafka.
    logger::log_info(&format!(
        "Start producing messages to Kafka on {}",
        config.kafka.bootstrapserver
    ));

    // Iterate through the list of messages to be produced.
    for message in messages {
        // Log the message value being produced.
        logger::log_info(&format!("{:?}", message.value));

        // Create a Kafka FutureRecord with message payload, key, and headers.
        let producing_message = producer
            .send(
                FutureRecord::to(&config.kafka.topics)
                    .payload(&format!("{}", message.value))
                    .key(&format!("{}", message.key))
                    .headers(OwnedHeaders::new().add(&message.key, "exif_data")),
                Duration::from_secs(0),
            )
            .await;

        // Handle the result of message production.
        match producing_message {
            Ok((int_value, long_value)) => {
                // Log successful message delivery.
                logger::log_debug(&format!("{:?} {} {}", producing_message, int_value, long_value));
            }
            Err((kafka_error, _)) => {
                // Log and return an error in case of delivery failure.
                logger::log_error(&format!(
                    "Error in delivering message to Kafka topic {:?}",
                    kafka_error
                ));
                return Err(kafka_error);
            }
        }
    }

    // Return Ok if all messages are successfully produced.
    Ok(())
}

// Define test cases for the producer function.
#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::message::{Message, PhotoData};
    use crate::producer::produce;

    // Test the produce function.
    #[tokio::test]
    async fn test_produce() {
        // Create a HashMap of PhotoData.
        let mut photo_data: HashMap<String, PhotoData> = HashMap::new();

        // Create a Message instance and add it to a vector of messages.
        let mut messages: Vec<Message> = Vec::new();
        photo_data
            .entry("title".to_string())
            .or_insert(PhotoData::default());
        let message = Message::new(photo_data);
        messages.push(message);

        // Call the produce function and assert that it returns Ok.
        let p = produce(messages).await;
        assert!(matches!(p, Ok(_)))
    }
}
