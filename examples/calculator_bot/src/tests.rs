use crate::{get_bot_storage, handler_tree::handler_tree, text, State};

use teloxide::dptree::deps;
use teloxide_tests::{MockBot, MockCallbackQuery, MockMessagePhoto, MockMessageText};

#[tokio::test]
async fn test_start() {
    let mut bot = MockBot::new(MockMessageText::new().text("/start"), handler_tree());

    bot.dependencies(deps![get_bot_storage().await]);
    bot.set_state(State::Start).await;

    bot.dispatch_and_check_last_text_and_state(text::WHAT_DO_YOU_WANT, State::WhatDoYouWant)
        .await;
    // This is a shortcut for:
    //
    // bot.dispatch().await;
    //
    // let state: State = bot.get_state().await;
    // assert_eq!(state, State::WhatDoYouWant);
    //
    // let responses = bot.get_responses();
    // let last_message = responses.sent_messages.last().unwrap();
    // assert_eq!(last_message.text().unwrap(), text::WHAT_DO_YOU_WANT);
    //
}

#[tokio::test]
async fn test_what_is_the_first_number() {
    let mut bot = MockBot::new(MockCallbackQuery::new().data("add"), handler_tree());

    bot.dependencies(deps![get_bot_storage().await]);
    bot.set_state(State::WhatDoYouWant).await;

    bot.dispatch_and_check_last_text_and_state(
        text::ENTER_THE_FIRST_NUMBER,
        State::GetFirstNumber {
            operation: "add".to_owned(),
        },
    )
    .await;
}

#[tokio::test]
async fn test_message_errors() {
    let mut bot = MockBot::new(MockMessageText::new().text("not a number"), handler_tree());

    bot.dependencies(deps![get_bot_storage().await]);
    bot.set_state(State::GetFirstNumber {
        operation: "add".to_owned(),
    })
    .await;

    bot.dispatch_and_check_last_text(text::PLEASE_ENTER_A_NUMBER)
        .await;

    // This makes a new update into the same bot instance, so we can check the second error type
    // using the same state and storage
    bot.update(MockMessagePhoto::new());
    bot.dispatch_and_check_last_text(text::PLEASE_SEND_TEXT)
        .await;
}

#[tokio::test]
async fn test_what_is_the_second_number() {
    let mut bot = MockBot::new(MockMessageText::new().text("5"), handler_tree());

    bot.dependencies(deps![get_bot_storage().await]);
    bot.set_state(State::GetFirstNumber {
        operation: "add".to_owned(),
    })
    .await;

    bot.dispatch_and_check_last_text_and_state(
        text::ENTER_THE_SECOND_NUMBER,
        State::GetSecondNumber {
            first_number: 5,
            operation: "add".to_owned(),
        },
    )
    .await;
}

#[tokio::test]
async fn test_add_result() {
    let mut bot = MockBot::new(MockMessageText::new().text("4"), handler_tree());

    bot.dependencies(deps![get_bot_storage().await]);
    bot.set_state(State::GetSecondNumber {
        first_number: 5,
        operation: "add".to_owned(),
    })
    .await;

    bot.dispatch_and_check_last_text_and_state(
        &(text::YOUR_RESULT.to_owned() + "9"),
        State::default(),
    )
    .await;
}

#[tokio::test]
async fn test_subtract_result() {
    let mut bot = MockBot::new(MockMessageText::new().text("4"), handler_tree());

    bot.dependencies(deps![get_bot_storage().await]);
    bot.set_state(State::GetSecondNumber {
        first_number: 5,
        operation: "subtract".to_owned(),
    })
    .await;

    bot.dispatch_and_check_last_text_and_state(
        &(text::YOUR_RESULT.to_owned() + "1"),
        State::default(),
    )
    .await;
}
