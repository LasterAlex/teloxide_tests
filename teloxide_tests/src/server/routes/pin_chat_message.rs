use std::sync::Mutex;

use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;

use crate::mock_bot::State;
use crate::server::routes::make_telegram_result;
use crate::server::MESSAGES;

use super::{check_if_message_exists, BodyChatId};

#[derive(Debug, Deserialize, Clone)]
pub struct PinChatMessageBody {
    pub chat_id: BodyChatId,
    pub message_id: i32,
    pub disable_notification: Option<bool>,
}

pub async fn pin_chat_message(
    state: web::Data<Mutex<State>>,
    body: web::Json<PinChatMessageBody>,
) -> impl Responder {
    check_if_message_exists!(body.message_id);
    let mut lock = state.lock().unwrap();
    lock.responses.pinned_chat_messages.push(body.into_inner());

    make_telegram_result(true)
}
