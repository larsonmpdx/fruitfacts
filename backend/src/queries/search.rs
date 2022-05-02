use super::super::schema_fts::*;
use super::super::schema_generated::*;
use super::super::schema_types::*;
use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use regex::Regex;
use serde::Deserialize;

pub fn variety_search_db(
    db_conn: &SqliteConnection,
    input: &str,
) -> Result<Vec<BasePlant>, diesel::result::Error> {
    // remove extra characters. leave spaces so we can treat separate words as separate
    // dashes get interpreted by fts. same with +*:^ AND OR NOT
    let re = Regex::new(r"[^0-9A-Za-z ]").unwrap();
    let cleaned = re.replace_all(input.trim(), ""); // also trim() for leading/trailing whitespace

    // replace multiple spaces with a single space
    let re = Regex::new(r"\s+").unwrap();
    let multiple_spaces_removed = re.replace_all(&cleaned, " ");

    // split on whitespace and insert OR statements for each space
    let statement = multiple_spaces_removed
        .split(' ')
        .collect::<Vec<&str>>()
        .join(" OR ");

    println!("input {input} cleaned: {cleaned} ORed: {statement}");

    // if string ends with an exact match for a type we have, then add a limit to the search
    // for example "pristine apple" should remove the word apple from the search (or maybe not?) and then add a filter for only apples
    // todo

    let values = fts_base_plants::table
        .select((fts_base_plants::rowid, fts_base_plants::rank))
        .filter(fts_base_plants::whole_row.eq(statement))
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
        .unwrap(); // todo - blockingerror unwrap?

    let results = match results {
        Ok(results) => results,
        Err(e) => {
            eprintln!("{}", e);
            return Err(actix_web::error::ErrorInternalServerError(""));
        }
    };

    Ok(HttpResponse::Ok().json(results))
}
