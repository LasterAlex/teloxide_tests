use std::sync::Mutex;

use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;
use teloxide::types::{MessageEntity, ParseMode, ReplyMarkup};

use crate::mock_bot::State;
use crate::server::routes::make_telegram_result;
use crate::server::EditedMessageCaption;

use super::{check_if_message_exists, BodyChatId};

#[derive(Debug, Deserialize, Clone)]
pub struct EditMessageCaptionBody {
    pub chat_id: Option<BodyChatId>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    pub caption: String,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    #[serde(default, with = "crate::server::routes::reply_markup_deserialize")]
    pub reply_markup: Option<ReplyMarkup>,
}

pub async fn edit_message_caption(
    state: web::Data<Mutex<State>>,
    body: web::Json<EditMessageCaptionBody>,
) -> impl Responder {
    match (
        body.chat_id.clone(),
        body.message_id,
        body.inline_message_id.clone(),
    ) {
        (Some(_), Some(message_id), None) => {
            let mut lock = state.lock().unwrap();
            check_if_message_exists!(lock, message_id);
            lock.messages
                .edit_message(message_id, "caption", body.caption.clone());
            lock.messages.edit_message(
                message_id,
                "caption_entities",
                body.caption_entities.clone().unwrap_or_default(),
            );

            let message = lock
                .messages
                .edit_message_reply_markup(message_id, body.reply_markup.clone())
                .unwrap();

            lock.responses
                .edited_messages_caption
                .push(EditedMessageCaption {
                    message: message.clone(),
                    bot_request: body.into_inner(),
                });

            make_telegram_result(message)
        }
        (None, None, Some(_)) => make_telegram_result(true),
        _ => ErrorBadRequest("No message_id or inline_message_id were provided").into(),
    }
}
