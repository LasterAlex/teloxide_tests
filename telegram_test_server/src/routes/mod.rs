use std::collections::HashMap;
use std::str::from_utf8;

use actix_web::HttpResponse;
use dataset::{MockPrivateChat, MockSupergroupChat};
use futures_util::stream::StreamExt as _;
use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use teloxide::types::{Chat, MessageEntity, ParseMode, ReplyMarkup};

pub mod answer_callback_query;
pub mod delete_message;
pub mod edit_message_caption;
pub mod edit_message_reply_markup;
pub mod edit_message_text;
pub mod send_message;
pub mod send_photo;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum BodyChatId {
    Text(String),
    Id(i64),
}

impl BodyChatId {
    pub fn id(&self) -> i64 {
        match self {
            BodyChatId::Text(_) => 123456789,
            BodyChatId::Id(id) => *id,
        }
    }

    pub fn chat(&self) -> Chat {
        let chat_id: i64 = self.id();
        if chat_id < 0 {
            MockSupergroupChat::new().id(chat_id).build()
        } else {
            MockPrivateChat::new().id(chat_id).build()
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileType {
    Photo,
    Video,
    Audio,
    Document,
    Sticker,
    Voice,
    VideoNote,
}

macro_rules! check_if_message_exists {
    ($msg_id:expr) => {
        if MESSAGES.get_message($msg_id).is_none() {
            return ErrorBadRequest("Message not found").into();
        }
    };
}

pub(crate) use check_if_message_exists;

pub async fn get_raw_multipart_fields(
    payload: &mut actix_multipart::Multipart,
) -> (HashMap<String, String>, HashMap<String, String>) {
    let mut raw_fields: HashMap<String, Vec<u8>> = HashMap::new();
    let mut raw_attachments: HashMap<String, Vec<u8>> = HashMap::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition().unwrap();
        let name = content_disposition.get_name().unwrap().to_string();
        let filename = content_disposition.get_filename().map(|s| s.to_string());

        let mut field_data = Vec::new();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            field_data.extend_from_slice(&data);
        }

        if let Some(fname) = filename {
            // Treat raw_fields with filenames as raw_attachments
            raw_attachments.insert(fname, field_data);
        } else {
            raw_fields.insert(name, field_data);
        }
    }

    // Now `raw_fields` contains all the regular fields and `raw_attachments` contains all the attachments.
    // Process the raw_fields as needed.
    let mut fields = HashMap::new();
    for (name, data) in raw_fields {
        fields.insert(
            name.to_string(),
            from_utf8(data.as_slice()).unwrap().to_string(),
        );
    }

    let mut attachments = HashMap::new();
    for (filename, data) in raw_attachments {
        attachments.insert(filename.to_string(), from_utf8(&data).unwrap().to_string());
    }

    (fields, attachments)
}

fn serialize_raw_fields(
    fields: HashMap<String, String>,
    attachments: HashMap<String, String>,
    file_type: FileType,
) -> Option<SendMessageCaptionMediaBody> {
    let attachment = attachments.keys().last();
    let (file_name, file_data) = match attachment {
        Some(attachment) => attachments.get_key_value(attachment)?,
        None => match file_type {
            FileType::Photo => (&"no_name.jpg".to_string(), fields.get("photo")?),
            FileType::Video => (&"no_name.mp4".to_string(), fields.get("video")?),
            FileType::Audio => (&"no_name.mp3".to_string(), fields.get("audio")?),
            FileType::Document => (&"no_name.txt".to_string(), fields.get("document")?),
            FileType::Sticker => (&"no_name.png".to_string(), fields.get("sticker")?),
            FileType::Voice => (&"no_name.mp3".to_string(), fields.get("voice")?),
            FileType::VideoNote => (&"no_name.mp4".to_string(), fields.get("video_note")?),
        },
    };
    Some(SendMessageCaptionMediaBody {
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
pub struct SendMessageCaptionMediaBody {
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

pub fn make_telegram_result<T>(result: T) -> HttpResponse
where
    T: Serialize,
{
    HttpResponse::Ok().body(
        json!({
            "ok": true,
            "result": result,
        })
        .to_string(),
    )
}
