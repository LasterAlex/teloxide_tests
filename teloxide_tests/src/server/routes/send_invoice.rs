use std::sync::Mutex;

use actix_web::{web, Responder};
use serde::Deserialize;
use teloxide::types::{LabeledPrice, Me, ReplyMarkup, ReplyParameters};

use super::{make_telegram_result, BodyChatId};
use crate::{server::SentMessageInvoice, state::State, MockMessageInvoice};

#[derive(Debug, Deserialize, Clone)]
pub struct SendMessageInvoiceBody {
    pub chat_id: BodyChatId,
    pub message_thread_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: Option<String>,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    pub max_tip_amount: Option<u32>,
    pub suggested_tip_amounts: Option<Vec<u32>>,
    pub start_parameter: Option<String>,
    pub provider_data: Option<String>,
    pub photo_url: Option<String>,
    pub photo_size: Option<String>,
    pub photo_width: Option<String>,
    pub photo_height: Option<String>,
    pub need_name: Option<bool>,
    pub need_phone_number: Option<bool>,
    pub need_email: Option<bool>,
    pub need_shipping_address: Option<bool>,
    pub send_phone_number_to_provider: Option<bool>,
    pub send_email_to_provider: Option<bool>,
    pub is_flexible: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

pub async fn send_invoice(
    body: web::Json<SendMessageInvoiceBody>,
    me: web::Data<Me>,
    state: web::Data<Mutex<State>>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();

    let chat = body.chat_id.chat();
    let mut message = MockMessageInvoice::new()
        .chat(chat)
        .title(body.title.clone())
        .description(body.description.clone())
        .start_parameter(body.start_parameter.clone().unwrap_or("".to_owned()))
        .total_amount(body.prices.first().unwrap().amount);
    message.from = Some(me.user.clone());

    // Commented until teloxides new release
    // message.has_protected_content = body.protect_content.unwrap_or(false);

    // if let Some(reply_parameters) = &body.reply_parameters {
    //     check_if_message_exists!(lock, reply_parameters.message_id.0);
    //     let reply_to_message = lock
    //         .messages
    //         .get_message(reply_parameters.message_id.0)
    //         .unwrap();
    //     message.reply_to_message = Some(Box::new(reply_to_message.clone()));
    // }
    // if let Some(ReplyMarkup::InlineKeyboard(markup)) = body.reply_markup.clone() {
    //     message.reply_markup = Some(markup);
    // }

    let last_id = lock.messages.max_message_id();
    let message = lock.messages.add_message(message.id(last_id + 1).build());

    lock.responses.sent_messages.push(message.clone());
    lock.responses
        .sent_messages_invoice
        .push(SentMessageInvoice {
            message: message.clone(),
            bot_request: body.into_inner(),
        });

    make_telegram_result(message)
}
