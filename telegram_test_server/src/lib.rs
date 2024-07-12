pub mod routes;
use actix_web::{web, App, HttpServer, Responder};
use lazy_static::lazy_static;
use routes::{
    answer_callback_query::*, delete_message::*, edit_message_caption::*,
    edit_message_reply_markup::*, edit_message_text::*, send_message::*, send_photo::*,
    SendMessageCaptionMediaBody,
};
use serde::Serialize;
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Mutex,
};
use teloxide::types::{File, Message, ReplyMarkup};

#[derive(Clone, Debug)]
pub struct SentMessageText {
    // For better syntax, this is a struct, not a tuple
    pub message: Message,
    pub bot_request: SendMessageTextBody,
}

#[derive(Clone, Debug)]
pub struct SentMessagePhoto {
    pub message: Message,
    pub bot_request: SendMessageCaptionMediaBody,
}

#[derive(Clone, Debug)]
pub struct EditedMessageText {
    pub message: Message,
    pub bot_request: EditMessageTextBody,
}

#[derive(Clone, Debug)]
pub struct EditedMessageCaption {
    pub message: Message,
    pub bot_request: EditMessageCaptionBody,
}

#[derive(Clone, Debug)]
pub struct DeletedMessage {
    pub message: Message,
    pub bot_request: DeleteMessageBody,
}

#[derive(Clone, Debug)]
pub struct EditedMessageReplyMarkup {
    pub message: Message,
    pub bot_request: EditMessageReplyMarkupBody,
}

#[derive(Clone, Debug, Default)]
pub struct Responses {
    pub sent_messages: Vec<Message>, // Just for convenience for simple tasks
    pub sent_messages_text: Vec<SentMessageText>,
    pub sent_messages_photo: Vec<SentMessagePhoto>,
    pub edited_messages_text: Vec<EditedMessageText>,
    pub edited_messages_caption: Vec<EditedMessageCaption>,
    pub edited_messages_reply_markup: Vec<EditedMessageReplyMarkup>,
    pub deleted_messages: Vec<DeletedMessage>,
    pub answered_callback_queries: Vec<AnswerCallbackQueryBody>,
}

lazy_static! {
    pub static ref MESSAGES: Mutex<Vec<Message>> = Mutex::new(vec![]);  // Messages storage, just in case
    pub static ref FILES: Mutex<Vec<File>> = Mutex::new(vec![]);  // Messages storage, just in case
    pub static ref RESPONSES: Mutex<Responses> = Mutex::new(Responses::default());  //
    pub static ref LAST_MESSAGE_ID: AtomicI32 = AtomicI32::new(0);
}

impl MESSAGES {
    pub fn max_message_id(&self) -> i32 {
        LAST_MESSAGE_ID.load(Ordering::Relaxed)
    }

    pub fn edit_message<T>(&self, message_id: i32, field: &str, value: T) -> Option<Message>
    where
        T: Serialize,
    {
        let mut messages = self.lock().unwrap(); // Get the message lock
        let message = messages.iter().find(|m| m.id.0 == message_id)?; // Find the message
                                                                       // (return None if not found)

        let mut json = serde_json::to_value(&message).ok()?; // Convert the message to JSON
        json[field] = serde_json::to_value(value).ok()?; // Edit the field
        let new_message: Message = serde_json::from_value(json).ok()?; // Convert back to Message

        messages.retain(|m| m.id.0 != message_id); // Remove the old message
        messages.push(new_message.clone()); // Add the new message
        Some(new_message) // Profit!
    }

    pub fn edit_message_reply_markup(
        &self,
        message_id: i32,
        reply_markup: Option<ReplyMarkup>,
    ) -> Option<Message> {
        match reply_markup {
            // Only the inline keyboard can be inside of a message
            Some(ReplyMarkup::InlineKeyboard(reply_markup)) => {
                MESSAGES.edit_message(message_id, "reply_markup", reply_markup)
            }
            _ => MESSAGES.get_message(message_id),
        }
    }

    pub fn add_message(&self, message: Message) -> Message {
        self.lock().unwrap().push(message.clone());
        LAST_MESSAGE_ID.fetch_add(1, Ordering::Relaxed);
        message
    }

    pub fn get_message(&self, message_id: i32) -> Option<Message> {
        self.lock()
            .unwrap()
            .iter()
            .find(|m| m.id.0 == message_id)
            .cloned()
    }

    pub fn delete_message(&self, message_id: i32) -> Option<Message> {
        let mut messages = self.lock().unwrap();
        let message = messages.iter().find(|m| m.id.0 == message_id).cloned()?;
        messages.retain(|m| m.id.0 != message_id);
        Some(message)
    }
}

pub async fn ping() -> impl Responder {
    "pong"
}

pub async fn main(port: Mutex<u16>) {
    // MESSAGES don't care if they are cleaned or not
    *RESPONSES.lock().unwrap() = Responses::default();

    let pong = reqwest::get(format!("http://127.0.0.1:{}/ping", port.lock().unwrap())).await;

    if pong.is_err()
    // If it errored, no server is running, we need to start it
    {
        HttpServer::new(move || {
            App::new()
                .route("/ping", web::get().to(ping))
                .route("/bot{token}/SendMessage", web::post().to(send_message))
                .route("/bot{token}/SendPhoto", web::post().to(send_photo))
                .route(
                    "/bot{token}/EditMessageText",
                    web::post().to(edit_message_text),
                )
                .route(
                    "/bot{token}/EditMessageCaption",
                    web::post().to(edit_message_caption),
                )
                .route(
                    "/bot{token}/EditMessageReplyMarkup",
                    web::post().to(edit_message_reply_markup),
                )
                .route("/bot{token}/DeleteMessage", web::post().to(delete_message))
                .route(
                    "/bot{token}/AnswerCallbackQuery",
                    web::post().to(answer_callback_query),
                )
        })
        .bind(format!("127.0.0.1:{}", port.lock().unwrap().to_string()))
        .unwrap()
        .workers(1)
        .run()
        .await
        .unwrap()
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use dataset::*;
    use serial_test::serial;
    use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

    #[test]
    #[serial]
    fn test_add_messages() {
        MESSAGES.lock().unwrap().clear();
        LAST_MESSAGE_ID.store(0, Ordering::Relaxed);
        MESSAGES.add_message(message_common::MockMessageText::new("123").id(1).build());
        MESSAGES.add_message(message_common::MockMessageText::new("123").id(2).build());
        MESSAGES.add_message(message_common::MockMessageText::new("123").id(3).build());
        assert_eq!(MESSAGES.max_message_id(), 3);
    }

    #[test]
    #[serial]
    fn test_edit_messages() {
        MESSAGES.lock().unwrap().clear();
        MESSAGES.add_message(message_common::MockMessageText::new("123").id(1).build());
        MESSAGES.edit_message(1, "text", "1234");
        assert_eq!(MESSAGES.get_message(1).unwrap().text().unwrap(), "1234");
    }

    #[test]
    #[serial]
    fn test_get_messages() {
        MESSAGES.lock().unwrap().clear();
        MESSAGES.add_message(message_common::MockMessageText::new("123").id(1).build());
        assert_eq!(MESSAGES.get_message(1).unwrap().text().unwrap(), "123");
    }

    #[test]
    #[serial]
    fn test_delete_messages() {
        MESSAGES.lock().unwrap().clear();
        MESSAGES.add_message(message_common::MockMessageText::new("123").id(1).build());
        MESSAGES.delete_message(1);
        assert_eq!(MESSAGES.get_message(1), None);
    }

    #[test]
    #[serial]
    fn test_edit_message_reply_markup() {
        MESSAGES.lock().unwrap().clear();
        MESSAGES.add_message(message_common::MockMessageText::new("123").id(1).build());
        MESSAGES.edit_message_reply_markup(
            1,
            Some(ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup::new(
                vec![vec![InlineKeyboardButton::callback("123", "123")]],
            ))),
        );
        assert_eq!(
            MESSAGES
                .get_message(1)
                .unwrap()
                .reply_markup()
                .unwrap()
                .inline_keyboard[0][0]
                .text,
            "123"
        );
    }
}
