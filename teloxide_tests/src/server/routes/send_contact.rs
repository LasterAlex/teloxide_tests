use crate::server::{SentMessageContact, MESSAGES, RESPONSES};
use crate::MockMessageContact;
use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;
use teloxide::types::{Me, ReplyMarkup};

use crate::server::routes::check_if_message_exists;

use super::{make_telegram_result, BodyChatId};

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
    pub reply_to_message_id: Option<i32>,
}

pub async fn send_contact(
    body: web::Json<SendMessageContactBody>,
    me: web::Data<Me>,
) -> impl Responder {
    let chat = body.chat_id.chat();
    let mut message = // Creates the message, which will be mutated to fit the needed shape
        MockMessageContact::new().chat(chat);
    message.from = Some(me.user.clone());
    message.phone_number = body.phone_number.clone();
    message.first_name = body.first_name.clone();
    message.last_name = body.last_name.clone();
    message.vcard = body.vcard.clone();
    message.has_protected_content = body.protect_content.unwrap_or(false);

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
    responses_lock
        .sent_messages_contact
        .push(SentMessageContact {
            message: message.clone(),
            bot_request: body.into_inner(),
        });

    make_telegram_result(message)
}

