use super::*;

#[test]
fn my_test() {
    assert_eq!(string_to_day_number("Jan 1"), 1);
    assert_eq!(string_to_day_number("February 28"), 59);
    assert_eq!(string_to_day_number("February 29"), 60);
    assert_eq!(string_to_day_number("March 1"), 61);
    assert_eq!(string_to_day_number("August 12"), 225);
    assert_eq!(string_to_day_number("Dec 31"), 366);
}
