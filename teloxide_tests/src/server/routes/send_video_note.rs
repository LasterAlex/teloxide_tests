use std::{collections::HashMap, sync::Mutex};

use actix_multipart::Multipart;
use actix_web::{error::ErrorBadRequest, web, Responder};
use rand::distr::{Alphanumeric, SampleString};
use serde::Deserialize;
use teloxide::types::{
    BusinessConnectionId, EffectId, FileId, FileUniqueId, Me, ReplyMarkup, ReplyParameters, Seconds,
};

use super::{get_raw_multipart_fields, make_telegram_result, BodyChatId};
use crate::{
    proc_macros::SerializeRawFields,
    server::{
        routes::{check_if_message_exists, Attachment, FileType, SerializeRawFields},
        SentMessageVideoNote,
    },
    state::State,
    MockMessageVideoNote,
};

pub async fn send_video_note(
    mut payload: Multipart,
    me: web::Data<Me>,
    state: web::Data<Mutex<State>>,
) -> impl Responder {
    let (fields, attachments) = get_raw_multipart_fields(&mut payload).await;
    let mut lock = state.lock().unwrap();
    let body =
        SendMessageVideoNoteBody::serialize_raw_fields(&fields, &attachments, FileType::Voice)
            .unwrap();
    let chat = body.chat_id.chat();

    let mut message = MockMessageVideoNote::new().chat(chat.clone());
    message.from = Some(me.user.clone());
    message.has_protected_content = body.protect_content.unwrap_or(false);

    if let Some(reply_parameters) = &body.reply_parameters {
        check_if_message_exists!(lock, reply_parameters.message_id.0);
        let reply_to_message = lock
            .messages
            .get_message(reply_parameters.message_id.0)
            .unwrap();
        message.reply_to_message = Some(Box::new(reply_to_message.clone()));
    }
    if let Some(ReplyMarkup::InlineKeyboard(markup)) = body.reply_markup.clone() {
        message.reply_markup = Some(markup);
    }

    let file_id = FileId(Alphanumeric.sample_string(&mut rand::rng(), 16));
    let file_unique_id = FileUniqueId(Alphanumeric.sample_string(&mut rand::rng(), 8));

    message.file_id = file_id;
    message.file_unique_id = file_unique_id;
    message.duration = body.duration.unwrap_or(Seconds::from_seconds(0));
    message.length = body.length.unwrap_or(100);
    message.file_size = body.file_data.bytes().len() as u32;
    message.effect_id = body.message_effect_id.clone();
    message.business_connection_id = body.business_connection_id.clone();

    let last_id = lock.messages.max_message_id();
    let message = lock.messages.add_message(message.id(last_id + 1).build());

    lock.files.push(teloxide::types::File {
        meta: message.video_note().unwrap().file.clone(),
        path: body.file_name.to_owned(),
    });
    lock.responses.sent_messages.push(message.clone());
    lock.responses
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
    pub message_effect_id: Option<EffectId>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
    pub business_connection_id: Option<BusinessConnectionId>,
}
