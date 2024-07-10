use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use serde::Deserialize;
use serde_json::json;
use teloxide::types::ReplyMarkup;

use crate::{DeletedMessage, MESSAGES, RESPONSES};

#[derive(Debug, Deserialize, Clone)]
pub struct DeleteMessageBody {
    pub chat_id: i64,
    pub message_id: i32,
    pub reply_markup: Option<ReplyMarkup>,
}

pub async fn delete_message(body: web::Json<DeleteMessageBody>) -> impl Responder {
    let Some(deleted_message) = MESSAGES.delete_message(body.message_id) else {
        return HttpResponse::BadRequest().body(
            json!({
                "ok": false,
                "error_code": 400,
                "result": "Message to delete not found",
            })
            .to_string(),
        );
    };
    RESPONSES
        .lock()
        .unwrap()
        .deleted_messages
        .push(DeletedMessage {
            message: deleted_message.clone(),
            bot_request: body.into_inner(),
        });

    HttpResponse::Ok().body(
        json!({ // This is how telegram returns the message
            "ok": true,
            "result": true,
        })
        .to_string(),
    )
}
