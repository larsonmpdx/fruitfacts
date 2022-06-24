use super::super::schema_fts::*;
use super::super::schema_generated::*;
use super::super::schema_types::*;
use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use regex::Regex;
use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Result};

#[derive(Deserialize)]
pub struct SearchQuery {
    search_type: Option<String>, // base plants or collection items. todo: user items, "search all"
    search: Option<String>,      // search string like "PF 11"
    name: Option<String>, // exact plant name (allows getting a single base plant). probably doesn't make sense when used with search
    patents: Option<bool>,
    #[serde(rename = "type")]
    type_: Option<String>, // apple, peach, etc.
    page: Option<String>, // 0-N or "mid" for the patent midpoint page if unknown (so our first patent page link can work)
    per_page: Option<i32>,
    sort: Option<String>, // search by search quality, type then name, name then type, patent expiration (special case, also compute the middle patent page), harvest time
    relative_harvest: Option<String>, // minimum, maximum days

    // collection items search only
    collection: Option<String>, // collection path, or collection ID (number)

    // base plants search only:
    notoriety: Option<String>,

    // todo:
    distance: Option<String>, // max distance, goes with "from"
    from: Option<String>,     // goes with distance, a zip code or point or something
}

// base plants search:
// 1. do name search if specified
// 2. do regular filter search: patents yes/no, type, relative harvest, notoriety, sort
// 3. intersection of these (either in database or out of database)
// 4. figure out pagination (total size, and pick a page to return)

// collection items search:
// 1. do name search if specified? or I could omit name filter on this for now
// 2. regular filter search: same as above? notoriety maybe doesn't make sense. big add is filter by collection yes/no
// 3. intersection?
// 4. figure out pagination (total size, and pick a page to return)
// 5. look up and return collection+location info, if specified

// how much overlap can be found between these two paths? and eventually user items search

// queries to bring under this:
// - plain variety search (against base plants) - base plants searchType, and search. limit to first N or whatever
// - fancy search and filter for an "advanced search" page, with pagination - same as above but no limit and paginate, sort by whatever, etc.
// - get a single variety (get a single base plant) - base plants searchType, name and type
// - get patents - base plants searchType, patents:true, probably sort by expiration
// - get paginated plants (browse plants) - base plants searchType, type=plant type, page set, etc.
// do this 2nd, it's the only non-base-plants one:
// - get a single collection - collection items searchType, and collection ID or path. would allow filtering or sorting from the collection page

#[derive(Default, Queryable, Serialize)]
pub struct SearchReturn {
    search_type: Option<String>, // base plants or collection
    count: Option<i32>,          // total count of search results (if paginated)
    page: Option<i32>,
    patent_midpoint_page: Option<i32>, // special case: if we did a patent search, which page has the transition from past to future expirations?
    pub base_plants: Option<Vec<BasePlant>>,

    // only if constrained to one collection:
    pub collection_items: Option<Vec<CollectionItem>>,
    pub collection: Option<Collection>,
    pub locations: Vec<Location>,
}

// todo: I want to bring all of the search & filter queries into one API

pub fn search_db(db_conn: &SqliteConnection, query: &SearchQuery) -> Result<SearchReturn> {
    if (query.search_type.is_none()) {
        return Err(anyhow!("search_type not set"));
    }

    match query.search_type.as_ref().unwrap().as_str() {
        "base" => {
            let search_results;
            if (query.search.is_none()) {
                search_results = None;
            } else {
                let input = query.search.as_ref().unwrap();
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
                    for type_ in crate::import_db::types_generated::TYPES.iter() {
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
                search_results = match values {
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

                        Some(results)
                    }
                    Err(error) => None,
                }
            }

            Err(anyhow!(""))
        }
        "coll" => Err(anyhow!("")),
        _ => Err(anyhow!("")),
    }
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
// "delight cherry plum"
// "delight plum"
// "valor plum"

// todo: if we have an exact match for a type name (or type aka name) then remove that word, use it to suggest that type
// todo - this kind of type search plus a full text search on the collections json files
#[get("/api/search")]
async fn variety_search(
    query: web::Query<SearchQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results = web::block(move || search_db(&conn, &query)).await.unwrap(); // todo - blockingerror unwrap?

    let results = match results {
        Ok(results) => results,
        Err(e) => {
            eprintln!("{}", e);
            return Err(actix_web::error::ErrorInternalServerError(""));
        }
    };

    Ok(HttpResponse::Ok().json(results))
}
