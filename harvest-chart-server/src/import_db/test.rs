use super::*;

#[test]
fn test_dates() {
    assert_eq!(string_to_day_number("Jan 1"), 1);
    assert_eq!(string_to_day_number("February 28"), 59);
    assert_eq!(string_to_day_number("February 29"), 60);
    assert_eq!(string_to_day_number("March 1"), 61);
    assert_eq!(string_to_day_number("August 12"), 225);
    assert_eq!(string_to_day_number("Dec 31"), 366);

    assert_eq!(string_to_day_number("early January"), 5);
    assert_eq!(string_to_day_number("early to mid jan"), 10);
    assert_eq!(string_to_day_number("early-mid jan"), 10);
    assert_eq!(string_to_day_number("mid jan"), 15);
    assert_eq!(string_to_day_number("mid-late jan"), 20);
    assert_eq!(string_to_day_number("late February"), 56);
}

#[test]
fn test_database_loading() {
    let db_conn = super::establish_connection();
    super::reset_database(&db_conn);

    let items_loaded = super::load_all(&db_conn);
    assert_gt!(items_loaded.plants_found, 200);
    assert_gt!(items_loaded.types_found, 15);
}
