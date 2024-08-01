use actix_web::{web, Responder};
use serde::Deserialize;

use crate::server::routes::make_telegram_result;
use crate::server::RESPONSES;

use super::BodyChatId;

#[derive(Debug, Deserialize, Clone)]
pub struct SendChatActionBody {
    pub chat_id: BodyChatId,
    pub message_thread_id: Option<i64>,
    pub action: String,
}

pub async fn send_chat_action(body: web::Json<SendChatActionBody>) -> impl Responder {
    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.sent_chat_actions.push(body.into_inner());

    make_telegram_result(true)
}
