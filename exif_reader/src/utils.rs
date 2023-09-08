use regex::Regex;
use std::str::FromStr;

pub fn convert_coordinate(raw_coord: &str) -> f32 {
    let re = Regex::new(r"^(\d+) deg (\d+) min (\d*[.]?\d+)").unwrap();
    for r in re.captures_iter(raw_coord) {
        let deg: f32 = FromStr::from_str(&r[1]).unwrap();
        let min: f32 = FromStr::from_str(&r[2]).unwrap();
        let sec: f32 = FromStr::from_str(&r[3]).unwrap();
        let coord = deg + min / 60.0 + sec / 3600.0;
        return coord;
    }
    return 0.0;
}

#[cfg(test)]
mod test {
    use crate::utils::convert_coordinate;
    
    #[test]
    fn test_coord() {
        let raw_coord = vec!["53 deg 43 min 23.808 sec N", "41 deg 42 min 34.9 sec N"];

        println!("{}", convert_coordinate(raw_coord[1]));
        assert_eq!(convert_coordinate(raw_coord[0]), 53.72328);
    }

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
}