use crate::server::SetMessageReaction;
use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;
use teloxide::types::ReactionType;

use crate::server::{routes::check_if_message_exists, MESSAGES, RESPONSES};

use super::{make_telegram_result, BodyChatId};

#[derive(Debug, Deserialize, Clone)]
pub struct SetMessageReactionBody {
    pub chat_id: BodyChatId,
    pub message_id: i32,
    pub reaction: Option<Vec<ReactionType>>,
    pub is_big: Option<bool>,
}

pub async fn set_message_reaction(body: web::Json<SetMessageReactionBody>) -> impl Responder {
    check_if_message_exists!(body.message_id);

    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock
        .set_message_reaction
        .push(SetMessageReaction {
            bot_request: body.into_inner(),
        });

    make_telegram_result(true)
}
