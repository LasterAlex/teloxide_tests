use crate::{get_bot_storage, handlers::*, text, MyDialogue};
use crate::{handlers::StartCommand, State};
use dptree::case;
use std::error::Error;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::dispatching::UpdateFilterExt;
use teloxide::prelude::*;
use teloxide::{
    dispatching::{
        dialogue::{self, ErasedStorage},
        UpdateHandler,
    },
    types::Update,
};

async fn check_if_the_state_is_ok(update: Update) -> bool {
    // This function doesn't have anything to do with tests, but i thought i would put it here,
    // because i've encountered that if you update the state, and the user is on that
    // state, it just errors out, softlocking the user. Very bad.
    let chat_id = match update.chat_id() {
        Some(chat_id) => chat_id,
        None => return true,
    };
    let dialogue = MyDialogue::new(get_bot_storage().await, chat_id);
    match dialogue.get().await {
        Ok(_) => true,
        Err(_) => {
            // This error happens if redis has a state saved for the user, but that state
            // doesn't fit into anything that State has, so it just errors out. Very bad.
            let bot = Bot::from_env();
            bot.send_message(chat_id, text::SORRY_BOT_UPDATED)
                .await
                .unwrap();
            dialogue.update(State::default()).await.unwrap();
            return false;
        }
    }
}

pub fn handler_tree() -> UpdateHandler<Box<dyn Error + Send + Sync + 'static>> {
    // Just a schema, nothing extraordinary
    let normal_branch = dialogue::enter::<Update, ErasedStorage<State>, State, _>()
        .branch(
            Update::filter_message()
                .filter_command::<StartCommand>()
                .branch(case![StartCommand::Start].endpoint(start)),
        )
        .branch(
            Update::filter_callback_query()
                .branch(case![State::WhatDoYouWant].endpoint(what_is_the_first_number)),
        )
        .branch(
            Update::filter_message()
                .branch(
                    case![State::GetFirstNumber { operation }].endpoint(what_is_the_second_number),
                )
                .branch(
                    case![State::GetSecondNumber {
                        first_number,
                        operation
                    }]
                    .endpoint(get_result),
                ),
        );

    // If the dialogue errors out - do not go further
    let catch_updated_dialogue_branch = dptree::entry()
        .filter_async(check_if_the_state_is_ok)
        .branch(normal_branch);

    catch_updated_dialogue_branch
}
