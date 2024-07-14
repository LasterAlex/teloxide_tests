use teloxide::{prelude::*, types::ParseMode};

use crate::{text, HandlerResult};

pub async fn bot_phrase(bot: Bot, msg: Message) -> HandlerResult {
    if let Some(reply) = msg.reply_to_message() {
        if let Some(text) = msg.text() {
            if text == "hug" {
                let bot_text = "🤗 | (me) hugged (reply)";

                let me_link =
                    text::make_link(msg.from().unwrap().full_name(), msg.from().unwrap().id.0);
                let reply_link = text::make_link(
                    reply.from().unwrap().full_name(),
                    reply.from().unwrap().id.0,
                );

                bot.send_message(
                    msg.chat.id,
                    bot_text
                        .replace("(me)", &me_link)
                        .replace("(reply)", &reply_link),
                )
                .parse_mode(ParseMode::Html)
                .await?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{handler_tree::handler_tree, text};
    use teloxide_tests::{MockBot, MockGroupChat, MockMessageText, MockUser};

    #[tokio::test]
    async fn test_phrase() {
        let chat = MockGroupChat::new().build();

        let reply_message = MockMessageText::new()
            .text("some reply message")
            .chat(chat.clone())
            .from(MockUser::new().first_name("reply").id(5678).build());

        let me_message = MockMessageText::new()
            .text("hug")
            .chat(chat.clone())
            .from(MockUser::new().first_name("me").id(1234).build())
            .reply_to_message(reply_message.build());

        let bot = MockBot::new(me_message, handler_tree());

        // Parse mode doesn't yet work, so it still has link text. But that isn't a problem!
        bot.dispatch_and_check_last_text(
            &format!(
                "🤗 | {} hugged {}",
                text::make_link("me".to_string(), 1234),
                text::make_link("reply".to_string(), 5678)
            )
            .to_string(),
        )
        .await;
    }

    #[tokio::test]
    async fn test_no_phrase() {
        let chat = MockGroupChat::new().build();

        let me_message = MockMessageText::new()
            .text("hug")
            .chat(chat.clone())
            .from(MockUser::new().first_name("me").id(1234).build());

        let bot = MockBot::new(me_message.clone(), handler_tree());

        // No text should be sent
        bot.dispatch().await;
        assert!(bot.get_responses().sent_messages.is_empty())
    }
}
