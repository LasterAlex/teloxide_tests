//! This is a copy of the repo
//! https://github.com/LasterAlex/AlbumTeloxideBot/blob/main/src/main.rs
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

use dotenv::dotenv;
use teloxide::dispatching::UpdateHandler;
use teloxide::prelude::*;
use teloxide::types::{
    InputFile, InputMedia, InputMediaAudio, InputMediaDocument, InputMediaPhoto, InputMediaVideo,
    UpdateKind,
};
use tokio::time::{sleep, Duration};

type AlbumStorage = Arc<Mutex<HashMap<String, Vec<Message>>>>;
// Arc allows data to be shared between threads/async functions
// Mutex ensures that only one thread can modify the data at a time
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

fn get_album_storage_for_chat(chat_id: ChatId, album: AlbumStorage) -> Vec<Message> {
    album // Just Arc Mutex things
        .lock()
        .unwrap()
        .get(&chat_id.to_string())
        .unwrap()
        .to_owned()
}

async fn get_album(msg: Message, album: AlbumStorage) -> Option<Vec<Message>> {
    // Insert message into album

    album // Just Arc Mutex things
        .lock()
        .unwrap()
        .entry(msg.chat.id.to_string())
        .or_insert_with(Vec::new) // If there is no entry
        .push(msg.clone());

    // Record length
    let prev_length = get_album_storage_for_chat(msg.chat.id, album.clone()).len();

    sleep(Duration::from_millis(100)).await; // Latency to get new albums

    // Because it is an Arc Mutex, the items are updated, and we can just get it again
    let new_len = get_album_storage_for_chat(msg.chat.id, album.clone()).len();

    // If the length of the album changed, return None, as it is not the last message
    if new_len != prev_length {
        return None;
    }

    // If length did change, all the album messages are recieved, and we can return the album
    let final_album = get_album_storage_for_chat(msg.chat.id, album.clone());
    album.lock().unwrap().remove(&msg.chat.id.to_string()); // Remove album, because all messages have been recieved
    Some(final_album)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct DefaultKey(ChatId); // Copied from source code

fn default_distribution_function(update: &Update) -> Option<DefaultKey> {
    if let UpdateKind::Message(message) = &update.kind {
        if message.media_group_id().is_some() {
            return None; // For some reason teloxide wants to process media group messages sequentially, or synchronously,
                         // and this code tells it "No grouping, you process them asynchronously all at once", so that the Latency in
                         // the get_album function actually waits for new updates, not just haulting everything
                         // because the handler didn't return yet
        }
    }
    update.chat().map(|c| c.id).map(DefaultKey) // If there is no media group, return to default
                                                // sequential grouping
}

fn handler_tree() -> UpdateHandler<Box<dyn Error + Send + Sync + 'static>> {
    dptree::entry().branch(Update::filter_message().endpoint(example_handler))
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file

    let bot = Bot::from_env();

    // Create album mutex
    let album_messages: AlbumStorage = Arc::new(Mutex::new(HashMap::new()));

    Dispatcher::builder(bot, handler_tree())
        .dependencies(dptree::deps![
            // Add storages or other dependencies here, it will work as expected
            album_messages // By adding this arc mutex to dependencies, all handlers can access it
                           // just by adding a parameter of the same type, dptree takes care of it
        ])
        .distribution_function(default_distribution_function)
        // Change the distribution function from default to ours
        .build()
        .dispatch()
        .await;
}

async fn example_handler(bot: Bot, msg: Message, album_mutex: AlbumStorage) -> HandlerResult {
    let album = get_album(msg.clone(), album_mutex).await; // Get either all the messages, or
                                                           // None, which means that it is not the last message in the album, and we chould return
    let album_messages: Vec<Message>; // Uninitialized variable, so that scoping is correct
    match album {
        Some(album_unwrapped) => album_messages = album_unwrapped,
        None => return Ok(()), // If not the last message, return
    }

    // Now we have all the media group messages in the album_messages variable
    // And parameter msg is the last message in the album
    // So, if there is a media group, let's send it back to the user
    if msg.media_group_id().is_some() {
        let mut media_group = vec![];
        let mut caption = Some(format!("Detected {} messages!", album_messages.len()));
        for message in album_messages {
            // Add messages to media group
            if let Some(audio) = message.audio() {
                let mut input = InputMediaAudio::new(InputFile::file_id(audio.file.id.clone()));
                input.caption = caption.clone();
                media_group.push(InputMedia::Audio(input));
            }
            if let Some(document) = message.document() {
                let mut input =
                    InputMediaDocument::new(InputFile::file_id(document.file.id.clone()));
                input.caption = caption.clone();
                media_group.push(InputMedia::Document(input));
            }
            if let Some(photo) = message.photo() {
                let mut input =
                    InputMediaPhoto::new(InputFile::file_id(photo.last().unwrap().file.id.clone()));
                input.caption = caption.clone();
                media_group.push(InputMedia::Photo(input));
            }
            if let Some(video) = message.video() {
                let mut input = InputMediaVideo::new(InputFile::file_id(video.file.id.clone()));
                input.caption = caption.clone();
                media_group.push(InputMedia::Video(input));
            }
            // Only four message types can be in a media group, we don't need to add anything else

            caption = None; // So that only the first message in the album has a caption
        }
        bot.send_media_group(msg.chat.id, media_group).await?;
    } else {
        // If no media group, just send a text message
        bot.send_message(
            msg.chat.id,
            format!(
                "Detected {} messages without media group!",
                album_messages.len(),
            ),
        )
        .await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use teloxide::dptree::deps;
    use teloxide_tests::{MockBot, MockMessagePhoto, MockMessageText};

    #[tokio::test]
    async fn test_get_one_message() {
        let bot = MockBot::new(MockMessagePhoto::new(), handler_tree());
        let album_storage: AlbumStorage = Arc::new(Mutex::new(HashMap::new()));

        bot.dependencies(deps![album_storage]);
        bot.dispatch_and_check_last_text("Detected 1 messages without media group!")
            .await;
    }

    #[tokio::test]
    async fn test_multiple_text_messages() {
        let bot = MockBot::new(vec![MockMessageText::new(); 3], handler_tree());
        let album_storage: AlbumStorage = Arc::new(Mutex::new(HashMap::new()));

        bot.dependencies(deps![album_storage]);
        // ATTENTION!!! This is NOT how it would work in real life. Because we are simulating
        // an update, there is no distribution function, so in reality you would see that behaviour
        // only if you set the distribution function to always return None.
        // I don't consider it a big deal, the default distribution function is more for real
        // users, who may send multiple messages in a row, for testing no distribution function is
        // fine
        bot.dispatch_and_check_last_text("Detected 3 messages without media group!")
            .await;
        // In reality it would be 3 messages with the text "Detected 1 messages without media group!"
    }

    #[tokio::test]
    async fn test_get_album() {
        // This sends all three messages consecutively, making an album simulation, because
        // telegram would've sent them exactly the same way
        let bot = MockBot::new(
            vec![MockMessagePhoto::new().media_group_id("123"); 3],
            handler_tree(),
        );
        let album_storage: AlbumStorage = Arc::new(Mutex::new(HashMap::new()));

        bot.dependencies(deps![album_storage]);

        bot.dispatch().await;

        let responses = bot.get_responses();
        let sent_media_group = responses.sent_media_group.last().unwrap(); // This is the sent
                                                                           // media group
        let sent_messages = responses.sent_messages;
        assert_eq!(
            // Only the first message has a caption
            sent_media_group.messages.first().unwrap().caption(),
            Some("Detected 3 messages!")
        );
        assert_eq!(sent_media_group.messages.len(), 3);
        assert_eq!(sent_messages.len(), 3);  // Just a sanity check
    }
}
