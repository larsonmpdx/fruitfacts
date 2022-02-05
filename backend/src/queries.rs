use super::schema_fts::*;
use super::schema_generated::*;
use super::schema_types::*;
use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Queryable, Debug, Serialize)]
pub struct BasePlantsItemForPatents {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,

    pub notoriety_score: Option<f32>,
    pub marketing_name: Option<String>,

    pub uspp_number: Option<String>,
    pub uspp_expiration: Option<i64>,

    pub release_year: Option<i32>,
    pub released_by: Option<String>,
    pub release_collection_id: Option<i32>,
}

// get patents from start_n to end_n in either the future or past. with two queries we can get +/-N
// the user can then step into the past or future with queries for +N to +3N (first future page) etc.
pub fn get_recent_patents_db_subquery(
    db_conn: &SqliteConnection,
    future: bool,
    start_n: i32,
    end_n: i32,
    n: i32,
) -> Result<Vec<BasePlantsItemForPatents>, diesel::result::Error> {
    let mut query = base_plants::table
        .select((
            base_plants::name,
            base_plants::type_,
            base_plants::notoriety_score,
            base_plants::marketing_name,
            base_plants::uspp_number,
            base_plants::uspp_expiration,
            base_plants::release_year,
            base_plants::released_by,
            base_plants::release_collection_id,
            // todo: optional minimum notoriety
            // todo: more fields, to make the recent patents table more interesting
            // probably releaser, release year if available and release collection link
            // marketing name
        ))
        .filter(base_plants::uspp_expiration.is_not_null())
        .into_boxed();

    let now = chrono::Utc::now().timestamp();
    if future {
        query = query.filter(base_plants::uspp_expiration.gt(now));
        query = query.order(base_plants::uspp_expiration.asc());
    } else {
        query = query.filter(base_plants::uspp_expiration.lt(now));
        query = query.order(base_plants::uspp_expiration.desc());
    }

    let start = n * start_n;
    let end = n * end_n;
    let length = end - start;
    query = query.limit((length).into()).offset(start.into());
    eprintln!("{start} to {end}, length {length}"); // todo - figure out off-by-1s so we aren't skipping anything between pages or overlapping

    query.load::<BasePlantsItemForPatents>(db_conn)
}

// page 0: centered on now (N past patents plus N future patents)
// page -1: the next page (2N in size) in the past
// page 1: next future page
// etc.
pub fn get_recent_patents_db(
    db_conn: &SqliteConnection,
    page_in: Option<i32>,
    per_page_in: Option<i32>,
) -> Result<Vec<BasePlantsItemForPatents>, diesel::result::Error> {
    let page;
    if page_in.is_none() {
        page = 0;
    } else {
        page = page_in.unwrap();
    }

    // N is half a page
    const N_MAX: i32 = 50;
    let mut n;
    if per_page_in.is_none() {
        n = 30;
    } else {
        n = per_page_in.unwrap() / 2;
        if n > N_MAX {
            n = N_MAX;
        }
    }

    match page {
        page if page > 0 => {
            // page 1: 1 to 3
            // page 2: 3 to 5
            // page 3: 5 to 7
            // ...
            // page*2 -1 to page*2 + 1
            return get_recent_patents_db_subquery(db_conn, true, page * 2 - 1, page * 2 + 1, n);
        }
        page if page < 0 => {
            let result = get_recent_patents_db_subquery(
                db_conn,
                false,
                (page.abs() - 1) * 2 + 1,
                page.abs() * 2 + 1,
                n,
            );

            if result.is_err() {
                return result;
            }
            let mut result = result.unwrap();
            result.reverse();
            return Ok(result);
        }
        _ => {
            // 0
            let future = get_recent_patents_db_subquery(db_conn, true, 0, 1, n);
            let past = get_recent_patents_db_subquery(db_conn, false, 0, 1, n);

            if future.is_err() {
                return future;
            }

            if past.is_err() {
                return past;
            }

            let mut future_vec = future.unwrap();
            let mut past_vec = past.unwrap();
            past_vec.reverse();
            past_vec.append(&mut future_vec);
            return Ok(past_vec);
        }
    }
}

#[derive(Deserialize)]
struct GetPatentsQuery {
    page: Option<i32>,
    #[serde(rename = "perPage")]
    per_page: Option<i32>,
}

#[get("/api/patents")]
async fn get_recent_patents(
    query: web::Query<GetPatentsQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let patents = web::block(move || get_recent_patents_db(&conn, query.page, query.per_page))
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
    directories: Vec<String>,
    collections: Vec<CollectionsForPaths>,
}

pub fn get_collections_db(
    db_conn: &SqliteConnection,
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
        .order(collections::path.asc())
        .load::<CollectionsForPaths>(db_conn);

    match db_return {
        Ok(collections) => {
            let mut output: CollectionsReturn = Default::default();
            let mut directories: HashSet<String> = Default::default();
            for collection in collections {
                if let Some(collection_path) = &collection.path {
                    if collection_path == path {
                        output.collections.push(collection);
                    } else {
                        // remove multi-level subdirectories (more than one '/' after our search directory)
                        let collection_path = collection_path.to_string();
                        let trimmed = crate::import_db::rem_first_n(&collection_path, path.len());
                        if trimmed.matches('/').count() == 1 {
                            directories.insert(collection_path); // this is a hashset so we'll get paths de-duplicated here
                        } else {
                            // println!("excluding subdir {}", collection_path)
                        }
                    }
                }
            }

            let mut directories_vector = Vec::from_iter(directories);
            directories_vector.sort();
            output.directories = directories_vector;

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
    db_conn: &SqliteConnection,
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
        .first(db_conn);

    match collection {
        Ok(collection) => {
            let mut output: CollectionReturn = Default::default();

            let locations = Location::belonging_to(&collection)
                .load::<Location>(db_conn)
                .expect("Error loading locations");

            let items = CollectionItem::belonging_to(&collection)
                .load::<CollectionItem>(db_conn)
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
#[get("/api/collections/{path:.*}")] // the ":.*" part is a regex to get the entire tail of the path
async fn get_collections(
    path: web::Path<Path>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    if path.path.is_empty() || path.path.ends_with('/') {
        // get all subdirectories and all collections at this path
        let collections = web::block(move || get_collections_db(&conn, &path.path))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        Ok(HttpResponse::Ok().json(collections))
    } else {
        // doesn't end in '/': get an individual collection

        let collection = web::block(move || get_collection_db(&conn, &path.path))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        Ok(HttpResponse::Ok().json(collection))
    }
}

#[derive(Default, Serialize)]
pub struct PlantsReturn {
    plants: Vec<BasePlant>,
}

pub fn get_plants_db(
    db_conn: &SqliteConnection,
    type_: &str,
    page: Option<i32>,
) -> Result<PlantsReturn, diesel::result::Error> {
    const PER_PAGE: i32 = 50;

    let mut query = base_plants::table
        .filter(base_plants::type_.eq(type_))
        .limit(PER_PAGE.into())
        .into_boxed();

    if let Some(page) = page {
        query = query.offset((page * PER_PAGE).into());
    }

    // todo - apply other things: order by, asc/desc, etc.

    let plants: Result<Vec<BasePlant>, diesel::result::Error> = query.load(db_conn);

    println!("get plants: {} page {:?}", type_, page);

    match plants {
        Ok(plants) => Ok(PlantsReturn { plants }),
        Err(error) => Err(error),
    }
}

#[derive(Default, Serialize)]
pub struct PlantReturn {
    base: Option<BasePlant>,
    collection: Vec<CollectionItem>,
}

pub fn get_plant_db(
    db_conn: &SqliteConnection,
    type_: &str,
    name: &str,
) -> Result<PlantReturn, diesel::result::Error> {
    let plant: Result<BasePlant, diesel::result::Error> = base_plants::dsl::base_plants
        .filter(base_plants::type_.eq(type_))
        .filter(base_plants::name.eq(name))
        .first(db_conn);

    println!("get plant: {} {}", type_, name);

    match plant {
        Ok(plant) => {
            let collection_plants: Result<Vec<CollectionItem>, diesel::result::Error> =
                collection_items::dsl::collection_items
                    .filter(collection_items::type_.eq(type_))
                    .filter(collection_items::name.eq(name))
                    .load(db_conn);

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

#[derive(Deserialize)]
struct GetPlantQuery {
    page: Option<i32>,
}

// /plants/type/ - all plants of this type. paginated?
// /plants/type/plant name - this specific plant
#[get("/api/plants/{type_}/{plant:.*}")] // the ":.*" part is a regex to get the entire tail of the path
async fn get_plant(
    path: web::Path<GetPlantPath>,
    query: web::Query<GetPlantQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    if path.plant.is_empty() {
        let collection = web::block(move || get_plants_db(&conn, &path.type_, query.page))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        Ok(HttpResponse::Ok().json(collection))
    } else {
        // one plant

        let collection = web::block(move || get_plant_db(&conn, &path.type_, &path.plant))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        Ok(HttpResponse::Ok().json(collection))
    }
}

#[derive(Serialize)]
struct RecentChanges {
    build_info: BuildInfo,
    recent_changes: RecentChangesDB,
}

#[derive(Serialize)]
struct BuildInfo {
    git_hash: String,
    git_unix_time: String,
    git_commit_count: String,
}

#[derive(Default, Queryable, Debug, Serialize)]
pub struct CollectionChanges {
    pub id: i32,

    pub path: Option<String>,
    pub filename: Option<String>,

    pub title: Option<String>,

    pub git_edit_time: Option<i64>,
}

#[derive(Default, Serialize)]
pub struct RecentChangesDB {
    pub collection_changes: Vec<CollectionChanges>,
    pub base_plants_count: i64,
    pub references_count: i64,
}

pub fn get_recent_changes_db(db_conn: &SqliteConnection) -> Result<RecentChangesDB> {
    let mut output: RecentChangesDB = Default::default();

    let db_return = collections::dsl::collections
        .select((
            collections::id,
            collections::path,
            collections::filename,
            collections::title,
            collections::git_edit_time,
        ))
        .order(collections::git_edit_time.desc())
        .limit(10)
        .load::<CollectionChanges>(db_conn);

    match db_return {
        Ok(collections) => {
            output.collection_changes = collections;
        }
        Err(error) => {
            return Err(error.into());
        }
    }

    let collections_count = collections::dsl::collections.count().first::<i64>(db_conn);
    if let Ok(count) = collections_count {
        output.references_count = count;
    }

    let base_plants_count = base_plants::dsl::base_plants.count().first::<i64>(db_conn);
    if let Ok(count) = base_plants_count {
        output.base_plants_count = count;
    }

    Ok(output)
}

#[get("/api/recent_changes")]
async fn get_recent_changes(pool: web::Data<DbPool>) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let changes_db = web::block(move || get_recent_changes_db(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(RecentChanges {
        build_info: BuildInfo {
            git_hash: env!("GIT_HASH").to_string(),
            git_unix_time: env!("GIT_UNIX_TIME").to_string(),
            git_commit_count: env!("GIT_MAIN_COMMIT_COUNT").to_string(),
        },
        recent_changes: changes_db,
    }))
}

no_arg_sql_function!(random, ());

pub fn get_fact_db(db_conn: &SqliteConnection) -> Result<Fact, diesel::result::Error> {
    facts::dsl::facts
        .order(random)
        .limit(1)
        .first::<Fact>(db_conn)
}

#[get("/api/fact")]
async fn get_fact(pool: web::Data<DbPool>) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let fact = web::block(move || get_fact_db(&conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(fact))
}

pub fn variety_search_db(
    db_conn: &SqliteConnection,
    name: &str,
) -> Result<Vec<BasePlant>, diesel::result::Error> {
    // extra characters. leave spaces so FTS still gets to match two different words
    // dashes get interpreted by fts. same with +*:^ AND OR NOT
    let re = Regex::new(r"[^0-9A-Za-z ]").unwrap();
    let cleaned = re.replace_all(name, "");

    println!("input {} cleaned: {}", name, cleaned);

    // todo: match extra words against our list of types. remove them or use them for a type filter
    // like "surefire cherry" -> cherry should be removed unless we can beef up fts search to allow it

    let values = fts_base_plants::table
        .select((fts_base_plants::rowid, fts_base_plants::rank))
        .filter(fts_base_plants::whole_row.eq(cleaned))
        .order(fts_base_plants::rank.asc())
        .limit(10)
        .load::<FtsBasePlants>(db_conn);
    // todo - maybe limit 100 or something? we want to get a bunch though in case we're limiting to only one variety later
    // todo - report total search results if limiting to N

    println!("{:?}", values);

    // todo: filter by type, order or limit notoriety
    match values {
        Ok(values) => {
            let ids_nullable: Vec<_> = values.iter().map(|x| x.rowid).collect();

            let results = base_plants::dsl::base_plants
                .filter(base_plants::id.eq_any(ids_nullable))
                .load::<BasePlant>(db_conn)
                .unwrap();

            println!("{:?}", results);

            Ok(results)
        }
        Err(error) => Err(error),
    }
}

#[derive(Deserialize)]
struct SearchPath {
    string: String,
}

// searches to support:
// plain variety search: "red" -> "redhaven" "early redhaven" ...
// with type: "redhaven peach" -> "redhaven" and also suggest the category "peach"
// rules: if we have an exact match for a type name (or type aka name) then remove that word, use it to suggest that type
// todo - this kind of type search plus a full text search on the collections json files
#[get("/api/search/{string}")]
async fn variety_search(
    path: web::Path<SearchPath>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results = web::block(move || variety_search_db(&conn, &path.string))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}
