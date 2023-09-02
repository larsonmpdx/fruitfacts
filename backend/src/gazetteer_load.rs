// on program start, reads "zip code tabulation areas" *.txt from /gazetteer_load/ into a memory structure
// then is able to do conversions from zip to lat/lon

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct MapCoordinates {
    pub lat: f64,
    pub lon: f64,
}

lazy_static! {
    static ref TYPE_TO_CANDLE: HashMap<u32, MapCoordinates> = gazetteer_load();
}

fn gazetteer_load() -> HashMap<u32, MapCoordinates> {
    let mut output: HashMap<u32, MapCoordinates> = Default::default();

    let gazetteer_file = include_str!("gazetteer_load/2021_Gaz_zcta_national.txt");

    // contents look like:
    // GEOID  ALAND      AWATER  ALAND_SQMI  AWATER_SQMI  INTPTLAT   INTPTLONG
    // 00601  166847909  799292  64.42       0.309        18.180555  -66.749961
    // ...
    // [0]    [1]        [2]     [3]         [4]          [5]        [6]

    for line in gazetteer_file.lines().skip(1) {
        let split = line.split('\t').collect::<Vec<&str>>();
        if split.len() >= 7 {
            let zip_result = split[0].trim().parse::<u32>();
            let lat_result = split[5].trim().parse::<f64>();
            let lon_result = split[6].trim().parse::<f64>();

            if zip_result.is_err() || lat_result.is_err() || lon_result.is_err() {
                panic!("error parsing gazetteer file: zip {} -> {zip_result:?} {} -> lat {lat_result:?} lon {} -> {lon_result:?}",
                split[0].trim(),
                split[5].trim(),
                split[6].trim())
            }
            let zip = zip_result.unwrap();
            let lat = lat_result.unwrap();
            let lon = lon_result.unwrap();

            output.insert(zip, MapCoordinates { lat, lon });
        }
    }

    assert_gt!(output.keys().len(), 5); // would like this to be 33k+ but I have a truncated demo file in there for CI. look at the file by hand to see, when it's updated

    output
}

pub fn get_zip_coordinates(zip: u32) -> Option<MapCoordinates> {
    TYPE_TO_CANDLE.get(&zip).cloned()
}

// from the search query string, either lat/lon like "45.687631,-122.824202" or zip code like "zip:97231"
pub fn from_to_location(input: &str) -> Result<MapCoordinates> {
    match input {
        s if s.starts_with("zip:") => {
            let zip_string: String = s.chars().skip(4).collect();
            match zip_string.parse::<u32>() {
                Ok(zip) => match get_zip_coordinates(zip) {
                    Some(coordinates) => Ok(coordinates),
                    None => Err(anyhow!("zip code not found")),
                },
                Err(_) => Err(anyhow!("zip didn't parse as a number")),
            }
        }
        s => {
            // regular lat/lon coordinates
            let split = s.split(',').collect::<Vec<&str>>();
            if split.len() != 2 {
                return Err(anyhow!("lat/lon wrong number of elements"));
            }

            let lat_result = split[0].parse::<f64>();
            let lon_result = split[1].parse::<f64>();

            if lat_result.is_err() || lon_result.is_err() {
                return Err(anyhow!("lat/lon didn't parse"));
            }

            let lat = lat_result.unwrap();
            let lon = lon_result.unwrap();
            Ok(MapCoordinates { lat, lon })
        }
    }
}
