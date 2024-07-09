pub mod routes;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use routes::send_message::{send_message, SendMessageBody};
use std::sync::Mutex;
use teloxide::types::Message;

pub struct Responses {
    pub sent_messages: Vec<(SendMessageBody, Message)>,
}

lazy_static! {
    pub static ref MESSAGES: Mutex<Vec<Message>> = Mutex::new(vec![]);
    pub static ref RESPONSES: Mutex<Responses> = Mutex::new(Responses {
        sent_messages: vec![]
    });
}

impl MESSAGES {
    pub fn max_message_id(&self) -> i32 {
        self.lock()
            .unwrap()
            .iter()
            .map(|m| m.id.0)
            .max()
            .unwrap_or(0)
    }

    pub fn edit_message(&self, message_id: i32, field: &str, value: &str) -> Option<Message> {
        let mut messages = self.lock().unwrap();
        let message = messages.iter().find(|m| m.id.0 == message_id)?;
        let mut json = serde_json::to_value(&message).ok()?;
        json[field] = value.into();
        let new_message: Message = serde_json::from_value(json).ok()?;
        messages.retain(|m| m.id.0 != message_id);
        messages.push(new_message.clone());
        Some(new_message)
    }

    pub fn add_message(&self, message: Message) {
        self.lock().unwrap().push(message);
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

pub async fn test(url: web::Path<String>) -> impl Responder {
    println!("{}", url);
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/bot{token}/SendMessage", web::post().to(send_message))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
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
