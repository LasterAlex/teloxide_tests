use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;
use teloxide::types::{MessageEntity, ParseMode, ReplyMarkup};

use crate::routes::make_telegram_result;
use crate::{EditedMessageCaption, MESSAGES, RESPONSES};

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
    pub reply_markup: Option<ReplyMarkup>,
}

pub async fn edit_message_caption(body: web::Json<EditMessageCaptionBody>) -> impl Responder {
    match (
        body.chat_id.clone(),
        body.message_id,
        body.inline_message_id.clone(),
    ) {
        (Some(_), Some(message_id), None) => {
            check_if_message_exists!(message_id);
            MESSAGES.edit_message(message_id, "caption", body.caption.clone());
            MESSAGES.edit_message(
                message_id,
                "caption_entities",
                body.caption_entities.clone().unwrap_or_default(),
            );

            let message = MESSAGES
                .edit_message_reply_markup(message_id, body.reply_markup.clone())
                .unwrap();

            let mut responses_lock = RESPONSES.lock().unwrap();
            responses_lock
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
