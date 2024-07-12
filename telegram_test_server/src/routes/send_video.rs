use crate::routes::{FileType, SerializeRawFields};
use std::collections::HashMap;
use std::str::FromStr;

use actix_multipart::Multipart;
use actix_web::error::ErrorBadRequest;
use actix_web::Responder;
use dataset::{MockMessageVideo, MockVideo};
use mime::Mime;
use proc_macros::SerializeRawFields;
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use teloxide::types::{MessageEntity, ParseMode, ReplyMarkup};

use crate::{routes::check_if_message_exists, SentMessageVideo, FILES, MESSAGES, RESPONSES};

use super::{get_raw_multipart_fields, make_telegram_result, BodyChatId};

pub async fn send_video(mut payload: Multipart) -> impl Responder {
    let (fields, attachments) = get_raw_multipart_fields(&mut payload).await;
    let body =
        SendMessageVideoBody::serialize_raw_fields(&fields, &attachments, FileType::Video).unwrap();
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
        path: body.file_name.to_owned(),
    });
    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.sent_messages.push(message.clone());
    responses_lock.sent_messages_video.push(SentMessageVideo {
        message: message.clone(),
        bot_request: body,
    });

    make_telegram_result(message)
}

#[derive(Debug, Clone, Deserialize, SerializeRawFields)]
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
