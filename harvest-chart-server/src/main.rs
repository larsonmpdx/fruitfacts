#[macro_use]
extern crate diesel;
mod schema;

#[cfg(test)]
mod test;

extern crate dotenv;

use chrono::prelude::*;
use diesel::prelude::*;
use schema::base_plants::dsl::*;
// use diesel::sqlite;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::fs;

#[macro_use]
extern crate diesel_migrations;
embed_migrations!();

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
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
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

fn rem_last_n(value: &str, n: isize) -> &str {
    let mut chars = value.chars();
    for _ in 0..n {
        chars.next_back();
    }
    chars.as_str()
}

fn main() {
    let db_conn = establish_connection();
    let _ = diesel::delete(base_plants).execute(&db_conn);
    embedded_migrations::run(&db_conn).unwrap();

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
                                    let plant_type;
                                    if filename.starts_with("Oddball") {
                                        plant_type = plant.type_.clone().unwrap();
                                    } else {
                                        plant_type = filename.to_string();
                                    }

                                    println!("inserting");
                                    let rows_inserted = diesel::insert_into(base_plants)
                                        .values((
                                            name.eq(&plant.name),
                                            type_.eq(&plant_type),
                                            description.eq(&plant.description),
                                            patent.eq(&plant.patent),
                                            relative_harvest.eq(&plant.relative_harvest), // todo harvest start+end
                                        ))
                                        .execute(&db_conn);
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

    if !plant_database_found {
        println!("directory plant_database/ not found");
        std::process::exit(1);
    }
}
