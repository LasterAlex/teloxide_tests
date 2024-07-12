use std::collections::HashMap;
use std::str::from_utf8;

use actix_web::HttpResponse;
use dataset::{MockPrivateChat, MockSupergroupChat};
use futures_util::stream::StreamExt as _;
use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use teloxide::types::Chat;

pub mod answer_callback_query;
pub mod delete_message;
pub mod edit_message_caption;
pub mod edit_message_reply_markup;
pub mod edit_message_text;
pub mod send_message;
pub mod send_photo;
pub mod send_video;

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
