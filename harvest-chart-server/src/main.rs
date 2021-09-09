#[cfg(test)]
#[macro_use]
extern crate more_asserts;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;
embed_migrations!();

extern crate dotenv;

mod import_db;
mod schema_generated;
mod schema_types;

fn main() {
    let db_conn = import_db::establish_connection();
    import_db::reset_database(&db_conn);
    let items_loaded = import_db::load_all(&db_conn);

    if items_loaded.plants_found == 0 {
        println!("directory \"plant_database\" not found");
        std::process::exit(1);
    }

    // todo
}
