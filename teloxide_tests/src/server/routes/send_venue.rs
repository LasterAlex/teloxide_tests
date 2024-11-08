use std::sync::Mutex;

use crate::mock_bot::State;
use crate::server::SentMessageVenue;
use crate::{MockLocation, MockMessageVenue};
use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;
use teloxide::types::{Me, ReplyMarkup, ReplyParameters};

use crate::server::routes::check_if_message_exists;

use super::{make_telegram_result, BodyChatId};

#[derive(Debug, Deserialize, Clone)]
pub struct SendMessageVenueBody {
    pub chat_id: BodyChatId,
    pub message_thread_id: Option<i64>,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    #[serde(default, with = "crate::server::routes::reply_markup_deserialize")]
    pub reply_markup: Option<ReplyMarkup>,
    pub reply_parameters: Option<ReplyParameters>,
}

pub async fn send_venue(
    body: web::Json<SendMessageVenueBody>,
    me: web::Data<Me>,
    state: web::Data<Mutex<State>>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();
    let chat = body.chat_id.chat();
    let mut message = // Creates the message, which will be mutated to fit the needed shape
        MockMessageVenue::new().chat(chat);
    message.from = Some(me.user.clone());
    message.has_protected_content = body.protect_content.unwrap_or(false);
    message.location = MockLocation::new()
        .latitude(body.latitude)
        .longitude(body.longitude)
        .build();
    message.title = body.title.clone();
    message.address = body.address.clone();
    message.foursquare_id = body.foursquare_id.clone();
    message.foursquare_type = body.foursquare_type.clone();
    message.google_place_id = body.google_place_id.clone();
    message.google_place_type = body.google_place_type.clone();

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
    lock.responses.sent_messages_venue.push(SentMessageVenue {
        message: message.clone(),
        bot_request: body.into_inner(),
    });

    make_telegram_result(message)
}
