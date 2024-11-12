use std::sync::Mutex;

use actix_web::{error::ErrorBadRequest, web, Responder};
use serde::Deserialize;

use super::{check_if_message_exists, BodyChatId};
use crate::{server::routes::make_telegram_result, state::State};

#[derive(Debug, Deserialize, Clone)]
pub struct UnpinChatMessageBody {
    pub chat_id: BodyChatId,
    pub message_id: Option<i32>,
}

pub async fn unpin_chat_message(
    state: web::Data<Mutex<State>>,
    body: web::Json<UnpinChatMessageBody>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();
    if let Some(message_id) = body.message_id {
        check_if_message_exists!(lock, message_id);
    }
    lock.responses
        .unpinned_chat_messages
        .push(body.into_inner());

    make_telegram_result(true)
}
