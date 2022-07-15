use crate::gazetteer_load;

#[test]
fn test_latitude_normalize() {
    assert_eq!(from_to_location(""), None);
}
