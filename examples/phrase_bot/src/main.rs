pub mod handler_tree;
pub mod handlers;
pub mod text;

use std::error::Error;

use dotenv::dotenv;
use teloxide::dispatching::dialogue::serializer::Cbor;
use teloxide::dispatching::dialogue::{Dialogue, ErasedStorage, RedisStorage, Storage};
use teloxide::prelude::*;
use handler_tree::handler_tree;
use handlers::*;

pub type MyDialogue = Dialogue<State, ErasedStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;
pub type MyStorage = std::sync::Arc<ErasedStorage<State>>;

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Start,
    WhatIsYourNickname,
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
