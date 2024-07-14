use teloxide::prelude::*;
use teloxide::types::KeyboardRemove;
use teloxide::{macros::BotCommands, payloads::SendMessageSetters};

use crate::keyboards::menu_keyboard;
use crate::{keyboards, text, HandlerResult, MyDialogue, State};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum StartCommand {
    #[command()]
    Start,
    Cancel,
}

pub async fn start(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, text::START)
        .reply_markup(keyboards::menu_keyboard())
        .await?;
    dialogue.update(State::Start).await?;
    Ok(())
}

pub async fn cancel(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, text::CANCELED).await?;
    bot.send_message(msg.chat.id, text::MENU)
        .reply_markup(keyboards::menu_keyboard())
        .await?;
    dialogue.update(State::Start).await?;
    Ok(())
}

async fn send_menu(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, text::MENU)
        .reply_markup(menu_keyboard())
        .await?;
    dialogue.update(State::Start).await?;
    Ok(())
}

pub async fn profile(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, text::PROFILE).await?;
    Ok(())
}

pub async fn change_nickname(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, text::CHANGE_NICKNAME)
        .reply_markup(KeyboardRemove::new())
        .await?;
    dialogue.update(State::ChangeNickname).await?;
    Ok(())
}

pub async fn changed_nickname(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    let text = match msg.text() {
        Some(text) => text,
        None => {
            bot.send_message(msg.chat.id, text::PLEASE_SEND_TEXT)
                .await?;
            return Ok(());
        }
    };
    bot.send_message(msg.chat.id, text::CHANGED_NICKNAME.to_owned() + text)
        .await?;
    send_menu(bot, msg, dialogue).await
}

pub async fn delete_phrase(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, text::DELETE_PHRASE)
        .reply_markup(KeyboardRemove::new())
        .await?;
    dialogue.update(State::WhatPhraseToDelete).await?;
    Ok(())
}

pub async fn deleted_phrase(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    let _number = match msg.text() {
        Some(text) => match text.trim().parse::<usize>() {
            Ok(number) => number,
            Err(_) => {
                bot.send_message(msg.chat.id, text::PLEASE_SEND_NUMBER)
                    .await?;
                return Ok(());
            }
        },
        None => {
            bot.send_message(msg.chat.id, text::PLEASE_SEND_TEXT)
                .await?;
            return Ok(());
        }
    };
    bot.send_message(msg.chat.id, text::DELETED_PHRASE).await?;
    send_menu(bot, msg, dialogue).await
}

#[cfg(test)]
mod tests {
    use crate::{get_bot_storage, handler_tree::handler_tree};

    use super::*;
    use dptree::deps;
    use teloxide::types::ReplyMarkup;
    use teloxide_tests::{MockBot, MockMessageDocument, MockMessageText};

    #[tokio::test]
    async fn test_start() {
        let bot = MockBot::new(MockMessageText::new().text("/start"), handler_tree());

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::Start).await;

        bot.dispatch_and_check_last_text_and_state(text::START, State::Start)
            .await;
        let responces = bot.get_responses();
        assert_eq!(
            responces
                .sent_messages_text
                .last()
                .unwrap()
                .bot_request
                .reply_markup,
            Some(ReplyMarkup::Keyboard(keyboards::menu_keyboard()))
        );
    }

    #[tokio::test]
    async fn test_cancel() {
        // Cancel is universal, so only one test is needed
        let bot = MockBot::new(MockMessageText::new().text("/cancel"), handler_tree());

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::ChangeNickname).await;

        bot.dispatch_and_check_last_text_and_state(text::MENU, State::Start)
            .await;
        let responces = bot.get_responses();
        assert_eq!(
            responces
                .sent_messages_text
                .last()
                .unwrap()
                .bot_request
                .reply_markup,
            Some(ReplyMarkup::Keyboard(keyboards::menu_keyboard())) // Just checking the keyboard
        );

        assert_eq!(
            responces.sent_messages.first().unwrap().text(),
            Some(text::CANCELED)
        );
    }

    #[tokio::test]
    async fn test_profile() {
        let bot = MockBot::new(
            MockMessageText::new().text(keyboards::PROFILE_BUTTON),
            handler_tree(),
        );

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::Start).await;

        bot.dispatch_and_check_last_text(text::PROFILE).await;
    }

    #[tokio::test]
    async fn test_change_nickname() {
        let bot = MockBot::new(
            MockMessageText::new().text(keyboards::CHANGE_NICKNAME_BUTTON),
            handler_tree(),
        );

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::Start).await;

        bot.dispatch_and_check_last_text_and_state(text::CHANGE_NICKNAME, State::ChangeNickname)
            .await;
    }

    #[tokio::test]
    async fn test_changed_nickname() {
        let bot = MockBot::new(MockMessageText::new().text("nickname"), handler_tree());

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::ChangeNickname).await;

        bot.dispatch_and_check_last_text_and_state(text::MENU, State::Start)
            .await;
        let responces = bot.get_responses();
        assert_eq!(
            responces
                .sent_messages_text
                .last()
                .unwrap()
                .bot_request
                .reply_markup,
            Some(ReplyMarkup::Keyboard(keyboards::menu_keyboard()))
        );
        assert_eq!(
            responces.sent_messages.first().unwrap().text(),
            Some(text::CHANGED_NICKNAME.to_owned() + "nickname").as_deref()
        );
    }

    #[tokio::test]
    async fn test_delete_phrase() {
        let bot = MockBot::new(
            MockMessageText::new().text(keyboards::REMOVE_PHRASE_BUTTON),
            handler_tree(),
        );

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::Start).await;

        bot.dispatch_and_check_last_text_and_state(text::DELETE_PHRASE, State::WhatPhraseToDelete)
            .await;
    }

    #[tokio::test]
    async fn test_deleted_phrase() {
        let bot = MockBot::new(MockMessageText::new().text("not a number"), handler_tree());

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::WhatPhraseToDelete).await;

        bot.dispatch_and_check_last_text(text::PLEASE_SEND_NUMBER)
            .await;
        bot.update(MockMessageDocument::new());
        bot.dispatch_and_check_last_text(text::PLEASE_SEND_TEXT)
            .await;
        bot.update(MockMessageText::new().text("1"));
        bot.dispatch_and_check_last_text_and_state(text::MENU, State::Start)
            .await;
        let responces = bot.get_responses();
        assert_eq!(
            responces.sent_messages.first().unwrap().text(),
            Some(text::DELETED_PHRASE)
        );
    }
}
