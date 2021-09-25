table! {
    base_plants (plant_id) {
        plant_id -> Integer,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        aka -> Nullable<Text>,
        description -> Nullable<Text>,
        patent -> Nullable<Text>,
    }
}

table! {
    collection_items (collection_item_id) {
        collection_item_id -> Integer,
        collection_id -> Integer,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        patent -> Nullable<Text>,
        description -> Nullable<Text>,
        relative_harvest -> Nullable<Text>,
        harvest_start -> Nullable<Integer>,
        harvest_end -> Nullable<Integer>,
    }
}

table! {
    collections (collection_id) {
        collection_id -> Integer,
        user_id -> Integer,
        path -> Nullable<Text>,
        filename -> Nullable<Text>,
        title -> Nullable<Text>,
        author -> Nullable<Text>,
        description -> Nullable<Text>,
        url -> Nullable<Text>,
        published -> Nullable<Text>,
        reviewed -> Nullable<Text>,
        accessed -> Nullable<Text>,
        location -> Nullable<Text>,
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

joinable!(collection_items -> collections (collection_id));

allow_tables_to_appear_in_same_query!(
    base_plants,
    collection_items,
    collections,
    plant_types,
    users,
);
