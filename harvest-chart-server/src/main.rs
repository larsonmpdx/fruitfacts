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

pub fn reset_database(db_conn: &SqliteConnection) {
    let _ = diesel::delete(base_plants).execute(db_conn);
    embedded_migrations::run(db_conn).unwrap();
}

fn main() {
    let db_conn = establish_connection();
    reset_database(&db_conn);

    let plants_found = import_db::load_base_plants(&db_conn);

    if plants_found == 0 {
        println!("directory plant_database/ not found");
        std::process::exit(1);
    }

    // todo
}
