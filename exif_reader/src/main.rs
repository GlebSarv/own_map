// Import necessary modules and libraries.
mod config;
mod directory_reader;
mod message;
mod producer;
mod utils;
mod logger;


// Import the 'produce' function from the 'producer' module.
use producer::produce;

// Import dotenv for environment variable loading, tonic for gRPC, and other modules.
use dotenv::dotenv;
use tonic::{transport::Server, Request, Response};

// Import the generated gRPC code for ExifReaders service.
use exif_readers::exif_readers_server::{ExifReaders, ExifReadersServer};
use exif_readers::{ExifReaderRequest, ExifReadersReply};

pub mod exif_readers {
    tonic::include_proto!("exif_readers");
}

// Define a struct for the ExifReaderService.
#[derive(Debug, Default)]
pub struct ExifReaderService {}

// Implement the gRPC service trait for ExifReaderService.
#[tonic::async_trait]
impl ExifReaders for ExifReaderService {
    // Define the 'walking_directory' method for the gRPC service.
    async fn walking_directory(
        &self,
        request: Request<ExifReaderRequest>,
    ) -> Result<tonic::Response<ExifReadersReply>, tonic::Status> {
        
        // Log an informational message indicating the start of a remote client request.
        logger::log_info("Starting request from remote client");
        
        // Extract the 'directory_name' from the gRPC request.
        let directory_name = &request.into_inner().directory_name;
        
        // Log a debug message containing the 'directory_name'.
        logger::log_debug("{directory_name}");

        // Attempt to retrieve messages by walking the specified directory.
        if let Ok(messages) = directory_reader::walking(directory_name) {
            // Produce the retrieved messages.
            let result = produce(messages).await;

            // Check the result of message production.
            if result.is_err() {
                // Log an error message if message production fails.
                logger::log_error(&format!("Error while producing messages, {:?}", result));
            } else {
                // Log an informational message if message production is successful.
                logger::log_info("Successfully delivering messages");
            }
        }

        // Return a gRPC response indicating the completion of the request.
        Ok(Response::new(ExifReadersReply {}))
    }
}

// Define the main function.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from a .env file.
    dotenv().ok();

    // Initialize the logger.
    logger::init_logger();
    
    // Log an informational message indicating the start of the service.
    logger::log_info("Start service");

    // Retrieve gRPC configuration from environment variables.
    let grpc_conf = config::Config::from_env().unwrap();
    let addr = format!("{}:{}", grpc_conf.grpcserver.server, grpc_conf.grpcserver.port).parse().unwrap();

    // Create an instance of the ExifReaderService.
    let serv = ExifReaderService::default();
    
    // Log an informational message indicating the start of the gRPC server.
    logger::log_info(&format!("Start gRPC server on {}:{}", grpc_conf.grpcserver.server, grpc_conf.grpcserver.port));
    
    // Start the gRPC server and serve on the specified address.
    Server::builder()
        .add_service(ExifReadersServer::new(serv))
        .serve(addr)
        .await?;

    // Return a successful result.
    Ok(())
}

// Define a module for testing.
#[cfg(test)]
mod test {
    use crate::directory_reader::walking;
    use crate::producer::produce;

    // Define an integration test function.
    #[tokio::test]
    async fn integration_test() {
        // Define a test directory.
        let directory = "../test_data/";

        // Retrieve messages by walking the test directory.
        let messages = walking(directory);
        assert!(matches!(messages, Ok(_)));

        // Produce the retrieved messages.
        let result = produce(messages.unwrap()).await;
        assert!(matches!(result, Ok(_)));
    }
}