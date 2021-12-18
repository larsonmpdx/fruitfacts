#[macro_use]
extern crate more_asserts;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;
embed_migrations!();

extern crate dotenv;

pub mod git_info;
pub mod import_db;
pub mod queries;
pub mod auth;
mod schema_fts;
mod schema_generated;
mod schema_types;
