use actix_web::Responder;
use serde_json::json;

use super::make_telegram_result;

pub async fn get_updates() -> impl Responder {
    make_telegram_result(json!([]))
}
