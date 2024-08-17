// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        published -> Bool,
        body -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        hair_color -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
