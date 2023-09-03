use config::{Config as ENVConfig, ConfigError, Environment};
use serde::Deserialize;

/// KafkaConfig structure to defining parameters for producing messages to kafka
/// - bootstrapserver: host:port kafka server
/// - topics: topics for producing message
/// - timeout: time for refusing 

#[derive(Debug, Deserialize)]
pub struct KafkaConfig {
    pub bootstrapserver: String,
    pub topics: String,
    pub timeout: i32,
}

/// Config structure to defining KafkaConfig
#[derive(Debug, Deserialize)]
pub struct Config {
    pub kafka: KafkaConfig,
}


impl Config {
    /// implementing method for reading data from .env file
    pub fn from_env() -> Result<Self, ConfigError> {
        let builder = ENVConfig::builder()
            .set_default("default", 1)?
            .add_source(Environment::default())
            .build()?;

        builder.try_deserialize::<Config>()
    }
}

#[cfg(test)]
mod test {

    use crate::config::Config;
    use dotenv::dotenv;

    /// testing reading .env file
    
    #[test]
    fn test_config() {
        dotenv().ok();
        let kp = Config::from_env();
        assert!(matches!(kp, Ok(_)));
    }
}
