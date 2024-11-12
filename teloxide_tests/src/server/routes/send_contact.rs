use std::sync::Mutex;

use actix_web::{error::ErrorBadRequest, web, Responder};
use serde::Deserialize;
use teloxide::types::{Me, ReplyMarkup, ReplyParameters};

use super::{make_telegram_result, BodyChatId};
use crate::{
    server::{routes::check_if_message_exists, SentMessageContact},
    state::State,
    MockMessageContact,
};

#[derive(Debug, Deserialize, Clone)]
pub struct SendMessageContactBody {
    pub chat_id: BodyChatId,
    pub message_thread_id: Option<i64>,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_markup: Option<ReplyMarkup>,
    pub reply_parameters: Option<ReplyParameters>,
}

pub async fn send_contact(
    body: web::Json<SendMessageContactBody>,
    me: web::Data<Me>,
    state: web::Data<Mutex<State>>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();
    let chat = body.chat_id.chat();
    let mut message = // Creates the message, which will be mutated to fit the needed shape
        MockMessageContact::new().chat(chat);
    message.from = Some(me.user.clone());
    message.phone_number = body.phone_number.clone();
    message.first_name = body.first_name.clone();
    message.last_name = body.last_name.clone();
    message.vcard = body.vcard.clone();
    message.has_protected_content = body.protect_content.unwrap_or(false);

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
    lock.responses
        .sent_messages_contact
        .push(SentMessageContact {
            message: message.clone(),
            bot_request: body.into_inner(),
        });

    make_telegram_result(message)
}
