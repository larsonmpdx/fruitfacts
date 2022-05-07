// CRUD user plant lists and list entries

use actix_web::cookie::Cookie;
use actix_web::HttpRequest;
use actix_web::{get, post, web, HttpResponse};

use super::super::schema_generated::*;
use super::super::schema_types::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// todo:
// CRUD user lists
// CRUD list entries (and harvest time suggestions)

// get all user lists (pagination todo I guess)



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

