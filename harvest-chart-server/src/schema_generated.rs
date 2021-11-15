table! {
    base_plants (plant_id) {
        plant_id -> Integer,
        name -> Text,
        name_fts -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        aka -> Nullable<Text>,
        aka_fts -> Nullable<Text>,
        marketing_name -> Nullable<Text>,
        description -> Nullable<Text>,
        uspp_number -> Nullable<Text>,
        uspp_expiration -> Nullable<BigInt>,
        release_year -> Nullable<Integer>,
        release_year_note -> Nullable<Text>,
        released_by -> Nullable<Text>,
        release_collection_id -> Nullable<Integer>,
    }
}

table! {
    collection_items (collection_item_id) {
        collection_item_id -> Integer,
        collection_title -> Nullable<Text>,
        collection_id -> Integer,
        location_id -> Nullable<Integer>,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        category -> Nullable<Text>,
        category_description -> Nullable<Text>,
        disease_resistance -> Nullable<Text>,
        chill -> Nullable<Text>,
        description -> Nullable<Text>,
        harvest_text -> Nullable<Text>,
        harvest_relative -> Nullable<Text>,
        harvest_start -> Nullable<Integer>,
        harvest_end -> Nullable<Integer>,
        harvest_start_is_midpoint -> Nullable<Integer>,
        harvest_start_2 -> Nullable<Integer>,
        harvest_end_2 -> Nullable<Integer>,
        harvest_start_2_is_midpoint -> Nullable<Integer>,
    }
}

table! {
    collections (location_id) {
        location_id -> Integer,
        collection_id -> Integer,
        user_id -> Integer,
        git_edit_time -> Nullable<BigInt>,
        path -> Nullable<Text>,
        filename -> Nullable<Text>,
        title -> Nullable<Text>,
        author -> Nullable<Text>,
        description -> Nullable<Text>,
        url -> Nullable<Text>,
        published -> Nullable<Text>,
        reviewed -> Nullable<Text>,
        accessed -> Nullable<Text>,
        location_name -> Nullable<Text>,
        latitude -> Nullable<Double>,
        longitude -> Nullable<Double>,
    }
}

table! {
    plant_types (plant_type_id) {
        plant_type_id -> Integer,
        name -> Text,
        latin_name -> Nullable<Text>,
    }
}

table! {
    users (user_id) {
        user_id -> Integer,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    base_plants,
    collection_items,
    collections,
    plant_types,
    users,
);
