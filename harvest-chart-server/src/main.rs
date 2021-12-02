use harvest_chart_server::import_db;
use harvest_chart_server::queries;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

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
    } else if import_db::count_base_plants(&db_conn) == 0 {
        panic!(r#"no plants found in database, import the database first with "-r""#)
    }

    let connspec = "database.sqlite3";
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    println!("starting http server");
    HttpServer::new(move || {
        let cors = Cors::permissive(); // todo - maybe remove this on release?

        App::new()
            .wrap(cors)
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            // .wrap(middleware::Logger::default())
            .service(queries::get_recent_patents)
            .service(queries::get_collections)
            .service(queries::get_build_info)
            .service(queries::get_plant)
            .service(queries::variety_search)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
