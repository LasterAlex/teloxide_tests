use crate::server::{SentMessagePoll, MESSAGES, RESPONSES};
use crate::MockMessagePoll;
use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use chrono::DateTime;
use serde::Deserialize;
use teloxide::types::{Me, MessageEntity, ParseMode, PollOption, PollType, ReplyMarkup};

use crate::server::routes::check_if_message_exists;

use super::{make_telegram_result, BodyChatId};

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
    pub open_period: Option<u16>,
    pub close_date: Option<u16>,
    pub is_closed: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_markup: Option<ReplyMarkup>,
    pub reply_to_message_id: Option<i32>,
}

pub async fn send_poll(body: web::Json<SendMessagePollBody>, me: web::Data<Me>) -> impl Responder {
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
    responses_lock.sent_messages_poll.push(SentMessagePoll {
        message: message.clone(),
        bot_request: body.into_inner(),
    });

    make_telegram_result(message)
}
