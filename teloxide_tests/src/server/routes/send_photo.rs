use crate::mock_bot::State;
use crate::server::routes::Attachment;
use crate::server::routes::{FileType, SerializeRawFields};

use std::collections::HashMap;
use std::sync::Mutex;

use crate::dataset::{MockMessagePhoto, MockPhotoSize};
use crate::proc_macros::SerializeRawFields;
use actix_multipart::Multipart;
use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use teloxide::types::{
    LinkPreviewOptions, Me, MessageEntity, ParseMode, ReplyMarkup, ReplyParameters,
};

use crate::server::{routes::check_if_message_exists, SentMessagePhoto, MESSAGES, RESPONSES};

use super::{get_raw_multipart_fields, make_telegram_result, BodyChatId};

pub async fn send_photo(
    mut payload: Multipart,
    me: web::Data<Me>,
    state: web::Data<Mutex<State>>,
) -> impl Responder {
    let (fields, attachments) = get_raw_multipart_fields(&mut payload).await;
    let body =
        SendMessagePhotoBody::serialize_raw_fields(&fields, &attachments, FileType::Photo).unwrap();
    let chat = body.chat_id.chat();

    let mut message = // Creates the message, which will be mutated to fit the needed shape
        MockMessagePhoto::new().chat(chat);
    message.from = Some(me.user.clone());
    message.has_protected_content = body.protect_content.unwrap_or(false);
    message.caption = body.caption.clone();
    message.caption_entities = body.caption_entities.clone().unwrap_or_default();

    if let Some(reply_parameters) = &body.reply_parameters {
        check_if_message_exists!(reply_parameters.message_id.0);
        let reply_to_message = MESSAGES.get_message(reply_parameters.message_id.0).unwrap();
        message.reply_to_message = Some(Box::new(reply_to_message.clone()));
    }
    if let Some(ReplyMarkup::InlineKeyboard(markup)) = body.reply_markup.clone() {
        message.reply_markup = Some(markup);
    }

    let file_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let file_unique_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);

    message.photo = vec![MockPhotoSize::new()
        .file_id(file_id.clone())
        .file_unique_id(file_unique_id.clone())
        .file_size(body.file_data.bytes().len() as u32)
        .build()];

    let last_id = MESSAGES.max_message_id();
    let message = MESSAGES.add_message(message.id(last_id + 1).build());

    state.lock().unwrap().files.push(teloxide::types::File {
        meta: message.photo().unwrap()[0].file.clone(),
        path: body.file_name.to_owned(),
    });
    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.sent_messages.push(message.clone());
    responses_lock.sent_messages_photo.push(SentMessagePhoto {
        message: message.clone(),
        bot_request: body,
    });

    make_telegram_result(message)
}

#[derive(Debug, Clone, Deserialize, SerializeRawFields)]
pub struct SendMessagePhotoBody {
    pub chat_id: BodyChatId,
    pub file_name: String,
    pub file_data: String,
    pub caption: Option<String>,
    pub message_thread_id: Option<i64>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    #[serde(default, with = "crate::server::routes::reply_markup_deserialize")]
    pub reply_markup: Option<ReplyMarkup>,
    pub reply_parameters: Option<ReplyParameters>,
}
