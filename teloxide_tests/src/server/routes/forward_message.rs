use std::sync::Mutex;

use actix_web::{error::ErrorBadRequest, web, Responder};
use serde::Deserialize;
use teloxide::types::{Me, MessageId, MessageKind, MessageOrigin};

use super::{make_telegram_result, BodyChatId};
use crate::{
    server::{routes::check_if_message_exists, ForwardedMessage},
    state::State,
};

#[derive(Debug, Deserialize, Clone)]
pub struct ForwardMessageBody {
    pub chat_id: BodyChatId,
    pub from_chat_id: BodyChatId,
    pub message_id: i32,
    pub message_thread_id: Option<i32>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
}

pub async fn forward_message(
    body: web::Json<ForwardMessageBody>,
    me: web::Data<Me>,
    state: web::Data<Mutex<State>>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();

    check_if_message_exists!(lock, body.message_id);
    let mut message = lock.messages.get_message(body.message_id).unwrap();

    if message.has_protected_content() {
        return ErrorBadRequest("Message has protected content").into();
    }

    let message_clone = message.clone();
    if let MessageKind::Common(ref mut common) = message.kind {
        common.forward_origin = Some(if message.chat.is_channel() {
            MessageOrigin::Channel {
                date: message_clone.date,
                chat: message_clone.chat,
                message_id: message_clone.id,
                author_signature: None,
            }
        } else if let Some(sender_chat) = &message.sender_chat {
            MessageOrigin::Chat {
                date: message_clone.date,
                sender_chat: sender_chat.clone(),
                author_signature: None,
            }
        } else if let Some(user) = &message.from {
            MessageOrigin::User {
                date: message_clone.date,
                sender_user: user.clone(),
            }
        } else {
            // This is probably unreachable.
            MessageOrigin::HiddenUser {
                date: message_clone.date,
                sender_user_name: "Unknown user".to_string(),
            }
        });
        common.has_protected_content = body.protect_content.unwrap_or(false);
    }

    let last_id = lock.messages.max_message_id();
    message.id = MessageId(last_id + 1);
    message.chat = body.chat_id.chat();
    message.from = Some(me.user.clone());
    let message = lock.messages.add_message(message);

    lock.responses.sent_messages.push(message.clone());
    lock.responses.forwarded_messages.push(ForwardedMessage {
        message: message.clone(),
        bot_request: body.into_inner(),
    });

    make_telegram_result(message)
}
