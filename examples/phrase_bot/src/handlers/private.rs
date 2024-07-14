use teloxide::macros::BotCommands;
use teloxide::prelude::*;

use crate::{text, HandlerResult, MyDialogue, State};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum StartCommand {
    #[command()]
    Start,
}

pub async fn start(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, text::START).await?;
    dialogue.update(State::WhatIsYourNickname).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_bot_storage, handler_tree::handler_tree};

    use super::*;
    use dptree::deps;
    use teloxide_tests::{MockBot, MockMessageText};

    #[tokio::test]
    async fn test_start() {
        let bot = MockBot::new(MockMessageText::new().text("/start"), handler_tree());
        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::Start).await;
        bot.dispatch_and_check_last_text_and_state(text::START, State::WhatIsYourNickname)
            .await;
    }
}
