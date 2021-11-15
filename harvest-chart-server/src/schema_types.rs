use serde::Serialize;

// diesel requires us to maintain two copies of everything:
// * the database schema in "up.sql" which becomes schema_generated.rs, generated by diesel-cli
// * plus this file with rust structs, made by hand to match
//   * there's a 3rd copy, in the json struct in import_db.rs, that won't match exactly because we translate or fill in some things to make the json form simpler

#[derive(Queryable)]
pub struct PlantType {
    pub plant_type_id: i32,
    pub name: String,
    pub latin_name: Option<String>,
}

#[derive(Debug, Queryable)]
pub struct BasePlant {
    pub plant_id: i32,
    pub name: String,
    pub name_fts: String,
    pub type_: String,
    pub aka: Option<String>,
    pub aka_fts: Option<String>,
    pub marketing_name: Option<String>,
    pub description: Option<String>,
    pub uspp_number: Option<String>,
    pub uspp_expiration: Option<i64>,
    pub release_year: Option<i32>,
    pub release_year_note: Option<String>,
    pub released_by: Option<String>,
    pub release_collection_id: Option<i32>,
}

#[derive(Queryable)]
pub struct CollectionItems {
    pub collection_item_id: i32,

    pub collection_title: Option<String>,
    pub collection_id: i32,
    pub location_id: Option<i32>,

    pub name: String,
    pub type_: String,

    pub category: Option<String>,
    pub category_descripton: Option<String>,

    pub disease_resistance: Option<String>,
    pub chill: Option<String>,

    pub description: Option<String>,
    pub harvest_text: Option<String>,
    pub harvest_relative: Option<String>,
    pub harvest_start: Option<i32>,
    pub harvest_end: Option<i32>,
    pub harvest_start_is_midpoint: Option<i32>,

    pub harvest_start_2: Option<i32>,
    pub harvest_end_2: Option<i32>,
    pub harvest_start_2_is_midpoint: Option<i32>,
}

#[derive(Serialize, Queryable)]
pub struct Collections {
    pub location_id: i32,
    pub collection_id: i32,
    pub user_id: i32,

    pub path: Option<String>,
    pub filename: Option<String>,

    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub published: Option<String>,
    pub reviewed: Option<String>,
    pub accessed: Option<String>,

    pub location_name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Queryable)]
pub struct Users {
    pub user_id: i32,
    pub name: String,
}
