extern crate exif;

use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::utils::convert_coordinate;
use exif::{In, Tag};
use regex::Regex;
use serde_json::json;

/// PhotoData - strucure with photo's metadata
#[derive(Debug, Clone)]
pub struct PhotoData {
    lat: f32,
    long: f32,
    altitude: f32,
    name: String,
    path: String,
    timestamp: String,
}

/// default data for photo
impl Default for PhotoData {
    fn default() -> Self {
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

/// implementation methods for PhotoData
/// new - return new PhotoData object
///
/// set_long - parse metadata, and convert longitude from format `00 deg 00 min 00.0` to 00.0000
/// set_lat - parse metadata, and convert latitude from format `00 deg 00 min 00.0` to 00.0000
/// set_alitude - parse altitude
/// build - setting long, lat and altitude for PhotoData
impl PhotoData {
    fn new(name: String, path: String) -> PhotoData {
        PhotoData {
            lat: 0.0,
            long: 0.0,
            altitude: 0.0,
            name: name,
            path: path,
            timestamp: "".to_string(),
        }
    }

    fn set_long(&mut self, row_long: &str) {
        self.long = convert_coordinate(row_long)
    }

    fn set_lat(&mut self, row_lat: &str) {
        self.lat = convert_coordinate(row_lat)
    }

    fn set_altitude(&mut self, row_altitude: &str) {
        let re = Regex::new(r"^(\d*[.]?\d+)").unwrap();
        for r in re.captures_iter(row_altitude) {
            self.altitude = FromStr::from_str(&r[1]).unwrap();
        }
    }

    fn build(&mut self, tags: &str, values: &str) {
        match tags {
            "GPSLatitude" => self.set_lat(values),
            "GPSLongitude" => self.set_long(values),
            "GPSAltitude" => self.set_altitude(values),
            "DateTime" => self.timestamp = values.to_string(),
            &_ => (),
        };
    }
}

/// Implementing displayng of PhotoData structure
impl Display for PhotoData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "lat: {}, long: {}, alt: {}, {}, {}, {}",
            self.lat, self.long, self.altitude, self.name, self.path, self.timestamp
        )
    }
}

/// Message - structure, which is sending to kafka
/// - key: message key - String
/// - value: serde_json::Value - body of message
pub struct Message {
    pub key: String,
    pub value: serde_json::Value,
}

impl Message {
    pub fn new(exif: HashMap<String, PhotoData>) -> Self {
        let keys: Vec<String> = exif.clone().into_keys().collect();
        let mut title = "".to_string();
        let mut value = json!({});

        for key in keys.into_iter() {
            let data = exif.get(&key).unwrap();
            title = key;
            value = json!({
                "lat": data.lat,
                "long": data.long,
                "altitude": data.altitude,
            });
        }

        Message {
            key: title,
            value: value,
        }
    }
}

/// Getting metadata from photo
/// Args:
///     - filename
/// Output:
///     - respresents success or failure of data extraction
pub fn get_exif(filename: &str) -> Result<HashMap<String, PhotoData>, exif::Error> {
    // list of tags which are extract from photo
    let exif_tags = [
        Tag::GPSLatitude,
        Tag::GPSLongitude,
        Tag::GPSAltitude,
        Tag::DateTime,
    ];

    let mut photo: HashMap<String, PhotoData> = HashMap::new();

    for path in [filename] {
        // get file
        let file = std::fs::File::open(path)?;
        // read file
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        // reading metadata
        let exif = exifreader.read_from_container(&mut bufreader)?;
        // pushing new k,v for HashMap, if key unique, insert new PhotoData
        let data = photo
            .entry(path.to_string())
            .or_insert(PhotoData::new(path.to_string(), path.to_string()));

        // iteration via exif_tags, and extraction information tags inforation
        for &tag in exif_tags.iter() {
            if let Some(field) = exif.get_field(tag, In::PRIMARY) {
                let f = field.display_value().with_unit(&exif).to_string();
                data.build(&format!("{}", tag), &f);
            }
        }
    }

    Ok(photo)
}

#[cfg(test)]
mod test {

    use crate::message::{get_exif, PhotoData};

    #[test]
    fn test_get_exif() {
        let filename = "../test_data/test_1.jpg";
        let filedata = get_exif(&filename);
        assert!(matches!(filedata, Ok(_)));
        
        let filedata = filedata.unwrap();
        let filename = "../test_data/test_1.jpg";
        let metadata = filedata.get(filename).unwrap();
        
        assert_eq!(metadata.altitude, 27.813);
        assert_eq!(metadata.long, 39.032085);
        assert_eq!(metadata.lat, 45.043938);
    }

    #[test]
    fn test_wrong_directory() {
        let filename = "../../test_data/test_1.jpg";
        let filedata = get_exif(&filename);
        assert!(matches!(filedata, Err(_)));
    }

    #[test]
    fn test_default() {
        let photo_data = PhotoData::default();

        assert_eq!(photo_data.altitude, 0.0);
        assert_eq!(photo_data.long, 0.0);
        assert_eq!(photo_data.lat, 0.0);
    }
}
