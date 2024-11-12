use std::sync::Mutex;

use actix_web::{error::ErrorBadRequest, web, Responder};
use serde::Deserialize;

use super::{check_if_message_exists, BodyChatId};
use crate::{server::routes::make_telegram_result, state::State};

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
    let mut lock = state.lock().unwrap();
    check_if_message_exists!(lock, body.message_id);
    lock.responses.pinned_chat_messages.push(body.into_inner());
    make_telegram_result(true)
}
