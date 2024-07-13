//! This crate aims to make a mocked bot for unit testing teloxide bots with an actual fake server!
//!
//! [[`examples/hello_world`](https://github.com/LasterAlex/teloxide_tests/tree/master/examples/hello_world)]
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
//! complete, but you still can use it for what it has! Right now this bot supports such endpoints:
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
//!
//! (do not worry about /GetMe and /GetUpdates, they are not needed!)
//!
//! And also fake file downloading!
//!
//! If you see something that works in teloxide, but doesn't work in this crate, while it should
//! (a missing endpoint doesn't qualify as a bug), please open an issue on the GitHub repo!
//! All feedback and suggestions are very welcome!
//! You can contact me via telegram: [@laster_alex](https://t.me/laster_alex)
//!
//! And huge thanks to the teloxide team, their code was amazing to work with! Without all of the
//! code comments, i would've gone insane. The crate itself is also just amazing!
//!
//! To start, i recommend you look at the [[`examples github`]](https://github.com/LasterAlex/teloxide_tests/tree/master/examples) folder
//! Or MockBot struct documentation
//!
//! The only thing you need to change in your existing bot is to shift your dptree to some function, because the
//! sole reason for this crates existance is to test that tree, and we need easy access to it!
//! Just follow the examples!
//!
pub mod mock_bot;
#[cfg(test)]
mod tests;

pub use dataset::*;
pub use mock_bot::MockBot;
