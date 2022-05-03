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

    let mut statement = multiple_spaces_removed.split(' ').collect::<Vec<&str>>();

    // if we have multiple search words, also add a search element which is all of them concatenated
    // allows searching for "pf 11" which would otherwise be two chars
    let split_for_count = multiple_spaces_removed.split(' ').collect::<Vec<&str>>();

    // look at the last element to see if one of our types starts with this - if so we'll restrict results to this type
    let mut restrict_to_type: Option<String> = None;
    if split_for_count.len() >= 2 {
        for type_ in crate::import_db::generated::TYPES.iter() {
            if *type_.to_lowercase() == split_for_count.last().unwrap().to_lowercase() {
                restrict_to_type = Some(type_.to_string());
                break;
            }
        }
    }

    if restrict_to_type.is_some() {
        statement.pop(); // we got a type by matching on the last search element, remove it from the FTS search words
    }

    let mut statement_string = statement
        .clone()
        .into_iter()
        .map(|x| format!("\"{x}\"")) // double quote each element - allows searching for special characters or keywords like "OR"
        .collect::<Vec<String>>()
        .join(" OR ");

    // if we have multiple words, try adding one last search term which is all of them concatenated
    // this helps us with "pf 1" for example
    if split_for_count.len() >= 2 {
        statement_string.push_str(&format!(" OR \"{}\"", statement.join("")));
    }

    println!("input {input} cleaned: {cleaned} ORed: {statement_string}");

    const N_MAX: i64 = 10;

    let mut query = fts_base_plants::table
        .select((fts_base_plants::rowid, fts_base_plants::rank))
        .filter(fts_base_plants::whole_row.eq(statement_string))
        .order(fts_base_plants::rank.asc())
        .into_boxed();

    match restrict_to_type {
        None => {
            query = query.limit(N_MAX);
        }
        _ => {
            query = query.limit(N_MAX * 10); // we gather more if we're going to later restrict by type
        }
    }

    let values = query.load::<FtsBasePlants>(db_conn);
    // todo - maybe limit 100 or something? we want to get a bunch though in case we're limiting to only one variety later
    // todo - report total search results if limiting to N

    println!("{:?}", values);

    // todo: order or limit by notoriety
    match values {
        Ok(values) => {
            let ids_nullable: Vec<_> = values.iter().map(|x| x.rowid).collect();

            // step through IDs and get the original row
            // this lets us preserve FTS ranking
            let mut results: Vec<BasePlant> = Default::default();

            match restrict_to_type {
                None => {
                    for id in &ids_nullable {
                        let result = base_plants::dsl::base_plants
                            .filter(base_plants::id.eq(id))
                            .first::<BasePlant>(db_conn)
                            .unwrap();

                        results.push(result);
                    }
                }

                Some(type_) => {
                    for id in &ids_nullable {
                        let result = base_plants::dsl::base_plants
                            .filter(base_plants::id.eq(id))
                            .filter(base_plants::type_.eq(&type_))
                            .first::<BasePlant>(db_conn); // this row may or may not match our type filter

                        if let Ok(result) = result {
                            results.push(result);
                            if results.len() as i64 >= N_MAX {
                                break; // we limit here because our fts search was allowed to be for more results before we did our type filter
                            }
                        }
                    }
                }
            };

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

// search test cases:
// "red" -> "redhaven" "early redhaven" ...
// "pf 11" -> should find pf 11 peach, treating this as two words wouldn't find it because of fts searching based on trigraphs
// "pf 1" -> pf 1 peach
// "pf-11" -> pf 11 peach
// "pf 1 peach" -> pf 1 peach (multiple search terms, short, etc.)
// "liberty apple" -> should be an exact match, and not return "dapple dandy" (contains the word apple)
//    may also suggest the apple page
// "liberty peach" -> should be an exact match, not finding "burpeachwhatever"

// todo: if we have an exact match for a type name (or type aka name) then remove that word, use it to suggest that type
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
