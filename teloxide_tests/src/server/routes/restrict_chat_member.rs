use actix_web::{web, Responder};
use serde::Deserialize;
use teloxide::types::ChatPermissions;

use crate::server::routes::make_telegram_result;
use crate::server::RESPONSES;

use super::BodyChatId;

#[derive(Debug, Deserialize, Clone)]
pub struct RestrictChatMemberBody {
    pub chat_id: BodyChatId,
    pub user_id: u64,
    pub permissions: ChatPermissions,
    pub use_independent_chat_permissions: Option<bool>,
    pub until_date: Option<i64>,
}

pub async fn restrict_chat_member(body: web::Json<RestrictChatMemberBody>) -> impl Responder {
    // Idk what to verify here
    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock
        .restricted_chat_members
        .push(body.into_inner());

    make_telegram_result(true)
}
