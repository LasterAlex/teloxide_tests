// @generated automatically by Diesel CLI.

diesel::table! {
    phrases (id) {
        id -> Int4,
        user_id -> Int8,
        emoji -> Text,
        text -> Text,
        bot_text -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        nickname -> Nullable<Text>,
    }
}

diesel::joinable!(phrases -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(phrases, users,);
