use walkdir::WalkDir;
use crate::message::get_exif;

pub fn walking(directory: &str) -> Result<String, walkdir::Error> {
    
    for entry in WalkDir::new(directory) {
        if entry.is_err() {
            return  Err(entry.err().unwrap());
        }
        
        if !entry.as_ref().unwrap().file_type().is_dir() {
            let filename = entry?.path().display().to_string();
            let exif = get_exif(filename);
            println!("{:?}", exif);

        }
    }

    Ok("ok".to_string())
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