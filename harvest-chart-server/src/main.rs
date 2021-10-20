#[cfg(test)]
#[macro_use]
extern crate more_asserts;

#[macro_use]
extern crate diesel;
use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[macro_use]
extern crate diesel_migrations;
embed_migrations!();

extern crate dotenv;

mod import_db;
mod queries;
mod schema_generated;
mod schema_types;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_conn = import_db::establish_connection();
    import_db::reset_database(&db_conn);
    let items_loaded = import_db::load_all(&db_conn);

    if items_loaded.base_plants_found == 0 {
        println!("directory \"plant_database\" not found");
        std::process::exit(1);
    }
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start HTTP server
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
