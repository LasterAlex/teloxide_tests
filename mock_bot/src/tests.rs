use super::*;
use dataset::*;
use serde::{Deserialize, Serialize};
use teloxide::dptree::case;
use teloxide::net::Download;
use teloxide::payloads::SendPhotoSetters;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile, MessageEntity};
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
    #[command()]
    EditReplyMarkup,
    #[command()]
    Photo,
    #[command()]
    Video,
    #[command()]
    Document,
    #[command()]
    EditCaption,
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
        AllCommands::EditReplyMarkup => {
            bot.edit_message_reply_markup(msg.chat.id, sent_message.id)
                .reply_markup(InlineKeyboardMarkup::new(vec![vec![
                    InlineKeyboardButton::callback("test", "test"),
                ]]))
                .await?;
        }
        AllCommands::Photo => {
            let photo = InputFile::memory("somedata".to_string()).file_name("test.jpg");
            bot.send_photo(msg.chat.id, photo)
                .caption("test")
                .caption_entities(vec![MessageEntity::bold(0, 3)])
                .reply_to_message_id(msg.id)
                .await?;
        }
        AllCommands::Video => {
            let video = InputFile::memory("somedata".to_string()).file_name("test.mp4");
            bot.send_video(msg.chat.id, video)
                .caption("test")
                .caption_entities(vec![MessageEntity::bold(0, 3)])
                .reply_to_message_id(msg.id)
                .await?;
        }
        AllCommands::EditCaption => {
            let photo = InputFile::file_id("fileid".to_string());
            let photo_message = bot.send_photo(msg.chat.id, photo).await?;
            bot.edit_message_caption(msg.chat.id, photo_message.id)
                .caption("edited")
                .await?;
        }
        AllCommands::Document => {
            let document = InputFile::file("/home/laster/http_requests.txt".to_string()).file_name("test.txt");
            let document_message = bot
                .send_document(msg.chat.id, document)
                .caption("test")
                .caption_entities(vec![MessageEntity::bold(0, 3)])
                .reply_to_message_id(msg.id)
                .await?;
            let gotten_document = bot
                .get_file(document_message.document().unwrap().file.id.clone())
                .await?;
            assert!(
                gotten_document.meta.unique_id
                    == document_message.document().unwrap().file.unique_id
            );
            let mut dest = tokio::fs::File::create("test.txt").await?;

            bot.download_file(&gotten_document.path, &mut dest).await?;
            assert!(tokio::fs::read_to_string("test.txt").await.is_ok());
            tokio::fs::remove_file("test.txt").await?;
        }
    }
    Ok(())
}

async fn callback_handler(
    bot: Bot,
    call: CallbackQuery,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    bot.answer_callback_query(call.id)
        .text(call.data.unwrap())
        .await?;
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
        .branch(Update::filter_callback_query().endpoint(callback_handler))
}

#[tokio::test]
async fn test_echo() {
    let bot = MockBot::new(MockMessageText::new("/echo echo"), get_schema());

    bot.dispatch().await;

    let last_response = bot.get_responses().sent_messages.pop().unwrap();

    assert_eq!(last_response.text(), Some("/echo echo"));
}

#[tokio::test]
async fn test_send_photo() {
    let bot = MockBot::new(MockMessageText::new("/photo"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_sent_photo = bot.get_responses().sent_messages_photo.pop().unwrap();
    assert_eq!(last_sent_message.caption(), Some("test"));
    assert_eq!(
        last_sent_message.reply_to_message().unwrap().text(),
        Some("/photo")
    );
    assert_eq!(last_sent_message.caption_entities().unwrap().len(), 1);
    assert_eq!(last_sent_photo.bot_request.file_name, "test.jpg");
    assert_eq!(last_sent_photo.bot_request.file_data, "somedata");
}

#[tokio::test]
async fn test_send_video() {
    let bot = MockBot::new(MockMessageText::new("/video"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_sent_photo = bot.get_responses().sent_messages_video.pop().unwrap();
    assert_eq!(last_sent_message.caption(), Some("test"));
    assert_eq!(
        last_sent_message.reply_to_message().unwrap().text(),
        Some("/video")
    );
    assert_eq!(last_sent_message.caption_entities().unwrap().len(), 1);
    assert_eq!(last_sent_photo.bot_request.file_name, "test.mp4");
    assert_eq!(last_sent_photo.bot_request.file_data, "somedata");
}

#[tokio::test]
async fn test_send_document() {
    let bot = MockBot::new(MockMessageText::new("/document"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_sent_photo = bot.get_responses().sent_messages_document.pop().unwrap();
    assert_eq!(last_sent_message.caption(), Some("test"));
    assert_eq!(
        last_sent_message.reply_to_message().unwrap().text(),
        Some("/document")
    );
    assert_eq!(last_sent_message.caption_entities().unwrap().len(), 1);
    assert_eq!(last_sent_photo.bot_request.file_name, "test.txt");
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
async fn test_edit_caption() {
    let bot = MockBot::new(MockMessageText::new("/editcaption"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_edited_response = bot.get_responses().edited_messages_caption.pop().unwrap();

    assert_eq!(last_sent_message.caption(), None);
    assert_eq!(last_edited_response.message.caption(), Some("edited"));
}

#[tokio::test]
async fn test_edit_reply_markup() {
    let bot = MockBot::new(MockMessageText::new("/editreplymarkup"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_edited_response = bot
        .get_responses()
        .edited_messages_reply_markup
        .pop()
        .unwrap();

    assert_eq!(last_sent_message.reply_markup(), None);
    assert_eq!(
        last_edited_response
            .message
            .reply_markup()
            .unwrap()
            .inline_keyboard[0][0]
            .text,
        "test"
    );
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

#[tokio::test]
async fn test_answer_callback_query() {
    let bot = MockBot::new(MockCallbackQuery::new().data("test"), get_schema());

    bot.dispatch().await;

    let answered_callback = bot.get_responses().answered_callback_queries.pop().unwrap();

    assert_eq!(answered_callback.text, Some("test".to_string()));
}
