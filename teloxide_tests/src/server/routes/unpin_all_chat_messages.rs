use actix_web::{web, Responder};
use serde::Deserialize;

use crate::server::routes::make_telegram_result;
use crate::server::RESPONSES;

use super::BodyChatId;

#[derive(Debug, Deserialize, Clone)]
pub struct UnpinAllChatMessagesBody {
    pub chat_id: BodyChatId,
}

pub async fn unpin_all_chat_messages(body: web::Json<UnpinAllChatMessagesBody>) -> impl Responder {
    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock
        .unpinned_all_chat_messages
        .push(body.into_inner());

    make_telegram_result(true)
}
