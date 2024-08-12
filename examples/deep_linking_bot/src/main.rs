pub mod handler_tree;
pub mod handlers;
#[cfg(test)]
pub mod tests;
pub mod text;

use dptree::deps;
use handler_tree::handler_tree;
use std::error::Error;
use teloxide::types::Me;
use teloxide::{dispatching::dialogue::InMemStorage, macros::BotCommands, prelude::*};

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Start,
    WriteToSomeone {
        id: i64,
    },
}

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum StartCommand {
    #[command()]
    Start(String), // Because deep linking (links like https://t.me/some_bot?start=123456789) is the
                   // same as sending "/start 123456789", we can treat it as just an argument to a command
                   //
                   // https://core.telegram.org/bots/features#deep-linking
}

pub fn add_deep_link(text: &str, me: Me, chat_id: ChatId) -> String {
    text.replace(
        "{deep_link}", // Just a shortcut to not write it multiple times
        format!("{}?start={}", me.tme_url(), chat_id.0).as_str(),
    )
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); // Loads the .env file

    let bot = Bot::from_env();

    Dispatcher::builder(bot, handler_tree())
        .dependencies(deps![InMemStorage::<State>::new()])
        .build()
        .dispatch()
        .await;
}
