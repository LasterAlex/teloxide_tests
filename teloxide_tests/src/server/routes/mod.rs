use std::collections::HashMap;
use std::str::from_utf8;

use crate::dataset::{MockPrivateChat, MockSupergroupChat};
use actix_web::HttpResponse;
use futures_util::stream::StreamExt as _;
use futures_util::TryStreamExt;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use teloxide::types::{
    Chat, ForceReply, KeyboardMarkup, KeyboardRemove, MessageEntity, ParseMode, ReplyMarkup,
    Seconds, True,
};

pub mod answer_callback_query;
pub mod ban_chat_member;
pub mod copy_message;
pub mod delete_message;
pub mod download_file;
pub mod edit_message_caption;
pub mod edit_message_reply_markup;
pub mod edit_message_text;
pub mod forward_message;
pub mod get_file;
pub mod pin_chat_message;
pub mod restrict_chat_member;
pub mod send_animation;
pub mod send_audio;
pub mod send_chat_action;
pub mod send_contact;
pub mod send_dice;
pub mod send_document;
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
                file_data: from_utf8(&data.1).unwrap().to_string(),
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

pub fn deserialize_reply_markup(value: Value) -> Option<ReplyMarkup> {
    let selective = value
        .get("selective")
        .map(|x| serde_json::from_value(x.clone()).ok())
        .flatten()
        == Some(true);
    let input_field_placeholder: Option<String> = value
        .get("input_field_placeholder")
        .map(|x| serde_json::from_value(x.clone()).ok())
        .flatten();
    if value.get("keyboard").is_some() {
        let is_persistent: bool = value
            .get("is_persistent")
            .map(|x| serde_json::from_value(x.clone()).ok())
            .flatten()
            == Some(true);
        let one_time_keyboard = value
            .get("one_time_keyboard")
            .map(|x| serde_json::from_value(x.clone()).ok())
            .flatten()
            == Some(true);
        let resize_keyboard = value
            .get("resize_keyboard")
            .map(|x| serde_json::from_value(x.clone()).ok())
            .flatten()
            == Some(true);

        return Some(ReplyMarkup::Keyboard(KeyboardMarkup {
            keyboard: serde_json::from_value(value["keyboard"].clone()).unwrap(),
            is_persistent,
            selective,
            input_field_placeholder: input_field_placeholder.unwrap_or("".to_string()),
            one_time_keyboard,
            resize_keyboard,
        }));
    } else if value.get("inline_keyboard").is_some() {
        return serde_json::from_value(value).ok();
    } else if value.get("remove_keyboard").is_some() {
        return Some(ReplyMarkup::KeyboardRemove(KeyboardRemove {
            remove_keyboard: True,
            selective,
        }));
    } else if value.get("force_reply").is_some() {
        return Some(ReplyMarkup::ForceReply(ForceReply {
            force_reply: True,
            input_field_placeholder,
            selective,
        }));
    }

    return None;
}

pub(crate) mod reply_markup_deserialize {
    use super::deserialize_reply_markup;
    use serde::{Deserialize, Deserializer};
    use serde_json::Value;
    use teloxide::types::ReplyMarkup;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<ReplyMarkup>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Option<Value> = Option::deserialize(deserializer)?;
        match value {
            Some(value) => {
                if !value.is_null() {
                    Ok(deserialize_reply_markup(value))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    #[test]
    fn test() {
        use teloxide::types::KeyboardRemove;
        #[derive(serde::Deserialize, Debug, PartialEq)]
        struct Struct {
            #[serde(default, with = "crate::server::routes::reply_markup_deserialize")]
            reply_markup: Option<ReplyMarkup>,
        }

        {
            let s: Struct =
                serde_json::from_str("{\"reply_markup\": {\"remove_keyboard\":\"True\"}}").unwrap();
            assert_eq!(
                s,
                Struct {
                    reply_markup: Some(ReplyMarkup::KeyboardRemove(KeyboardRemove {
                        remove_keyboard: teloxide::types::True,
                        selective: false
                    }))
                }
            );
        }

        {
            let s: Struct = serde_json::from_str("{}").unwrap();
            assert_eq!(s, Struct { reply_markup: None })
        }
    }
}
