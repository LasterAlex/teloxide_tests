use std::sync::Mutex;

use actix_web::{error::ErrorBadRequest, web, Responder};
use serde::Deserialize;
use teloxide::types::{LinkPreviewOptions, MessageEntity, ParseMode, ReplyMarkup};

use crate::{
    mock_bot::State,
    server::{routes::make_telegram_result, EditedMessageText},
};

use super::{check_if_message_exists, BodyChatId};

#[derive(Debug, Deserialize, Clone)]
pub struct EditMessageTextBody {
    pub chat_id: Option<BodyChatId>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    pub text: String,
    pub parse_mode: Option<ParseMode>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(default, with = "crate::server::routes::reply_markup_deserialize")]
    pub reply_markup: Option<ReplyMarkup>,
}

pub async fn edit_message_text(
    body: web::Json<EditMessageTextBody>,
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

            lock.messages
                .edit_message(message_id, "text", body.text.clone());
            lock.messages.edit_message(
                message_id,
                "entities",
                body.entities.clone().unwrap_or(vec![]),
            );
            let message = lock
                .messages
                .edit_message_reply_markup(message_id, body.reply_markup.clone())
                .unwrap();

            lock.responses.edited_messages_text.push(EditedMessageText {
                message: message.clone(),
                bot_request: body.into_inner(),
            });

            make_telegram_result(message)
        }
        // No implementation for inline messages yet, so just return success
        (None, None, Some(_)) => make_telegram_result(true),
        _ => ErrorBadRequest("No message_id or inline_message_id were provided").into(),
    }
}
