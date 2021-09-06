#[cfg(test)]
mod test;

use super::schema::base_plants::dsl::*;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::fs;

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

fn rem_last_n(value: &str, n: isize) -> &str {
    let mut chars = value.chars();
    for _ in 0..n {
        chars.next_back();
    }
    chars.as_str()
}

fn string_to_day_number(input: &str) -> u32 {
    // wrap this with a year and time of day so we can parse it, then get the day of the year back out.  2020 was a leap year
    match NaiveDateTime::parse_from_str(
        &("2020 ".to_owned() + input + " 12:01:01"),
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

pub fn load_base_plants(db_conn: &SqliteConnection) -> bool {
    // look for a dir "plant_database/" up to three levels up so users can mess this up a little
    let max_up_traversal_levels = 3;
    let mut plant_database_found = false;
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
                    plant_database_found = true;
                    let file_paths = fs::read_dir(path).unwrap();

                    for file_path in file_paths {
                        let path_ = file_path.unwrap().path();

                        if fs::metadata(path_.clone()).unwrap().is_file() {
                            if path_.extension().unwrap().to_str().unwrap() == "json" {
                                println!("found: {}", path_.display());

                                let contents = fs::read_to_string(path_.clone()).unwrap();

                                let plants: Vec<PlantJson> =
                                    serde_json::from_str(&contents).unwrap();

                                let filename = rem_last_n(
                                    path_.as_path().file_name().unwrap().to_str().unwrap(),
                                    5,
                                ); // 5: ".json"

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
                                        harvest_start_day = string_to_day_number(
                                            plant.harvest_start.as_ref().unwrap(),
                                        );
                                    } else {
                                        harvest_start_day = 0;
                                    }
                                    if plant.harvest_end.is_some() {
                                        harvest_end_day = string_to_day_number(
                                            plant.harvest_end.as_ref().unwrap(),
                                        );
                                    } else {
                                        harvest_end_day = 0;
                                    }

                                    println!("inserting");
                                    let rows_inserted = diesel::insert_into(base_plants)
                                        .values((
                                            name.eq(&plant.name),
                                            type_.eq(&plant_type),
                                            description.eq(&plant.description),
                                            patent.eq(&plant.patent),
                                            relative_harvest.eq(&plant.relative_harvest), // todo harvest start+end
                                            harvest_start.eq(harvest_start_day as i32),
                                            harvest_end.eq(harvest_end_day as i32),
                                            harvest_time_reference
                                                .eq(&plant.harvest_time_reference),
                                        ))
                                        .execute(db_conn);
                                    assert_eq!(Ok(1), rows_inserted);
                                }
                            }
                        }
                    }
                    break;
                }
            }
            Err(_) => {
                println!("not a dir")
            }
        }

        i += 1;
    }
    return plant_database_found;
}
