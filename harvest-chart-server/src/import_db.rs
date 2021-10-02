#[cfg(test)]
mod test;

use super::schema_generated::base_plants;
use super::schema_generated::collection_items;
use super::schema_generated::collections;
use super::schema_generated::plant_types;
use super::schema_types::*;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::fs;
use walkdir::WalkDir;

extern crate regex;
use regex::Regex;

use serde::{Deserialize, Serialize};
//use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct BasePlantJson {
    name: String,
    #[serde(alias = "type")]
    type_: Option<String>, // optional because we can get type from the filename
    description: Option<String>,
    #[serde(alias = "AKA")]
    aka: Option<Vec<String>>,
    patent: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct CollectionJson {
    title: String,
    author: Option<String>,
    description: Option<String>,
    url: Option<String>,
    published: Option<String>,
    reviewed: Option<String>,
    accessed: Option<String>,

    locations: Vec<CollectionLocationJson>,
    categories: Option<Vec<CollectionCategoryJson>>,
    plants: Vec<CollectionPlantJson>,
}

#[derive(Serialize, Deserialize)]
struct CollectionLocationJson {
    short_name: Option<String>,
    name: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize, Deserialize)]
struct CollectionCategoryJson {
    name: String,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CollectionPlantJson {
    // only for lists of names like we see in some guides for "here's a list of scab-resistant apples"
    names: Option<Vec<String>>,
    category: Option<String>,
    category_description: Option<String>,

    name: Option<String>, // optional because we can get "names" or "name"
    #[serde(alias = "type")]
    type_: String,

    locations: Option<Vec<serde_json::Value>>, // this will be either a bare string or CollectionPlantLocationsJson if it includes ripening times

    // type 1: each element is a string
    //     "locations": ["I", "II", "III", "IV"],

    // type 2: each element is an object
    //     "locations": [
    //         {"San Joaquin Valley": "Oct-Nov"},
    //         {"Sacramento Valley": "late Oct-Nov"},
    //         ...
    //     ]
    description: Option<String>,
    harvest_time_relative: Option<String>,
    harvest_time: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct TypeJson {
    name: String,
    latin_name: Option<String>,
}

fn rem_last_n(value: &str, n: usize) -> &str {
    let mut chars = value.chars();
    for _ in 0..n {
        chars.next_back();
    }
    chars.as_str()
}

fn rem_first_n(value: &str, n: usize) -> &str {
    let mut chars = value.chars();
    for _ in 0..n {
        chars.next();
    }
    chars.as_str()
}

#[derive(Debug, PartialEq, Eq)]
enum DateParseType {
    Undefined,
    Unparsed,
    StartOnly,
    Midpoint,
    TwoDates,
}

impl Default for DateParseType {
    fn default() -> Self {
        DateParseType::Undefined
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
struct DayRangeOutput {
    parse_type: DateParseType,
    start: Option<u32>,
    end: Option<u32>,
}

#[derive(Debug, PartialEq, Eq)]
enum MonthLocationType {
    NoMonth,
    MonthAtBeginning,
    MonthAtEnd,
}

fn month_location(input: &str) -> MonthLocationType {
    let month_at_beginning_regex =
        Regex::new(r#"^(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)"#).unwrap();

    match month_at_beginning_regex.captures(&input.to_lowercase()) {
        Some(_) => return MonthLocationType::MonthAtBeginning,
        None => {
            let month_at_end_regex =
                Regex::new(r#"(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)[^0-9]*$"#).unwrap();

            match month_at_end_regex.captures(&input.to_lowercase()) {
                Some(_) => return MonthLocationType::MonthAtEnd,
                None => return MonthLocationType::NoMonth,
            }
        }
    }
}

fn get_month(input: &str) -> String {
    let month_names_regex =
        Regex::new(r#"(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)"#).unwrap();

    match month_names_regex.captures(&input.to_lowercase()) {
        Some(matches) => return matches[1].to_owned(),
        None => panic!("no month found in string {}", input),
    }
}

// should this date string be treated as being centered on a single month?
// we accept either "mid september" or "september" for this
// we're looking to distinguish this from regular start dates which, when charted,
// would have a window *after* the date instead of *centered on* the date
fn is_a_midpoint(input: &str) -> bool {
    let month: String;
    let no_whitespace: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    if no_whitespace.to_lowercase().starts_with("mid") {
        month = no_whitespace.chars().skip(3).collect();
    } else {
        month = input.to_string();
    }

    // month name abbreviations with any chars other than numbers after
    let month_names_regex =
        Regex::new(r#"^(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)[^0-9]*$"#).unwrap();

    match month_names_regex.captures(&month.to_lowercase()) {
        Some(_) => return true,
        None => return false,
    }
}

fn average(numbers: &Vec<u32>) -> u32 {
    numbers.iter().sum::<u32>() / numbers.len() as u32
}

// "early to late August" -> "early August" and "late August"
// "late August to mid September" -> "late August" and "mid September"
// "Sep 20-30" -> "Sep 20" and "Sep 30"
// "Sep 25-Oct 5" -> "Sep 25" and "Oct 5"
// if none of these match, maybe it's a single date, pass it through to string_to_day_number() unchanged

// special case: "average of: July 6, June 29"
// this is for extension pubs that give a set of dates based on measurements in multiple years
// break it apart on the commas then parse each and average the start dates, and return a start only date

// report the way it was parsed:
// * as a start date (like peaches, "September 15", "early September")
// * midpoint ("September" or "mid September") where we'd like the graphed date range to also be centered
// * two dates ("September 15-30")
// single dates get a window put after them (window size configured outside this import), midpoints get a window centered on them, two dates stay as they are

// also parse "early/mid/late" and "0%,50%,100%" relative ripening times (return a percentage)

fn string_to_day_range(input: &str) -> Option<DayRangeOutput> {
    let mut output = DayRangeOutput::default();

    // escape hatch for "time within season" strings which we aren't parsing for now
    let time_within_season_regex =
        Regex::new(r#"^(early-mid|mid-late|early|mid|late|early season|mid season|late season)$"#)
            .unwrap();

    if let Some(_) = time_within_season_regex.captures(&input.to_lowercase()) {
        output.parse_type = DateParseType::Unparsed;
        return Some(output);
    }

    // escape hatch for some kinda indefinite time ranges in some extension pubs
    let indefinite_times_regex = Regex::new(r#"^(summer)$"#).unwrap();

    if let Some(_) = indefinite_times_regex.captures(&input.to_lowercase()) {
        output.parse_type = DateParseType::Unparsed;
        return Some(output);
    }

    // special case for a list of days
    let average_of_start = "average of: ";
    if input.starts_with("average of: ") {
        let list = rem_first_n(input, average_of_start.len());
        let split = list.split(",").collect::<Vec<&str>>();

        let mut parsed_days = Vec::new();
        for item in split {
            if let Some(parsed_day) = string_to_day_number(item) {
                parsed_days.push(parsed_day);
            } else {
                panic!(r#"error parsing average date from {}"#, input);
            }
        }

        output.parse_type = DateParseType::StartOnly;
        output.start = Some(average(&parsed_days));
        return Some(output);
    }

    // does it have "to" or "-" in it? if so, split on that and see if the right side is only a number
    // if it is, it's something like sep 20-30, and we need to pull sep from the left side and give it to the right side
    // if not, feed both sides

    // could be: September 15-30, or mid-late September, or Oct 5 - Oct 30
    if input.contains("-") || input.contains(" to ") {
        let split;

        if input.contains("-") {
            split = input.split("-").collect::<Vec<&str>>();
            assert_eq!(split.len(), 2, "date string had more than one '-'");
        } else if input.contains(" to ") {
            split = input.split(" to ").collect::<Vec<&str>>();
            assert_eq!(split.len(), 2, "date string had more than one ' to '");
        } else {
            panic!("shouldn't get here, '-' or ' to ' match")
        }

        // first see if the whole thing parses ok, if so return that
        // for mid-late September
        if string_to_day_number(input).is_some() {
            output.parse_type = DateParseType::StartOnly;
            output.start = Some(string_to_day_number(input).unwrap());
            return Some(output);
        }

        // if one part parses and not the other, then add the month from the parsing one to the not-parsing one
        if string_to_day_number(split[0]).is_none() && string_to_day_number(split[1]).is_none() {
            return None;
        }

        if string_to_day_number(split[0]).is_some() && string_to_day_number(split[1]).is_none() {
            match month_location(split[0]) {
                MonthLocationType::MonthAtBeginning => {
                    output.parse_type = DateParseType::TwoDates;
                    output.start = Some(string_to_day_number(split[0]).unwrap());
                    output.end = Some(
                        string_to_day_number(&format!("{} {}", get_month(split[0]), split[1]))
                            .unwrap(),
                    );
                    return Some(output);
                }
                MonthLocationType::MonthAtEnd => {
                    output.parse_type = DateParseType::TwoDates;
                    output.start = Some(string_to_day_number(split[0]).unwrap());
                    output.end = Some(
                        string_to_day_number(&format!("{} {}", split[1], get_month(split[0])))
                            .unwrap(),
                    );
                    return Some(output);
                }
                MonthLocationType::NoMonth => panic!("no month found in string {}", split[0]),
            }
        }

        if string_to_day_number(split[0]).is_none() && string_to_day_number(split[1]).is_some() {
            match month_location(split[1]) {
                MonthLocationType::MonthAtBeginning => {
                    output.parse_type = DateParseType::TwoDates;
                    output.start =
                        string_to_day_number(&format!("{} {}", get_month(split[1]), split[0]));
                    if output.start.is_none() {
                        panic!(r#"couldn't parse date: {:}"#, input);
                    }
                    output.end = Some(string_to_day_number(split[1]).unwrap());
                    return Some(output);
                }
                MonthLocationType::MonthAtEnd => {
                    output.parse_type = DateParseType::TwoDates;
                    output.start =
                        string_to_day_number(&format!("{} {}", split[0], get_month(split[1])));
                    if output.start.is_none() {
                        panic!(r#"couldn't parse date: {:}"#, input);
                    }
                    output.end = Some(string_to_day_number(split[1]).unwrap());
                    return Some(output);
                }
                MonthLocationType::NoMonth => panic!("no month found in string {}", split[0]),
            }
        }

        // finally see if the two halves both parse ok as-is, if so return that
        // for Oct 5 - Oct 30 or mid September - mid October
        if (string_to_day_number(split[0])).is_some() && (string_to_day_number(split[1])).is_some()
        {
            output.parse_type = DateParseType::TwoDates;
            output.start = Some(string_to_day_number(split[0]).unwrap());
            output.end = Some(string_to_day_number(split[1]).unwrap());
            return Some(output);
        }

        return None;
    }

    // if no "to" or "-" then it's a single date
    // detect bare months or mid month to set the midpoint enum, otherwise it's a start only

    if is_a_midpoint(input) {
        output.parse_type = DateParseType::Midpoint;
    } else {
        output.parse_type = DateParseType::StartOnly;
    }
    match string_to_day_number(input) {
        Some(start) => {
            output.start = Some(start);
            return Some(output);
        }
        None => return None,
    }
}

// parse a single date to a day of the year
// "September"
// "late September"
// "early-mid October"
// "Sep 25"
fn string_to_day_number(input: &str) -> Option<u32> {
    let mut month_and_day_string = input.to_string();

    if !(input).contains(char::is_whitespace) {
        // for bare month names
        month_and_day_string = format!("{} 15", input);
    } else {
        // for inputs like "early March"
        let text_and_month_regex =
            Regex::new(r#"(early to mid|mid to late|early-mid|mid-late|early|mid|late) (.*)"#)
                .unwrap();

        if let Some(matches) = text_and_month_regex.captures(&input.to_lowercase()) {
            let day_of_month;
            if matches.len() >= 3 {
                match &matches[1] {
                    "early" => day_of_month = 5,
                    "early to mid" | "early-mid" => day_of_month = 10,
                    "mid" => day_of_month = 15,
                    "mid to late" | "mid-late" => day_of_month = 20,
                    "late" => day_of_month = 25,
                    _ => panic!("matched a date prefix not in this match statement"),
                }

                month_and_day_string = format!("{} {}", &matches[2], day_of_month.to_string());
            }
        }
        // default: parse the input as it came, for dates that already look like like "September 25"
    }

    // wrap this with a year and time of day so we can parse it, then get the day of the year back out
    // 2020 was a leap year
    match NaiveDateTime::parse_from_str(
        &("2020 ".to_owned() + &month_and_day_string + " 12:01:01"),
        "%Y %B %d %H:%M:%S",
    ) {
        Ok(parsed) => {
            return Some(parsed.ordinal());
        }
        Err(_) => {
            //    eprintln!(
            //        "date parsing: {} with input {}",
            //        e,
            //        "2020 ".to_owned() + &month_and_day_string + " 12:01:01"
            //    );
            return None;
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct PatentInfo {
    uspp_number: Option<i32>,
    uspp_expiration: Option<Date<Utc>>,
}

fn string_to_patent_info(input: &str) -> PatentInfo {
    let mut output = PatentInfo {
        uspp_number: None,
        uspp_expiration: None,
    };

    let uspp_regex = Regex::new(r#"USPP([0-9]+)"#).unwrap();
    let google_format_date_regex =
        Regex::new(r#"(?:expires|expired) ([0-9]{4})-([0-9]{2})-([0-9]{2})"#).unwrap();
    let plain_year_date_regex = Regex::new(r#"(?:expires|expired) ([0-9]{4})"#).unwrap();

    if let Some(matches) = uspp_regex.captures(&input) {
        if matches.len() >= 2 {
            output.uspp_number = Some(matches[1].parse::<i32>().unwrap());
        }
    }

    // date can be either "2017-01-02" or "2017" and we presume Jan 1.  year-only dates should be used for past dates only
    match google_format_date_regex.captures(&input) {
        Some(matches) => {
            if matches.len() >= 4 {
                output.uspp_expiration = Some(Utc.ymd(
                    matches[1].parse::<i32>().unwrap(),
                    matches[2].parse::<u32>().unwrap(),
                    matches[3].parse::<u32>().unwrap(),
                ));
            }
        }
        None => {
            if let Some(matches) = plain_year_date_regex.captures(&input) {
                if matches.len() >= 2 {
                    output.uspp_expiration =
                        Some(Utc.ymd(matches[1].parse::<i32>().unwrap(), 1, 1));
                }
            }
        }
    }

    return output;
}

pub struct LoadAllReturn {
    pub base_plants_found: isize,
    pub base_types_found: isize,
    pub reference_items: LoadReferencesReturn,
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn reset_database(db_conn: &SqliteConnection) {
    let _ = diesel::delete(base_plants::dsl::base_plants).execute(db_conn);
    let _ = diesel::delete(plant_types::dsl::plant_types).execute(db_conn);
    let _ = diesel::delete(collections::dsl::collections).execute(db_conn);
    let _ = diesel::delete(collection_items::dsl::collection_items).execute(db_conn);
    super::embedded_migrations::run(db_conn).unwrap();
}

pub fn load_all(db_conn: &SqliteConnection) -> LoadAllReturn {
    let database_dir = get_database_dir().unwrap();

    let base_plants_found = load_base_plants(db_conn, database_dir.clone());
    let base_types_found = load_types(db_conn, database_dir.clone());
    let load_references_return = load_references(db_conn, database_dir);

    calculate_ripening_times(db_conn);
    check_database(db_conn);

    return LoadAllReturn {
        base_plants_found: base_plants_found,
        base_types_found: base_types_found,
        reference_items: load_references_return,
    };
}

fn get_database_dir() -> Option<std::path::PathBuf> {
    let max_up_traversal_levels = 3;
    let mut i = 0;

    while i <= max_up_traversal_levels {
        let mut path = std::path::PathBuf::from(".");
        for _ in 0..i {
            path = path.join("..");
        }
        path = path.join("plant_database");
        println!("dir: {}", path.display());
        match fs::metadata(path.clone()) {
            Ok(md) => {
                if md.is_dir() {
                    return Some(path);
                }
            }
            Err(_) => {
                println!("not a dir")
            }
        }
        i += 1;
    }

    return None;
}

fn simplify_path(input: &str) -> &str {
    let v: Vec<&str> = input.split("references").collect();

    let after_references = v.last().unwrap();
    println!("split result: {}", after_references);

    if after_references.len() == 0 {
        return after_references;
    }

    match after_references.chars().next().unwrap() {
        '/' => return rem_first_n(after_references, 1),
        '\\' => return rem_first_n(after_references, 1),
        _ => return after_references,
    }
}

pub struct AkaFormatted {
    pub aka: Option<String>,
    pub aka_fts: Option<String>,
}

lazy_static! {
    static ref SPECIAL_CHARACTERS_REGEX: Regex = Regex::new(r#"["’'.!#,\-— ]"#).unwrap();
}

fn format_name_fts_string(name: &String) -> String {
    return SPECIAL_CHARACTERS_REGEX.replace_all(&name, "").to_string();
}

// turn an array like ["20th Century", "Twentieth Century"] into "aka" and "aka_fts" strings
// aka:      comma-separated list (remove commas in names)
// aka_fts:  same, but without characters like '-' and ' ' to make full text search work better
fn format_aka_strings(aka_array: &Option<Vec<String>>) -> AkaFormatted {
    if let Some(aka_array) = aka_array {
        let mut aka_string_builder = "".to_string();
        let mut aka_fts_string_builder = "".to_string();

        let mut first_element = true;
        for aka_element in aka_array {
            let commas_regex = Regex::new(r",").unwrap();
            let without_commas = commas_regex.replace_all(aka_element, "");
            let without_special_characters = SPECIAL_CHARACTERS_REGEX.replace_all(aka_element, "");

            if first_element {
                first_element = false;
            } else {
                aka_string_builder += ",";
                aka_fts_string_builder += ",";
            }

            aka_string_builder += &without_commas;
            aka_fts_string_builder += &without_special_characters;
        }
        return AkaFormatted {
            aka: Some(aka_string_builder),
            aka_fts: Some(aka_fts_string_builder),
        };
    } else {
        return AkaFormatted {
            aka: None,
            aka_fts: None,
        };
    }
}

pub fn load_base_plants(db_conn: &SqliteConnection, database_dir: std::path::PathBuf) -> isize {
    // look for a dir "plant_database/" up to three levels up so users can mess this up a little

    let mut plants_found = 0;

    let file_paths = fs::read_dir(database_dir.clone().join("plants")).unwrap();

    for file_path in file_paths {
        let path_ = file_path.unwrap().path();

        if fs::metadata(path_.clone()).unwrap().is_file() {
            if path_.extension().unwrap().to_str().unwrap() == "json" {
                println!("found: {}", path_.display());

                let contents = fs::read_to_string(path_.clone()).unwrap();

                let plants: Vec<BasePlantJson> = serde_json::from_str(&contents).unwrap();

                let filename =
                    rem_last_n(path_.as_path().file_name().unwrap().to_str().unwrap(), 5); // 5: ".json"

                for plant in &plants {
                    // for the "Oddball.json" file, get type from each item's json
                    // all others get type from the filename
                    let plant_type;
                    if filename.starts_with("Oddball") {
                        plant_type = plant.type_.clone();
                    } else {
                        plant_type = Some(filename.to_string());
                    }

                    let aka_formatted = format_aka_strings(&plant.aka);

                    let mut patent_info = Default::default();
                    if let Some(patent_string) = &plant.patent {
                        patent_info = string_to_patent_info(&patent_string);
                    }

                    let mut uspp_expiration_string = None;
                    if let Some(uspp_expiration) = patent_info.uspp_expiration {
                        uspp_expiration_string = Some(uspp_expiration.to_string());
                    }

                    println!("inserting");
                    let rows_inserted = diesel::insert_into(base_plants::dsl::base_plants)
                        .values((
                            base_plants::name.eq(&plant.name),
                            base_plants::name_fts.eq(format_name_fts_string(&plant.name)),
                            base_plants::type_.eq(&plant_type.unwrap()),
                            base_plants::aka.eq(&aka_formatted.aka),
                            base_plants::aka_fts.eq(&aka_formatted.aka_fts),
                            base_plants::description.eq(&plant.description),
                            base_plants::uspp_number.eq(patent_info.uspp_number),
                            base_plants::uspp_expiration.eq(uspp_expiration_string),
                        ))
                        .execute(db_conn);
                    assert_eq!(Ok(1), rows_inserted);
                    plants_found += 1;
                }
            }
        }
    }

    return plants_found;
}

fn load_types(db_conn: &SqliteConnection, database_dir: std::path::PathBuf) -> isize {
    let mut types_found = 0;

    let types_path = database_dir.join("types.json");
    if !fs::metadata(types_path.clone()).unwrap().is_file() {
        panic!("didn't find types.json");
    }

    let contents = fs::read_to_string(types_path.clone()).unwrap();

    let types_parsed: Vec<TypeJson> = serde_json::from_str(&contents).unwrap();

    for type_element in &types_parsed {
        let rows_inserted = diesel::insert_into(plant_types::dsl::plant_types)
            .values((
                plant_types::name.eq(&type_element.name),
                plant_types::latin_name.eq(&type_element.latin_name),
            ))
            .execute(db_conn);
        assert_eq!(Ok(1), rows_inserted);
        types_found += 1;
    }
    return types_found;
}

fn get_category_description(
    category: &Option<String>,
    category_description: &Option<String>,
    categories: &Option<Vec<CollectionCategoryJson>>,
) -> Option<String> {
    // if category_description is empty, see if we can get it from the top-level list of categories
    // this is supported to save typing out the same category description a bunch of times
    if category.is_some() && category_description.is_none() && categories.is_some() {
        for category_top_level in categories.as_ref().unwrap() {
            if &category_top_level.name == category.as_ref().unwrap() {
                return category_top_level.description.clone();
            }
        }
    }

    return category_description.clone();
}

fn add_collection_plant(
    collection_id: i32,
    location_name: &Option<String>,
    harvest_time: &Option<String>,
    plant_name: &String,
    plant: &CollectionPlantJson,
    category_description: &Option<String>,
    db_conn: &SqliteConnection,
) -> isize {
    let mut harvest_start = None;
    let mut harvest_end = None;
    let mut harvest_start_is_midpoint = None;
    let mut harvest_start_2 = None; // fig breba+main
    let mut harvest_end_2 = None; // fig breba+main
    let mut harvest_2_start_is_midpoint = None; // fig breba+main
    if let Some(harvest_time) = harvest_time {
        if harvest_time.is_empty() {
            panic!(
                r#"harvest time was an empty string for {:?}: {}"#,
                plant, harvest_time
            );
        }

        // for harvest times like "Jun/Sep" which are for fig breba+main crops
        if harvest_time.contains("/") {
            let split = harvest_time.split("/").collect::<Vec<&str>>();
            assert_eq!(
                split.len(),
                2,
                r#"date string had multiple '/' {:?}: {}"#,
                plant,
                harvest_time
            );

            match string_to_day_range(split[0]) {
                Some(day_range) => {
                    if let Some(start) = day_range.start {
                        harvest_start = Some(i32::try_from(start).unwrap());
                    }
                    if let Some(end) = day_range.end {
                        harvest_end = Some(i32::try_from(end).unwrap());
                    }
                    if day_range.parse_type == DateParseType::Midpoint {
                        harvest_start_is_midpoint = Some(1);
                    }
                }
                None => {
                    panic!(r#"couldn't parse date for {:?}: {}"#, plant, harvest_time);
                }
            }
            match string_to_day_range(split[1]) {
                Some(day_range) => {
                    if let Some(start) = day_range.start {
                        harvest_start_2 = Some(i32::try_from(start).unwrap());
                    }
                    if let Some(end) = day_range.end {
                        harvest_end_2 = Some(i32::try_from(end).unwrap());
                    }
                    if day_range.parse_type == DateParseType::Midpoint {
                        harvest_2_start_is_midpoint = Some(1);
                    }
                }
                None => {
                    panic!(r#"couldn't parse date for {:?}: {}"#, plant, harvest_time);
                }
            }
        } else {
            match string_to_day_range(harvest_time) {
                Some(day_range) => {
                    if let Some(start) = day_range.start {
                        harvest_start = Some(i32::try_from(start).unwrap());
                    }
                    if let Some(end) = day_range.end {
                        harvest_end = Some(i32::try_from(end).unwrap());
                    }
                    if day_range.parse_type == DateParseType::Midpoint {
                        harvest_start_is_midpoint = Some(1);
                    }
                }
                None => {
                    panic!(r#"couldn't parse date for {:?}: {}"#, plant, harvest_time);
                }
            }
        }
    }

    let rows_inserted = diesel::insert_into(collection_items::dsl::collection_items)
        .values((
            collection_items::collection_id.eq(collection_id),
            collection_items::location_name.eq(location_name),
            collection_items::name.eq(plant_name),
            collection_items::type_.eq(&plant.type_),
            collection_items::category.eq(&plant.category),
            collection_items::category_description.eq(category_description),
            collection_items::description.eq(&plant.description),
            collection_items::harvest_relative.eq(&plant.harvest_time_relative),
            collection_items::harvest_text.eq(harvest_time),
            collection_items::harvest_start.eq(harvest_start),
            collection_items::harvest_end.eq(harvest_end),
            collection_items::harvest_start_is_midpoint.eq(harvest_start_is_midpoint),
            collection_items::harvest_start_2.eq(harvest_start_2),
            collection_items::harvest_end_2.eq(harvest_end_2),
            collection_items::harvest_2_start_is_midpoint.eq(harvest_2_start_is_midpoint),
        ))
        .execute(db_conn);
    assert_eq!(
        Ok(1),
        rows_inserted,
        "failed inserting {} {:?}",
        plant_name,
        rows_inserted
    );

    return 1;
}

// if we're given a location like "I" and we have "short_name" matching "I" in our top-level locations list,
// return "name" from the top level
fn get_location_name(
    plant_location_name: Option<String>,
    locations: &Vec<CollectionLocationJson>,
) -> Option<String> {
    match plant_location_name {
        Some(plant_location_name) => {
            for location_top_level in locations {
                if let Some(short_name) = &location_top_level.short_name {
                    if short_name.eq(&plant_location_name) {
                        return Some(location_top_level.name.clone());
                    }
                }
            }

            // no translation (this will be most of the time)
            return Some(plant_location_name.to_string());
        }
        None => {
            // no name given: if we have a single top-level location, return that
            if locations.len() == 1 {
                return Some(locations[0].name.clone());
            } else {
                // if we have multiple top-level locations, return none
                return None;
            }
        }
    }
}

fn maybe_add_base_plant(
    plant_name: &str,
    plant: &CollectionPlantJson,
    db_conn: &SqliteConnection,
) -> isize {
    let base_plant_result = base_plants::dsl::base_plants
        .filter(base_plants::name.eq(&plant_name))
        .filter(base_plants::type_.eq(&plant.type_))
        .first::<BasePlant>(db_conn);

    if base_plant_result.is_err() {
        // a plant in a reference that isn't already in base_plants - need to add

        let rows_inserted = diesel::insert_into(base_plants::dsl::base_plants)
            .values((
                base_plants::name.eq(&plant_name),
                base_plants::name_fts.eq(format_name_fts_string(&plant_name.to_string())),
                base_plants::type_.eq(&plant.type_),
            ))
            .execute(db_conn);
        assert_eq!(Ok(1), rows_inserted);
        return 1;
    } else {
        return 0;
    }
}

fn add_collection_plant_by_location(
    collection_number: i32,
    plant_name: &String,
    plant: &CollectionPlantJson,
    category_description: &Option<String>,
    collection_locations: &Vec<CollectionLocationJson>,
    db_conn: &SqliteConnection,
) -> isize {
    // see if plant.locations exists

    let mut plants_added: isize = 0;
    if plant.locations.is_some() {
        for location in plant.locations.clone().unwrap() {
            if location.is_string() {
                // type 1: "locations": ["I", "II", "III", "IV"] - this type has the same ripening time for all locations
                // (which is surely inaccurate, but it's what we get with some extension publications)

                // we get harvest time for each location from the base harvest time values

                println!("location: {} {}", location, location.to_string());

                plants_added += add_collection_plant(
                    collection_number,
                    &get_location_name(
                        Some(location.as_str().unwrap().to_string()),
                        &collection_locations,
                    ), // the .as_str()... nastiness is because serde wants to carry the "it's a json string" idea to the point of printing it a certain way in rust. as_str() tells it not to
                    &plant.harvest_time,
                    plant_name,
                    &plant,
                    &category_description,
                    db_conn,
                );
            } else {
                // deserialize to type II like
                //     "locations": [
                //         {"San Joaquin Valley": "Oct-Nov"},
                //         {"Sacramento Valley": "late Oct-Nov"},
                //         ...
                //     ]

                // we get harvest time from the locations array (and it's absolute only, not relative)

                let location_objects: HashMap<String, String> =
                    serde_json::from_value(location).unwrap();

                for (location_name, harvest_time) in location_objects {
                    if harvest_time == "NA" {
                        // skip this - see the UC Davis charts for example
                        continue;
                    }

                    plants_added += add_collection_plant(
                        collection_number,
                        &get_location_name(Some(location_name), &collection_locations),
                        &Some(harvest_time),
                        plant_name,
                        &plant,
                        &category_description,
                        db_conn,
                    );
                }
            }
        }

        // the plant needs to match one of our locations, either name or short_name
    } else {
        // no location given in the plant json

        plants_added += add_collection_plant(
            collection_number,
            &get_location_name(None, &collection_locations),
            &plant.harvest_time,
            plant_name,
            &plant,
            &category_description,
            db_conn,
        );
    }

    return plants_added;
}

pub struct LoadReferencesReturn {
    pub reference_locations_found: isize,
    pub reference_base_plants_added: isize,
    pub reference_plants_added: isize,
}

fn load_references(
    db_conn: &SqliteConnection,
    database_dir: std::path::PathBuf,
) -> LoadReferencesReturn {
    let mut collection_number = 0;

    let mut reference_locations_found = 0;
    let mut reference_base_plants_added = 0;
    let mut reference_plants_added = 0;

    // traverse /plant_database/references/
    // create a collections table entry for each location in this reference, or only one if there's only one location

    for entry in WalkDir::new(std::path::PathBuf::from(database_dir).join("references"))
        .max_depth(5)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path_ = entry.path();

        if fs::metadata(path_.clone()).unwrap().is_file() {
            if path_.extension().unwrap().to_str().unwrap() == "json" {
                println!("found reference: {}", path_.display());

                let contents = fs::read_to_string(path_.clone()).unwrap();

                let collection: CollectionJson = serde_json::from_str(&contents).unwrap();

                let filename = rem_last_n(path_.file_name().unwrap().to_str().unwrap(), 5); // 5: ".json"

                let path = simplify_path(path_.parent().unwrap().to_str().unwrap());

                collection_number += 1;
                for location in &collection.locations {
                    println!("inserting");
                    let rows_inserted = diesel::insert_into(collections::dsl::collections)
                        .values((
                            collections::collection_id.eq(collection_number),
                            collections::user_id.eq(0), // todo - codify this as the root/fake user
                            collections::path.eq(&path),
                            collections::filename.eq(&filename),
                            collections::title.eq(&collection.title),
                            collections::author.eq(&collection.author),
                            collections::description.eq(&collection.description),
                            collections::url.eq(&collection.url),
                            collections::published.eq(&collection.published),
                            collections::reviewed.eq(&collection.reviewed),
                            collections::accessed.eq(&collection.accessed),
                            collections::location.eq(&location.name),
                            collections::latitude.eq(&location.latitude),
                            collections::longitude.eq(&location.longitude),
                        ))
                        .execute(db_conn);
                    assert_eq!(Ok(1), rows_inserted);
                    reference_locations_found += 1;
                }

                for plant in collection.plants {
                    if plant.names.is_none() && plant.name.is_none() {
                        panic!(r#"plant missing both "name" and "names" {:?}"#, plant)
                    }
                    if plant.names.is_some() && plant.name.is_some() {
                        panic!(r#"plant has both "name" and "names" {:?}"#, plant)
                    }

                    let category_description = get_category_description(
                        &plant.category,
                        &plant.category_description,
                        &collection.categories,
                    );

                    if plant.names.is_some() {
                        for plant_name in plant.names.clone().unwrap() {
                            // multi-plant lists are used for extension guides that give, for example,
                            // a list of "all of the scab resistant apples" but don't tie that to one location
                            // or give descriptions for each apple
                            // we want to preserve the list so it can be displayed off in a corner or whatever
                            reference_base_plants_added +=
                                maybe_add_base_plant(&plant_name, &plant, db_conn);

                            reference_plants_added += add_collection_plant_by_location(
                                collection_number,
                                &plant_name,
                                &plant,
                                &category_description,
                                &collection.locations,
                                db_conn,
                            );
                        }
                    } else if plant.name.is_some() {
                        reference_base_plants_added +=
                            maybe_add_base_plant(plant.name.as_ref().unwrap(), &plant, db_conn);

                        reference_plants_added += add_collection_plant_by_location(
                            collection_number,
                            plant.name.as_ref().unwrap(),
                            &plant,
                            &category_description,
                            &collection.locations,
                            db_conn,
                        );
                    }
                }
            }
        }
    }

    // plant category existince is checked later in check_database()

    // for each plant, create an entry in the collection_items database for each location, with a foreign key to that location's collections table entry
    return LoadReferencesReturn {
        reference_locations_found: reference_locations_found,
        reference_base_plants_added: reference_base_plants_added,
        reference_plants_added: reference_plants_added,
    };
}

fn calculate_ripening_times(db_conn: &SqliteConnection) {
    // todo: 2nd procesing pass:
    // look for all plants with only a relative ripening time and try to fill in their absolute times
    // example is an extension publication listing peaches as redhaven+5 or whatever,
    // but also giving an absolute time for redhaven in the same pub
}

fn check_database(db_conn: &SqliteConnection) {
    // find all types and make sure each is in the types table
    let types_from_plants = base_plants::dsl::base_plants
        .select(base_plants::type_)
        .distinct()
        .load::<String>(db_conn);

    for type_from_plants in &types_from_plants.unwrap() {
        let _ = plant_types::dsl::plant_types
            .filter(plant_types::name.eq(type_from_plants))
            .first::<PlantType>(db_conn)
            .expect(&format!(
                "imported a plant with a category not in types.json: {}",
                type_from_plants
            ));
    }

    // todo: for all base plants, ensure none of the names match an "AKA" name which would be a duplicate
}
