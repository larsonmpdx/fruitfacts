use crate::gazetteer_load;

#[test]
fn test_latitude_normalize() {
    assert_eq!(gazetteer_load::from_to_location(""), None);
    assert_eq!(
        gazetteer_load::from_to_location("zip:97231"),
        Some(gazetteer_load::MapCoordinates {
            lat: 45.687631,
            lon: -122.824202,
        })
    );
}
