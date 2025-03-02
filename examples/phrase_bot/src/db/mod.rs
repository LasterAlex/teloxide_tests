//! A sepatate file for simplicity. This file has all the database related functions
//! that are used in the bot.
pub mod models;
pub mod schema;
use diesel::prelude::*;
use models::*;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(id: i64) -> Result<User, diesel::result::Error> {
    let conn = &mut establish_connection();

    let new_user = NewUser { id };

    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
}

pub fn delete_user(id: i64) -> Result<usize, diesel::result::Error> {
    let conn = &mut establish_connection();

    diesel::delete(schema::users::table.find(id)).execute(conn)
}

pub fn full_user_redeletion(id: i64, nickname: Option<String>) {
    // For tests, to fully reset the user
    let _ = delete_user(id).unwrap();

    create_user(id).unwrap();
    if let Some(nickname) = nickname {
        change_user_nickname(id, nickname).unwrap();
    }
}

pub fn change_user_nickname(id: i64, nickname: String) -> Result<User, diesel::result::Error> {
    let conn = &mut establish_connection();

    diesel::update(schema::users::table.find(id))
        .set(schema::users::nickname.eq(nickname))
        .get_result(conn)
}

pub fn get_user(id: i64) -> Result<User, diesel::result::Error> {
    let conn = &mut establish_connection();

    schema::users::table.find(id).first(conn)
}

pub fn get_user_phrases(id: i64) -> Result<Vec<Phrase>, diesel::result::Error> {
    let conn = &mut establish_connection();

    schema::phrases::table
        .filter(schema::phrases::user_id.eq(id))
        .load(conn)
}

pub fn create_phrase(
    user_id: i64,
    emoji: String,
    text: String,
    bot_text: String,
) -> Result<Phrase, diesel::result::Error> {
    let conn = &mut establish_connection();

    let new_phrase = NewPhrase {
        user_id,
        emoji,
        text,
        bot_text,
    };

    diesel::insert_into(schema::phrases::table)
        .values(&new_phrase)
        .returning(Phrase::as_returning())
        .get_result(conn)
}

pub fn delete_phrase(id: i32) -> Result<usize, diesel::result::Error> {
    let conn = &mut establish_connection();

    diesel::delete(schema::phrases::table.find(id)).execute(conn)
}
