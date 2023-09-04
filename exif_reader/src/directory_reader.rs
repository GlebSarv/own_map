use crate::message::{get_exif, Message};
use walkdir::WalkDir;

/// Recursive traversal directories with photos
/// Args:
///     - directory: &str
/// Output:
///     - respresents success or failure of recursive traversal

pub fn walking(directory: &str) -> Result<Vec<Message>, walkdir::Error> {
    let mut messages: Vec<Message> = Vec::new();
    // walking by directories
    for entry in WalkDir::new(directory) {
        if entry.is_err() {
            return Err(entry.err().unwrap());
        }
        // if file isn't directory, exctraction EXIF information
        if !entry.as_ref().unwrap().file_type().is_dir() {
            let filename = entry?.path().display().to_string();
            match get_exif(filename) {
                Ok(e) => messages.push(Message::new(e)),
                Err(_error) => (),
            }
        }
    }

    Ok(messages)
}

#[cfg(test)]
mod test {
    use crate::directory_reader::walking;

    #[test]
    fn test_walk_directory() {
        let directory = "../test_data/";
        let result = walking(directory);
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn test_wrong_directory() {
        let directory = "../../test_data/";
        let result = walking(directory);
        assert!(matches!(result, Err(_)));
    }
}
