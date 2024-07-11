use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder};
use dataset::{chat::MockPrivateChat, MockMessagePhoto, MockSupergroupChat};
use serde_json::json;
use teloxide::types::ReplyMarkup;

use crate::{SentMessagePhoto, MESSAGES, RESPONSES};

use super::{get_raw_multipart_fields, serialize_raw_fields};

pub async fn send_photo(mut payload: Multipart) -> impl Responder {
    let (fields, attachments) = get_raw_multipart_fields(&mut payload).await;
    let body = serialize_raw_fields(fields, attachments).unwrap();
    let chat_id = body.chat_id.id();
    let chat = if chat_id < 0 {
        MockSupergroupChat::new().id(chat_id).build()
    } else {
        MockPrivateChat::new().id(chat_id).build()
    };

    let mut message = // Creates the message, which will be mutated to fit the needed shape
        MockMessagePhoto::new().chat(chat);
    message.caption = body.caption.clone();
    message.caption_entities = body.caption_entities.clone().unwrap_or_default();
    match body.reply_to_message_id {
        Some(ref id) => {
            message.reply_to_message = Some(Box::new(
                MESSAGES
                    .get_message(*id)
                    .expect("Message to reply to was not found"),
            ))
        }
        None => {}
    }
    match body.reply_markup {
        // Only the inline keyboard can be inside of a message
        Some(ReplyMarkup::InlineKeyboard(ref markup)) => {
            message.reply_markup = Some(markup.clone())
        }
        _ => {}
    }
    let last_id = MESSAGES.max_message_id();
    let message = message.id(last_id + 1).build();
    MESSAGES.add_message(message.clone());
    RESPONSES
        .lock()
        .unwrap()
        .sent_messages
        .push(message.clone());
    RESPONSES
        .lock()
        .unwrap()
        .sent_messages_photo
        .push(SentMessagePhoto {
            message: message.clone(),
            bot_request: body,
        });

    HttpResponse::Ok().body(
        json!({ // This is how telegram returns the message
            "ok": true,
            "result": message,
        })
        .to_string(),
    )
}
