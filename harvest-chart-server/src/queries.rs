use super::schema_generated::*;
use actix_web::{get, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use regex::Regex;
use std::collections::HashSet;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use super::schema_types::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize)]
pub struct BasePlantsItemForPatents {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub uspp_number: Option<String>,
    pub uspp_expiration: Option<i64>,
}

pub fn get_recent_patents_db(
    conn: &SqliteConnection,
) -> Result<Vec<BasePlantsItemForPatents>, diesel::result::Error> {
    base_plants::dsl::base_plants
        .select((
            base_plants::name,
            base_plants::type_,
            base_plants::uspp_number,
            base_plants::uspp_expiration,
        ))
        .filter(base_plants::uspp_expiration.is_not_null())
        .order(base_plants::uspp_expiration.desc())
        .load::<BasePlantsItemForPatents>(conn)
}

#[get("/patents")]
async fn get_recent_patents(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let patents = web::block(move || get_recent_patents_db(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(patents))
}

#[derive(Queryable, Debug, Serialize)]
pub struct CollectionsForPaths {
    pub id: i32,

    pub path: Option<String>,
    pub filename: Option<String>,

    pub title: Option<String>,
}

#[derive(Default, Serialize)]
pub struct CollectionsReturn {
    directories: HashSet<String>,
    collections: Vec<CollectionsForPaths>,
}

pub fn get_collections_db(
    conn: &SqliteConnection,
    path: &str,
) -> Result<CollectionsReturn, diesel::result::Error> {
    let db_return = collections::dsl::collections
        .select((
            collections::id,
            collections::path,
            collections::filename,
            collections::title,
        ))
        .filter(collections::path.like(path.to_owned() + r#"%"#))
        .load::<CollectionsForPaths>(conn);

    match db_return {
        Ok(collections) => {
            let mut output: CollectionsReturn = Default::default();
            for collection in collections {
                if let Some(collection_path) = &collection.path {
                    if collection_path == path {
                        output.collections.push(collection);
                    } else {
                        output.directories.insert(collection_path.to_string()); // this is a hashset so we'll get paths de-duplicated here
                    }
                }
            }
            // output.collections = collections;

            Ok(output)
        }
        Err(error) => Err(error),
    }
}

#[derive(Default, Serialize)]
pub struct CollectionReturn {
    collection: Option<Collection>,
    locations: Vec<Location>,
    items: Vec<CollectionItem>,
}

pub fn get_collection_db(
    conn: &SqliteConnection,
    path: &str,
) -> Result<CollectionReturn, diesel::result::Error> {
    println!("{}", path);

    // this could be done with rfind('/') or similar to get rid of the regex
    // todo: at least limit the length of the incoming text to protect the regex
    let slash_regex = Regex::new(r#"(.*)/(.*)"#).unwrap();

    let mut dir: String = Default::default();
    let mut file: String = Default::default();
    if let Some(matches) = slash_regex.captures(path) {
        if matches.len() >= 3 {
            if let Some(dir_match) = matches.get(1) {
                dir = dir_match.as_str().to_string();
            }
            if let Some(file_match) = matches.get(2) {
                file = file_match.as_str().to_string();
            }
        }
    } else {
        file = path.to_string();
    }
    dir.push('/');

    println!("{:#?} {:#?}", dir, file);

    let collection: Result<Collection, diesel::result::Error> = collections::dsl::collections
        .filter(collections::path.eq(dir))
        .filter(collections::filename.eq(file))
        .order(collections::id)
        .first(conn);

    match collection {
        Ok(collection) => {
            let mut output: CollectionReturn = Default::default();

            let locations = Location::belonging_to(&collection)
                .load::<Location>(conn)
                .expect("Error loading locations");

            let items = CollectionItem::belonging_to(&collection)
                .load::<CollectionItem>(conn)
                .expect("Error loading items");

            output.collection = Some(collection);
            output.locations = locations;
            output.items = items;
            Ok(output)
        }
        Err(error) => Err(error),
    }
}

#[derive(Deserialize)]
struct Path {
    path: String,
}
// /collections/path/ - get subdirectories starting at this path, and collection names at this path
// /collections/path/collection - get a single collection
#[get("/collections/{path:.*}")] // the ":.*" part is a regex to get the entire tail of the path
async fn get_collections(
    info: web::Path<Path>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    if info.path.is_empty() || info.path.ends_with('/') {
        // get all subdirectories and all collections at this path
        let collections = web::block(move || get_collections_db(&conn, &info.path))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        Ok(HttpResponse::Ok().json(collections))
    } else {
        // doesn't end in '/': get an individual collection

        let collection = web::block(move || get_collection_db(&conn, &info.path))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        Ok(HttpResponse::Ok().json(collection))
    }
}

#[derive(Default, Serialize)]
pub struct PlantReturn {
    base: Option<BasePlant>,
    collection: Vec<CollectionItem>,
}

pub fn get_plant_db(
    conn: &SqliteConnection,
    type_: &str,
    name: &str,
) -> Result<PlantReturn, diesel::result::Error> {
    let plant: Result<BasePlant, diesel::result::Error> = base_plants::dsl::base_plants
        .filter(base_plants::type_.eq(type_))
        .filter(base_plants::name.eq(name))
        .first(conn);

    println!("get plant: {} {}", type_, name);

    match plant {
        Ok(plant) => {
            let collection_plants: Result<Vec<CollectionItem>, diesel::result::Error> =
                collection_items::dsl::collection_items
                    .filter(collection_items::type_.eq(type_))
                    .filter(collection_items::name.eq(name))
                    .load(conn);

            // todo: limit number to maybe 10, ordered by significance, and return the total number if we were limited so we can show "see all"

            match collection_plants {
                Ok(collection_plants) => {
                    let output = PlantReturn {
                        base: Some(plant),
                        collection: collection_plants,
                    };
                    Ok(output)
                }
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
    }
}

#[derive(Deserialize)]
struct GetPlantPath {
    type_: String,
    plant: String,
}

// /plants/type/ - all plants of this type. paginated?
// /plants/type/plant name - this specific plant
#[get("/plants/{type_}/{plant:.*}")] // the ":.*" part is a regex to get the entire tail of the path
async fn get_plant(
    info: web::Path<GetPlantPath>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    if info.plant.is_empty() {
        // todo: all plants in this category. pagination?
        Ok(HttpResponse::Ok().json(()))
    } else {
        // one plant

        let collection = web::block(move || get_plant_db(&conn, &info.type_, &info.plant))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        Ok(HttpResponse::Ok().json(collection))
    }
}

#[derive(Serialize)]
struct BuildInfo {
    git_hash: String,
    git_unix_time: String,
    git_commit_count: String,
}

#[get("/build_info")]
async fn get_build_info() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(BuildInfo {
        git_hash: env!("GIT_HASH").to_string(),
        git_unix_time: env!("GIT_UNIX_TIME").to_string(),
        git_commit_count: env!("GIT_MAIN_COMMIT_COUNT").to_string(),
    }))
}
