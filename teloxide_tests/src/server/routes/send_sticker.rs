use crate::server::routes::Attachment;
use crate::server::routes::{FileType, SerializeRawFields};
use crate::server::SentMessageSticker;
use crate::MockMessageSticker;
use std::collections::HashMap;

use crate::proc_macros::SerializeRawFields;
use actix_multipart::Multipart;
use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;
use teloxide::types::{Me, ReplyMarkup, ReplyParameters};

use crate::server::{routes::check_if_message_exists, FILES, MESSAGES, RESPONSES};

use super::{get_raw_multipart_fields, make_telegram_result, BodyChatId};

pub async fn send_sticker(mut payload: Multipart, me: web::Data<Me>) -> impl Responder {
    let (fields, attachments) = get_raw_multipart_fields(&mut payload).await;
    let body =
        SendMessageStickerBody::serialize_raw_fields(&fields, &attachments, FileType::Sticker)
            .unwrap();
    let chat = body.chat_id.chat();

    let mut message = MockMessageSticker::new().chat(chat);
    message.from = Some(me.user.clone());
    message.has_protected_content = body.protect_content.unwrap_or(false);
    message.emoji = body.emoji.clone();

    // Idk how to get sticker kind and sticker format from this, sooooooooooo im not doing it,
    // ain't nobody testing that

    if let Some(reply_parameters) = &body.reply_parameters {
        check_if_message_exists!(reply_parameters.message_id.0);
        let reply_to_message = MESSAGES.get_message(reply_parameters.message_id.0).unwrap();
        message.reply_to_message = Some(Box::new(reply_to_message.clone()));
    }
    if let Some(ReplyMarkup::InlineKeyboard(markup)) = body.reply_markup.clone() {
        message.reply_markup = Some(markup);
    }

    let last_id = MESSAGES.max_message_id();
    let message = MESSAGES.add_message(message.id(last_id + 1).build());

    FILES.lock().unwrap().push(teloxide::types::File {
        meta: message.sticker().unwrap().file.clone(),
        path: body.file_name.to_owned(),
    });
    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.sent_messages.push(message.clone());
    responses_lock
        .sent_messages_sticker
        .push(SentMessageSticker {
            message: message.clone(),
            bot_request: body,
        });

    make_telegram_result(message)
}

#[derive(Debug, Clone, Deserialize, SerializeRawFields)]
pub struct SendMessageStickerBody {
    pub chat_id: BodyChatId,
    pub file_name: String,
    pub file_data: String,
    pub message_thread_id: Option<i64>,
    pub emoji: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    #[serde(default, with = "crate::server::routes::reply_markup_deserialize")]
    pub reply_markup: Option<ReplyMarkup>,
    pub reply_parameters: Option<ReplyParameters>,
}
