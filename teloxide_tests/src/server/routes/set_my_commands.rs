use actix_web::Responder;

use super::make_telegram_result;

pub async fn set_my_commands() -> impl Responder {
    // Dummy response
    make_telegram_result(true)
}
