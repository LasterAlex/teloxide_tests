use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use serde::Deserialize;
use serde_json::json;
use teloxide::types::ReplyMarkup;

use crate::{EditedMessageReplyMarkup, MESSAGES, RESPONSES};

#[derive(Debug, Deserialize, Clone)]
pub struct EditMessageReplyMarkupBody {
    chat_id: Option<i64>,
    message_id: Option<i32>,
    inline_message_id: Option<String>,
    reply_markup: Option<ReplyMarkup>,
}

pub async fn edit_message_reply_markup(
    body: web::Json<EditMessageReplyMarkupBody>,
) -> impl Responder {
    match (
        body.chat_id,
        body.message_id,
        body.inline_message_id.clone(),
    ) {
        (Some(_), Some(message_id), None) => {
            let message = match body.reply_markup.clone() {
                Some(reply_markup) => {
                    MESSAGES.edit_message(message_id, "reply_markup", reply_markup)
                }
                None => MESSAGES.edit_message(message_id, "reply_markup", None::<()>),
            };
            let Some(message) = message else {
                return HttpResponse::BadRequest().body(
                    json!({
                        "ok": false,
                        "error_code": 400,
                        "description": "Message not found",
                    })
                    .to_string(),
                );
            };

            RESPONSES
                .lock()
                .unwrap()
                .edited_messages_reply_markup
                .push(EditedMessageReplyMarkup {
                    message: message.clone(),
                    bot_request: body.into_inner(),
                });

            HttpResponse::Ok().body(
                json!({
                    "ok": true,
                    "result": message,
                })
                .to_string(),
            )
        }
        (None, None, Some(_)) => HttpResponse::Ok().body(
            json!({
                "ok": true,
                "result": true,
            })
            .to_string(),
        ),
        _ => HttpResponse::BadRequest().body(
            json!({
                "ok": false,
                "error_code": 400,
                "description": "Missing chat_id, message_id or inline_message_id",
            })
            .to_string(),
        ),
    }
}
