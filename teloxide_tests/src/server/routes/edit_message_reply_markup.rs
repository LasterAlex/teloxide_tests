use std::sync::Mutex;

use actix_web::{error::ErrorBadRequest, web, Responder};
use serde::Deserialize;
use teloxide::types::{BusinessConnectionId, ReplyMarkup};

use super::BodyChatId;
use crate::{
    server::{
        routes::{check_if_message_exists, make_telegram_result},
        EditedMessageReplyMarkup,
    },
    state::State,
};

#[derive(Debug, Deserialize, Clone)]
pub struct EditMessageReplyMarkupBody {
    pub chat_id: Option<BodyChatId>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    pub reply_markup: Option<ReplyMarkup>,
    pub business_connection_id: Option<BusinessConnectionId>,
}

pub async fn edit_message_reply_markup(
    body: web::Json<EditMessageReplyMarkupBody>,
    state: web::Data<Mutex<State>>,
) -> impl Responder {
    match (
        body.chat_id.clone(),
        body.message_id,
        body.inline_message_id.clone(),
    ) {
        (Some(_), Some(message_id), None) => {
            let mut lock = state.lock().unwrap();
            check_if_message_exists!(lock, message_id);

            let message = match body.reply_markup.clone() {
                Some(reply_markup) => lock
                    .messages
                    .edit_message_field(message_id, "reply_markup", reply_markup)
                    .unwrap(),
                None => lock
                    .messages
                    .edit_message_field(message_id, "reply_markup", None::<()>)
                    .unwrap(),
            };

            lock.responses
                .edited_messages_reply_markup
                .push(EditedMessageReplyMarkup {
                    message: message.clone(),
                    bot_request: body.into_inner(),
                });

            make_telegram_result(message)
        }
        (None, None, Some(_)) => make_telegram_result(true),
        _ => ErrorBadRequest("No message_id or inline_message_id were provided").into(),
    }
}
