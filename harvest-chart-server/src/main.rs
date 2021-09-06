#[macro_use]
extern crate diesel;
mod import_db;
mod schema;

extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use schema::base_plants::dsl::*;
use std::env;

#[macro_use]
extern crate diesel_migrations;
embed_migrations!();

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let db_conn = establish_connection();
    let _ = diesel::delete(base_plants).execute(&db_conn);
    embedded_migrations::run(&db_conn).unwrap();

    let plant_database_found = import_db::load_base_plants(&db_conn);

    if !plant_database_found {
        println!("directory plant_database/ not found");
        std::process::exit(1);
    }

    // todo
}
