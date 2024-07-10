use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use serde::Deserialize;
use serde_json::json;

use crate::RESPONSES;

#[derive(Debug, Deserialize, Clone)]
pub struct AnswerCallbackQueryBody {
    pub callback_query_id: String,
    pub text: Option<String>,
    pub show_alert: Option<bool>,
    pub url: Option<String>,
    pub cache_time: Option<i32>,
}

pub async fn answer_callback_query(body: web::Json<AnswerCallbackQueryBody>) -> impl Responder {
    RESPONSES
        .lock()
        .unwrap()
        .answered_callback_queries
        .push(body.into_inner());
    HttpResponse::Ok().body(
        json!({
            "ok": true,  // Who cares about checking the output, just check the request
            "result": true,
        })
        .to_string(),
    )
}
