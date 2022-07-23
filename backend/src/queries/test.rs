use crate::queries::map::latitude_normalize;
use super::search::{distance_to_degrees, DistanceDegrees};

#[test]
fn test_latitude_normalize() {
    assert_eq!(latitude_normalize(0.0), 0.0);
    assert_eq!(latitude_normalize(180.0), -180.0);
    assert_eq!(latitude_normalize(-180.0), -180.0);
    assert_eq!(latitude_normalize(359.0), -1.0);
    assert_eq!(latitude_normalize(360.0), 0.0);
    assert_eq!(latitude_normalize(361.0), 1.0);
    assert_eq!(latitude_normalize(1000.0), -80.0);
    assert_eq!(latitude_normalize(-1000.0), 80.0);
    assert_eq!(latitude_normalize(-124.98151399125442), -124.98151399125442);
    assert_eq!(latitude_normalize(-116.2177107239959), -116.2177107239959);
    assert_eq!(latitude_normalize(-193.94874081126167), 166.0512591887383);
    assert_eq!(latitude_normalize(-175.0567314859299), -175.0567314859299);
}


#[test]
fn test_distance_to_degrees() {
    // todo, once the formula is put in and we handle mi and km
    assert_eq!(distance_to_degrees("25mi"), Some(DistanceDegrees { lat: 5.0, lon: 5.0 }))
}