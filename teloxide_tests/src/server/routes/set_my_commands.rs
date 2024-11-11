use std::sync::Mutex;

use actix_web::{web, Responder};
use serde::Deserialize;
use teloxide::types::{BotCommand, BotCommandScope};

use crate::state::State;

use super::make_telegram_result;

#[derive(Debug, Deserialize, Clone)]
pub struct SetMyCommandsBody {
    pub commands: Vec<BotCommand>,
    pub scope: Option<BotCommandScope>,
    pub language_code: Option<String>,
}

pub async fn set_my_commands(
    state: web::Data<Mutex<State>>,
    body: web::Json<SetMyCommandsBody>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();
    lock.responses.set_my_commands.push(body.into_inner());

    make_telegram_result(true)
}
