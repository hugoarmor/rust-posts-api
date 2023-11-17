// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        token -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        author_id -> Nullable<Int4>,
    }
}

diesel::joinable!(posts -> authors (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    posts,
);
