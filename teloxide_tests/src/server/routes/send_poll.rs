use std::sync::Mutex;

use actix_web::{error::ErrorBadRequest, web, Responder};
use chrono::DateTime;
use serde::Deserialize;
use teloxide::types::{
    Me, MessageEntity, ParseMode, PollOption, PollType, ReplyMarkup, ReplyParameters, Seconds,
};

use super::{make_telegram_result, BodyChatId};
use crate::{
    server::{routes::check_if_message_exists, SentMessagePoll},
    state::State,
    MockMessagePoll,
};

#[derive(Debug, Deserialize, Clone)]
pub struct SendMessagePollBody {
    pub chat_id: BodyChatId,
    pub message_thread_id: Option<i64>,
    pub question: String,
    pub question_parse_mode: Option<ParseMode>,
    pub question_entities: Option<Vec<MessageEntity>>,
    pub options: Vec<String>,
    pub is_anonymous: Option<bool>,
    pub r#type: Option<PollType>,
    pub allows_multiple_answers: Option<bool>,
    pub correct_option_id: Option<u8>,
    pub explanation: Option<String>,
    pub explanation_parse_mode: Option<ParseMode>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<Seconds>,
    pub close_date: Option<u16>,
    pub is_closed: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_markup: Option<ReplyMarkup>,
    pub reply_parameters: Option<ReplyParameters>,
}

pub async fn send_poll(
    state: web::Data<Mutex<State>>,
    body: web::Json<SendMessagePollBody>,
    me: web::Data<Me>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();
    let chat = body.chat_id.chat();
    let mut message = // Creates the message, which will be mutated to fit the needed shape
        MockMessagePoll::new().chat(chat);
    message.from = Some(me.user.clone());
    message.has_protected_content = body.protect_content.unwrap_or(false);

    message.question = body.question.clone();
    let mut options = vec![];
    for option in body.options.iter() {
        options.push(PollOption {
            text: option.clone(),
            voter_count: 0,
        });
    }
    message.options = options;
    message.is_anonymous = body.is_anonymous.unwrap_or(false);
    message.poll_type = body.r#type.clone().unwrap_or(PollType::Regular);
    message.allows_multiple_answers = body.allows_multiple_answers.unwrap_or(false);
    message.correct_option_id = body.correct_option_id;
    message.explanation = body.explanation.clone();
    message.explanation_entities = body.explanation_entities.clone();
    message.open_period = body.open_period;
    message.close_date = DateTime::from_timestamp(body.close_date.unwrap_or(0) as i64, 0);

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
    lock.responses.sent_messages_poll.push(SentMessagePoll {
        message: message.clone(),
        bot_request: body.into_inner(),
    });

    make_telegram_result(message)
}
