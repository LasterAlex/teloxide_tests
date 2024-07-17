use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use crate::dataset::message_common::MockMessageText;
use serde::Deserialize;
use teloxide::types::{MessageEntity, ParseMode, ReplyMarkup};

use crate::server::{routes::check_if_message_exists, SentMessageText, MESSAGES, RESPONSES};

use super::{make_telegram_result, BodyChatId};

#[derive(Debug, Deserialize, Clone)]
pub struct SendMessageTextBody {
    pub chat_id: BodyChatId,
    pub text: String,
    pub message_thread_id: Option<i64>,
    pub parse_mode: Option<ParseMode>,
    pub entities: Option<Vec<MessageEntity>>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_markup: Option<ReplyMarkup>,
    pub reply_to_message_id: Option<i32>,
}

pub async fn send_message(body: web::Json<SendMessageTextBody>) -> impl Responder {
    let chat = body.chat_id.chat();
    let mut message = // Creates the message, which will be mutated to fit the needed shape
        MockMessageText::new().text(&body.text).chat(chat);

    message.entities = body.entities.clone().unwrap_or_default();
    if let Some(id) = body.reply_to_message_id {
        check_if_message_exists!(id);
        message.reply_to_message = Some(Box::new(MESSAGES.get_message(id).unwrap()))
    }
    if let Some(ReplyMarkup::InlineKeyboard(markup)) = body.reply_markup.clone() {
        message.reply_markup = Some(markup);
    }

    let last_id = MESSAGES.max_message_id();
    let message = MESSAGES.add_message(message.id(last_id + 1).build());

    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.sent_messages.push(message.clone());
    responses_lock.sent_messages_text.push(SentMessageText {
        message: message.clone(),
        bot_request: body.into_inner(),
    });

    make_telegram_result(message)
}
