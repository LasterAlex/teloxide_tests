use std::sync::Mutex;

use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;
use serde_json::json;
use teloxide::types::{
    Me, MediaAnimation, MediaAudio, MediaDocument, MediaKind, MediaPhoto, MediaVideo, MediaVoice,
    MessageEntity, MessageId, MessageKind, ParseMode, ReplyMarkup,
};

use crate::mock_bot::State;
use crate::server::routes::check_if_message_exists;
use crate::server::CopiedMessage;

use super::{make_telegram_result, BodyChatId};

#[derive(Debug, Deserialize, Clone)]
pub struct CopyMessageBody {
    pub chat_id: BodyChatId,
    pub message_thread_id: Option<i64>,
    pub from_chat_id: BodyChatId,
    pub message_id: i32,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    #[serde(default, with = "crate::server::routes::reply_markup_deserialize")]
    pub reply_markup: Option<ReplyMarkup>,
}

pub async fn copy_message(
    body: web::Json<CopyMessageBody>,
    me: web::Data<Me>,
    state: web::Data<Mutex<State>>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();
    let chat = body.chat_id.chat();
    check_if_message_exists!(lock, body.message_id);
    let mut message = lock.messages.get_message(body.message_id).unwrap();
    message.chat = chat;
    message.from = Some(me.user.clone());

    if let MessageKind::Common(ref mut common) = message.kind {
        common.forward_origin = None;
        common.external_reply = None;
        match common.media_kind {
            MediaKind::Animation(MediaAnimation {
                ref mut caption,
                ref mut caption_entities,
                ..
            })
            | MediaKind::Audio(MediaAudio {
                ref mut caption,
                ref mut caption_entities,
                ..
            })
            | MediaKind::Document(MediaDocument {
                ref mut caption,
                ref mut caption_entities,
                ..
            })
            | MediaKind::Photo(MediaPhoto {
                ref mut caption,
                ref mut caption_entities,
                ..
            })
            | MediaKind::Video(MediaVideo {
                ref mut caption,
                ref mut caption_entities,
                ..
            })
            | MediaKind::Voice(MediaVoice {
                ref mut caption,
                ref mut caption_entities,
                ..
            }) => {
                *caption = body.caption.clone();
                *caption_entities = body.caption_entities.clone().unwrap_or_default();
            }
            _ => {}
        };
        if let Some(ReplyMarkup::InlineKeyboard(markup)) = body.reply_markup.clone() {
            common.reply_markup = Some(markup);
        }
        common.has_protected_content = body.protect_content.unwrap_or(false);
    }

    let last_id = lock.messages.max_message_id();
    message.id = MessageId(last_id + 1);
    message.chat = body.chat_id.chat();
    let message = lock.messages.add_message(message);

    lock.responses.sent_messages.push(message.clone());
    lock.responses.copied_messages.push(CopiedMessage {
        message_id: message.id,
        bot_request: body.into_inner(),
    });

    make_telegram_result(json!({
        "message_id": message.id.0
    }))
}
