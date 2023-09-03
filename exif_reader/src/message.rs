extern crate exif;

use std::collections::HashMap;
use std::fmt::{Display, Debug};
use std::str::FromStr;

use exif::{Tag, In};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct PhotoData {
    lat: f32,
    long: f32,
    altitude: f32,
    name: String,
    path: String,
    timestamp: String
}

impl PhotoData {
    
    fn new(name: String, path: String) -> PhotoData {
        PhotoData { lat: 0.0, long: 0.0, altitude: 0.0, name: name, path: path, timestamp: "".to_string() }
    }

    fn set_long(&mut self, row_long: &str) {
        
        let re = Regex::new(r"^(\d+) deg (\d+) min (\d*[.]?\d+)").unwrap();
        for r in re.captures_iter(row_long) {
            let deg: f32 = FromStr::from_str(&r[1]).unwrap();
            let min: f32 = FromStr::from_str(&r[2]).unwrap();
            let sec: f32 = FromStr::from_str(&r[3]).unwrap();
            let long =  deg+min/60.0+sec/3600.0;
            self.long = long;
        }
    }

    fn set_lat(&mut self, row_lat: &str) {
        
        let re = Regex::new(r"^(\d+) deg (\d+) min (\d*[.]?\d+)").unwrap();
        for r in re.captures_iter(row_lat) {
            let deg: f32 = FromStr::from_str(&r[1]).unwrap();
            let min: f32 = FromStr::from_str(&r[2]).unwrap();
            let sec: f32 = FromStr::from_str(&r[3]).unwrap();
            let lat = deg+min/60.0+sec/3600.0;
            self.lat = lat;
        }
    }

    fn set_altitude(&mut self, row_altitude: &str) {
        
        let re = Regex::new(r"^(\d*[.]?\d+)").unwrap();
        for r in re.captures_iter(row_altitude) {
            self.altitude = FromStr::from_str(&r[1]).unwrap();
        }
    }

    fn factory(&mut self, tags: &str, values: &str) {
        
        match tags {
            "GPSLatitude" => self.set_lat(values),
            "GPSLongitude" => self.set_long(values),
            "GPSAltitude"=> self.set_altitude(values),
            "DateTime"  => {self.timestamp = values.to_string()},
            &_ => ()
        };
    }
}


impl Display for PhotoData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lat: {}, long: {}, alt: {}, {}, {}, {}", self.lat, self.long, self.altitude, self.name, self.path, self.timestamp)
    }
}


pub fn get_exif(filename: String) -> Result<HashMap<String, PhotoData>, exif::Error> {

    let exif_tags = [Tag::GPSLatitude, Tag::GPSLongitude, Tag::GPSAltitude, Tag::DateTime];
    
    let mut photo: HashMap<String, PhotoData> = HashMap::new();
        
    for path in &[filename] {
        
        
        let file = std::fs::File::open(path)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;
        let data  = photo.entry(path.to_string()).or_insert(PhotoData::new(
                 path.to_string(),
                 path.to_string(),
            ));

        for &tag in exif_tags.iter() {
            if let Some(field) = exif.get_field(tag, In::PRIMARY) {
                let f = field.display_value().with_unit(&exif).to_string();
                data.factory(&format!("{}", tag), &f);   
            }
        }
        println!("{}", data);
    }

    Ok(photo)
}


#[cfg(test)]
mod test {
    use crate::message::get_exif;

    #[test]
    fn test_get_exif() {
        let filename = "../test_data/test_1.jpg".to_string();
        
        let filedata = get_exif(filename);
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
        let filename = "../../test_data/test_1.jpg".to_string();
        let filedata = get_exif(filename);
        assert!(matches!(filedata, Err(_)));
    }
}