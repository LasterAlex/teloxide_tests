use std::collections::HashMap;
use std::str::FromStr;

use actix_multipart::Multipart;
use actix_web::error::ErrorBadRequest;
use actix_web::{dev::ResourcePath, Responder};
use dataset::{MockMessageVideo, MockVideo};
use mime::Mime;
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use teloxide::types::{MessageEntity, ParseMode, ReplyMarkup};

use crate::{routes::check_if_message_exists, SentMessageVideo, FILES, MESSAGES, RESPONSES};

use super::{get_raw_multipart_fields, make_telegram_result, BodyChatId};

pub async fn send_video(mut payload: Multipart) -> impl Responder {
    let (fields, attachments) = get_raw_multipart_fields(&mut payload).await;
    let body = serialize_raw_fields(fields, attachments).unwrap();
    let chat = body.chat_id.chat();

    let mut message = MockMessageVideo::new().chat(chat.clone());
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

    message.video = MockVideo::new()
        .file_id(file_id.clone())
        .file_unique_id(file_unique_id.clone())
        .file_size(body.file_data.bytes().len() as u32)
        .file_name(body.file_name.clone())
        .width(body.width.unwrap_or(100))
        .height(body.height.unwrap_or(100))
        .duration(body.duration.unwrap_or(100))
        .mime_type(Mime::from_str("video/mp4").unwrap())
        .build();

    let last_id = MESSAGES.max_message_id();
    let message = MESSAGES.add_message(message.id(last_id + 1).build());

    FILES.lock().unwrap().push(teloxide::types::File {
        meta: message.video().unwrap().file.clone(),
        path: body.file_data.path().to_owned(),
    });
    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.sent_messages.push(message.clone());
    responses_lock.sent_messages_video.push(SentMessageVideo {
        message: message.clone(),
        bot_request: body,
    });

    make_telegram_result(message)
}

fn serialize_raw_fields(
    fields: HashMap<String, String>,
    attachments: HashMap<String, String>,
) -> Option<SendMessageVideoBody> {
    let attachment = attachments.keys().last();
    let (file_name, file_data) = match attachment {
        Some(attachment) => attachments.get_key_value(attachment)?,
        None => (&"no_name.mp4".to_string(), fields.get("video")?),
    };
    Some(SendMessageVideoBody {
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
        duration: serde_json::from_str(&fields.get("duration").unwrap_or(&String::new())).ok(),
        width: serde_json::from_str(&fields.get("width").unwrap_or(&String::new())).ok(),
        height: serde_json::from_str(&fields.get("height").unwrap_or(&String::new())).ok(),
        has_spoiler: serde_json::from_str(&fields.get("has_spoiler").unwrap_or(&String::new()))
            .ok(),
        show_caption_above_media: serde_json::from_str(
            &fields
                .get("show_caption_above_media")
                .unwrap_or(&String::new()),
        )
        .ok(),
        supports_streaming: serde_json::from_str(
            &fields.get("supports_streaming").unwrap_or(&String::new()),
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
pub struct SendMessageVideoBody {
    pub chat_id: BodyChatId,
    pub message_thread_id: Option<i64>,
    pub file_name: String,
    pub file_data: String,
    pub duration: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub has_spoiler: Option<bool>,
    pub supports_streaming: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_markup: Option<ReplyMarkup>,
    pub reply_to_message_id: Option<i32>,
}
