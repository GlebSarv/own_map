mod config;
mod directory_reader;
mod message;
mod producer;
mod utils;

use producer::produce;

use clap::{App, Arg};
use dotenv::dotenv;
use log::{error, info};
use tonic::{transport::Server, Request, Response, Status};


use exif_readers::exif_readers_server::{ExifReaders, ExifReadersServer};
use exif_readers::{ExifReaderRequest, ExifReadersReply};

pub mod exif_readers {
    tonic::include_proto!("exif_readers");
}

#[derive(Debug, Default)]
pub struct ExifReaderService{}

#[tonic::async_trait]
impl ExifReaders for ExifReaderService {
    async fn walking_directory(
        &self,
        request: Request<ExifReaderRequest>
    ) -> Result<tonic::Response<ExifReadersReply>, tonic::Status> {
        println!("Starting request");
        let directory_name = &request.into_inner().directory_name;

        if let Ok(messages) = directory_reader::walking(directory_name) {
            let result = produce(messages).await;

            if result.is_err() {
                error!("Error while producing messages");
            } else {
                info!("Succsessfully delivering messages")
            }
        }

        Ok(Response::new(ExifReadersReply {}))
    }
}

/// Main function
///
/// Required argument `directory` to define the directory in which photos will be searched
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // let directory = matches.value_of("directory").unwrap();

    // if let Ok(messages) = directory_reader::walking(directory) {
    //     let result = produce(messages).await;

    //     if result.is_err() {
    //         error!("Error while producing messages");
    //     } else {
    //         info!("Succsessfully delivering messages")
    //     }
    // }
    let addr = "127.0.0.1:50051".parse().unwrap();
    let serv = ExifReaderService::default();

    Server::builder()
        .add_service(ExifReadersServer::new(serv))
        .serve(addr)
        .await?;

    Ok(())    
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
