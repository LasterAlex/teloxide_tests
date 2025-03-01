use std::{collections::HashMap, str::from_utf8};

use actix_web::{error::ResponseError, http::header::ContentType, HttpResponse};
use futures_util::{stream::StreamExt as _, TryStreamExt};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use serde_json::json;
use teloxide::{
    types::{Chat, MessageEntity, ParseMode, Seconds},
    ApiError,
};

use crate::dataset::{MockPrivateChat, MockSupergroupChat};

pub mod answer_callback_query;
pub mod ban_chat_member;
pub mod copy_message;
pub mod delete_message;
pub mod delete_messages;
pub mod download_file;
pub mod edit_message_caption;
pub mod edit_message_reply_markup;
pub mod edit_message_text;
pub mod forward_message;
pub mod get_file;
pub mod get_me;
pub mod get_updates;
pub mod get_webhook_info;
pub mod pin_chat_message;
pub mod restrict_chat_member;
pub mod send_animation;
pub mod send_audio;
pub mod send_chat_action;
pub mod send_contact;
pub mod send_dice;
pub mod send_document;
pub mod send_invoice;
pub mod send_location;
pub mod send_media_group;
pub mod send_message;
pub mod send_photo;
pub mod send_poll;
pub mod send_sticker;
pub mod send_venue;
pub mod send_video;
pub mod send_video_note;
pub mod send_voice;
pub mod set_message_reaction;
pub mod set_my_commands;
pub mod unban_chat_member;
pub mod unpin_all_chat_messages;
pub mod unpin_chat_message;

/// Telegram accepts both `i64` and `String` for chat_id,
/// so it is a wrapper for both
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum BodyChatId {
    Text(String),
    Id(i64),
}

impl BodyChatId {
    /// Returns the ID of the chat
    pub fn id(&self) -> i64 {
        match self {
            BodyChatId::Text(_) => 123456789,
            BodyChatId::Id(id) => *id,
        }
    }

    /// Returns the chat
    pub fn chat(&self) -> Chat {
        let chat_id: i64 = self.id();
        if chat_id < 0 {
            MockSupergroupChat::new().id(chat_id).build()
        } else {
            MockPrivateChat::new().id(chat_id).build()
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum MediaGroupInputMedia {
    InputMediaAudio(MediaGroupInputMediaAudio),
    InputMediaDocument(MediaGroupInputMediaDocument),
    InputMediaPhoto(MediaGroupInputMediaPhoto),
    InputMediaVideo(MediaGroupInputMediaVideo),
}

#[derive(Debug, Deserialize, Clone)]
pub struct MediaGroupInputMediaAudio {
    pub r#type: String,
    pub file_name: String,
    pub file_data: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<Seconds>,
    pub performer: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MediaGroupInputMediaDocument {
    pub r#type: String,
    pub file_name: String,
    pub file_data: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_content_type_detection: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MediaGroupInputMediaPhoto {
    pub r#type: String,
    pub file_name: String,
    pub file_data: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub has_spoiler: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MediaGroupInputMediaVideo {
    pub r#type: String,
    pub file_name: String,
    pub file_data: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<Seconds>,
    pub supports_streaming: Option<bool>,
    pub has_spoiler: Option<bool>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum FileType {
    Photo,
    Video,
    Audio,
    Document,
    Sticker,
    Voice,
    VideoNote,
    Animation,
}

pub struct Attachment {
    pub raw_name: String,
    pub file_name: String,
    pub file_data: String,
}

pub trait SerializeRawFields {
    fn serialize_raw_fields(
        fields: &HashMap<String, String>,
        attachments: &HashMap<String, Attachment>,
        file_type: FileType,
    ) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Debug, Serialize)]
struct TelegramResponse {
    ok: bool,
    description: String,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct BotApiError {
    error: ApiError,
}

impl BotApiError {
    pub fn new(error: ApiError) -> Self {
        Self { error }
    }
}

impl std::fmt::Display for BotApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

impl ResponseError for BotApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let response = TelegramResponse {
            ok: false,
            description: self.error.to_string(),
        };
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&response).unwrap())
    }
}

// TODO: replace usages with appropriate error values from teloxide::ApiError.
macro_rules! check_if_message_exists {
    ($lock:expr, $msg_id:expr) => {
        if $lock.messages.get_message($msg_id).is_none() {
            return ErrorBadRequest("Message not found").into();
        }
    };
}

pub(crate) use check_if_message_exists;

pub async fn get_raw_multipart_fields(
    payload: &mut actix_multipart::Multipart,
) -> (HashMap<String, String>, HashMap<String, Attachment>) {
    let mut raw_fields: HashMap<String, Vec<u8>> = HashMap::new();
    let mut raw_attachments: HashMap<String, (String, Vec<u8>)> = HashMap::new();

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
            let mut attachment_key = fname.clone();
            if raw_attachments.contains_key(&fname) {
                // If two files have the same name, add a random string to the filename
                attachment_key = fname
                    .split('.')
                    .enumerate()
                    .map(|(i, s)| {
                        if i == 0 {
                            format!(
                                "{s}{}",
                                Alphanumeric.sample_string(&mut rand::thread_rng(), 5)
                            )
                        } else {
                            s.to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(".");
            }
            raw_attachments.insert(attachment_key, (name, field_data));
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
        attachments.insert(
            filename.to_string(),
            Attachment {
                raw_name: data.0.to_string(),
                file_name: filename.to_string(),
                file_data: from_utf8(&data.1)
                    .unwrap_or("error_getting_data")
                    .to_string(),
            },
        );
    }

    (fields, attachments)
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
