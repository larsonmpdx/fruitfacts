#[cfg(test)]
mod test;

use super::schema_generated::base_plants;
use super::schema_generated::plant_types;
use super::schema_types::*;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::fs;

extern crate regex;
use regex::Regex;

use serde::{Deserialize, Serialize};
//use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct PlantJson {
    name: String,
    #[serde(alias = "type")]
    type_: Option<String>,
    description: Option<String>,
    patent: Option<String>,
    relative_harvest: Option<String>,
    harvest_start: Option<String>,
    harvest_end: Option<String>,
    harvest_time_reference: Option<String>,
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

// "Sept 25"
// "late September"
// "early-mid October"
fn string_to_day_number(input: &str) -> u32 {
    let input_regex =
        Regex::new(r#"(early to mid|mid to late|early-mid|mid-late|early|mid|late) (.*)"#).unwrap();

    let mut month_and_day_string = input.to_string();

    if !(input).contains(char::is_whitespace) {
        // assume this is a bare month name if there's no whitespace
        // pick a day that's in the middle of the month somewhere but with a one weeks span will be about centered
        // in the future I want to be able to set a span for the whole month when given a single month like this
        month_and_day_string = format!("{} 12", input);
    } else {
        match input_regex.captures(&input.to_lowercase()) {
            Some(matches) => {
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
            None => (),
        }
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
            eprintln!("date parsing: {}", e);
            return 0;
        }
    }
}

pub struct LoadAllReturn {
    pub plants_found: isize,
    pub types_found: isize,
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
    super::embedded_migrations::run(db_conn).unwrap();
}

pub fn load_all(db_conn: &SqliteConnection) -> LoadAllReturn {
    let database_dir = get_database_dir().unwrap();

    let plants_found = load_base_plants(db_conn, database_dir.clone());
    let types_found = load_types(db_conn, database_dir);

    check_database(db_conn);

    return LoadAllReturn {
        plants_found: plants_found,
        types_found: types_found,
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

                let plants: Vec<PlantJson> = serde_json::from_str(&contents).unwrap();

                let filename =
                    rem_last_n(path_.as_path().file_name().unwrap().to_str().unwrap(), 5); // 5: ".json"

                for plant in &plants {
                    // for the "Oddball.json" file, get type from each item's json
                    // all others get type from the filename
                    let plant_type;
                    if filename.starts_with("Oddball") {
                        plant_type = plant.type_.clone().unwrap();
                    } else {
                        plant_type = filename.to_string();
                    }

                    let harvest_start_day;
                    let harvest_end_day;
                    if plant.harvest_start.is_some() {
                        harvest_start_day =
                            string_to_day_number(plant.harvest_start.as_ref().unwrap());
                    } else {
                        harvest_start_day = 0;
                    }
                    if plant.harvest_end.is_some() {
                        harvest_end_day = string_to_day_number(plant.harvest_end.as_ref().unwrap());
                    } else {
                        harvest_end_day = 0;
                    }

                    println!("inserting");
                    let rows_inserted = diesel::insert_into(base_plants::dsl::base_plants)
                        .values((
                            base_plants::name.eq(&plant.name),
                            base_plants::type_.eq(&plant_type),
                            base_plants::description.eq(&plant.description),
                            base_plants::patent.eq(&plant.patent),
                            base_plants::relative_harvest.eq(&plant.relative_harvest), // todo harvest start+end
                            base_plants::harvest_start.eq(harvest_start_day as i32),
                            base_plants::harvest_end.eq(harvest_end_day as i32),
                            base_plants::harvest_time_reference.eq(&plant.harvest_time_reference),
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
