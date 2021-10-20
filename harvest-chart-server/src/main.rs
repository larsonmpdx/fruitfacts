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

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_conn = import_db::establish_connection();
    import_db::reset_database(&db_conn);
    let items_loaded = import_db::load_all(&db_conn);

    if items_loaded.base_plants_found == 0 {
        println!("directory \"plant_database\" not found");
        std::process::exit(1);
    }

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
