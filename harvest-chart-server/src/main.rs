#[cfg(test)]
#[macro_use]
extern crate more_asserts;

#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

#[macro_use]
extern crate diesel_migrations;
embed_migrations!();

extern crate dotenv;

mod import_db;
mod queries;
mod schema_generated;
mod schema_types;

use actix_web::{App, HttpServer};

extern crate clap;
use clap::{crate_version, App as ClapApp, Arg};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = ClapApp::new("")
        .version(crate_version!())
        .arg(
            Arg::with_name("reload_db")
                .short("r")
                .long("reload_db")
                .required(false)
                .takes_value(false)
                .help("reload db"),
        )
        .get_matches();

    let db_conn = import_db::establish_connection();
    if matches.is_present("reload_db") {
        import_db::reset_database(&db_conn);
        let items_loaded = import_db::load_all(&db_conn);

        if items_loaded.base_plants_found == 0 {
            panic!("directory \"plant_database\" not found");
        }
    } else {
        if import_db::count_base_plants(&db_conn) == 0 {
            panic!(r#"no plants found in database, import the database first with "-r""#)
        }
    }

    let connspec = "database.sqlite3";
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    println!("starting http server");
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            //    .wrap(middleware::Logger::default())
            .service(queries::get_recent_patents)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
