use actix_web::{web, Responder};
use serde::Deserialize;

use crate::server::RESPONSES;

use super::make_telegram_result;

#[derive(Debug, Deserialize, Clone)]
pub struct AnswerCallbackQueryBody {
    pub callback_query_id: String,
    pub text: Option<String>,
    pub show_alert: Option<bool>,
    pub url: Option<String>,
    pub cache_time: Option<i32>,
}

pub async fn answer_callback_query(body: web::Json<AnswerCallbackQueryBody>) -> impl Responder {
    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock
        .answered_callback_queries
        .push(body.into_inner());
    make_telegram_result(true)
}
