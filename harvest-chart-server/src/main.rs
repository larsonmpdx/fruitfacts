#[macro_use]
extern crate diesel;
mod schema;

extern crate dotenv;

use diesel::prelude::*;
use schema::base_plants::dsl::*;
// use diesel::sqlite;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::fs;


use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct PlantJson {
    name: String,
    #[serde(alias = "type")]
    type_: Option<String>,
    description: Option<String>,
    patent: Option<String>,
    relative_harvest: Option<String>,
    harvest_start: Option<i16>,
    harvest_end: Option<i16>
}


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let conn = establish_connection();

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

                                let contents = fs::read_to_string(path_).unwrap();

                                let plants: Vec<PlantJson> = serde_json::from_str(&contents).unwrap();

                                for plant in &plants {
                                    let _ = diesel::insert_into(base_plants)
                                    .values((name.eq(&plant.name), type_.eq(&plant.type_.as_ref().unwrap())))
                                    .execute(&conn);
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
