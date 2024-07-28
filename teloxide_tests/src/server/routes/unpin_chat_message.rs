use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;

use crate::server::routes::make_telegram_result;
use crate::server::{MESSAGES, RESPONSES};

use super::{check_if_message_exists, BodyChatId};

#[derive(Debug, Deserialize, Clone)]
pub struct UnpinChatMessageBody {
    pub chat_id: BodyChatId,
    pub message_id: Option<i32>,
}

pub async fn unpin_chat_message(body: web::Json<UnpinChatMessageBody>) -> impl Responder {
    if let Some(message_id) = body.message_id {
        check_if_message_exists!(message_id);
    }
    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock
        .unpinned_chat_messages
        .push(body.into_inner());

    make_telegram_result(true)
}
