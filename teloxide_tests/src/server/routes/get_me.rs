use actix_web::{web, Responder};
use teloxide::types::Me;

use super::make_telegram_result;

pub async fn get_me(me: web::Data<Me>) -> impl Responder {
    make_telegram_result(me)
}
