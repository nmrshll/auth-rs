table! {
    posts (id) {
        id -> Int8,
        created_at -> Timestamp,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        author_id -> Int8,
    }
}

table! {
    users (id) {
        id -> Int8,
        created_at -> Timestamp,
        email -> Varchar,
        hash_pass -> Varchar,
    }
}

joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
