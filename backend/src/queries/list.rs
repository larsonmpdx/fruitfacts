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
    pub user: Option<String>, // id if formatted like "id:123", name otherwise. becomse location.user_id
    pub location: Option<String>, // id if formatted like "id:123", name otherwise. becomes location.id
    pub collection_item_id: Option<i32>, // for collection item edit/delete
    pub collection_id: Option<i32>, // to check if this is set and reject the update
    pub delete: Option<bool>,
}

// insert/update/delete a list based on the control data given
// * user ID: checked against the session's user ID
//   - delete a record: must have ID and delete=true. user ID must match existing list's user ID
//   - update a record: id: ID present and delete is missing or false, and record correctly decodes. user ID must match existing list's user ID
//   - insert a record: if no id and no delete field, and record correctly decodes
// todo: this is a candidate for a generic function for a few kinds of database types
// just check that user id matches what we're inserting/updating/deleting and do it
#[post("/api/list")]
async fn create_list(
    req: HttpRequest,
    body: web::Bytes,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut db_conn = pool.get().expect("couldn't get db connection from pool");

    let (session_value, _outgoing_cookie) = crate::queries::auth::get_session_value(req, false);
    if session_value.is_none() {
        return Ok(HttpResponse::InternalServerError().finish());
    }

    let session = session::get_session(&mut db_conn, session_value.unwrap());
    if session.is_err() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let session = session.unwrap();

    // parse our input looking for "id: [id]" and also our struct without ID
    // if an ID was specified, this is an update. with no ID it's an insert
    let control_data = serde_json::from_str::<ControlData>(std::str::from_utf8(&body).unwrap())?;

    let control_data_user_id =
        crate::queries::search::get_user_id(&control_data.user, &mut db_conn);

    if control_data_user_id.is_err() {
        return Ok(HttpResponse::InternalServerError().body("user ID problem"));
    }
    let control_data_user_id = control_data_user_id.unwrap();

    if control_data_user_id != session.user_id {
        return Ok(HttpResponse::InternalServerError().body("user ID mismatch"));
    }

    if control_data.collection_id.is_some() {
        // don't allow editing the built-in locations through this api which use collection ID instead of user ID
        // so - just reject any that have collection_id set
        return Ok(HttpResponse::InternalServerError().body("can't edit built-ins"));
    }

    // todo - use sqlite upsert once available in diesel 2.0
    // see https://stackoverflow.com/questions/68614536/how-do-i-upsert-in-sqlite-using-diesel

    // overwrite the lat/lon fields with our incoming text version

    let rows_changed;
    if control_data.location.is_some() {
        let control_data_location_id =
            crate::queries::search::get_location_id(&control_data.location, &mut db_conn);

        if control_data_location_id.is_err() {
            return Ok(HttpResponse::InternalServerError().body("location ID problem"));
        }
        let control_data_location_id = control_data_location_id.unwrap();

        if control_data.delete == Some(true) {
            // given an ID and delete=true: delete
            rows_changed = diesel::delete(
                locations::dsl::locations
                    .filter(locations::id.eq(control_data_location_id))
                    .filter(locations::user_id.eq(session.user_id)),
            )
            .execute(&mut db_conn);

            // todo - delete list items too
            // maybe with a transaction?
        } else {
            // ID provided but not deleting, try an update
            let mut location_no_id =
                serde_json::from_str::<LocationNoID>(std::str::from_utf8(&body).unwrap())?;
            location_no_id.user_id = Some(control_data_user_id);

            rows_changed = diesel::update(
                locations::dsl::locations
                    .filter(locations::id.eq(control_data_location_id))
                    .filter(locations::user_id.eq(session.user_id)),
            )
            .set(&location_no_id)
            .execute(&mut db_conn);

            // todo - handle public field, if it changes we need to update the public field on our list items too
            // maybe with a transaction?
        }
    } else {
        // no ID provided, regular insert
        let mut location_no_id =
            serde_json::from_str::<LocationNoID>(std::str::from_utf8(&body).unwrap())?;
        location_no_id.user_id = Some(control_data_user_id);

        rows_changed = diesel::insert_into(locations::dsl::locations)
            .values(&location_no_id)
            .execute(&mut db_conn);
        println!("tried adding user list"); // todo remove
    }

    if rows_changed == Ok(1) {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }

    // todo - add "is public" to lists
    // todo - think about naming list vs. location
    // todo - get user's lists. either our own user ID, or if they're public I guess?
}

// add a plant to a list
#[post("/api/list/entry")]
async fn add_plant_to_list(
    req: HttpRequest,
    body: web::Bytes,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut db_conn = pool.get().expect("couldn't get db connection from pool");

    let (session_value, _outgoing_cookie) = crate::queries::auth::get_session_value(req, false);
    if session_value.is_none() {
        return Ok(HttpResponse::InternalServerError().finish());
    }

    let session = session::get_session(&mut db_conn, session_value.unwrap());
    if session.is_err() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let session = session.unwrap();

    // parse our input looking for "id: [id]" and also our struct without ID
    // if an ID was specified, this is an update. with no ID it's an insert
    let control_data = serde_json::from_str::<ControlData>(std::str::from_utf8(&body).unwrap())?;

    let control_data_user_id =
        crate::queries::search::get_user_id(&control_data.user, &mut db_conn);

    if control_data_user_id.is_err() {
        return Ok(HttpResponse::InternalServerError().body("user ID problem"));
    }
    let control_data_user_id = control_data_user_id.unwrap();

    if control_data_user_id != session.user_id {
        return Ok(HttpResponse::InternalServerError().body("user ID mismatch"));
    }

    if control_data.collection_id.is_some() {
        // don't allow editing the built-in locations through this api which use collection ID instead of user ID
        // so - just reject any that have collection_id set
        return Ok(HttpResponse::InternalServerError().body("can't edit built-ins"));
    }

    // step 1 - look up the location, it must exist and match this user. its public/not public setting will be used for the list item
    if control_data.location.is_none() {
        return Ok(HttpResponse::InternalServerError().body("location required"));
    }
    
    let control_data_location_id =
        crate::queries::search::get_location_id(&control_data.location, &mut db_conn);

    if control_data_location_id.is_err() {
        return Ok(HttpResponse::InternalServerError().body("location ID problem"));
    }
    let control_data_location_id = control_data_location_id.unwrap();

    let db_location = locations::dsl::locations
        .filter(locations::id.eq(control_data_location_id))
        .filter(locations::user_id.eq(session.user_id))
        .order(locations::id.desc())
        .first::<Location>(&mut db_conn);

    if db_location.is_err() {
        return Ok(HttpResponse::InternalServerError().body("location not found"));
    }

    let db_location = db_location.unwrap();

    // have a location, now we can use it for the "public" field

    let rows_changed;
    if let Some(control_data_collection_item_id) = control_data.collection_item_id {
        if control_data.delete == Some(true) {
            // given an ID and delete=true: delete
            rows_changed = diesel::delete(
                collection_items::dsl::collection_items
                    .filter(collection_items::id.eq(control_data_collection_item_id))
                    .filter(collection_items::user_id.eq(session.user_id)),
            )
            .execute(&mut db_conn);
        } else {
            // ID provided but not deleting, try an update
            let mut collection_item_no_id =
                serde_json::from_str::<CollectionItemNoID>(std::str::from_utf8(&body).unwrap())?;

                // force these fields to be the values we've already sanitized
                collection_item_no_id.user_id = Some(control_data_user_id);
                collection_item_no_id.location_id = Some(control_data_location_id);
                collection_item_no_id.collection_id = None;
                collection_item_no_id.public = db_location.public;

            rows_changed = diesel::update(
                collection_items::dsl::collection_items
                    .filter(collection_items::id.eq(control_data_collection_item_id))
                    .filter(collection_items::user_id.eq(session.user_id)),
            )
            .set(&collection_item_no_id)
            .execute(&mut db_conn);
        }
    } else {
        // no ID provided, regular insert
        let mut collection_item_no_id =
            serde_json::from_str::<CollectionItemNoID>(std::str::from_utf8(&body).unwrap())?;

            // force these fields to be the values we've already sanitized
            collection_item_no_id.user_id = Some(control_data_user_id);
            collection_item_no_id.location_id = Some(control_data_location_id);
            collection_item_no_id.collection_id = None;
            collection_item_no_id.public = db_location.public;

        rows_changed = diesel::insert_into(collection_items::dsl::collection_items)
            .values(&collection_item_no_id)
            .execute(&mut db_conn);
        println!("tried adding user collection item"); // todo remove
    }

    if rows_changed == Ok(1) {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}


// todo notes - some or all of this is done

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