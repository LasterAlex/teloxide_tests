use teloxide::{
    dispatching::{UpdateFilterExt, UpdateHandler},
    net::Download,
    prelude::*,
};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

async fn download_document(bot: Bot, message: Message) -> HandlerResult {
    if let Some(document) = message.document() {
        let file = bot.get_file(document.file.id.clone()).await?; // Get the file

        // Make the destination file
        let mut dest = tokio::fs::File::create("test.txt").await?;

        // Download the file (its always a dummy, but it works as a check)
        bot.download_file(&file.path, &mut dest).await?;

        // Just a check that the file was downloaded
        assert!(tokio::fs::read_to_string("test.txt").await.is_ok());

        bot.send_message(message.chat.id, "Downloaded!").await?;

        tokio::fs::remove_file("test.txt").await?; // Just a cleanup
    } else {
        bot.send_message(message.chat.id, "Not a document").await?;
    }
    Ok(())
}

fn handler_tree() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    // A simple handler. But you need to make it into a separate thing!
    dptree::entry().branch(Update::filter_message().endpoint(download_document))
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); // Loads the .env file

    let bot = Bot::from_env();

    Dispatcher::builder(bot, handler_tree())
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use teloxide_tests::{MockBot, MockMessageDocument, MockMessageText};

    #[tokio::test]
    async fn test_not_a_document() {
        let bot = MockBot::new(MockMessageText::new().text("Hi!"), handler_tree());
        bot.dispatch_and_check_last_text("Not a document").await;
    }

    #[tokio::test]
    async fn test_download_document_and_check() {
        let bot = MockBot::new(MockMessageDocument::new(), handler_tree());
        bot.dispatch_and_check_last_text("Downloaded!").await;
    }
}
