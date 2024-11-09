use std::sync::Mutex;

use actix_web::{web, Responder};
use serde::Deserialize;

use crate::mock_bot::State;
use crate::server::routes::make_telegram_result;

use super::BodyChatId;

#[derive(Debug, Deserialize, Clone)]
pub struct BanChatMemberBody {
    pub chat_id: BodyChatId,
    pub user_id: u64,
    pub until_date: Option<i64>,
    pub revoke_messages: Option<bool>,
}

pub async fn ban_chat_member(
    state: web::Data<Mutex<State>>,
    body: web::Json<BanChatMemberBody>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();
    let chat_id = body.chat_id.id();
    if body.revoke_messages.is_some() && body.revoke_messages.unwrap() {
        for message in lock.messages.messages.clone() {
            if message.chat.id.0 == chat_id
                && message.from.is_some()
                && message.from.unwrap().id.0 == body.user_id
            {
                lock.messages.delete_message(message.id.0);
            }
        }
    }
    lock.responses.banned_chat_members.push(body.into_inner());

    make_telegram_result(true)
}
