use std::sync::Mutex;

use actix_web::{web, Responder};
use serde::Deserialize;

use crate::{server::routes::make_telegram_result, state::State};

use super::BodyChatId;

#[derive(Debug, Deserialize, Clone)]
pub struct UnbanChatMemberBody {
    pub chat_id: BodyChatId,
    pub user_id: u64,
    pub only_if_banned: Option<bool>,
}

pub async fn unban_chat_member(
    state: web::Data<Mutex<State>>,
    body: web::Json<UnbanChatMemberBody>,
) -> impl Responder {
    // Idk what to verify here
    let mut lock = state.lock().unwrap();
    lock.responses.unbanned_chat_members.push(body.into_inner());

    make_telegram_result(true)
}
