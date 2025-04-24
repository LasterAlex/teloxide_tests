use teloxide::{prelude::*, types::Me};

use crate::{add_deep_link, text, HandlerResult, MyDialogue, StartCommand, State};

pub async fn start(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    command: StartCommand,
    me: Me,
) -> HandlerResult {
    let chat_id = msg.chat.id;
    // If you have multiple commands, this will need to become if let
    let StartCommand::Start(arg) = command;
    if arg.is_empty() {
        // This means that it is just a regular link like https://t.me/some_bot
        bot.send_message(chat_id, add_deep_link(text::START, me, chat_id))
            .await?;
        dialogue.update(State::default()).await?;
    } else {
        // And this means that the link is like this: https://t.me/some_bot?start=123456789
        if let Ok(id) = arg.parse::<i64>() {
            bot.send_message(chat_id, text::SEND_YOUR_MESSAGE).await?;
            dialogue.update(State::WriteToSomeone { id }).await?;
        } else {
            bot.send_message(chat_id, text::WRONG_LINK).await?;
            dialogue.update(State::default()).await?;
        }
    }

    Ok(())
}

pub async fn send_message(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    state: State,
    me: Me,
) -> HandlerResult {
    let State::WriteToSomeone { id } = state else {
        // Shouldn't ever happen
        return Ok(());
    };

    if let Some(text) = msg.text() {
        // Trying to send a message to the user
        let sent_result = bot
            .send_message(
                ChatId(id),
                text::YOU_HAVE_A_NEW_MESSAGE.replace("{message}", text),
            )
            .parse_mode(teloxide::types::ParseMode::Html)
            .await;

        // And if no error is returned, success!
        if sent_result.is_ok() {
            bot.send_message(
                msg.chat.id,
                add_deep_link(text::MESSAGE_SENT, me, msg.chat.id),
            )
            .await?;
        } else {
            bot.send_message(msg.chat.id, text::ERROR_SENDING_MESSAGE)
                .await?;
        }
        dialogue.update(State::default()).await?;
    } else {
        // You can add support for more messages yourself!
        bot.send_message(msg.chat.id, text::SEND_TEXT).await?;
    }
    Ok(())
}
