use actix_web::{web, Responder};
use serde::Deserialize;

use crate::server::routes::make_telegram_result;
use crate::server::{MESSAGES, RESPONSES};

use super::BodyChatId;

#[derive(Debug, Deserialize, Clone)]
pub struct BanChatMemberBody {
    pub chat_id: BodyChatId,
    pub user_id: u64,
    pub until_date: Option<i64>,
    pub revoke_messages: Option<bool>,
}

pub async fn ban_chat_member(body: web::Json<BanChatMemberBody>) -> impl Responder {
    let chat_id = body.chat_id.id();
    let messages = MESSAGES.lock().unwrap().clone();
    if body.revoke_messages.is_some() && body.revoke_messages.unwrap() {
        for message in messages {
            if message.chat.id.0 == chat_id
                && message.from().is_some()
                && message.from().unwrap().id.0 == body.user_id
            {
                MESSAGES.delete_message(message.id.0);
            }
        }
    }
    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.banned_chat_members.push(body.into_inner());

    make_telegram_result(true)
}
