#[macro_use]
extern crate diesel;

#[macro_use]
extern crate more_asserts;

#[macro_use]
extern crate diesel_migrations;
embed_migrations!();

extern crate dotenv;

mod import_db;
mod schema;
mod schemaTypes;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use schema::base_plants::dsl::*;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let db_conn = establish_connection();
    import_db::reset_database(&db_conn);

    let items_loaded = import_db::load_all(&db_conn);

    if items_loaded.plants_found == 0 {
        println!("directory plant_database/ not found");
        std::process::exit(1);
    }

    // todo
}
