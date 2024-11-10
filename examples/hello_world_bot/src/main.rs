use teloxide::{
    dispatching::{UpdateFilterExt, UpdateHandler},
    prelude::*,
};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

async fn hello_world(bot: Bot, message: Message) -> HandlerResult {
    bot.send_message(message.chat.id, "Hello World!").await?;
    Ok(())
}

fn handler_tree() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    // A simple handler. But you need to make it into a separate thing!
    dptree::entry().branch(Update::filter_message().endpoint(hello_world))
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
    use teloxide_tests::{MockBot, MockMessageText};

    #[tokio::test]
    async fn test_hello_world() {
        // This is a message builder. You can check the docs for more info about mocked types
        let mock_message = MockMessageText::new().text("Hi!");
        // This creates a fake bot that will send the mock_message after we dispatch it as if it was sent by the user
        // If you wanted, you could've made vec![MockMessageText::new().text("Hi!"), MockMessageText::new().text("Hello!")],
        // and both updates would've been sent one after the other. You also can make a MockMessagePhoto, MockMessageDocument, etc
        let mut bot = MockBot::new(mock_message, handler_tree());
        // This will dispatch the update
        bot.dispatch().await;

        // We can now check the sent messages
        let responses = bot.get_responses(); // This returns a struct that has all of the recieved
                                             // updates and requests. You can treat that function like a variable, because it basically is.
        let message = responses
            .sent_messages // This is a list of all sent messages. Be warned, editing or deleting
            // messages do not affect this list!
            .last()
            .expect("No sent messages were detected!");
        assert_eq!(message.text(), Some("Hello World!"));

        // There is also a more specialized field, sent_messages_text:
        let message_text = responses
            .sent_messages_text // This has a list request bodies and sent messages of only text messages, no photo, audio, etc.
            // messages
            .last()
            .expect("No sent messages were detected!");
        assert_eq!(message_text.message.text(), Some("Hello World!"));
        // The 'bot_request' field is what the bot sent to the fake server. It has some fields that
        // can't be accessed by looking only at the resulted message. For example, drop-down style keyboards can't
        // be seen in the regular message, like the parse_mode.
        assert_eq!(message_text.bot_request.parse_mode, None);
        // Also, it is highly discouraged to use the raw bot fields like bot.updates and bot.bot,
        // abstractions exist for a reason!!! Do not use them unless you know what you are doing!
    }
}
