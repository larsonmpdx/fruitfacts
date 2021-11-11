use super::*;
#[test]
fn test_is_a_midpoint() {
    assert!(!is_a_midpoint(""));
    assert!(!is_a_midpoint("mid"));
    assert!(!is_a_midpoint("Sep 15-30"));
    assert!(!is_a_midpoint("Sep 15"));

    assert!(is_a_midpoint("sep"));
    assert!(is_a_midpoint("mid October"));
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
    assert_eq!(string_to_day_number("Jan 1"), Some(1));
    assert_eq!(string_to_day_number("February 28"), Some(59));
    assert_eq!(string_to_day_number("February 29"), Some(60));
    assert_eq!(string_to_day_number("March 1"), Some(61));
    assert_eq!(string_to_day_number("August 12"), Some(225));
    assert_eq!(string_to_day_number("Sep 20"), Some(264));
    assert_eq!(string_to_day_number("Dec 31"), Some(366));

    assert_eq!(string_to_day_number("early January"), Some(5));
    assert_eq!(string_to_day_number("early to mid jan"), Some(10));
    assert_eq!(string_to_day_number("early-mid jan"), Some(10));
    assert_eq!(string_to_day_number("mid jan"), Some(15));
    assert_eq!(string_to_day_number("mid-late jan"), Some(20));
    assert_eq!(string_to_day_number("late February"), Some(56));
    assert_eq!(string_to_day_number("early August"), Some(218));
    assert_eq!(string_to_day_number("mid-late August"), Some(233));
    assert_eq!(string_to_day_number("late August"), Some(238));

    assert_eq!(string_to_day_number("mar"), Some(75));
    assert_eq!(string_to_day_number("April"), Some(106));

    assert_eq!(string_to_day_number("eary Jun"), None);

    assert_eq!(
        string_to_day_number(" Around April 16 (Gainesville, FL)"),
        Some(107)
    );
    assert_eq!(
        string_to_day_number("Around May 4 (Gainesville, FL)"),
        Some(125)
    );

    assert_eq!(
        string_to_day_number("9/25"),
        string_to_day_number("September 25")
    );
    assert_eq!(
        string_to_day_number("10/15"),
        string_to_day_number("October 15")
    );
}

#[test]
fn test_day_range() {
    assert_eq!(
        string_to_day_range("eary Jun"), // misspelled - parse error
        None
    );
    assert_eq!(
        string_to_day_range("Early").unwrap(), // this refers to "early within season" and we're not parsing it for now, just store the text
        DayRangeOutput {
            parse_type: DateParseType::Unparsed,
            start: None,
            end: None
        }
    );
    assert_eq!(
        string_to_day_range("Early August").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::StartOnly,
            start: Some(218),
            end: None
        }
    );
    assert_eq!(
        string_to_day_range("August 7").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::StartOnly,
            start: Some(220),
            end: None
        }
    );
    assert_eq!(
        string_to_day_range("August 15").unwrap(), // not midpoint, even though it's mid month, because it's an exact date
        DayRangeOutput {
            parse_type: DateParseType::StartOnly,
            start: Some(228),
            end: None
        }
    );
    assert_eq!(
        string_to_day_range("mid-late August").unwrap(), // not a recommended format because the '-' gets it parsed as two dates
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: Some(228),
            end: Some(238)
        }
    );
    assert_eq!(
        string_to_day_range("mid to late August").unwrap(), // same
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: Some(228),
            end: Some(238)
        }
    );
    assert_eq!(
        string_to_day_range("August").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::Midpoint,
            start: Some(228),
            end: None
        }
    );
    assert_eq!(
        string_to_day_range("mid September").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::Midpoint,
            start: Some(259),
            end: None
        }
    );

    assert_eq!(
        string_to_day_range("early to late August").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: Some(218),
            end: Some(238)
        }
    );

    assert_eq!(
        string_to_day_range("late August to mid September").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: Some(238),
            end: Some(259)
        }
    );

    assert_eq!(
        string_to_day_range("Sep 20-30").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: Some(264),
            end: Some(274)
        }
    );
    assert_eq!(
        string_to_day_range("Sep 25-Oct 5").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: Some(269),
            end: Some(279)
        }
    );
    assert_eq!(
        string_to_day_range("June 21 to July 10").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: Some(173),
            end: Some(192)
        }
    );
    assert_eq!(
        string_to_day_range("Oct-Nov").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: Some(289),
            end: Some(320)
        }
    );
    assert_eq!(
        string_to_day_range("late Oct-Nov").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::TwoDates,
            start: Some(299),
            end: Some(320)
        }
    );
    assert_eq!(
        string_to_day_range("July 6").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::StartOnly,
            start: Some(188),
            end: None
        }
    );
    assert_eq!(
        string_to_day_range("June 29").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::StartOnly,
            start: Some(181),
            end: None
        }
    );
    assert_eq!(
        string_to_day_range("10/15").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::StartOnly,
            start: Some(289),
            end: None
        }
    );
    assert_eq!(
        string_to_day_range("average of: July 6, June 29").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::StartOnly,
            start: Some(184),
            end: None
        }
    );
    assert_eq!(
        string_to_day_range("First Harvest: Around April 30 (Gainesville, FL)").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::StartOnly,
            start: Some(121),
            end: None
        }
    );
    assert_eq!(
        string_to_day_range("50% Harvest: Around April 25 (Gainesville, FL)").unwrap(),
        DayRangeOutput {
            parse_type: DateParseType::Midpoint,
            start: Some(116),
            end: None
        }
    );
}

#[test]
fn test_parse_relative_harvest() {
    assert_eq!(parse_relative_harvest("hi"), None);
    assert_eq!(
        parse_relative_harvest("Redhaven -32"),
        Some(HarvestRelativeParsed {
            name: "Redhaven".to_string(),
            relative_days: -32
        })
    );
    assert_eq!(
        parse_relative_harvest("Redhaven +0"),
        Some(HarvestRelativeParsed {
            name: "Redhaven".to_string(),
            relative_days: 0
        })
    );
    assert_eq!(
        parse_relative_harvest("Redhaven +45"),
        Some(HarvestRelativeParsed {
            name: "Redhaven".to_string(),
            relative_days: 45
        })
    );
    assert_eq!(
        parse_relative_harvest("Redhaven -2 weeks"),
        Some(HarvestRelativeParsed {
            name: "Redhaven".to_string(),
            relative_days: -14
        })
    );
    assert_eq!(
        parse_relative_harvest("Redhaven +0.5 weeks"),
        Some(HarvestRelativeParsed {
            name: "Redhaven".to_string(),
            relative_days: 4
        })
    );
    assert_eq!(
        parse_relative_harvest("Redhaven -1.5 weeks"),
        Some(HarvestRelativeParsed {
            name: "Redhaven".to_string(),
            relative_days: -11
        })
    );
    assert_eq!(
        parse_relative_harvest("Redhaven +9 weeks"),
        Some(HarvestRelativeParsed {
            name: "Redhaven".to_string(),
            relative_days: 63
        })
    );
    assert_eq!(
        parse_relative_harvest(" with spaces-42"),
        Some(HarvestRelativeParsed {
            name: "with spaces".to_string(),
            relative_days: -42
        })
    );
    assert_eq!(
        parse_relative_harvest("Bing -10 to -12"),
        Some(HarvestRelativeParsed {
            name: "Bing".to_string(),
            relative_days: -11
        })
    );
    assert_eq!(
        parse_relative_harvest("Bing +14 to +15"),
        Some(HarvestRelativeParsed {
            name: "Bing".to_string(),
            relative_days: 14
        })
    );
    assert_eq!(
        parse_relative_harvest("Concord -6 weeks"),
        Some(HarvestRelativeParsed {
            name: "Concord".to_string(),
            relative_days: -42
        })
    );
    assert_eq!(
        parse_relative_harvest("Concord -4 to -5 weeks"),
        Some(HarvestRelativeParsed {
            name: "Concord".to_string(),
            relative_days: -31
        })
    );
    assert_eq!(
        parse_relative_harvest("Delicious -5 to -4 weeks"),
        Some(HarvestRelativeParsed {
            name: "Delicious".to_string(),
            relative_days: -31
        })
    );
}

#[test]
fn test_patent_parsing() {
    assert_eq!(
        string_to_patent_info(""),
        PatentInfo {
            uspp_number: None,
            uspp_expiration: None
        }
    );

    assert_eq!(
        string_to_patent_info("https://patents.google.com/patent/USPP9881 expired 2014"),
        PatentInfo {
            uspp_number: Some("9881".to_string()),
            uspp_expiration: Some(NaiveDate::from_ymd(2014, 1, 1).and_hms(12, 0, 0))
        }
    );

    assert_eq!(
        string_to_patent_info("https://patents.google.com/patent/USPP30925 expires 2038-03-07"),
        PatentInfo {
            uspp_number: Some("30925".to_string()),
            uspp_expiration: Some(NaiveDate::from_ymd(2038, 3, 7).and_hms(12, 0, 0))
        }
    );
}

#[test]
#[ignore] // long runtime
fn test_database_loading() {
    let db_conn = super::establish_connection();
    super::reset_database(&db_conn);

    let items_loaded = super::load_all(&db_conn);

    println!("loaded: {:#?}", items_loaded);

    assert_ge!(items_loaded.base_plants_found, 234);
    assert_ge!(items_loaded.base_types_found, 47);
    assert_ge!(items_loaded.reference_items.reference_locations_found, 47);
    assert_ge!(items_loaded.reference_items.reference_base_plants_added, 2372);
    assert_ge!(items_loaded.reference_items.reference_plants_added, 4644);
}

#[test]
fn test_format_name_fts_string() {
    assert_eq!(format_name_fts_string(r#"tulare! (tm)"#), "tulare");
    assert_eq!(format_name_fts_string(r#"Santina ®"#), "santina");
}

#[test]
fn test_simplify_path() {
    assert_eq!(simplify_path(r#".\..\plant_database\references"#), "");
    assert_eq!(
        simplify_path("./../plant_database/references/Oregon"),
        "Oregon"
    );
    assert_eq!(
        simplify_path("./../plant_database/references/Oregon/Willamette Valley"),
        "Oregon/Willamette Valley"
    );
}
