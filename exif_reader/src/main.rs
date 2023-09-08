mod config;
mod directory_reader;
mod message;
mod producer;
mod utils;
mod logger;

use producer::produce;

use dotenv::dotenv;
use tonic::{transport::Server, Request, Response};


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
        
        logger::log_info("Starting request from remote client");
        
        let directory_name = &request.into_inner().directory_name;
        logger::log_debug("{directory_name}");

        if let Ok(messages) = directory_reader::walking(directory_name) {
            let result = produce(messages).await;

            if result.is_err() {
                logger::log_error(&format!("Error while producing messages, {:?}", result));
            } else {
                logger::log_info("Succsessfully delivering messages");
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

    logger::init_logger();
    logger::log_info("Start service");

    let grpc_conf = config::Config::from_env().unwrap();
    let addr = format!("{}:{}", grpc_conf.grpcserver.server, grpc_conf.grpcserver.port).parse().unwrap();
    
    let serv = ExifReaderService::default();
    logger::log_info(&format!("Start gRPC server on {}:{}", grpc_conf.grpcserver.server, grpc_conf.grpcserver.port));
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
