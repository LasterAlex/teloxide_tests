use teloxide::prelude::*;

use crate::HandlerResult;

pub async fn phrase(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "None yet").await?;
    Ok(())
}
