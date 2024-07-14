mod handler_tree;
mod handlers;
pub mod text;
#[cfg(test)]
mod tests;

use std::error::Error;

use dotenv::dotenv;
use teloxide::dispatching::dialogue::serializer::Cbor;
use teloxide::dispatching::dialogue::{Dialogue, ErasedStorage, RedisStorage, Storage};
use teloxide::prelude::*;
use handler_tree::handler_tree;

pub type MyDialogue = Dialogue<State, ErasedStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;
pub type MyStorage = std::sync::Arc<ErasedStorage<State>>;

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Start, // The default state, from which you can send '/start'
    WhatDoYouWant, // We ask, what do you want, to add or subtract
    GetFirstNumber {
        // We got what the user wants to do, and we ask for the first number
        operation: String,
    },
    GetSecondNumber {
        // Now ask for the second number
        first_number: i32,
        operation: String,
    },
}

pub async fn get_bot_storage() -> MyStorage {
    let storage: MyStorage = RedisStorage::open(dotenv::var("REDIS_URL").unwrap(), Cbor)
        // For reasons unknown to me, Binary serializer doesn't accept json-like objects,
        // so im using it. If you want to use InMemStorage, just change
        // ErasedStorage to InMemStorage (dont forget to do it in the handler_tree.rs), 
        // and make this function return InMemStorage::<State>::new()
        .await
        .unwrap()
        .erase();
    storage
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    let bot = Bot::from_env();

    Dispatcher::builder(bot, handler_tree())
        .dependencies(dptree::deps![get_bot_storage().await])
        .build()
        .dispatch()
        .await;
}
