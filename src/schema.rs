// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    todo_item (id) {
        id -> Int4,
        list_id -> Nullable<Int4>,
        title -> Nullable<Text>,
        checked -> Nullable<Bool>,
    }
}

diesel::table! {
    todo_list (id) {
        id -> Int4,
        title -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    todo_item,
    todo_list,
    users,
);
