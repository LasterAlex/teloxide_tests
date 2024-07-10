use super::*;
use dataset::*;
use serde::{Deserialize, Serialize};
use teloxide::dptree::case;
use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateFilterExt,
    },
    dptree::deps,
    macros::BotCommands,
};

//
//
//

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
enum State {
    #[default]
    Start,
    NotStart,
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

async fn handler_with_not_start_state(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    bot.send_message(msg.chat.id, "Not start!").await?;

    dialogue.update(State::Start).await?;
    Ok(())
}

fn get_dialogue_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(
        Update::filter_message()
            .branch(case![State::NotStart].endpoint(handler_with_not_start_state))
            .endpoint(handler_with_state),
    )
}

#[tokio::test]
async fn test_echo_with_start_state() {
    let bot = MockBot::new(MockMessageText::new("test"), get_dialogue_schema());
    let storage = InMemStorage::<State>::new();
    bot.dependencies(deps![storage]);
    bot.set_state(State::Start).await;

    bot.dispatch().await;

    let last_response = bot.get_responses().sent_messages.pop().unwrap();
    let state: State = bot.get_state().await;
    assert_eq!(state, State::NotStart);

    assert_eq!(last_response.text(), Some("test"));
}

#[tokio::test]
async fn test_echo_with_not_start_test() {
    let bot = MockBot::new(MockMessageText::new("test"), get_dialogue_schema());
    let storage = InMemStorage::<State>::new();
    bot.dependencies(deps![storage]);
    bot.set_state(State::NotStart).await;

    bot.dispatch().await;

    let last_response = bot.get_responses().sent_messages.pop().unwrap();
    let state: State = bot.get_state().await;
    assert_eq!(state, State::Start);

    assert_eq!(last_response.text(), Some("Not start!"));
}

//
//
//

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum AllCommands {
    #[command()]
    Echo,
    #[command()]
    Edit,
    #[command()]
    Delete,
}

type MyDialogue = Dialogue<State, InMemStorage<State>>;

async fn handler(
    bot: Bot,
    msg: Message,
    cmd: AllCommands,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let sent_message = bot.send_message(msg.chat.id, msg.text().unwrap()).await?;
    assert!(msg.text().unwrap() == sent_message.text().unwrap()); // The message actually made it through!
    match cmd {
        AllCommands::Echo => {}
        AllCommands::Edit => {
            bot.edit_message_text(msg.chat.id, sent_message.id, "edited")
                .await?;
        }
        AllCommands::Delete => {
            bot.delete_message(msg.chat.id, sent_message.id).await?;
        }
    }
    Ok(())
}

fn get_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<AllCommands>()
                .endpoint(handler),
        )
        .branch(Update::filter_message().endpoint(handler))
}

#[tokio::test]
async fn test_echo() {
    let bot = MockBot::new(MockMessageText::new("/echo echo"), get_schema());

    bot.dispatch().await;

    let last_response = bot.get_responses().sent_messages.pop().unwrap();

    assert_eq!(last_response.text(), Some("/echo echo"));
}

#[tokio::test]
async fn test_edit_message() {
    let bot = MockBot::new(MockMessageText::new("/edit"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_edited_response = bot.get_responses().edited_messages_text.pop().unwrap();

    assert_eq!(last_sent_message.text(), Some("/edit"));
    assert_eq!(last_edited_response.message.text(), Some("edited"));
}

#[tokio::test]
async fn test_delete_message() {
    let bot = MockBot::new(MockMessageText::new("/delete"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_deleted_response = bot.get_responses().deleted_messages.pop().unwrap();

    assert_eq!(last_sent_message.text(), Some("/delete"));
    assert_eq!(last_deleted_response.message.id, last_sent_message.id);
}
