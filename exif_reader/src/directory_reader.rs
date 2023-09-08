use crate::message::{get_exif, Message};
use crate::logger;

use walkdir::WalkDir;

// walking is a function that traverses the specified directory, extracts EXIF data from image files,
// and returns a Result containing a vector of Message instances or a walkdir::Error.
// Parameters:
// - directory: A string representing the directory path to traverse and extract EXIF data from.
// Returns:
// - Result<Vec<Message>, walkdir::Error>: A Result containing a vector of Message instances if successful,
//   or a walkdir::Error if an error occurs during directory traversal.
pub fn walking(directory: &str) -> Result<Vec<Message>, walkdir::Error> {
    // Create an empty vector to store the extracted messages.
    let mut messages: Vec<Message> = Vec::new();

    // Iterate over entries (files and directories) in the specified directory.
    for entry in WalkDir::new(directory) {
        // Check if there was an error accessing the entry.
        if entry.is_err() {
            // Log an error message and return the encountered error.
            logger::log_error("Bad directory {directory}");
            return Err(entry.err().unwrap());
        }

        // Check if the entry is not a directory (i.e., it's a file).
        if !entry.as_ref().unwrap().file_type().is_dir() {
            // Get the filename from the entry.
            let filename = entry?.path().display().to_string();

            // Attempt to extract EXIF data from the file using the get_exif function.
            match get_exif(&filename) {
                // If successful, log a debug message and push a new Message instance to the messages vector.
                Ok(e) => {
                    logger::log_debug(&format!("Push new message for {}: {:?}", filename, e));
                    messages.push(Message::new(e))
                },
                // If an error occurs during EXIF extraction, log the error message.
                Err(error) => {
                    logger::log_debug(&format!("{}", error.to_string()));
                    ()
                },
            }
        }
    }

    // Return the vector of extracted messages wrapped in a Result.
    Ok(messages)
}

#[cfg(test)]
mod test {
    use crate::directory_reader::walking;

    #[test]
    fn test_walk_directory() {
        // Specify the directory path for testing.
        let directory = "../test_data/";

        // Call the walking function with the specified directory and capture the result.
        let result = walking(directory);

        // Assert that the result is Ok, indicating success.
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn test_wrong_directory() {
        // Specify a non-existent directory path for testing.
        let directory = "../../test_data/";

        // Call the walking function with the specified directory and capture the result.
        let result = walking(directory);

        // Assert that the result is Err, indicating an error due to the non-existent directory.
        assert!(matches!(result, Err(_)));
    }
}
