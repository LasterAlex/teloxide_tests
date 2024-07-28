//! A fake telegram bot API for testing purposes. Read more in teloxide_tests crate.
pub mod routes;
use actix_web::{dev::ServerHandle, web, App, HttpResponse, HttpServer, Responder};
use actix_web_lab::extract::Path;
use lazy_static::lazy_static;
use routes::{
    answer_callback_query::*, delete_message::*, download_file::download_file,
    edit_message_caption::*, edit_message_reply_markup::*, edit_message_text::*, get_file::*,
    pin_chat_message::*, send_document::*, send_message::*, send_photo::*, send_video::*,
    unpin_all_chat_messages::*, unpin_chat_message::*,
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
    pub bot_request: SendMessagePhotoBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageVideo {
    pub message: Message,
    pub bot_request: SendMessageVideoBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageDocument {
    pub message: Message,
    pub bot_request: SendMessageDocumentBody,
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
    /// All of the sent messages, including text, photo, audio, etc.
    /// Be warned, editing or deleting messages do not affect this list!
    pub sent_messages: Vec<Message>,
    /// This has only messages that are text messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_text: Vec<SentMessageText>,
    /// This has only messages that are photo messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_photo: Vec<SentMessagePhoto>,
    /// This has only messages that are video messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_video: Vec<SentMessageVideo>,
    /// This has only messages that are document messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_document: Vec<SentMessageDocument>,
    /// This has only edited by the bot text messages.
    /// The `.message` field has the new edited message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub edited_messages_text: Vec<EditedMessageText>,
    /// This has only edited by the bot caption messages.
    /// The `.message` field has the new edited message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub edited_messages_caption: Vec<EditedMessageCaption>,
    /// This has only messages whos reply markup was edited by the bot.
    /// The `.message` field has the new edited message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub edited_messages_reply_markup: Vec<EditedMessageReplyMarkup>,
    /// This has only messages which were deleted by the bot.
    /// The `.message` field has the deleted message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub deleted_messages: Vec<DeletedMessage>,
    /// This has only the requests that were sent to the fake server to answer callback queries.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub answered_callback_queries: Vec<AnswerCallbackQueryBody>,
    /// This has only the requests that were sent to the fake server to pin messages.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub pinned_chat_messages: Vec<PinChatMessageBody>,
    /// This has only the requests that were sent to the fake server to unpin messages.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub unpinned_chat_messages: Vec<UnpinChatMessageBody>,
    pub unpinned_all_chat_messages: Vec<UnpinAllChatMessagesBody>,
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

#[derive(Default)]
struct StopHandle {
    inner: parking_lot::Mutex<Option<ServerHandle>>,
}

impl StopHandle {
    /// Sets the server handle to stop.
    pub(crate) fn register(&self, handle: ServerHandle) {
        *self.inner.lock() = Some(handle);
    }

    /// Sends stop signal through contained server handle.
    pub(crate) fn stop(&self, graceful: bool) {
        #[allow(clippy::let_underscore_future)]
        let _ = self.inner.lock().as_ref().unwrap().stop(graceful);
    }
}

async fn stop(Path(graceful): Path<bool>, stop_handle: web::Data<StopHandle>) -> HttpResponse {
    stop_handle.stop(graceful);
    HttpResponse::NoContent().finish()
}

pub async fn main(port: Mutex<u16>) {
    // MESSAGES don't care if they are cleaned or not
    *RESPONSES.lock().unwrap() = Responses::default();

    let pong = reqwest::get(format!("http://127.0.0.1:{}/ping", port.lock().unwrap())).await;

    if pong.is_err()
    // If it errored, no server is running, we need to start it
    {
        let stop_handle = web::Data::new(StopHandle::default());
        // let _ = env_logger::builder()
        //     .filter_level(log::LevelFilter::Info)
        //     .format_target(false)
        //     .format_timestamp(None)
        //     .try_init();
        let server = HttpServer::new({
            let stop_handle = stop_handle.clone();

            move || {
                App::new()
                    // .wrap(Logger::default())
                    .app_data(stop_handle.clone())
                    .route("/ping", web::get().to(ping))
                    .route("/stop/{graceful}", web::post().to(stop))
                    .route("/bot{token}/GetFile", web::post().to(get_file))
                    .route("/bot{token}/SendMessage", web::post().to(send_message))
                    .route("/bot{token}/SendPhoto", web::post().to(send_photo))
                    .route("/bot{token}/SendVideo", web::post().to(send_video))
                    .route("/bot{token}/SendDocument", web::post().to(send_document))
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
                    .route(
                        "/bot{token}/PinChatMessage",
                        web::post().to(pin_chat_message),
                    )
                    .route(
                        "/bot{token}/UnpinChatMessage",
                        web::post().to(unpin_chat_message),
                    )
                    .route(
                        "/bot{token}/UnpinAllChatMessages",
                        web::post().to(unpin_all_chat_messages),
                    )
                    .route("/file/bot{token}/{file_name}", web::get().to(download_file))
            }
        })
        .bind(format!("127.0.0.1:{}", port.lock().unwrap().to_string()))
        .unwrap()
        .workers(1)
        .run();

        stop_handle.register(server.handle());

        server.await.unwrap();
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dataset::*;
    use serial_test::serial;
    use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

    #[test]
    #[serial]
    fn test_add_messages() {
        MESSAGES.lock().unwrap().clear();
        LAST_MESSAGE_ID.store(0, Ordering::Relaxed);
        MESSAGES.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(1)
                .build(),
        );
        MESSAGES.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(2)
                .build(),
        );
        MESSAGES.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(3)
                .build(),
        );
        assert_eq!(MESSAGES.max_message_id(), 3);
    }

    #[test]
    #[serial]
    fn test_edit_messages() {
        MESSAGES.lock().unwrap().clear();
        MESSAGES.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(1)
                .build(),
        );
        MESSAGES.edit_message(1, "text", "1234");
        assert_eq!(MESSAGES.get_message(1).unwrap().text().unwrap(), "1234");
    }

    #[test]
    #[serial]
    fn test_get_messages() {
        MESSAGES.lock().unwrap().clear();
        MESSAGES.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(1)
                .build(),
        );
        assert_eq!(MESSAGES.get_message(1).unwrap().text().unwrap(), "123");
    }

    #[test]
    #[serial]
    fn test_delete_messages() {
        MESSAGES.lock().unwrap().clear();
        MESSAGES.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(1)
                .build(),
        );
        MESSAGES.delete_message(1);
        assert_eq!(MESSAGES.get_message(1), None);
    }

    #[test]
    #[serial]
    fn test_edit_message_reply_markup() {
        MESSAGES.lock().unwrap().clear();
        MESSAGES.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(1)
                .build(),
        );
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