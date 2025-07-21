use teloxide::{prelude::*, types::ParseMode};

use crate::{db, text, HandlerResult};

pub async fn bot_phrase(bot: Bot, msg: Message) -> HandlerResult {
    if let Some(reply) = msg.reply_to_message() {
        if let Some(text) = msg.text() {
            let user_from = msg.from.clone().unwrap();
            let reply_from = reply.from.clone().unwrap();
            let user_from_id = user_from.clone().id.0 as i64;
            let reply_from_id = reply_from.clone().id.0 as i64;
            let user_phrases = db::get_user_phrases(user_from_id).unwrap();
            // Gets all the phrases and tries to find a matching one in the db
            let phrase = user_phrases
                .iter()
                .find(|phrase| phrase.text.to_lowercase() == text.to_lowercase());

            if let Some(phrase) = phrase {
                // If successfull, start making the test string
                let raw_text = format!("{} | {}", phrase.emoji, phrase.bot_text);

                let me_user = db::get_user(user_from_id);
                let reply_user = db::get_user(reply_from_id);

                let me_nickname = match me_user {
                    Ok(user) => user.nickname.unwrap_or(user_from.full_name()),
                    Err(_) => user_from.full_name(),
                };

                let reply_nickname = match reply_user {
                    Ok(user) => user.nickname.unwrap_or(reply_from.full_name()),
                    Err(_) => reply_from.full_name(),
                };

                let me_link = text::make_link(me_nickname, user_from_id as u64);
                let reply_link = text::make_link(reply_nickname, reply_from_id as u64);

                bot.send_message(
                    msg.chat.id,
                    raw_text
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
    use teloxide_tests::{MockBot, MockGroupChat, MockMessageText, MockUser};

    use crate::{db, dptree::deps, get_bot_storage, handler_tree::handler_tree, text};

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

        let mut bot = MockBot::new(me_message, handler_tree());
        bot.dependencies(deps![get_bot_storage().await]);
        // !!! IMPORTANT !!! same as in test_delete_phrase in private handlers, do all db stuff
        // after creating the bot
        db::full_user_redeletion(1234, Some("nick1".to_string()));
        db::full_user_redeletion(5678, Some("nick2".to_string()));
        db::create_phrase(
            1234,
            "🤗".to_string(),
            "hug".to_string(),
            "(me) hugged (reply)".to_string(),
        )
        .unwrap();

        // Parse mode doesn't yet work, so it still has link text. But that isn't a problem!
        bot.dispatch_and_check_last_text(
            &format!(
                "🤗 | {} hugged {}",
                text::make_link("nick1".to_string(), 1234),
                text::make_link("nick2".to_string(), 5678)
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

        let mut bot = MockBot::new(me_message.clone(), handler_tree());
        bot.dependencies(deps![get_bot_storage().await]);
        db::full_user_redeletion(1234, None);
        db::create_phrase(
            1234,
            "🤗".to_string(),
            "hug".to_string(),
            "(me) hugged (reply)".to_string(),
        )
        .unwrap();

        // No text should be sent
        bot.dispatch().await;
        assert!(bot.get_responses().sent_messages.is_empty())
    }
}
