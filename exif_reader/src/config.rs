use config::{Config as ENVConfig, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KafkaConfig {
    pub bootstrapserver: String,
    pub topics: String,
    pub timeout: i32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub kafka: KafkaConfig,
}

impl Config {
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
    #[test]
    fn test_config() {
        dotenv().ok();
        let kp = Config::from_env();
        assert!(matches!(kp, Ok(_)));
    }
}
