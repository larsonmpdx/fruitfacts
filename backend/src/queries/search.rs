use super::super::schema_fts::*;
use super::super::schema_generated::*;
use super::super::schema_types::*;
use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::Write as _; // for write!() macro

use anyhow::{anyhow, Result};

#[derive(Deserialize)]
pub struct SearchQuery {
    #[serde(rename = "searchType")]
    search_type: Option<String>, // base plants or collection items. todo: user items, "search all"
    search: Option<String>, // search string like "PF 11"
    name: Option<String>, // exact plant name (allows getting a single base plant). probably doesn't make sense when used with search
    patents: Option<bool>,
    #[serde(rename = "type")]
    type_: Option<String>, // apple, peach, etc.
    page: Option<String>, // 0-N or "mid" for the patent midpoint page if unknown (so our first patent page link can work)
    #[serde(rename = "perPage")]
    per_page: Option<i32>,
    #[serde(rename = "orderBy")]
    order_by: Option<String>, // sort options: notoriety, search quality, type then name, name then type, patent expiration (special case, also compute the middle patent page), harvest time
    order: Option<String>, // "asc" or "desc". desc if omitted
    #[serde(rename = "relativeHarvestMin")]
    relative_harvest_min: Option<String>,
    #[serde(rename = "relativeHarvestMax")]
    relative_harvest_max: Option<String>,

    // collection items search only (id or path, or I guess both)
    collection_id: Option<String>,
    collection_path: Option<String>,

    // base plants search only:
    #[serde(rename = "notorietyMin")]
    notoriety_min: Option<String>,

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

#[skip_serializing_none]
#[derive(Default, Queryable, Serialize)]
pub struct SearchReturn {
    #[serde(rename = "searchType")]
    pub search_type: Option<String>, // base plants or collection
    pub count: Option<i64>, // total count of search results (if paginated)
    pub page: Option<i64>, // we pass this out in case of a patent midpoint page and for completeness
    #[serde(rename = "patentMidpointPage")]
    pub patent_midpoint_page: Option<i64>, // special case: if we did a patent search, which page has the transition from past to future expirations?
    #[serde(rename = "basePlants")]
    pub base_plants: Option<Vec<BasePlant>>,

    // only if constrained to one collection:
    #[serde(rename = "collectionItems")]
    pub collection_items: Option<Vec<CollectionItem>>,
    pub collection: Option<Collection>,
    pub locations: Option<Vec<Location>>,
}

// todo: I want to bring all of the search & filter queries into one API

pub fn search_db(db_conn: &SqliteConnection, query: &SearchQuery) -> Result<SearchReturn> {
    if query.search_type.is_none() {
        return Err(anyhow!("search_type not set"));
    }

    match query.search_type.as_ref().unwrap().as_str() {
        "base" => {
            // we make three queries because boxed diesel queries can't be copied, so we need to make running copies
            // so we can later use them for counts
            // see https://github.com/diesel-rs/diesel/issues/2277
            let mut base_query = base_plants::table.into_boxed();
            let mut count_query = base_plants::table.into_boxed(); // for patent count
            let mut base_query3 = base_plants::table.into_boxed(); // for overall count

            if query.search.is_some() {
                // todo - support searching with just one or two letters (fts uses "trigraphs" and won't search for these)
                // just do a begins-with search on name or AKA columns

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
                    let _ = write!(statement_string, " OR \"{}\"", statement.join(""));
                }

                println!("input {input} cleaned: {cleaned} ORed: {statement_string}");

                let search_query = fts_base_plants::table
                    .select((fts_base_plants::rowid, fts_base_plants::rank))
                    .filter(fts_base_plants::whole_row.eq(statement_string))
                    .order(fts_base_plants::rank.asc())
                    .into_boxed();

                // todo - if we have restrict_to_type set, apply that (?) - or do we want to pull that logic out the the front end?

                // no limit within search - we may sort later, by something else

                let values = search_query.load::<FtsBasePlants>(db_conn);
                // println!("{:?}", values);

                // todo: order or limit by notoriety
                match values {
                    Ok(values) => {
                        if let Some(type_) = restrict_to_type {
                            base_query = base_query.filter(base_plants::type_.eq(type_.clone()));
                            count_query = count_query.filter(base_plants::type_.eq(type_.clone()));
                            base_query3 = base_query3.filter(base_plants::type_.eq(type_));
                        };

                        let ids_only: Vec<i32> =
                            values.into_iter().map(|entry| entry.rowid).collect();

                        base_query = base_query.filter(base_plants::id.eq_any(ids_only.clone()));
                        count_query = count_query.filter(base_plants::id.eq_any(ids_only.clone()));
                        base_query3 = base_query3.filter(base_plants::id.eq_any(ids_only));
                    }
                    Err(_error) => {
                        return Err(anyhow!("some kind of search problem (todo)"));
                    }
                }
            }

            if let Some(name) = &query.name {
                base_query = base_query.filter(base_plants::name.eq(name));
                count_query = count_query.filter(base_plants::name.eq(name));
                base_query3 = base_query3.filter(base_plants::name.eq(name));
            }

            // patents: Option<bool>
            if let Some(patents) = &query.patents {
                if *patents {
                    base_query = base_query.filter(base_plants::uspp_expiration.is_not_null());
                    count_query = count_query.filter(base_plants::uspp_expiration.is_not_null());
                    base_query3 = base_query3.filter(base_plants::uspp_expiration.is_not_null());
                } else {
                    // finding items without patents is probably not useful but whatever
                    base_query = base_query.filter(base_plants::uspp_expiration.is_null());
                    count_query = count_query.filter(base_plants::uspp_expiration.is_null());
                    base_query3 = base_query3.filter(base_plants::uspp_expiration.is_null());
                }
            }

            if let Some(type_) = &query.type_ {
                base_query = base_query.filter(base_plants::type_.eq(type_));
                count_query = count_query.filter(base_plants::type_.eq(type_));
                base_query3 = base_query3.filter(base_plants::type_.eq(type_));
            }

            if let Some(sort) = &query.order_by {
                let order_asc;
                if let Some(order) = &query.order {
                    match order.as_str() {
                        "asc" => order_asc = true,
                        "desc" => order_asc = false,
                        _ => return Err(anyhow!("unknown order \"{order}\"")),
                    }
                } else {
                    order_asc = false; // default descending, highest notoriety first. todo: probably different defaults for each sort type? like letter A first
                }

                match sort.as_str() {
                    "notoriety" => {
                        if order_asc {
                            base_query = base_query.order(base_plants::notoriety_score.asc());
                            count_query = count_query.order(base_plants::notoriety_score.asc());
                            base_query3 = base_query3.order(base_plants::notoriety_score.asc());
                        } else {
                            base_query = base_query.order(base_plants::notoriety_score.desc());
                            count_query = count_query.order(base_plants::notoriety_score.desc());
                            base_query3 = base_query3.order(base_plants::notoriety_score.desc());
                        }
                    }
                    "type_then_name" => {
                        if order_asc {
                            base_query = base_query.order(base_plants::type_.asc());
                            count_query = count_query.order(base_plants::type_.asc());
                            base_query3 = base_query3.order(base_plants::type_.asc());

                            base_query = base_query.then_order_by(base_plants::name.asc());
                            count_query = count_query.then_order_by(base_plants::name.asc());
                            base_query3 = base_query3.then_order_by(base_plants::name.asc());
                        } else {
                            base_query = base_query.order(base_plants::type_.desc());
                            count_query = count_query.order(base_plants::type_.desc());
                            base_query3 = base_query3.order(base_plants::type_.desc());

                            // note - we keep ascending order on the 2nd order by thing. that makes more sense to me
                            base_query = base_query.then_order_by(base_plants::name.asc());
                            count_query = count_query.then_order_by(base_plants::name.asc());
                            base_query3 = base_query3.then_order_by(base_plants::name.asc());
                        }
                    }
                    "name_then_type" => {
                        if order_asc {
                            base_query = base_query.order(base_plants::name.asc());
                            count_query = count_query.order(base_plants::name.asc());
                            base_query3 = base_query3.order(base_plants::name.asc());

                            base_query = base_query.then_order_by(base_plants::type_.asc());
                            count_query = count_query.then_order_by(base_plants::type_.asc());
                            base_query3 = base_query3.then_order_by(base_plants::type_.asc());
                        } else {
                            base_query = base_query.order(base_plants::name.desc());
                            count_query = count_query.order(base_plants::name.desc());
                            base_query3 = base_query3.order(base_plants::name.desc());

                            // note - we keep ascending order on the 2nd order by thing. that makes more sense to me
                            base_query = base_query.then_order_by(base_plants::type_.asc());
                            count_query = count_query.then_order_by(base_plants::type_.asc());
                            base_query3 = base_query3.then_order_by(base_plants::type_.asc());
                        }
                    }
                    "patent_expiration" => {
                        if order_asc {
                            base_query = base_query.order(base_plants::uspp_expiration.asc());
                            count_query = count_query.order(base_plants::uspp_expiration.asc());
                            base_query3 = base_query3.order(base_plants::uspp_expiration.asc());
                        } else {
                            base_query = base_query.order(base_plants::uspp_expiration.desc());
                            count_query = count_query.order(base_plants::uspp_expiration.desc());
                            base_query3 = base_query3.order(base_plants::uspp_expiration.desc());
                        }
                    }
                    "harvest_time" => {
                        if order_asc {
                            base_query = base_query.order(base_plants::harvest_relative.asc());
                            count_query = count_query.order(base_plants::harvest_relative.asc());
                            base_query3 = base_query3.order(base_plants::harvest_relative.asc());
                        } else {
                            base_query = base_query.order(base_plants::harvest_relative.desc());
                            count_query = count_query.order(base_plants::harvest_relative.desc());
                            base_query3 = base_query3.order(base_plants::harvest_relative.desc());
                        }
                    }
                    "search_quality" => {
                        // todo - this needs some fancy handling to carry through sort order in the query
                        // or otherwise sort the results after they're returned
                        return Err(anyhow!("search_quality sort type not implemented (todo)"));
                    }
                    _ => return Err(anyhow!("unknown sort type \"{sort}\"")),
                }
            }

            // if we're sorting by patent expiration, figure out the patent midpoint page (will only exist if we have a per_page)
            // so we can show a link to it, or provide it directly if requested as "page=mid"

            let mut patent_midpoint_page = None;
            if let Some(sort) = &query.order_by {
                if sort == "patent_expiration" {
                    if let Some(per_page) = &query.per_page {
                        // special case - might also need to check that we're sorting by expiration

                        let now = chrono::Utc::now().timestamp(); // todo - make this a parameter?
                        count_query = count_query.filter(base_plants::uspp_expiration.lt(now));
                        let prior_patent_count_result = count_query.count().first::<i64>(db_conn);

                        if let Err(error) = prior_patent_count_result {
                            return Err(error.into());
                        }
                        let prior_patent_count = prior_patent_count_result.unwrap();

                        // if prior patents count is 50 and we have 50 per page, we want to show page 2 (1-referenced)
                        // if it's 51, show page 2
                        // if 49, show page 1
                        patent_midpoint_page = Some(1 + prior_patent_count / *per_page as i64);
                    }
                }
            }

            let mut page_output = None;
            if let Some(page) = &query.page {
                if let Some(per_page) = &query.per_page {
                    if page == "mid" {
                        if patent_midpoint_page.is_none() {
                            return Err(anyhow!("requested patent midpoint page but we didn't find one (maybe not sorting by patent_expiration?)"));
                        }

                        page_output = patent_midpoint_page;
                        base_query =
                            base_query.offset(patent_midpoint_page.unwrap() * *per_page as i64);
                    } else {
                        let page_i32_result = page.parse::<i32>();
                        if page_i32_result.is_err() {
                            return Err(anyhow!("\"page\" wasn't \"mid\" or an integer"));
                        }
                        let page_i32 = page_i32_result.unwrap();
                        page_output = Some(page_i32.into());

                        base_query = base_query.offset((page_i32 * per_page).into());
                    }
                } else {
                    return Err(anyhow!("got \"page\" without \"per_page\""));
                }
            }

            // todo relative_harvest (min and max)
            if let Some(relative_harvest_min) = &query.relative_harvest_min {
                // todo
            }
            if let Some(relative_harvest_max) = &query.relative_harvest_max {
                // todo
            }

            // todo collection (path or ID) - collection items only
            if let Some(collection_id) = &query.collection_id {
                // todo - limit to this collection ID (different search type though)
            }
            if let Some(collection_path) = &query.collection_path {
                // todo
            }

            // todo notoriety_min (base plants only)
            if let Some(notoriety_min) = &query.notoriety_min {
                // todo
            }

            // todo distance, from (collection items only) - there may be value to a search filter for "mentioned within x miles of zip code"

            let overall_count_result = base_query3.count().first::<i64>(db_conn);
            if let Err(error) = overall_count_result {
                return Err(error.into());
            }
            let overall_count = overall_count_result.unwrap();

            // todo - etc. - and we can have a path to omit this and get it from the items count if we have no limit, I guess

            // todo midpoint if getting patents, and sorted by expiration date

            // per_page can be used as a limit if page isn't specified
            if let Some(per_page) = &query.per_page {
                base_query = base_query.limit(*per_page as i64);
            }

            let base_plants: Result<Vec<BasePlant>, diesel::result::Error> =
                base_query.load(db_conn);

            match base_plants {
                Ok(base_plants) => Ok(SearchReturn {
                    base_plants: Some(base_plants),
                    count: Some(overall_count),
                    page: page_output,
                    patent_midpoint_page,
                    ..Default::default()
                }),
                Err(error) => Err(error.into()),
            }
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
