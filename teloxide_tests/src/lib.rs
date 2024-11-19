//! This crate aims to make a mocked bot for unit testing teloxide bots with an actual fake server!
//!
//! [[`examples/hello_world`](https://github.com/LasterAlex/teloxide_tests/tree/master/examples/hello_world_bot)]
//! ```no_run
//! use teloxide::{
//!     dispatching::{UpdateFilterExt, UpdateHandler},
//!     prelude::*,
//! };
//!
//! type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//!
//! async fn hello_world(bot: Bot, message: Message) -> HandlerResult {
//!     bot.send_message(message.chat.id, "Hello World!").await?;
//!     Ok(())
//! }
//!
//! fn handler_tree() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
//!     dptree::entry().branch(Update::filter_message().endpoint(hello_world))
//! }
//!
//! #[tokio::main]
//! async fn main() {  // A regular bot dispatch
//!     dotenv::dotenv().ok();
//!     let bot = Bot::from_env();
//!     Dispatcher::builder(bot, handler_tree())
//!         .enable_ctrlc_handler()
//!         .build()
//!         .dispatch()
//!         .await;
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!     use teloxide_tests::{MockBot, MockMessageText};
//!
//!     #[tokio::test]
//!     async fn test_hello_world() {  // A testing bot dispatch
//!         let bot = MockBot::new(MockMessageText::new().text("Hi!"), handler_tree());
//!         bot.dispatch().await;
//!         let message = bot.get_responses().sent_messages.last().unwrap();
//!         // This is a regular teloxide::types::Message!
//!         assert_eq!(message.text(), Some("Hello World!"));
//!     }
//! }
//! ```
//!
//! To run the tests, just run `cargo test` in the terminal! It's that easy! No internet connection required!
//!
//!
//! I haven't seen telegram bot testing tools that are up to my standards (for any bot api wrapper, not just teloxide),
//! so I decided to write this. This crate tries to give as much tooling for testing as reasonably possible,
//! while keeping it simple to work with and implement.
//! The goal of this crate is to test most of the teloxide and telegram features. This crate is not yet
//! complete, but you still can use it for what it has!
//!
//! ## Supported Endpoints
//!
//! - /AnswerCallbackQuery
//! - /DeleteMessage
//! - /EditMessageText
//! - /EditMessageReplyMarkup
//! - /EditMessageCaption
//! - /GetFile
//! - /SendMessage
//! - /SendDocument
//! - /SendPhoto
//! - /SendVideo
//! - /SendAudio
//! - /SendVoice
//! - /SendVideoNote
//! - /SendAnimation
//! - /SendLocation
//! - /SendVenue
//! - /SendContact
//! - /SendDice
//! - /SendPoll
//! - /SendSticker
//! - /SendChatAction
//! - /SendMediaGroup
//! - /SendInvoice
//! - /PinChatMessage
//! - /UnpinChatMessage
//! - /UnpinAllChatMessages
//! - /ForwardMessage
//! - /CopyMessage
//! - /BanChatMember
//! - /UnbanChatMember
//! - /RestrictChatMember
//! - /SetMessageReaction
//! - /SetMyCommands
//! - /GetMe
//!
//! More endpoints will be added as time goes on!
//!
//! (/GetUpdates and /GetWebhookInfo exist, but they are dummies)
//!
//! And also fake file downloading!
//!
//! ## Why even use unit tests?
//!
//! I've always found manual bot testing to be very time consuming and unreliable, especially when
//! the bot becomes large and very complex. This crate can solve this problem!
//!
//! As an example, here is a bot that i did not run once before i have written all of the code:
//! [`examples/phrase_bot`](https://github.com/LasterAlex/teloxide_tests/tree/master/examples/phrase_bot)   
//! (dont forget to read the README.md in the examples directory!)
//!
//! ## Other
//!
//! If you see something that works in teloxide, but doesn't work in this crate, while it should
//! (a missing endpoint doesn't qualify as a bug), please open an issue on the [GitHub repo!](https://github.com/LasterAlex/teloxide_tests)
//! All feedback and suggestions are very welcome!
//! Or you can write to the [@teloxide_tests](https://t.me/teloxide_tests) group!
//!
//! And huge thanks to the teloxide team, their code was amazing to work with! Without all of the
//! code comments, i would've gone insane. The crate itself is also just amazing!
//!
//! To start, i recommend you look at the [[`examples github`]](https://github.com/LasterAlex/teloxide_tests/tree/master/examples) folder
//!
//! Or [MockBot struct documentation](https://docs.rs/teloxide_tests/latest/teloxide_tests/mock_bot/struct.MockBot.html)
//!
//! The only thing you need to change in your existing bots is to shift your dptree (handler tree) to some function, because the
//! sole reason for this crates existance is to test that tree, and we need easy access to it!
//! Just follow the examples!
//!
//! And try to not use the raw bot fields unless you know what you are doing! They are public only
//! to give more options to those who seek it.
//!
//! ## **!!! IMPORTANT !!!**
//!
//! If you want to use the database or
//! something that is shared across all tests, DO IT __AFTER__ THE `MockBot::new()`!!!!!
//! The creation of the bot creates a safe lock that prevents other tests from starting, before
//! this bot becomes out of scope.
//! If you encounter issues regarding this, try to manually add `drop(bot);` at the end of your
//! tests!
//! Or use the [serial_test](https://crates.io/crates/serial_test) crate
//!
//! Please refer to the [phrase_bot example](https://github.com/LasterAlex/teloxide_tests/tree/master/examples/phrase_bot) for more information
//!
#![doc(
    html_logo_url = "https://github.com/user-attachments/assets/627beca8-5852-4c70-97e0-5f4fcb5e2040",
    html_favicon_url = "https://github.com/user-attachments/assets/627beca8-5852-4c70-97e0-5f4fcb5e2040"
)]
#![allow(clippy::too_long_first_doc_paragraph)]
#![allow(clippy::to_string_in_format_args)]
#![allow(clippy::new_without_default)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::map_flatten)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::borrow_interior_mutable_const)]
#![allow(clippy::declare_interior_mutable_const)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::search_is_some)]
#![allow(clippy::unwrap_or_default)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::needless_return)]
#![allow(clippy::bool_assert_comparison)]

mod dataset;
pub(crate) mod listener;
pub mod mock_bot;
pub mod server;
pub(crate) mod state;
#[cfg(test)]
mod tests;
pub(crate) mod utils;

pub use dataset::*;
pub use mock_bot::MockBot;
pub use server::Responses;
use teloxide_tests_macros as proc_macros;
