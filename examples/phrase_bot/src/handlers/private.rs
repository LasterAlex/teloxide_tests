use teloxide::prelude::*;
use teloxide::types::KeyboardRemove;
use teloxide::macros::BotCommands;

use crate::{keyboards, text, HandlerResult, MyDialogue, State};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum StartCommand {
    #[command()]
    Start,
}

pub async fn start(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, text::START)
        .reply_markup(keyboards::menu_keyboard())
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

#[cfg(test)]
mod tests {
    use crate::{get_bot_storage, handler_tree::handler_tree};

    use super::*;
    use dptree::deps;
    use teloxide::types::ReplyMarkup;
    use teloxide_tests::{MockBot, MockMessageText};

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
}
