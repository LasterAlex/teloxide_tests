use crate::server::routes::{check_if_message_exists, make_telegram_result};
use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;
use teloxide::types::ReplyMarkup;

use crate::server::{EditedMessageReplyMarkup, MESSAGES, RESPONSES};

use super::BodyChatId;

#[derive(Debug, Deserialize, Clone)]
pub struct EditMessageReplyMarkupBody {
    pub chat_id: Option<BodyChatId>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    #[serde(default, with = "crate::server::routes::reply_markup_deserialize")]
    pub reply_markup: Option<ReplyMarkup>,
}

pub async fn edit_message_reply_markup(
    body: web::Json<EditMessageReplyMarkupBody>,
) -> impl Responder {
    match (
        body.chat_id.clone(),
        body.message_id,
        body.inline_message_id.clone(),
    ) {
        (Some(_), Some(message_id), None) => {
            check_if_message_exists!(message_id);

            let message = match body.reply_markup.clone() {
                Some(reply_markup) => MESSAGES
                    .edit_message(message_id, "reply_markup", reply_markup)
                    .unwrap(),
                None => MESSAGES
                    .edit_message(message_id, "reply_markup", None::<()>)
                    .unwrap(),
            };

            let mut response_lock = RESPONSES.lock().unwrap();
            response_lock
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
