// diesel can't auto generate fts tables
// see https://github.com/diesel-rs/diesel/issues/1748

table! {
    fts_base_plants(rowid) {
        rowid -> Integer,

        #[sql_name = "fts_base_plants"]
        whole_row -> Text,
        rank -> Float,
    }
}
