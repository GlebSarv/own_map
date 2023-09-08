// Import required external crates and modules.
extern crate chrono;
use regex::Regex;
use std::str::FromStr;
use chrono::{NaiveDateTime, Utc, TimeZone};

// Function to convert raw coordinates (degrees, minutes, seconds) to decimal degrees.
pub fn convert_coordinate(raw_coord: &str) -> f32 {
    // Define a regular expression to match the expected coordinate format.
    let re = Regex::new(r"^(\d+) deg (\d+) min (\d*[.]?\d+)").unwrap();
    
    // Iterate through captured groups in the regex.
    for r in re.captures_iter(raw_coord) {
        // Parse degrees, minutes, and seconds from the captured groups.
        let deg: f32 = FromStr::from_str(&r[1]).unwrap();
        let min: f32 = FromStr::from_str(&r[2]).unwrap();
        let sec: f32 = FromStr::from_str(&r[3]).unwrap();
        
        // Calculate and return the decimal coordinate.
        let coord = deg + min / 60.0 + sec / 3600.0;
        return coord;
    }
    
    // Return 0.0 if no valid coordinate is found.
    return 0.0;
}

// Function to convert a timestamp string to ISO 8601 format.
pub fn convert_time_to_iso_format(tmstmp: &str) -> String {
    // Parse the timestamp string into a NaiveDateTime.
    let parsed_time = NaiveDateTime::parse_from_str(tmstmp, "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse the input string");

    // Convert the NaiveDateTime to a DateTime<Utc> (UTC time zone).
    let datetime_utc = Utc.from_utc_datetime(&parsed_time);

    // Format the DateTime in ISO 8601 format.
    let iso_format = datetime_utc.to_rfc3339();

    // Return the formatted ISO 8601 timestamp.
    iso_format
}

// Test module for the coordinate and timestamp conversion functions.
#[cfg(test)]
mod test {
    use super::*;

    // Test coordinate conversion function.
    #[test]
    fn test_coord() {
        let raw_coord = vec![
            "53 deg 43 min 23.808 sec N",
            "41 deg 42 min 34.9 sec N",
        ];

        // Test the conversion of a valid coordinate.
        assert_eq!(convert_coordinate(raw_coord[0]), 53.72328);
    }

    // Test coordinate conversion function with various input formats.
    #[test]
    fn test_wrong_coord() {
        let raw_coords = vec![
            ("53 deg 43 min 23.808 sec N", 53.72328),
            ("53", 0.0),
            ("53 deg", 0.0),
            ("deg 43 min 23.808 sec N", 0.0),
            ("53 dg 43 min 23.808 sec N", 0.0),
            ("", 0.0),
            ("53 dg 43 min 23.808 sec", 0.0),
            ("53 dg 43 min 23.808 N", 0.0),
            ("53 dg 43 min 23 sec N", 0.0),
            ("53 dg 43 min .808 sec N", 0.0),
        ];

        for raw_coord in raw_coords {
            let coord = convert_coordinate(raw_coord.0);
            assert_eq!(coord, raw_coord.1);
        }
    }

    // Test timestamp conversion function.
    #[test]
    fn test_convert_time() {
        let times = vec![
            ("2021-01-04 14:49:57", "2021-01-04T14:49:57+00:00"),
            ("2023-01-04 14:49:57", "2023-01-04T14:49:57+00:00"),
            ("2021-02-04 14:49:57", "2021-02-04T14:49:57+00:00"),
            ("2021-02-07 14:49:57", "2021-02-07T14:49:57+00:00"),
            ("2021-02-04 15:49:57", "2021-02-04T15:49:57+00:00"),
            ("2021-02-04 14:59:57", "2021-02-04T14:59:57+00:00"),
            ("2021-02-04 14:59:03", "2021-02-04T14:59:03+00:00"),
        ];

        for time in times {
            let iso_time = convert_time_to_iso_format(time.0);
            assert_eq!(iso_time, time.1)
        }
    }
}
