#[macro_use]
extern crate more_asserts;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;
pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations = embed_migrations!();

pub mod gazetteer_load;
pub mod git_info;
pub mod import_db;
pub mod queries;
mod schema_fts;
mod schema_generated;
mod schema_types;
pub mod session;

#[cfg(test)]
mod test;
