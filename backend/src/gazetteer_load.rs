// on program start, reads "zip code tabulation areas" *.txt from /gazetteer_load/ into a memory structure
// then is able to do conversions from zip to lat/lon

use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct MapCoordinates {
    pub lat: f64,
    pub lon: f64,
}

lazy_static! {
    static ref TYPE_TO_CANDLE: HashMap<u32, MapCoordinates> = gazetteer_load();
}

fn gazetteer_load() -> HashMap<u32, MapCoordinates> {
    let mut output = Default::default();

    // todo - load from a text file or panic
    
    // todo - check quantity loaded and panic if not enough

    output
}

pub fn get_zip_coordinates(zip: u32) -> Option<MapCoordinates> {
    None
}

// from the search query string, either lat/lon like "45.687631,-122.824202" or zip code like "zip:97231"
pub fn from_to_location(input: &str) -> Option<MapCoordinates> {
    // todo
    None
}
