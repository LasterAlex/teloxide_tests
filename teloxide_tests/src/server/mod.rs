//! A fake telegram bot API for testing purposes. Read more in teloxide_tests crate.
pub mod routes;
use actix_web::{
    web::{get, post, scope, Data, Json, ServiceConfig},
    App, HttpResponse, HttpServer, Responder,
};
use lazy_static::lazy_static;
pub use responses::*;
use routes::{
    answer_callback_query::*, ban_chat_member::*, copy_message::*, delete_message::*,
    download_file::download_file, edit_message_caption::*, edit_message_reply_markup::*,
    edit_message_text::*, forward_message::*, get_file::*, pin_chat_message::*,
    restrict_chat_member::*, send_animation::*, send_audio::*, send_chat_action::*,
    send_contact::*, send_dice::*, send_document::*, send_location::*, send_media_group::*,
    send_message::*, send_photo::*, send_poll::*, send_sticker::*, send_venue::*, send_video::*,
    send_video_note::*, send_voice::*, set_message_reaction::*, set_my_commands::*,
    unban_chat_member::*, unpin_all_chat_messages::*, unpin_chat_message::*,
};
use serde::Serialize;
use std::{
    error::Error,
    net::TcpListener,
    sync::{
        atomic::{AtomicI32, Ordering},
        Mutex,
    },
};
use teloxide::types::{File, Me, Message, ReplyMarkup};
use tokio::task::{JoinError, JoinHandle};
use tokio_util::sync::CancellationToken;

pub mod responses;

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

        let mut json = serde_json::to_value(message).ok()?; // Convert the message to JSON
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

#[allow(dead_code)]
pub async fn log_request(body: Json<serde_json::Value>) -> impl Responder {
    dbg!(body);
    HttpResponse::Ok()
}

#[allow(dead_code)]
pub struct ServerManager {
    pub port: u16,
    server: JoinHandle<()>,
    cancel_token: CancellationToken,
}

// #[warn(clippy::unwrap_used)]
impl ServerManager {
    pub async fn start(me: Me) -> Result<Self, Box<dyn Error>> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();

        let cancel_token = CancellationToken::new();

        let cancel_token_clone = cancel_token.clone();
        let _ = ctrlc::set_handler(move || {
            cancel_token_clone.cancel();
            std::process::exit(1);
        });

        let server = tokio::spawn(run_server(listener, me, cancel_token.clone()));

        if let Err(err) = Self::wait_for_server(port).await {
            cancel_token.cancel();
            server.await?;
            return Err(err.into());
        }

        Ok(Self {
            port,
            cancel_token,
            server,
        })
    }

    pub async fn stop(self) -> Result<(), JoinError> {
        self.cancel_token.cancel();
        self.server.await
    }

    async fn wait_for_server(port: u16) -> Result<(), String> {
        let url = format!("http://127.0.0.1:{}/ping", port);
        let max_tries = 200;

        for _ in 0..max_tries {
            if reqwest::get(&url).await.is_ok() {
                return Ok(());
            }
        }

        Err(format!("Failed to get the server on the port {}!", port))
    }
}

async fn run_server(listener: TcpListener, me: Me, cancel_token: CancellationToken) {
    // MESSAGES don't care if they are cleaned or not
    *RESPONSES.lock().unwrap() = Responses::default();

    let server = HttpServer::new({
        move || {
            App::new()
                // .wrap(actix_web::middleware::Logger::default())
                .app_data(Data::new(me.clone()))
                .configure(set_routes)
        }
    })
    .listen(listener)
    .unwrap()
    .workers(1)
    .run();

    let server_handle = server.handle();

    tokio::spawn(async move {
        cancel_token.cancelled().await;
        server_handle.stop(false).await;
    });

    server.await.unwrap();
}

fn set_routes(cfg: &mut ServiceConfig) {
    cfg.route("/ping", get().to(ping))
        .route("/file/bot{token}/{file_name}", get().to(download_file))
        .service(scope("/bot{token}").configure(set_bot_routes));
}

fn set_bot_routes(cfg: &mut ServiceConfig) {
    cfg.route("/GetFile", post().to(get_file))
        .route("/SendMessage", post().to(send_message))
        .route("/SendPhoto", post().to(send_photo))
        .route("/SendVideo", post().to(send_video))
        .route("/SendVoice", post().to(send_voice))
        .route("/SendAudio", post().to(send_audio))
        .route("/SendVideoNote", post().to(send_video_note))
        .route("/SendDocument", post().to(send_document))
        .route("/SendAnimation", post().to(send_animation))
        .route("/SendLocation", post().to(send_location))
        .route("/SendVenue", post().to(send_venue))
        .route("/SendContact", post().to(send_contact))
        .route("/SendSticker", post().to(send_sticker))
        .route("/SendChatAction", post().to(send_chat_action))
        .route("/SendDice", post().to(send_dice))
        .route("/SendPoll", post().to(send_poll))
        .route("/SendMediaGroup", post().to(send_media_group))
        .route("/EditMessageText", post().to(edit_message_text))
        .route("/EditMessageCaption", post().to(edit_message_caption))
        .route(
            "/EditMessageReplyMarkup",
            post().to(edit_message_reply_markup),
        )
        .route("/DeleteMessage", post().to(delete_message))
        .route("/ForwardMessage", post().to(forward_message))
        .route("/CopyMessage", post().to(copy_message))
        .route("/AnswerCallbackQuery", post().to(answer_callback_query))
        .route("/PinChatMessage", post().to(pin_chat_message))
        .route("/UnpinChatMessage", post().to(unpin_chat_message))
        .route("/UnpinAllChatMessages", post().to(unpin_all_chat_messages))
        .route("/BanChatMember", post().to(ban_chat_member))
        .route("/UnbanChatMember", post().to(unban_chat_member))
        .route("/RestrictChatMember", post().to(restrict_chat_member))
        .route("/SetMessageReaction", post().to(set_message_reaction))
        .route("/SetMyCommands", post().to(set_my_commands));
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
