// CRUD user plant lists and list entries

use actix_web::HttpRequest;
use actix_web::{post, web, HttpResponse};

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

#[post("/api/list")]
async fn create_list(
    req: HttpRequest,
    list: web::Json<Location>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let (session_value, _outgoing_cookie) = crate::queries::auth::get_session_value(req, false);
    if session_value.is_none() {
        return Ok(HttpResponse::InternalServerError().finish());
    }

    let db_conn = pool.get().expect("couldn't get db connection from pool");

    let session = session::get_session(&db_conn, session_value.unwrap());
    if session.is_err() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let session = session.unwrap();

    // todo - create list with this user ID. check user ID against session's user ID
    // todo - add "is public" to lists
    // todo - think about naming list vs. location
    // todo - get user's lists. either our own user ID, or if they're public I guess?

    Ok(HttpResponse::Ok().json(""))
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
