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
        std::process::exit(0);
    } else if import_db::count_base_plants(&db_conn) == 0 {
        panic!(r#"no plants found in database, import the database first with "cargo run -- --reload_db""#)
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
                let matched = origin
                    .as_bytes()
                    .ends_with(env!("FRONTEND_BASE").to_string().as_bytes());

                if !matched {
                    println!(
                        "cors failed, got {:?} expected {:?}",
                        origin,
                        env!("FRONTEND_BASE")
                    );
                }
                matched
                // todo - better handling of port for dev/release
            });

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(queries::get_recent_patents)
            .service(queries::get_collections)
            .service(queries::get_recent_changes)
            .service(queries::get_fact)
            .service(queries::get_plant)
            .service(queries::auth::get_auth_urls)
            .service(queries::auth::receive_oauth_redirect)
            .service(queries::auth::create_account)
            .service(queries::auth::get_full_user)
            .service(queries::auth::check_login)
            .service(queries::auth::logout)
            .service(queries::search::variety_search)
            .service(queries::map::locations_search)
    })
    .bind(("0.0.0.0", env!("BACKEND_PORT").parse::<u16>().unwrap()))? // 0.0.0.0 is actix-speak for "all local IPs"
    .run()
    .await
}
