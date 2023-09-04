mod config;
mod directory_reader;
mod message;
mod producer;
mod utils;

use clap::{App, Arg};
use dotenv::dotenv;
use log::{error, info};
use producer::produce;

/// Main function
///
/// Required argument `directory` to define the directory in which photos will be searched
#[tokio::main]
async fn main() {
    dotenv().ok();

    let matches = App::new("exif_reader")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .arg(
            Arg::with_name("directory")
                .short('d')
                .long("directory")
                .help("Specify directory with photo")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let directory = matches.value_of("directory").unwrap();

    if let Ok(messages) = directory_reader::walking(directory) {
        let result = produce(messages).await;

        if result.is_err() {
            error!("Error while producing messages");
        } else {
            info!("Succsessfully delivering messages")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::directory_reader::walking;
    use crate::producer::produce;

    #[tokio::test]
    async fn integration_test() {
        let directory = "../test_data/";

        let messages = walking(directory);
        assert!(matches!(messages, Ok(_)));

        let result = produce(messages.unwrap()).await;
        assert!(matches!(result, Ok(_)));
    }
}
