use std::sync::Mutex;

use actix_web::{error::ErrorBadRequest, web, Responder};
use serde::Deserialize;
use teloxide::types::{
    BusinessConnectionId, LinkPreviewOptions, Me, MessageEntity, ParseMode, ReplyMarkup,
    ReplyParameters,
};

use super::{make_telegram_result, BodyChatId};
use crate::{
    dataset::message_common::MockMessageText,
    server::{routes::check_if_message_exists, SentMessageText},
    state::State,
};

#[derive(Debug, Deserialize, Clone)]
pub struct SendMessageTextBody {
    pub chat_id: BodyChatId,
    pub text: String,
    pub message_thread_id: Option<i64>,
    pub parse_mode: Option<ParseMode>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_markup: Option<ReplyMarkup>,
    pub reply_parameters: Option<ReplyParameters>,
    pub business_connection_id: Option<BusinessConnectionId>,
}

pub async fn send_message(
    body: web::Json<SendMessageTextBody>,
    me: web::Data<Me>,
    state: web::Data<Mutex<State>>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();
    let chat = body.chat_id.chat();
    let mut message = // Creates the message, which will be mutated to fit the needed shape
        MockMessageText::new().text(&body.text).chat(chat);
    message.from = Some(me.user.clone());
    message.has_protected_content = body.protect_content.unwrap_or(false);
    message.effect_id = body.message_effect_id.clone();
    message.business_connection_id = body.business_connection_id.clone();

    message.entities = body.entities.clone().unwrap_or_default();
    if let Some(reply_parameters) = &body.reply_parameters {
        check_if_message_exists!(lock, reply_parameters.message_id.0);
        let reply_to_message = lock
            .messages
            .get_message(reply_parameters.message_id.0)
            .unwrap();
        message.reply_to_message = Some(Box::new(reply_to_message.clone()));
    }
    if let Some(ReplyMarkup::InlineKeyboard(markup)) = body.reply_markup.clone() {
        message.reply_markup = Some(markup);
    }

    let last_id = lock.messages.max_message_id();
    let message = lock.messages.add_message(message.id(last_id + 1).build());

    lock.responses.sent_messages.push(message.clone());
    lock.responses.sent_messages_text.push(SentMessageText {
        message: message.clone(),
        bot_request: body.into_inner(),
    });

    make_telegram_result(message)
}
