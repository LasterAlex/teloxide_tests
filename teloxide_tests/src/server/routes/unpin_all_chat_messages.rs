use std::sync::Mutex;

use actix_web::{web, Responder};
use serde::Deserialize;

use crate::{mock_bot::State, server::routes::make_telegram_result};

use super::BodyChatId;

#[derive(Debug, Deserialize, Clone)]
pub struct UnpinAllChatMessagesBody {
    pub chat_id: BodyChatId,
}

pub async fn unpin_all_chat_messages(
    state: web::Data<Mutex<State>>,
    body: web::Json<UnpinAllChatMessagesBody>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();
    lock.responses
        .unpinned_all_chat_messages
        .push(body.into_inner());

    make_telegram_result(true)
}
