pub mod routes;
use actix_web::{web, App, HttpServer};
use lazy_static::lazy_static;
use routes::{
    edit_message_text::{edit_message_text, EditMessageTextBody},
    send_message::{send_message, SendMessageBody},
};
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Mutex,
};
use teloxide::types::Message;

#[derive(Clone, Debug)]
pub struct SentMessage {
    // For better syntax, this is a struct, not a tuple
    pub message: Message,
    pub request: SendMessageBody,
}

#[derive(Clone, Debug)]
pub struct EditedMessageText {
    // For better syntax, this is a struct, not a tuple
    pub message: Message,
    pub request: EditMessageTextBody,
}

#[derive(Clone, Debug)]
pub struct Responses {
    pub sent_messages: Vec<SentMessage>,
    pub edited_messages_text: Vec<EditedMessageText>,
}

lazy_static! {
    pub static ref MESSAGES: Mutex<Vec<Message>> = Mutex::new(vec![]);  // Messages storage, just in case
    pub static ref RESPONSES: Mutex<Responses> = Mutex::new(Responses {  // This is what is needed from this server
        sent_messages: vec![],
        edited_messages_text: vec![],
    });
    pub static ref LAST_MESSAGE_ID: AtomicI32 = AtomicI32::new(0);
}

impl MESSAGES {
    pub fn max_message_id(&self) -> i32 {
        LAST_MESSAGE_ID.load(Ordering::Relaxed)
    }

    pub fn edit_message(&self, message_id: i32, field: &str, value: &str) -> Option<Message> {
        let mut messages = self.lock().unwrap(); // Get the message lock
        let message = messages.iter().find(|m| m.id.0 == message_id)?; // Find the message
                                                                       // (return None if not found)

        let mut json = serde_json::to_value(&message).ok()?; // Convert the message to JSON
        json[field] = value.into(); // Edit the field
        let new_message: Message = serde_json::from_value(json).ok()?; // Convert back to Message

        messages.retain(|m| m.id.0 != message_id); // Remove the old message
        messages.push(new_message.clone()); // Add the new message
        Some(new_message) // Profit!
    }

    pub fn add_message(&self, message: Message) {
        self.lock().unwrap().push(message);
        LAST_MESSAGE_ID.fetch_add(1, Ordering::Relaxed);
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

pub const SERVER_PORT: Mutex<u16> = Mutex::new(6504);
// The port is arbitrary. It is a mutex just in case someone uses 6504 port and needs to change it

pub async fn main() {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // MESSAGES don't care if they are cleaned or not
    RESPONSES.lock().unwrap().sent_messages.clear();
    RESPONSES.lock().unwrap().edited_messages_text.clear();

    HttpServer::new(move || {
        App::new()
            // .wrap(Logger::default())
            .route("/bot{token}/SendMessage", web::post().to(send_message))
            .route("/bot{token}/EditMessageText", web::post().to(edit_message_text))
    })
    .bind(format!(
        "127.0.0.1:{}",
        SERVER_PORT.lock().unwrap().to_string()
    ))
    .unwrap()
    .workers(1)
    .run()
    .await
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use dataset::*;
    use serial_test::serial;

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
}
