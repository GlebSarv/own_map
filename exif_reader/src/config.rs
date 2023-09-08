// Import necessary crates and modules.
use config::{Config as ENVConfig, ConfigError, Environment};
use serde::Deserialize;

// Define a struct for Kafka configuration.
#[derive(Debug, Deserialize)]
pub struct KafkaConfig {
    pub bootstrapserver: String,
    pub topics: String,
    pub timeout: i32,
}

// Enable a warning for non-camel-case type names.
#[warn(non_camel_case_types)]
// Define a struct for gRPC server configuration.
#[derive(Debug, Deserialize)]
pub struct gRPCServer {
    pub server: String,
    pub port: i32,
}

// Define a main configuration struct that aggregates Kafka and gRPC server configurations.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub kafka: KafkaConfig,
    pub grpcserver: gRPCServer,
}

impl Config {
    // Implement a constructor method to create a configuration from environment variables.
    pub fn from_env() -> Result<Self, ConfigError> {
        // Create a configuration builder, set defaults, add environment variables as a source, and build the configuration.
        let builder = ENVConfig::builder()
            .set_default("default", 1)?
            .add_source(Environment::default())
            .build()?;
        
        // Attempt to deserialize the configuration from the builder.
        builder.try_deserialize::<Config>()
    }
}

// Define a test module.
#[cfg(test)]
mod test {
    // Import necessary modules for testing.
    use crate::config::Config;
    use dotenv::dotenv;

    // Define a test function for configuration.
    #[test]
    fn test_config() {
        // Load environment variables from a .env file.
        dotenv().ok();
        // Create a configuration instance from environment variables.
        let kp = Config::from_env();
        // Assert that the configuration creation is successful.
        assert!(matches!(kp, Ok(_)));
    }
}

