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

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
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
                plant_database_found = true;
                let paths = fs::read_dir(path).unwrap();

                for path in paths {
                    println!("Name: {}", path.unwrap().path().display())
                }

                println!("is dir: {}", md.is_dir());
                break;
            }
            Err(_) => {
                println!("not dir")
            }
        }

        i += 1;
    }

    if !plant_database_found {
        println!("directory plant_database/ not found");
        std::process::exit(1);
    }

    let conn = establish_connection();

    let _ = diesel::insert_into(base_plants)
        .values((name.eq("Pristine"), type_.eq("Apple")))
        .execute(&conn);
}
