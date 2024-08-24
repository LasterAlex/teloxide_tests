use crate::server::{SentMessageLocation, MESSAGES, RESPONSES};
use crate::MockMessageLocation;
use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;
use teloxide::types::{Me, ReplyMarkup, ReplyParameters, Seconds};

use crate::server::routes::check_if_message_exists;

use super::{make_telegram_result, BodyChatId};

#[derive(Debug, Deserialize, Clone)]
pub struct SendMessageLocationBody {
    pub chat_id: BodyChatId,
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<Seconds>,
    pub heading: Option<u16>,
    pub proximity_alert_radius: Option<u32>,
    pub message_thread_id: Option<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_markup: Option<ReplyMarkup>,
    pub reply_parameters: Option<ReplyParameters>,
}

pub async fn send_location(
    body: web::Json<SendMessageLocationBody>,
    me: web::Data<Me>,
) -> impl Responder {
    let chat = body.chat_id.chat();
    let mut message = // Creates the message, which will be mutated to fit the needed shape
        MockMessageLocation::new().chat(chat).latitude(body.latitude).longitude(body.longitude);
    message.from = Some(me.user.clone());
    message.horizontal_accuracy = body.horizontal_accuracy;
    message.live_period = body.live_period;
    message.heading = body.heading;
    message.proximity_alert_radius = body.proximity_alert_radius;
    message.has_protected_content = body.protect_content.unwrap_or(false);

    if let Some(reply_parameters) = &body.reply_parameters {
        check_if_message_exists!(reply_parameters.message_id.0);
        let reply_to_message = MESSAGES.get_message(reply_parameters.message_id.0).unwrap();
        message.reply_to_message = Some(Box::new(reply_to_message.clone()));
    }
    if let Some(ReplyMarkup::InlineKeyboard(markup)) = body.reply_markup.clone() {
        message.reply_markup = Some(markup);
    }

    let last_id = MESSAGES.max_message_id();
    let message = MESSAGES.add_message(message.id(last_id + 1).build());

    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.sent_messages.push(message.clone());
    responses_lock
        .sent_messages_location
        .push(SentMessageLocation {
            message: message.clone(),
            bot_request: body.into_inner(),
        });

    make_telegram_result(message)
}
