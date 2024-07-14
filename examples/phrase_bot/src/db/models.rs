use super::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: i64,
    pub nickname: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub id: i64,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::phrases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, PartialEq)]
pub struct Phrase {
    pub id: i32,
    pub user_id: i64,
    pub emoji: String,
    pub text: String,
    pub bot_text: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::phrases)]
pub struct NewPhrase {
    pub user_id: i64,
    pub emoji: String,
    pub text: String,
    pub bot_text: String,
}
