use std::sync::Mutex;

use actix_web::{web, Responder};
use serde::Deserialize;

use crate::{server::routes::make_telegram_result, state::State};

use super::BodyChatId;

#[derive(Debug, Deserialize, Clone)]
pub struct SendChatActionBody {
    pub chat_id: BodyChatId,
    pub message_thread_id: Option<i64>,
    pub action: String,
}

pub async fn send_chat_action(
    state: web::Data<Mutex<State>>,
    body: web::Json<SendChatActionBody>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();
    lock.responses.sent_chat_actions.push(body.into_inner());

    make_telegram_result(true)
}
