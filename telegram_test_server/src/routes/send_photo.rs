use actix_multipart::Multipart;
use actix_web::error::ErrorBadRequest;
use actix_web::{dev::ResourcePath, Responder};
use dataset::{MockMessagePhoto, MockPhotoSize};
use rand::distributions::{Alphanumeric, DistString};
use teloxide::types::ReplyMarkup;

use crate::{routes::check_if_message_exists, SentMessagePhoto, FILES, MESSAGES, RESPONSES};

use super::{get_raw_multipart_fields, make_telegram_result, serialize_raw_fields, FileType};

pub async fn send_photo(mut payload: Multipart) -> impl Responder {
    let (fields, attachments) = get_raw_multipart_fields(&mut payload).await;
    let body = serialize_raw_fields(fields, attachments, FileType::Photo).unwrap();
    let chat = body.chat_id.chat();

    let mut message = // Creates the message, which will be mutated to fit the needed shape
        MockMessagePhoto::new().chat(chat);
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

    message.photo = vec![MockPhotoSize::new()
        .file_id(file_id.clone())
        .file_unique_id(file_unique_id.clone())
        .file_size(body.file_data.bytes().len() as u32)
        .build()];

    let last_id = MESSAGES.max_message_id();
    let message = MESSAGES.add_message(message.id(last_id + 1).build());

    FILES.lock().unwrap().push(teloxide::types::File {
        meta: message.photo().unwrap()[0].file.clone(),
        path: body.file_data.path().to_owned(),
    });
    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.sent_messages.push(message.clone());
    responses_lock.sent_messages_photo.push(SentMessagePhoto {
        message: message.clone(),
        bot_request: body,
    });

    make_telegram_result(message)
}
