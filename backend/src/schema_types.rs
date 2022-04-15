use super::schema_generated::{
    collection_items, collections, locations, user_oauth_entries, user_sessions,
};
use serde::Serialize;
use serde_with::skip_serializing_none;

// diesel requires us to maintain two copies of everything:
// * the database schema in "up.sql" which becomes schema_generated.rs, generated by diesel-cli
// * plus this file with rust structs, made by hand to match
//   * there's a 3rd copy, in the json struct in import_db.rs, that won't match exactly because we translate or fill in some things to make the json form simpler

// foreign key stuff: https://docs.diesel.rs/diesel/associations/index.html

#[derive(Queryable)]
pub struct PlantType {
    pub plant_type_id: i32,
    pub group_name: String,
    pub name: String,
    pub latin_name: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Queryable)]
pub struct BasePlant {
    pub id: i32,
    pub name: String,
    pub name_fts: String,
    #[serde(rename = "type")]
    pub type_: String,

    pub notoriety_score: Option<f32>,
    pub notoriety_score_explanation: Option<String>,
    pub number_of_references: i32,
    pub notoriety_highest_collection_score: Option<f32>,
    pub notoriety_highest_collection_score_id: Option<i32>,

    pub aka: Option<String>,
    pub aka_fts: Option<String>,
    pub marketing_name: Option<String>,
    pub description: Option<String>,
    pub uspp_number: Option<String>,
    pub uspp_expiration: Option<i64>,
    pub uspp_expiration_estimated: Option<i32>,
    pub release_year: Option<i32>,
    pub release_year_note: Option<String>,
    pub released_by: Option<String>,
    pub release_collection_id: Option<i32>,

    pub ignore_unless_in_others: i32,
    pub s_allele: Option<String>,

    pub harvest_relative: Option<i32>,
    pub harvest_relative_to: Option<String>,
    pub harvest_relative_to_type: Option<String>,
    pub harvest_relative_explanation: Option<String>,
}

#[skip_serializing_none]
#[derive(Identifiable, Serialize, Queryable, Associations, Debug)]
#[belongs_to(Collection)]
pub struct CollectionItem {
    pub id: i32,

    pub collection_id: i32,
    pub location_id: Option<i32>,
    pub location_number: i32,

    pub path_and_filename: Option<String>,
    pub marketing_name: Option<String>,

    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,

    pub category: Option<String>,
    pub category_descripton: Option<String>,

    pub disease_resistance: Option<String>,
    pub chill: Option<String>,
    pub s_allele: Option<String>,

    pub description: Option<String>,
    pub harvest_text: Option<String>,
    pub harvest_relative: Option<String>,
    pub harvest_start: Option<i32>,
    pub harvest_end: Option<i32>,

    pub calc_harvest_relative: Option<i32>,
    pub calc_harvest_relative_to: Option<String>,
    pub calc_harvest_relative_to_type: Option<String>,
    pub calc_harvest_relative_round: Option<f64>,
    pub calc_harvest_relative_explanation: Option<String>,

    pub harvest_start_2: Option<i32>,
    pub harvest_end_2: Option<i32>,
}

#[skip_serializing_none]
#[derive(Identifiable, Serialize, Queryable)]
pub struct Collection {
    pub id: i32,
    pub user_id: i32,
    pub git_edit_time: Option<i64>,

    pub path: String,
    pub filename: String,

    pub notoriety_type: String,
    pub notoriety_score: f32,
    pub notoriety_score_explanation: String,
    pub harvest_time_devalue_factor: Option<f32>,

    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub published: Option<String>,
    pub reviewed: Option<String>,
    pub accessed: Option<String>,
    pub needs_help: i32,
}

#[skip_serializing_none]
#[derive(Debug, Identifiable, Serialize, Queryable, Associations)]
#[belongs_to(Collection)]
pub struct Location {
    pub id: i32,
    pub location_number: i32, // which location within the collection is this? 0 is no location, 1 is the first location
    pub collection_id: i32,

    pub location_name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,

    pub notoriety_score: f32,
    pub collection_path: Option<String>,
    pub collection_filename: Option<String>,
    pub collection_title: Option<String>,
}

#[derive(Queryable, Associations)]
#[belongs_to(User)]
#[table_name = "user_oauth_entries"]
pub struct UserOauthEntry {
    pub id: i32,
    pub user_id: i32,
    pub unique_id: String,
    pub oauth_info: Option<String>,
}

#[derive(Clone, Queryable, Insertable, Associations)]
#[table_name = "user_sessions"]
#[belongs_to(User)]
pub struct UserSession {
    pub id: i32,
    pub user_id: i32,
    pub session_value: String,
    pub created: i64,
}

// same but without id field
#[derive(Clone)]
pub struct UserSessionToInsert {
    pub user_id: i32,
    pub session_value: String,
    pub created: i64,
}

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub location_name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Queryable)]
pub struct FtsBasePlants {
    pub rowid: i32,
    pub rank: f32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Fact {
    pub id: i32,
    pub contributor: String,
    pub fact: String,
    pub reference: String,
}
