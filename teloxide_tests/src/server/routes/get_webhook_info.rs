use actix_web::Responder;
use serde_json::json;

use super::make_telegram_result;

pub async fn get_webhook_info() -> impl Responder {
    make_telegram_result(
        json!({"url": "", "has_custom_certificate":false,"pending_update_count":0}),
    )
}
