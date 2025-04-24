use std::sync::Mutex;

use actix_web::{web, Responder};
use serde::Deserialize;

use super::BodyChatId;
use crate::{
    server::{
        routes::{delete_message::DeleteMessageBody, make_telegram_result},
        DeletedMessage,
    },
    state::State,
};

#[derive(Debug, Deserialize, Clone)]
pub struct DeleteMessagesBody {
    pub chat_id: BodyChatId,
    pub message_ids: Vec<i32>,
}

pub async fn delete_messages(
    state: web::Data<Mutex<State>>,
    body: web::Json<DeleteMessagesBody>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();
    let bot_request = body.into_inner();
    // deleteMessages skips messages that are not found, no error is returned.
    let mut deleted_messages = lock
        .messages
        .delete_messages(&bot_request.message_ids)
        .into_iter()
        .map(|m| DeletedMessage {
            message: m.clone(),
            bot_request: DeleteMessageBody {
                chat_id: bot_request.chat_id.clone(),
                message_id: m.id.0,
            },
        })
        .collect();

    lock.responses
        .deleted_messages
        .append(&mut deleted_messages);

    make_telegram_result(true)
}
