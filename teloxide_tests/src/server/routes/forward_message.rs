use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;
use teloxide::types::{
    ChatKind, Forward, ForwardedFrom, MessageId, MessageKind
};

use crate::server::ForwardedMessage;
use crate::server::{routes::check_if_message_exists, MESSAGES, RESPONSES};

use super::{make_telegram_result, BodyChatId};

#[derive(Debug, Deserialize, Clone)]
pub struct ForwardMessageBody {
    pub chat_id: BodyChatId,
    pub from_chat_id: BodyChatId,
    pub message_id: i32,
    pub message_thread_id: Option<i32>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
}

pub async fn forward_message(body: web::Json<ForwardMessageBody>) -> impl Responder {
    check_if_message_exists!(body.message_id);
    let mut message = MESSAGES.get_message(body.message_id).unwrap();

    if message.has_protected_content() {
        return ErrorBadRequest("Message has protected content").into();
    }

    let message_clone = message.clone();
    if let MessageKind::Common(ref mut common) = message.kind {
        common.forward = Some(Forward {
            date: message.date,
            signature: match message_clone.author_signature() {
                Some(signature) => Some(signature.to_string()),
                None => None,
            },
            message_id: if message.chat.is_channel() {
                Some(message.id.0)
            } else {
                None
            },
            from: match message.chat.kind {
                ChatKind::Private(_) => {match message_clone.from() {
                    Some(from) => ForwardedFrom::User(from.clone()),
                    None => ForwardedFrom::SenderName(message.chat.first_name().unwrap_or("").to_string()),
                }},
                ChatKind::Public(_) => ForwardedFrom::Chat(message_clone.chat.clone()),
            },
        })
    }

    let last_id = MESSAGES.max_message_id();
    message.id = MessageId(last_id + 1);
    message.chat = body.chat_id.chat();
    let message = MESSAGES.add_message(message);

    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.sent_messages.push(message.clone());
    responses_lock.forwarded_messages.push(ForwardedMessage {
        message: message.clone(),
        bot_request: body.into_inner(),
    });

    make_telegram_result(message)
}
