use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use serde::Deserialize;
use serde_json::json;
use teloxide::types::{MessageEntity, ParseMode, ReplyMarkup};

use crate::{EditedMessageText, MESSAGES, RESPONSES};

use super::BodyChatId;

#[derive(Debug, Deserialize, Clone)]
pub struct EditMessageTextBody {
    pub chat_id: Option<BodyChatId>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    pub text: String,
    pub parse_mode: Option<ParseMode>,
    pub entities: Option<Vec<MessageEntity>>,
    pub disable_web_page_preview: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

pub async fn edit_message_text(body: web::Json<EditMessageTextBody>) -> impl Responder {
    match (
        body.chat_id.clone(),
        body.message_id,
        body.inline_message_id.clone(),
    ) {
        (Some(_), Some(message_id), None) => {
            // This is a regular message, edit it
            if MESSAGES
                .edit_message(message_id, "text", body.text.clone())
                .is_none()
            {
                return HttpResponse::BadRequest().body(
                    json!({ // This is how telegram returns the message
                        "ok": false,
                        "error_code": 400,
                        "description": "Message not found",
                    })
                    .to_string(),
                );
            };
            MESSAGES.edit_message(
                message_id,
                "entities",
                body.entities.clone().unwrap_or(vec![]),
            );

            match body.reply_markup.clone() {
                // Only the inline keyboard can be inside of a message
                Some(ReplyMarkup::InlineKeyboard(reply_markup)) => {
                    MESSAGES.edit_message(message_id, "reply_markup", reply_markup);
                }
                _ => {}
            }

            let message = MESSAGES.get_message(message_id).unwrap();
            RESPONSES
                .lock()
                .unwrap()
                .edited_messages_text
                .push(EditedMessageText {
                    message: message.clone(),
                    bot_request: body.into_inner(),
                });

            HttpResponse::Ok().body(
                json!({ // This is how telegram returns the message
                    "ok": true,
                    "result": message,
                })
                .to_string(),
            )
        }
        (None, None, Some(_)) => HttpResponse::Ok().body(
            // No implementation for inline messages yet, so just return success
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
                "description": "No message_id or inline_message_id were provided",
            })
            .to_string(),
        ),
    }
}
