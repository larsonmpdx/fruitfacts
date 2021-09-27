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
use std::collections::HashMap;
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
    relative_harvest: Option<String>,
    harvest_time: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct TypeJson {
    name: String,
    latin_name: Option<String>,
}

fn rem_last_n(value: &str, n: isize) -> &str {
    let mut chars = value.chars();
    for _ in 0..n {
        chars.next_back();
    }
    chars.as_str()
}

fn rem_first(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}

#[derive(Debug, PartialEq, Eq)]
enum DateParseType {
    Undefined,
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
    start: u32,
    end: u32,
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

// "early to late August" -> "early August" and "late August"
// "late August to mid September" -> "late August" and "mid September"
// "Sep 20-30" -> "Sep 20" and "Sep 30"
// "Sep 25-Oct 5" -> "Sep 25" and "Oct 5"
// if none of these match, maybe it's a single date, pass it through to string_to_day_number() unchanged

// report the way it was parsed:
// * as a start date (like peaches, "September 15", "early September")
// * midpoint ("September" or "mid September") where we'd like the graphed date range to also be centered
// * two dates ("September 15-30")
// single dates get a window put after them (window size configured outside this import), midpoints get a window centered on them, two dates stay as they are

// also parse "early/mid/late" and "0%,50%,100%" relative ripening times (return a percentage)

fn string_to_day_range(input: &str) -> Option<DayRangeOutput> {
    let mut output = DayRangeOutput::default();

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
        if string_to_day_number(input) != 0 {
            output.parse_type = DateParseType::StartOnly;
            output.start = string_to_day_number(input);
            return Some(output);
        }

        // if one part parses and not the other, then add the month from the parsing one to the not-parsing one
        if (string_to_day_number(split[0]) == 0) && (string_to_day_number(split[1]) == 0) {
            return None;
        }

        if (string_to_day_number(split[0]) != 0) && (string_to_day_number(split[1]) == 0) {
            match month_location(split[0]) {
                MonthLocationType::MonthAtBeginning => {
                    output.parse_type = DateParseType::TwoDates;
                    output.start = string_to_day_number(split[0]);
                    output.end =
                        string_to_day_number(&format!("{} {}", get_month(split[0]), split[1]));
                    return Some(output);
                }
                MonthLocationType::MonthAtEnd => {
                    output.parse_type = DateParseType::TwoDates;
                    output.start = string_to_day_number(split[0]);
                    output.end =
                        string_to_day_number(&format!("{} {}", split[1], get_month(split[0])));
                    return Some(output);
                }
                MonthLocationType::NoMonth => panic!("no month found in string {}", split[0]),
            }
        }

        if (string_to_day_number(split[0]) == 0) && (string_to_day_number(split[1]) != 0) {
            match month_location(split[1]) {
                MonthLocationType::MonthAtBeginning => {
                    output.parse_type = DateParseType::TwoDates;
                    output.start =
                        string_to_day_number(&format!("{} {}", get_month(split[1]), split[0]));
                    output.end = string_to_day_number(split[1]);
                    return Some(output);
                }
                MonthLocationType::MonthAtEnd => {
                    output.parse_type = DateParseType::TwoDates;
                    output.start =
                        string_to_day_number(&format!("{} {}", split[0], get_month(split[1])));
                    output.end = string_to_day_number(split[1]);
                    return Some(output);
                }
                MonthLocationType::NoMonth => panic!("no month found in string {}", split[0]),
            }
        }

        // finally see if the two halves both parse ok as-is, if so return that
        // for Oct 5 - Oct 30 or mid September - mid October
        if ((string_to_day_number(split[0])) != 0) && ((string_to_day_number(split[1])) != 0) {
            output.parse_type = DateParseType::TwoDates;
            output.start = string_to_day_number(split[0]);
            output.end = string_to_day_number(split[1]);
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
    output.start = string_to_day_number(input);
    return Some(output);
}

// parse a single date to a day of the year
// "September"
// "late September"
// "early-mid October"
// "Sep 25"
fn string_to_day_number(input: &str) -> u32 {
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
            return parsed.ordinal();
        }
        Err(e) => {
            eprintln!(
                "date parsing: {} with input {}",
                e,
                "2020 ".to_owned() + &month_and_day_string + " 12:01:01"
            );
            return 0;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PatentInfo {
    uspp_number: u32,
    uspp_expiration: Date<Utc>,
}

fn string_to_patent_info(input: &str) -> PatentInfo {
    let mut output = PatentInfo {
        uspp_number: 0,
        uspp_expiration: Utc.ymd(1970, 01, 01),
    };

    let uspp_regex = Regex::new(r#"USPP([0-9]+)"#).unwrap();
    let google_format_date_regex =
        Regex::new(r#"(?:expires|expired) ([0-9]{4})-([0-9]{2})-([0-9]{2})"#).unwrap();
    let plain_year_date_regex = Regex::new(r#"(?:expires|expired) ([0-9]{4})"#).unwrap();

    if let Some(matches) = uspp_regex.captures(&input) {
        if matches.len() >= 2 {
            output.uspp_number = matches[1].parse::<u32>().unwrap()
        }
    }

    // date can be either "2017-01-02" or "2017" and we presume Jan 1.  year-only dates should be used for past dates only
    match google_format_date_regex.captures(&input) {
        Some(matches) => {
            if matches.len() >= 4 {
                output.uspp_expiration = Utc.ymd(
                    matches[1].parse::<i32>().unwrap(),
                    matches[2].parse::<u32>().unwrap(),
                    matches[3].parse::<u32>().unwrap(),
                );
            }
        }
        None => {
            if let Some(matches) = plain_year_date_regex.captures(&input) {
                if matches.len() >= 2 {
                    output.uspp_expiration = Utc.ymd(matches[1].parse::<i32>().unwrap(), 01, 01);
                }
            }
        }
    }

    return output;
}

pub struct LoadAllReturn {
    pub plants_found: isize,
    pub types_found: isize,
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

    let plants_found = load_base_plants(db_conn, database_dir.clone());
    let types_found = load_types(db_conn, database_dir.clone());
    let load_references_return = load_references(db_conn, database_dir);

    calculate_ripening_times(db_conn);
    check_database(db_conn);

    return LoadAllReturn {
        plants_found: plants_found,
        types_found: types_found,
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
        '/' => return rem_first(after_references),
        '\\' => return rem_first(after_references),
        _ => return after_references,
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

                    println!("inserting");
                    let rows_inserted = diesel::insert_into(base_plants::dsl::base_plants)
                        .values((
                            base_plants::name.eq(&plant.name),
                            base_plants::type_.eq(&plant_type.unwrap()),
                            base_plants::description.eq(&plant.description),
                            base_plants::patent.eq(&plant.patent),
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

    // todo: load types, figure out an error value if types don't work out
    let types_path = database_dir.join("types.json");
    if !fs::metadata(types_path.clone()).unwrap().is_file() {
        panic!("didn't find types.json");
    }

    let contents = fs::read_to_string(types_path.clone()).unwrap();

    let types_parsed: Vec<TypeJson> = serde_json::from_str(&contents).unwrap();

    for type_element in &types_parsed {
        // todo - create table schema, do insert

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

fn add_reference_plant(
    location_name: &str,
    harvest_time: &Option<String>,
    plant_name: &String,
    plant: &CollectionPlantJson,
    category_description: &Option<String>,
    db_conn: &SqliteConnection,
) -> isize {
    // todo:
    //     description: Option<String>,

    // todo: add ripening time
    //     relative_harvest: Option<String>,
    //     harvest_time: Option<String>,

    return 1;
}

// if we're given a location like "I" and we have "short_name" matching "I" in our top-level locations list,
// return "name" from the top level
fn get_location_name(location_name: &str, locations: &Vec<CollectionLocationJson>) -> String {
    for location_top_level in locations {
        if let Some(short_name) = &location_top_level.short_name {
            if short_name == location_name {
                return location_top_level.name.clone();
            }
        }
    }

    // no translation (this will be most of the time)
    return location_name.to_string();
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
        // need to add this

        let rows_inserted = diesel::insert_into(base_plants::dsl::base_plants)
            .values((
                base_plants::name.eq(&plant_name),
                base_plants::type_.eq(&plant.type_),
            ))
            .execute(db_conn);
        assert_eq!(Ok(1), rows_inserted);
        return 1;
    } else {
        return 0;
    }
}

fn add_reference_plant_by_location(
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

                plants_added += add_reference_plant(
                    &get_location_name(&location.to_string(), &collection_locations),
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
                    plants_added += add_reference_plant(
                        &get_location_name(&location_name, &collection_locations),
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
        // todo
        // if we have a single location, add it to that location
        // if we have multiple locations but we weren't given a location here, add it without a location
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
    let mut reference_locations_found = 0;
    let mut reference_base_plants_added = 0;
    let mut reference_plants_added = 0;
    // todo

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

                for location in &collection.locations {
                    println!("inserting");
                    let rows_inserted = diesel::insert_into(collections::dsl::collections)
                        .values((
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
                            // todo - multi-plant lists are used for extension guides that give, for example,
                            // a list of "all of the scab resistant apples" but don't tie that to one location
                            // or give descriptions for each apple
                            // we want to preserve the list so it can be displayed off in a corner or whatever
                            reference_base_plants_added +=
                                maybe_add_base_plant(&plant_name, &plant, db_conn);

                            reference_plants_added += add_reference_plant_by_location(
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

                        reference_plants_added += add_reference_plant_by_location(
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
    // todo: look for all plants with only a relative ripening time and try to fill in their absolute times
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
}
