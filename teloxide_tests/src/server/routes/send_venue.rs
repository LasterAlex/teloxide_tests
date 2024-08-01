use crate::server::{SentMessageVenue, MESSAGES, RESPONSES};
use crate::{MockLocation, MockMessageVenue};
use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;
use teloxide::types::{Me, ReplyMarkup};

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
    pub reply_markup: Option<ReplyMarkup>,
    pub reply_to_message_id: Option<i32>,
}

pub async fn send_venue(
    body: web::Json<SendMessageVenueBody>,
    me: web::Data<Me>,
) -> impl Responder {
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
        .sent_messages_venue
        .push(SentMessageVenue {
            message: message.clone(),
            bot_request: body.into_inner(),
        });

    make_telegram_result(message)
}