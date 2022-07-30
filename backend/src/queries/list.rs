// CRUD user plant lists and list entries

use actix_web::HttpRequest;
use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

use crate::session;

use super::super::schema_generated::*;
use super::super::schema_types::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// todo:
// CRUD user lists
// CRUD list entries (and harvest time suggestions)

// see search.rs for get lists

// this is to separate out the id and delete fields so we can get create/update/delete with one function
#[derive(Debug, Deserialize)]
pub struct ControlData {
    pub user_id: Option<i32>,
    pub id: Option<i32>,
    pub collection_id: Option<i32>, // to check if this is set and reject the update
    pub delete: Option<bool>,
}

// insert/update/delete a list based on the control data given
// * user ID: checked against the session's user ID
//   - delete a record: must have ID and delete=true
//   - update a record: id: ID present and delete is missing or false, and record correctly decodes
//   - insert a record: if no id and no delete field, and record correctly decodes
// todo: this is a candidate for a generic function for a few kinds of database types
// just check that user_id matches what we're inserting/updating/deleting and do it
#[post("/api/list")]
async fn create_list(
    req: HttpRequest,
    body: web::Bytes,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let db_conn = pool.get().expect("couldn't get db connection from pool");

    let (session_value, _outgoing_cookie) = crate::queries::auth::get_session_value(req, false);
    if session_value.is_none() {
        return Ok(HttpResponse::InternalServerError().finish());
    }

    let session = session::get_session(&db_conn, session_value.unwrap());
    if session.is_err() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let session = session.unwrap();

    // parse our input looking for "id: [id]" and also our struct without ID
    // if an ID was specified, this is an update. with no ID it's an insert
    let control_data = serde_json::from_str::<ControlData>(std::str::from_utf8(&body).unwrap())?;
    let location_no_id = serde_json::from_str::<LocationNoID>(std::str::from_utf8(&body).unwrap());

    if control_data.user_id.is_none() || (control_data.user_id != Some(session.user_id)) {
        return Ok(HttpResponse::InternalServerError().finish());
    }

    if control_data.collection_id.is_some() {
        // don't allow editing the built-in locations through this api which use collection ID instead of user ID
        // so - just reject any that have collection_id set
        return Ok(HttpResponse::InternalServerError().finish());
    }

    // todo - use sqlite upsert once available in diesel 2.0
    // see https://stackoverflow.com/questions/68614536/how-do-i-upsert-in-sqlite-using-diesel

    let rows_changed;
    if let Some(id) = control_data.id {
        if control_data.delete == Some(true) {
            // given an ID and delete=true: delete
            rows_changed = diesel::delete(locations::dsl::locations.filter(locations::id.eq(id)))
                .execute(&db_conn);

            // todo - delete list items too
            // maybe with a transaction?
        } else {
            // ID provided but not deleting, try an update
            rows_changed = diesel::update(locations::dsl::locations.filter(locations::id.eq(id)))
                .set(&location_no_id?)
                .execute(&db_conn);

            // todo - handle public field, if it changes we need to update the public field on our list items too
            // maybe with a transaction?
        }
    } else {
        // no ID provided, regular insert
        rows_changed = diesel::insert_into(locations::dsl::locations)
            .values(&location_no_id?)
            .execute(&db_conn);
    }

    if rows_changed == Ok(1) {
        return Ok(HttpResponse::Ok().finish());
    } else {
        return Ok(HttpResponse::InternalServerError().finish());
    }

    // todo - add "is public" to lists
    // todo - think about naming list vs. location
    // todo - get user's lists. either our own user ID, or if they're public I guess?
}

// create list:
// login token gets us user ID
// list name
// location (todo)
// returns: list ID or something

// read list:
// returns header plus all list entries

// update list:
// rename, change location

// delete list: based on list ID

// get suggested harvest time based on list (using list location and possibly exiting list entries)

// add plant entry to list
// name, type
// harvest time
// harvest duration
// harvest time reason

// read: not needed, part of the full list read

// edit list entry (based on plant entry ID): all of the same values as "add"

// delete list entry (based on plant entry ID)
