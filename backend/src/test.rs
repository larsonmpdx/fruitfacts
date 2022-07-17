use crate::gazetteer_load;

#[test]
fn test_latitude_normalize() {
    assert_eq!(
        gazetteer_load::from_to_location("")
            .unwrap_err()
            .to_string(),
        "lat/lon wrong number of elements"
    );

    assert_eq!(
        gazetteer_load::from_to_location("a,b")
            .unwrap_err()
            .to_string(),
        "lat/lon didn't parse"
    );

    assert_eq!(
        gazetteer_load::from_to_location("45.7,-122.8").unwrap(),
        gazetteer_load::MapCoordinates {
            lat: 45.7,
            lon: -122.8,
        }
    );

    assert_eq!(
        gazetteer_load::from_to_location("zip:uh")
            .unwrap_err()
            .to_string(),
        "zip didn't parse as a number"
    );
    assert_eq!(
        gazetteer_load::from_to_location("zip:4294967295")
            .unwrap_err()
            .to_string(),
        "zip code not found"
    );
    assert_eq!(
        gazetteer_load::from_to_location("zip:00601").unwrap(),
        gazetteer_load::MapCoordinates {
            lat: 18.180555,
            lon: -66.749961,
        }
    );
    assert_eq!(
        gazetteer_load::from_to_location("zip:97231").unwrap(),
        gazetteer_load::MapCoordinates {
            lat: 45.687631,
            lon: -122.824202,
        }
    );
    assert_eq!(
        gazetteer_load::from_to_location("zip:99929").unwrap(),
        gazetteer_load::MapCoordinates {
            lat: 56.221499,
            lon: -131.923588,
        }
    );
}
