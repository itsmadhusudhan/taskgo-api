use diesel::{allow_tables_to_appear_in_same_query, insertable, joinable, table};

table! {
    collections (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    tasks (id) {
        id -> Uuid,
        name -> Text,
        created_at -> Timestamp,
        collection_id -> Uuid,
    }
}

joinable!(tasks -> collections (collection_id));

allow_tables_to_appear_in_same_query!(collections, tasks,);
