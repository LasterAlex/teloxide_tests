use std::collections::HashMap;

use actix_multipart::Multipart;
use actix_web::error::ErrorBadRequest;
use actix_web::{dev::ResourcePath, Responder};
use dataset::{MockMessagePhoto, MockPhotoSize};
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use teloxide::types::{MessageEntity, ParseMode, ReplyMarkup};

use crate::{routes::check_if_message_exists, SentMessagePhoto, FILES, MESSAGES, RESPONSES};

use super::{get_raw_multipart_fields, make_telegram_result, BodyChatId};

pub async fn send_photo(mut payload: Multipart) -> impl Responder {
    let (fields, attachments) = get_raw_multipart_fields(&mut payload).await;
    let body = serialize_raw_fields(fields, attachments).unwrap();
    let chat = body.chat_id.chat();

    let mut message = // Creates the message, which will be mutated to fit the needed shape
        MockMessagePhoto::new().chat(chat);
    message.caption = body.caption.clone();
    message.caption_entities = body.caption_entities.clone().unwrap_or_default();

    if let Some(id) = body.reply_to_message_id {
        check_if_message_exists!(id);
        message.reply_to_message = Some(Box::new(MESSAGES.get_message(id).unwrap()))
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

    FILES.lock().unwrap().push(teloxide::types::File {
        meta: message.photo().unwrap()[0].file.clone(),
        path: body.file_data.path().to_owned(),
    });
    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.sent_messages.push(message.clone());
    responses_lock.sent_messages_photo.push(SentMessagePhoto {
        message: message.clone(),
        bot_request: body,
    });

    make_telegram_result(message)
}

fn serialize_raw_fields(
    fields: HashMap<String, String>,
    attachments: HashMap<String, String>,
) -> Option<SendMessagePhotoBody> {
    let attachment = attachments.keys().last();
    let (file_name, file_data) = match attachment {
        Some(attachment) => attachments.get_key_value(attachment)?,
        None => (&"no_name.jpg".to_string(), fields.get("photo")?),
    };
    Some(SendMessagePhotoBody {
        chat_id: serde_json::from_str(&fields.get("chat_id")?).ok()?,
        file_name: file_name.to_string(),
        file_data: file_data.to_string(),
        caption: fields.get("caption").cloned(),
        message_thread_id: serde_json::from_str(
            &fields.get("message_thread_id").unwrap_or(&String::new()),
        )
        .ok(),
        parse_mode: serde_json::from_str(&fields.get("parse_mode").unwrap_or(&String::new())).ok(),
        caption_entities: serde_json::from_str(
            &fields.get("caption_entities").unwrap_or(&String::new()),
        )
        .ok(),
        disable_web_page_preview: serde_json::from_str(
            &fields
                .get("disable_web_page_preview")
                .unwrap_or(&String::new()),
        )
        .ok(),
        disable_notification: serde_json::from_str(
            &fields.get("disable_notification").unwrap_or(&String::new()),
        )
        .ok(),
        protect_content: serde_json::from_str(
            &fields.get("protect_content").unwrap_or(&String::new()),
        )
        .ok(),
        message_effect_id: fields.get("message_effect_id").cloned(),
        reply_markup: serde_json::from_str(&fields.get("reply_markup").unwrap_or(&String::new()))
            .ok(),
        reply_to_message_id: serde_json::from_str(
            &fields.get("reply_to_message_id").unwrap_or(&String::new()),
        )
        .ok(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendMessagePhotoBody {
    pub chat_id: BodyChatId,
    pub file_name: String,
    pub file_data: String,
    pub caption: Option<String>,
    pub message_thread_id: Option<i64>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_markup: Option<ReplyMarkup>,
    pub reply_to_message_id: Option<i32>,
}
