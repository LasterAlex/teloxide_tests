use std::sync::Mutex;

use actix_web::{error::ErrorBadRequest, web, Responder};
use serde::Deserialize;
use teloxide::types::{DiceEmoji, ReplyMarkup, ReplyParameters};

use super::{make_telegram_result, BodyChatId};
use crate::{
    server::{routes::check_if_message_exists, SentMessageDice},
    state::State,
    MockMessageDice,
};

#[derive(Debug, Deserialize, Clone)]
pub struct SendMessageDiceBody {
    pub chat_id: BodyChatId,
    pub message_thread_id: Option<i64>,
    pub emoji: Option<DiceEmoji>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_markup: Option<ReplyMarkup>,
    pub reply_parameters: Option<ReplyParameters>,
}

pub async fn send_dice(
    state: web::Data<Mutex<State>>,
    body: web::Json<SendMessageDiceBody>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();
    let chat = body.chat_id.chat();
    let mut message = // Creates the message, which will be mutated to fit the needed shape
        MockMessageDice::new().chat(chat);
    message.emoji = body.emoji.clone().unwrap_or(MockMessageDice::EMOJI);
    // Random from 1 to 5 because it fits all the emoji
    message.value = (1 + rand::random::<u8>() % 5) as u8;
    if let Some(reply_parameters) = &body.reply_parameters {
        check_if_message_exists!(lock, reply_parameters.message_id.0);
    }

    let last_id = lock.messages.max_message_id();
    let message = lock.messages.add_message(message.id(last_id + 1).build());

    lock.responses.sent_messages.push(message.clone());
    lock.responses.sent_messages_dice.push(SentMessageDice {
        message: message.clone(),
        bot_request: body.into_inner(),
    });

    make_telegram_result(message)
}
