use actix_web::{error::ErrorBadRequest, web, Responder};
use serde::Deserialize;

use crate::server::FILES;

use super::make_telegram_result;

#[derive(Deserialize)]
pub struct GetFileQuery {
    file_id: String,
}

pub async fn get_file(query: web::Json<GetFileQuery>) -> impl Responder {
    let lock = FILES.lock().unwrap();
    let Some(file) = lock.iter().find(|f| f.id == query.file_id) else {
        return ErrorBadRequest("File not found").into();
    };
    make_telegram_result(file)
}