use crate::server::routes::Attachment;
use crate::{
    server::{
        routes::{FileType, SerializeRawFields},
        SentMessageVideoNote,
    },
    MockMessageVideoNote,
};
use std::collections::HashMap;

use crate::proc_macros::SerializeRawFields;
use actix_multipart::Multipart;
use actix_web::Responder;
use actix_web::{error::ErrorBadRequest, web};
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use teloxide::types::{Me, ReplyMarkup, ReplyParameters, Seconds};

use crate::server::{routes::check_if_message_exists, FILES, MESSAGES, RESPONSES};

use super::{get_raw_multipart_fields, make_telegram_result, BodyChatId};

pub async fn send_video_note(mut payload: Multipart, me: web::Data<Me>) -> impl Responder {
    let (fields, attachments) = get_raw_multipart_fields(&mut payload).await;
    let body =
        SendMessageVideoNoteBody::serialize_raw_fields(&fields, &attachments, FileType::Voice)
            .unwrap();
    let chat = body.chat_id.chat();

    let mut message = MockMessageVideoNote::new().chat(chat.clone());
    message.from = Some(me.user.clone());
    message.has_protected_content = body.protect_content.unwrap_or(false);

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

    message.file_id = file_id.clone();
    message.file_unique_id = file_unique_id.clone();
    message.duration = body.duration.unwrap_or(Seconds::from_seconds(0));
    message.length = body.length.unwrap_or(100);
    message.file_size = body.file_data.bytes().len() as u32;

    let last_id = MESSAGES.max_message_id();
    let message = MESSAGES.add_message(message.id(last_id + 1).build());

    FILES.lock().unwrap().push(teloxide::types::File {
        meta: message.video_note().unwrap().file.clone(),
        path: body.file_name.to_owned(),
    });
    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.sent_messages.push(message.clone());
    responses_lock
        .sent_messages_video_note
        .push(SentMessageVideoNote {
            message: message.clone(),
            bot_request: body,
        });

    make_telegram_result(message)
}

#[derive(Debug, Clone, Deserialize, SerializeRawFields)]
pub struct SendMessageVideoNoteBody {
    pub chat_id: BodyChatId,
    pub message_thread_id: Option<i64>,
    pub file_name: String,
    pub file_data: String,
    pub duration: Option<Seconds>,
    pub length: Option<u32>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
