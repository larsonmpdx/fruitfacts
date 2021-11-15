use crate::schema_types::Collections;
use super::schema_generated::*;
use std::collections::HashSet;
use actix_web::{get, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
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
    pub location_id: i32,
    pub collection_id: i32,

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

    if !path.is_empty() && !path.ends_with("/") {
        return Ok(Default::default())
    }

    let db_return = collections::dsl::collections
    .select((
        collections::location_id,
        collections::collection_id,
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

            return Ok(output);
        }
        Err(error) => {
            return Err(error);
        }
    }
}

#[derive(Deserialize)]
struct Path {
    path: String,
}

#[get("/collections/{path:.*}")] // the ":.*" part is a regex to get the entire tail of the path
async fn get_collections(info: web::Path<Path>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let collections = web::block(move || get_collections_db(&conn, &info.path))
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    // get all collections paths
    // if there's a further '/' in the path, put this in a directory array
    // if there isn't, put it in a collections array

    Ok(HttpResponse::Ok().json(collections))
}
