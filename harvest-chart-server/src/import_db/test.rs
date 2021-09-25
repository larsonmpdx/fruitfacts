use super::*;
#[test]
fn test_is_a_midpoint() {
    assert_eq!(is_a_midpoint(""), false);
    assert_eq!(is_a_midpoint("mid"), false);
    assert_eq!(is_a_midpoint("Sep 15-30"), false);
    assert_eq!(is_a_midpoint("Sep 15"), false);

    assert_eq!(is_a_midpoint("sep"), true);
    assert_eq!(is_a_midpoint("mid October"), true);
}

#[test]
fn test_month_location() {
    assert_eq!(month_location(""), MonthLocationType::NoMonth);
    assert_eq!(
        month_location("september 23"),
        MonthLocationType::MonthAtBeginning
    );
    assert_eq!(
        month_location("mid september"),
        MonthLocationType::MonthAtEnd
    );
    assert_eq!(month_location("Sep"), MonthLocationType::MonthAtBeginning);
}

#[test]
fn test_get_month() {
    assert_eq!(get_month("Sep"), "sep")
}

#[test]
fn test_dates() {
    assert_eq!(string_to_day_number("Jan 1"), 1);
    assert_eq!(string_to_day_number("February 28"), 59);
    assert_eq!(string_to_day_number("February 29"), 60);
    assert_eq!(string_to_day_number("March 1"), 61);
    assert_eq!(string_to_day_number("August 12"), 225);
    assert_eq!(string_to_day_number("Sep 20"), 264);
    assert_eq!(string_to_day_number("Dec 31"), 366);

    assert_eq!(string_to_day_number("early January"), 5);
    assert_eq!(string_to_day_number("early to mid jan"), 10);
    assert_eq!(string_to_day_number("early-mid jan"), 10);
    assert_eq!(string_to_day_number("mid jan"), 15);
    assert_eq!(string_to_day_number("mid-late jan"), 20);
    assert_eq!(string_to_day_number("late February"), 56);
    assert_eq!(string_to_day_number("early August"), 218);
    assert_eq!(string_to_day_number("mid-late August"), 233);
    assert_eq!(string_to_day_number("late August"), 238);

    assert_eq!(string_to_day_number("mar"), 75);
    assert_eq!(string_to_day_number("April"), 106);
}

#[test]
fn test_day_range() {
    assert_eq!(
        string_to_day_range("Early August").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::StartOnly,
            start: 218,
            end: 0
        }
    );
    assert_eq!(
        string_to_day_range("August 7").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::StartOnly,
            start: 220,
            end: 0
        }
    );
    assert_eq!(
        string_to_day_range("mid-late August").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::StartOnly,
            start: 233,
            end: 0
        }
    );

    assert_eq!(
        string_to_day_range("August").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::Midpoint,
            start: 228,
            end: 0
        }
    );
    assert_eq!(
        string_to_day_range("mid September").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::Midpoint,
            start: 259,
            end: 0
        }
    );

    assert_eq!(
        string_to_day_range("early to late August").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: 218,
            end: 238
        }
    );

    assert_eq!(
        string_to_day_range("late August to mid September").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: 238,
            end: 259
        }
    );
    assert_eq!(
        string_to_day_range("Sep 20-30").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: 264,
            end: 274
        }
    );
    assert_eq!(
        string_to_day_range("Sep 25-Oct 5").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: 269,
            end: 279
        }
    );
    assert_eq!(
        string_to_day_range("June 21 to July 10").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: 173,
            end: 192
        }
    );
    assert_eq!(
        string_to_day_range("Oct-Nov").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: 289,
            end: 320
        }
    );
    assert_eq!(
        string_to_day_range("late Oct-Nov").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: 299,
            end: 320
        }
    );
}

#[test]
fn test_patent_parsing() {
    assert_eq!(
        string_to_patent_info(""),
        PatentInfo {
            uspp_number: 0,
            uspp_expiration: Utc.ymd(1970, 01, 01)
        }
    );

    assert_eq!(
        string_to_patent_info("https://patents.google.com/patent/USPP9881 expired 2014"),
        PatentInfo {
            uspp_number: 9881,
            uspp_expiration: Utc.ymd(2014, 01, 01)
        }
    );

    assert_eq!(
        string_to_patent_info("https://patents.google.com/patent/USPP17827 expires 2026-01-18"),
        PatentInfo {
            uspp_number: 17827,
            uspp_expiration: Utc.ymd(2026, 01, 18)
        }
    );
}

#[test]
fn test_database_loading() {
    let db_conn = super::establish_connection();
    super::reset_database(&db_conn);

    let items_loaded = super::load_all(&db_conn);
    assert_gt!(items_loaded.plants_found, 275);
    assert_gt!(items_loaded.types_found, 15);
}

#[test]
fn test_simplify_path() {
    assert_eq!(simplify_path(r#".\..\plant_database\references"#), "");
    assert_eq!(simplify_path("./../plant_database/references/Oregon"), "Oregon");
    assert_eq!(simplify_path("./../plant_database/references/Oregon/Willamette Valley"), "Oregon/Willamette Valley");
}
