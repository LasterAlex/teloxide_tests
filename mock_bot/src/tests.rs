use super::*;
use dataset::*;
use serde::{Deserialize, Serialize};
use teloxide::{
    dispatching::dialogue::{self, InMemStorage},
    dptree::deps,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
enum State {
    #[default]
    Start,
    NotStart,
}

type MyDialogue = Dialogue<State, InMemStorage<State>>;

async fn handler(
    bot: Bot,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    bot.send_message(msg.chat.id, msg.text().unwrap()).await?;
    Ok(())
}

async fn handler_with_state(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    bot.send_message(msg.chat.id, msg.text().unwrap()).await?;

    dialogue.update(State::NotStart).await?;
    Ok(())
}

fn get_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dptree::entry().branch(Update::filter_message().endpoint(handler))
}

fn get_dialogue_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(Update::filter_message().endpoint(handler_with_state))
}

#[tokio::test]
async fn test_echo_hello() {
    let bot = MockBot::new(MockMessageText::new("hello"), get_schema());

    bot.dispatch().await;

    let last_response = bot.get_responses().sent_messages.pop().unwrap();

    assert_eq!(last_response.message.text(), Some("hello"));
}

#[tokio::test]
async fn test_echo_hi() {
    let bot = MockBot::new(MockMessageText::new("hi"), get_schema());

    bot.dispatch().await;

    let last_response = bot.get_responses().sent_messages.pop().unwrap();

    assert_eq!(last_response.message.text(), Some("hi"));
}

#[tokio::test]
async fn test_echo_with_state() {
    let bot = MockBot::new(MockMessageText::new("test"), get_dialogue_schema());
    let storage = InMemStorage::<State>::new();

    bot.dependencies(deps![storage.clone()]);
    bot.dispatch().await;

    let last_response = bot.get_responses().sent_messages.pop().unwrap();
    let state: State = bot.get_state(storage).await;
    assert_eq!(state, State::NotStart);

    assert_eq!(last_response.message.text(), Some("test"));
}
