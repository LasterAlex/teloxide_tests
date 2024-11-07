//! A fake telegram bot API for testing purposes. Read more in teloxide_tests crate.
pub mod routes;
use actix_web::{dev::ServerHandle, web, App, HttpResponse, HttpServer, Responder};
use actix_web_lab::extract::Path;
use lazy_static::lazy_static;
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
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Mutex,
};
use teloxide::types::{File, Me, Message, MessageId, ReplyMarkup};
use tokio_util::sync::CancellationToken;

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
pub struct SentMessageAudio {
    pub message: Message,
    pub bot_request: SendMessageAudioBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageVoice {
    pub message: Message,
    pub bot_request: SendMessageVoiceBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageVideoNote {
    pub message: Message,
    pub bot_request: SendMessageVideoNoteBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageDocument {
    pub message: Message,
    pub bot_request: SendMessageDocumentBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageAnimation {
    pub message: Message,
    pub bot_request: SendMessageAnimationBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageLocation {
    pub message: Message,
    pub bot_request: SendMessageLocationBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageVenue {
    pub message: Message,
    pub bot_request: SendMessageVenueBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageContact {
    pub message: Message,
    pub bot_request: SendMessageContactBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageDice {
    pub message: Message,
    pub bot_request: SendMessageDiceBody,
}

#[derive(Clone, Debug)]
pub struct SentMessagePoll {
    pub message: Message,
    pub bot_request: SendMessagePollBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageSticker {
    pub message: Message,
    pub bot_request: SendMessageStickerBody,
}

#[derive(Clone, Debug)]
pub struct SentMediaGroup {
    pub messages: Vec<Message>,
    pub bot_request: SendMediaGroupBody,
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

#[derive(Clone, Debug)]
pub struct ForwardedMessage {
    pub message: Message,
    pub bot_request: ForwardMessageBody,
}

#[derive(Clone, Debug)]
pub struct CopiedMessage {
    pub message_id: MessageId,
    pub bot_request: CopyMessageBody,
}

#[derive(Clone, Debug)]
pub struct SetMessageReaction {
    pub bot_request: SetMessageReactionBody,
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

    /// This has only messages that are audio messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_audio: Vec<SentMessageAudio>,

    /// This has only messages that are voice messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_voice: Vec<SentMessageVoice>,

    /// This has only messages that are video note messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_video_note: Vec<SentMessageVideoNote>,

    /// This has only messages that are document messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_document: Vec<SentMessageDocument>,

    /// This has only messages that are animation messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_animation: Vec<SentMessageAnimation>,

    /// This has only messages that are location messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_location: Vec<SentMessageLocation>,

    /// This has only messages that are venue messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_venue: Vec<SentMessageVenue>,

    /// This has only messages that are contact messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_contact: Vec<SentMessageContact>,

    /// This has only messages that are dice messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_dice: Vec<SentMessageDice>,

    /// This has only messages that are poll messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_poll: Vec<SentMessagePoll>,

    /// This has only messages that are stickers, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_sticker: Vec<SentMessageSticker>,

    /// This has only messages that are media group messages, sent by the bot.
    /// The `.messages` field has the sent by bot messages, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_media_group: Vec<SentMediaGroup>,

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

    /// This has only the requests that were sent to the fake server to forward messages.
    /// The `.message` field has the forwarded message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub forwarded_messages: Vec<ForwardedMessage>,

    /// This has only the requests that were sent to the fake server to copy messages.
    /// The `.message_id` field has the copied message id, and `.bot_request`
    /// has the request that was sent to the fake server
    pub copied_messages: Vec<CopiedMessage>,

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

    /// This has only the requests that were sent to the fake server to unpin all messages.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub unpinned_all_chat_messages: Vec<UnpinAllChatMessagesBody>,

    /// This has only the requests that were sent to the fake server to ban chat members.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub banned_chat_members: Vec<BanChatMemberBody>,

    /// This has only the requests that were sent to the fake server to unban chat members.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub unbanned_chat_members: Vec<UnbanChatMemberBody>,

    /// This has only the requests that were sent to the fake server to restrict chat members.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub restricted_chat_members: Vec<RestrictChatMemberBody>,

    /// This has only the requests that were sent to the fake server to send chat actions.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub sent_chat_actions: Vec<SendChatActionBody>,

    /// This has only the requests that were sent to the fake server to set message reactions.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub set_message_reaction: Vec<SetMessageReaction>,
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

#[allow(dead_code)]
pub async fn log_request(body: web::Json<serde_json::Value>) -> impl Responder {
    dbg!(body);
    HttpResponse::Ok()
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

pub async fn main(port: Mutex<u16>, me: Me, cancel_token: CancellationToken) {
    // MESSAGES don't care if they are cleaned or not
    *RESPONSES.lock().unwrap() = Responses::default();

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
                // .wrap(actix_web::middleware::Logger::default())
                .app_data(stop_handle.clone())
                .app_data(web::Data::new(me.clone()))
                .route("/ping", web::get().to(ping))
                .route("/stop/{graceful}", web::post().to(stop))
                .route("/bot{token}/GetFile", web::post().to(get_file))
                .route("/bot{token}/SendMessage", web::post().to(send_message))
                .route("/bot{token}/SendPhoto", web::post().to(send_photo))
                .route("/bot{token}/SendVideo", web::post().to(send_video))
                .route("/bot{token}/SendVoice", web::post().to(send_voice))
                .route("/bot{token}/SendAudio", web::post().to(send_audio))
                .route("/bot{token}/SendVideoNote", web::post().to(send_video_note))
                .route("/bot{token}/SendDocument", web::post().to(send_document))
                .route("/bot{token}/SendAnimation", web::post().to(send_animation))
                .route("/bot{token}/SendLocation", web::post().to(send_location))
                .route("/bot{token}/SendVenue", web::post().to(send_venue))
                .route("/bot{token}/SendContact", web::post().to(send_contact))
                .route("/bot{token}/SendSticker", web::post().to(send_sticker))
                .route(
                    "/bot{token}/SendChatAction",
                    web::post().to(send_chat_action),
                )
                .route("/bot{token}/SendDice", web::post().to(send_dice))
                .route("/bot{token}/SendPoll", web::post().to(send_poll))
                .route(
                    "/bot{token}/SendMediaGroup",
                    web::post().to(send_media_group),
                )
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
                    "/bot{token}/ForwardMessage",
                    web::post().to(forward_message),
                )
                .route("/bot{token}/CopyMessage", web::post().to(copy_message))
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
                .route("/bot{token}/BanChatMember", web::post().to(ban_chat_member))
                .route(
                    "/bot{token}/UnbanChatMember",
                    web::post().to(unban_chat_member),
                )
                .route(
                    "/bot{token}/RestrictChatMember",
                    web::post().to(restrict_chat_member),
                )
                .route(
                    "/bot{token}/SetMessageReaction",
                    web::post().to(set_message_reaction),
                )
                .route("/bot{token}/SetMyCommands", web::post().to(set_my_commands))
                .route("/file/bot{token}/{file_name}", web::get().to(download_file))
        }
    })
    .bind(format!("127.0.0.1:{}", port.lock().unwrap().to_string()))
    .unwrap()
    .workers(1)
    .run();

    stop_handle.register(server.handle());

    let server_handle = server.handle();

    tokio::spawn(async move {
        cancel_token.cancelled().await;
        server_handle.stop(false).await;
    });

    server.await.unwrap();
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
