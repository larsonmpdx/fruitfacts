table! {
    base_plants (id) {
        id -> Nullable<Integer>,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        description -> Nullable<Text>,
        patent -> Nullable<Text>,
        relative_harvest -> Nullable<Text>,
        harvest_start -> Nullable<Integer>,
        harvest_end -> Nullable<Integer>,
        harvest_time_reference -> Nullable<Text>,
    }
}

table! {
    collection_items (id) {
        id -> Nullable<Integer>,
        collection_id -> Text,
        notes -> Nullable<Text>,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        description -> Nullable<Text>,
        patent -> Nullable<Text>,
        relative_harvest -> Nullable<Text>,
        harvest_start -> Nullable<Integer>,
        harvest_end -> Nullable<Integer>,
        harvest_time_reference -> Nullable<Text>,
    }
}

table! {
    collections (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        name -> Text,
    }
}

table! {
    plant_types (id) {
        id -> Nullable<Integer>,
        name -> Text,
        latin_name -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
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