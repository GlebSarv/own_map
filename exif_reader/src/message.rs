// Import necessary external crates and libraries.
extern crate exif;

use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::utils::{convert_coordinate, convert_time_to_iso_format};
use exif::{In, Tag};
use regex::Regex;
use serde_json::json;

// Define a struct to hold photo data.
#[derive(Debug, Clone)]
pub struct PhotoData {
    lat: f32,
    long: f32,
    altitude: f32,
    name: String,
    path: String,
    timestamp: String,
}

// Implement the default trait for PhotoData.
impl Default for PhotoData {
    fn default() -> Self {
        // Create a default PhotoData instance.
        PhotoData {
            lat: 0.0,
            long: 0.0,
            altitude: 0.0,
            name: "name".to_string(),
            path: "path".to_string(),
            timestamp: "".to_string(),
        }
    }
}

// Implement methods for the PhotoData struct.
impl PhotoData {
    // Create a new PhotoData instance with specified name and path.
    fn new(name: String, path: String) -> PhotoData {
        // Initialize a PhotoData instance with default values.
        PhotoData {
            lat: 0.0,
            long: 0.0,
            altitude: 0.0,
            name: name,
            path: path,
            timestamp: "".to_string(),
        }
    }

    // Set the longitude value of PhotoData.
    fn set_long(&mut self, row_long: &str) {
        self.long = convert_coordinate(row_long)
    }

    // Set the latitude value of PhotoData.
    fn set_lat(&mut self, row_lat: &str) {
        self.lat = convert_coordinate(row_lat)
    }

    // Set the altitude value of PhotoData.
    fn set_altitude(&mut self, row_altitude: &str) {
        // Define a regular expression pattern for altitude extraction.
        let re = Regex::new(r"^(\d*[.]?\d+)").unwrap();
        for r in re.captures_iter(row_altitude) {
            self.altitude = FromStr::from_str(&r[1]).unwrap();
        }
    }

    // Build PhotoData attributes based on provided tags and values.
    fn build(&mut self, tags: &str, values: &str) {
        match tags {
            "GPSLatitude" => self.set_lat(values),
            "GPSLongitude" => self.set_long(values),
            "GPSAltitude" => self.set_altitude(values),
            "DateTime" => self.timestamp = convert_time_to_iso_format(values),
            &_ => (),
        };
    }
}

// Implement the Display trait for PhotoData.
impl Display for PhotoData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "lat: {}, long: {}, alt: {}, {}, {}, {}",
            self.lat, self.long, self.altitude, self.name, self.path, self.timestamp
        )
    }
}

// Define a struct to represent a message.
pub struct Message {
    pub key: String,
    pub value: serde_json::Value,
}

// Implement methods for the Message struct.
impl Message {
    // Create a new Message instance from a HashMap of PhotoData.
    pub fn new(exif: HashMap<String, PhotoData>) -> Self {
        // Extract keys from the HashMap.
        let keys: Vec<String> = exif.clone().into_keys().collect();
        let mut title = "".to_string();
        let mut value = json!({});

        for key in keys.into_iter() {
            let data = exif.get(&key).unwrap();
            title = key;
            // Build a JSON value with lat, long, altitude, and timestamp.
            value = json!({
                "lat": data.lat,
                "long": data.long,
                "altitude": data.altitude,
                "tmstmp": data.timestamp,
            });
        }

        Message {
            key: title,
            value: value,
        }
    }
}

// Define a function to extract EXIF data from a photo file.
pub fn get_exif(filename: &str) -> Result<HashMap<String, PhotoData>, exif::Error> {
    // Define an array of EXIF tags to extract.
    let exif_tags = [
        Tag::GPSLatitude,
        Tag::GPSLongitude,
        Tag::GPSAltitude,
        Tag::DateTime,
    ];

    let mut photo: HashMap<String, PhotoData> = HashMap::new();

    for path in [filename] {
        let file = std::fs::File::open(path)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;

        let data = photo
            .entry(path.to_string())
            .or_insert(PhotoData::new(path.to_string(), path.to_string()));

        for &tag in exif_tags.iter() {
            if let Some(field) = exif.get_field(tag, In::PRIMARY) {
                let f = field.display_value().with_unit(&exif).to_string();
                data.build(&format!("{}", tag), &f);
            }
        }
    }

    Ok(photo)
}

// Define a module for testing.
#[cfg(test)]
mod test {
    use crate::message::{get_exif, PhotoData};

    // Define a test function for extracting EXIF data.
    #[test]
    fn test_get_exif() {
        // Define a test photo file.
        let filename = "../test_data/test_1.jpg";
        let filedata = get_exif(&filename);
        assert!(matches!(filedata, Ok(_)));
        
        let filedata = filedata.unwrap();
        let filename = "../test_data/test_1.jpg";
        let metadata = filedata.get(filename).unwrap();
        
        // Perform assertions on extracted metadata.
        assert_eq!(metadata.altitude, 27.813);
        assert_eq!(metadata.long, 39.032085);
        assert_eq!(metadata.lat, 45.043938);
        assert_eq!(metadata.timestamp, "2021-01-04T14:49:57+00:00");
    }

    // Define a test function for handling an invalid directory.
    #[test]
    fn test_wrong_directory() {
        let filename = "../../test_data/test_1.jpg";
        let filedata = get_exif(&filename);
        assert!(matches!(filedata, Err(_)));
    }

    // Define a test function for default PhotoData values.
    #[test]
    fn test_default() {
        let photo_data = PhotoData::default();

        // Perform assertions on default values.
        assert_eq!(photo_data.altitude, 0.0);
        assert_eq!(photo_data.long, 0.0);
        assert_eq!(photo_data.lat, 0.0);
    }
}

