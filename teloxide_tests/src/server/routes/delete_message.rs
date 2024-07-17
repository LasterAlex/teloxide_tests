use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;

use crate::server::routes::make_telegram_result;
use crate::server::{DeletedMessage, MESSAGES, RESPONSES};

use super::{check_if_message_exists, BodyChatId};

#[derive(Debug, Deserialize, Clone)]
pub struct DeleteMessageBody {
    pub chat_id: BodyChatId,
    pub message_id: i32,
}

pub async fn delete_message(body: web::Json<DeleteMessageBody>) -> impl Responder {
    check_if_message_exists!(body.message_id);
    let deleted_message = MESSAGES.delete_message(body.message_id).unwrap();
    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.deleted_messages.push(DeletedMessage {
        message: deleted_message.clone(),
        bot_request: body.into_inner(),
    });

    make_telegram_result(true)
}
