use std::sync::Mutex;

use actix_web::{error::ErrorBadRequest, web, Responder};
use serde::Deserialize;
use teloxide::types::FileId;

use super::make_telegram_result;
use crate::state::State;

#[derive(Deserialize)]
pub struct GetFileQuery {
    file_id: FileId,
}

pub async fn get_file(
    query: web::Json<GetFileQuery>,
    state: web::Data<Mutex<State>>,
) -> impl Responder {
    let lock = state.lock().unwrap();
    let Some(file) = lock.files.iter().find(|f| f.id == query.file_id) else {
        return ErrorBadRequest("File not found").into();
    };
    make_telegram_result(file)
}
