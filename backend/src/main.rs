use harvest_chart_server::auth;
use harvest_chart_server::import_db;
use harvest_chart_server::queries;

use actix_cors::Cors;

use actix_web::{middleware::Logger, App, HttpServer};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

extern crate clap;
use clap::{crate_version, Arg, Command as ClapApp};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = ClapApp::new("")
        .version(crate_version!())
        .arg(
            Arg::new("reload_db")
                .short('r')
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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("starting http server");
    HttpServer::new(move || {
        let cors = Cors::default()
            .supports_credentials()
            .allowed_origin_fn(|origin, _req_head| {
                origin
                    .as_bytes()
                    .ends_with(env!("FRONTEND_BASE").to_string().as_bytes())
                // todo - better handling of port for dev/release
            });

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            // .wrap(middleware::Logger::default())
            .service(queries::get_recent_patents)
            .service(queries::get_collections)
            .service(queries::get_recent_changes)
            .service(queries::get_fact)
            .service(queries::get_plant)
            .service(queries::variety_search)
            .service(auth::get_auth_urls)
            .service(auth::receive_oauth_redirect)
            .service(auth::create_account)
            .service(auth::check_login)
            .service(auth::logout)
    })
    .bind(("127.0.0.1", env!("BACKEND_PORT").parse::<u16>().unwrap()))?
    .run()
    .await
}
