use teloxide::prelude::*;
use teloxide::types::KeyboardRemove;
use teloxide::{macros::BotCommands, payloads::SendMessageSetters};

use crate::db::models;
use crate::keyboards::menu_keyboard;
use crate::{db, keyboards, text, HandlerResult, MyDialogue, State};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum StartCommand {
    #[command()]
    Start,
    Cancel,
}

//
//  Commands
//

pub async fn start(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    let user = db::get_user(msg.chat.id.0);
    if user.is_err() {
        db::create_user(msg.chat.id.0).unwrap();
    }
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

//
//   Menu buttons
//

async fn send_menu(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, text::MENU)
        .reply_markup(menu_keyboard())
        .await?;
    dialogue.update(State::Start).await?;
    Ok(())
}

pub async fn profile(bot: Bot, msg: Message) -> HandlerResult {
    let user = db::get_user(msg.chat.id.0).unwrap();
    let all_phrases = db::get_user_phrases(msg.chat.id.0).unwrap();
    bot.send_message(msg.chat.id, text::profile(user.nickname, &all_phrases))
        .await?;
    Ok(())
}

pub async fn change_nickname(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, text::CHANGE_NICKNAME)
        .reply_markup(KeyboardRemove::new())
        .await?;
    dialogue.update(State::ChangeNickname).await?;
    Ok(())
}

pub async fn delete_phrase(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    let user_phrases = db::get_user_phrases(msg.chat.id.0).unwrap();
    bot.send_message(msg.chat.id, text::delete_phrase(&user_phrases))
        .reply_markup(KeyboardRemove::new())
        .await?;
    dialogue
        .update(State::WhatPhraseToDelete {
            phrases: user_phrases,
        })
        .await?;
    Ok(())
}

pub async fn add_phrase(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, text::what_is_new_phrase_emoji())
        .reply_markup(KeyboardRemove::new())
        .await?;
    dialogue.update(State::WhatIsNewPhraseEmoji).await?;
    Ok(())
}

//
//  Change nickname branch
//

pub async fn changed_nickname(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    let text = match msg.text() {
        Some(text) => text,
        None => {
            bot.send_message(msg.chat.id, text::PLEASE_SEND_TEXT)
                .await?;
            return Ok(());
        }
    };
    db::change_user_nickname(msg.chat.id.0, text.to_string()).unwrap();
    bot.send_message(msg.chat.id, text::CHANGED_NICKNAME.to_owned() + text)
        .await?;
    send_menu(bot, msg, dialogue).await
}

//
//   Delete phrase branch
//

pub async fn deleted_phrase(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    phrases: Vec<models::Phrase>,
) -> HandlerResult {
    let number = match msg.text() {
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
    if number > phrases.len() {
        bot.send_message(msg.chat.id, text::NO_SUCH_PHRASE).await?;
        return Ok(());
    }
    let phrase = &phrases[number - 1];
    db::delete_phrase(phrase.id).unwrap();
    bot.send_message(msg.chat.id, text::DELETED_PHRASE).await?;
    send_menu(bot, msg, dialogue).await
}

//
//  Add new phrase branch
//

pub async fn what_is_new_phrase_text(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
) -> HandlerResult {
    let text = match msg.text() {
        Some(text) => text,
        None => {
            bot.send_message(msg.chat.id, text::PLEASE_SEND_TEXT)
                .await?;
            return Ok(());
        }
    };
    if text.chars().count() > 3 {
        bot.send_message(msg.chat.id, text::NO_MORE_CHARACTERS)
            .await?;
        return Ok(());
    }
    bot.send_message(msg.chat.id, text::what_is_new_phrase_text(text))
        .await?;
    dialogue
        .update(State::WhatIsNewPhraseText {
            emoji: text.to_string(),
        })
        .await?;
    Ok(())
}

pub async fn what_is_new_phrase_bot_text(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    emoji: String,
) -> HandlerResult {
    let text = match msg.text() {
        Some(text) => text,
        None => {
            bot.send_message(msg.chat.id, text::PLEASE_SEND_TEXT)
                .await?;
            return Ok(());
        }
    };
    bot.send_message(msg.chat.id, text::what_is_new_phrase_bot_text(&emoji, text))
        .await?;
    dialogue
        .update(State::WhatIsNewPhraseBotText {
            emoji,
            text: text.to_string(),
        })
        .await?;
    Ok(())
}

pub async fn added_phrase(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    state_data: (String, String),
) -> HandlerResult {
    let text = match msg.text() {
        Some(text) => text,
        None => {
            bot.send_message(msg.chat.id, text::PLEASE_SEND_TEXT)
                .await?;
            return Ok(());
        }
    };
    bot.send_message(
        msg.chat.id,
        text::added_phrase(&state_data.0, &state_data.1, text),
    )
    .await?;
    db::create_phrase(msg.chat.id.0, state_data.0, state_data.1, text.to_string()).unwrap();
    send_menu(bot, msg, dialogue).await
}

//
//   Tests
//

#[cfg(test)]
mod tests {
    use crate::{get_bot_storage, handler_tree::handler_tree};

    use super::*;
    use dptree::deps;
    use teloxide::types::ReplyMarkup;
    use teloxide_tests::{MockBot, MockMessageDocument, MockMessageText, MockUser};

    #[tokio::test]
    async fn test_start() {
        let mut bot = MockBot::new(MockMessageText::new().text("/start"), handler_tree());
        // This fully deletes the user to test its creation
        let _ = db::delete_user(MockUser::ID as i64);

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::Start).await;

        bot.dispatch_and_check_last_text_and_state(text::START, State::Start)
            .await;
        let responses = bot.get_responses();
        assert_eq!(
            responses
                .sent_messages_text
                .last()
                .unwrap()
                .bot_request
                .reply_markup,
            Some(ReplyMarkup::Keyboard(keyboards::menu_keyboard()))
        );
        assert_eq!(db::get_user(MockUser::ID as i64).unwrap().nickname, None);
    }

    #[tokio::test]
    async fn test_cancel() {
        // Cancel is universal, so only one test is needed
        let mut bot = MockBot::new(MockMessageText::new().text("/cancel"), handler_tree());

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::ChangeNickname).await;

        bot.dispatch_and_check_last_text_and_state(text::MENU, State::Start)
            .await;
        let responses = bot.get_responses();
        assert_eq!(
            responses
                .sent_messages_text
                .last()
                .unwrap()
                .bot_request
                .reply_markup,
            Some(ReplyMarkup::Keyboard(keyboards::menu_keyboard())) // Just checking the keyboard
        );

        assert_eq!(
            responses.sent_messages.first().unwrap().text(),
            Some(text::CANCELED)
        );
    }

    #[tokio::test]
    async fn test_profile() {
        let mut bot = MockBot::new(
            MockMessageText::new().text(keyboards::PROFILE_BUTTON),
            handler_tree(),
        );
        db::full_user_redeletion(MockUser::ID as i64, None);

        let user = db::get_user(MockUser::ID as i64).unwrap();
        db::create_phrase(
            MockUser::ID as i64,
            "🤗".to_string(),
            "hug".to_string(),
            "(me) hugged (reply)".to_string(),
        )
        .unwrap();
        let all_phrases = db::get_user_phrases(MockUser::ID as i64).unwrap();

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::Start).await;

        bot.dispatch_and_check_last_text(&text::profile(user.nickname, &all_phrases))
            .await;
    }

    #[tokio::test]
    async fn test_change_nickname() {
        let mut bot = MockBot::new(
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
        let mut bot = MockBot::new(MockMessageText::new().text("nickname"), handler_tree());

        db::full_user_redeletion(MockUser::ID as i64, None);

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::ChangeNickname).await;

        bot.dispatch_and_check_last_text_and_state(text::MENU, State::Start)
            .await;

        let user = db::get_user(MockUser::ID as i64).unwrap();
        let responses = bot.get_responses();
        assert_eq!(
            responses
                .sent_messages_text
                .last()
                .unwrap()
                .bot_request
                .reply_markup,
            Some(ReplyMarkup::Keyboard(keyboards::menu_keyboard()))
        );
        assert_eq!(
            responses.sent_messages.first().unwrap().text(),
            Some(text::CHANGED_NICKNAME.to_owned() + "nickname").as_deref()
        );
        assert_eq!(user.nickname, Some("nickname".to_string()));
    }

    #[tokio::test]
    async fn test_delete_phrase() {
        // !!!!!!!! VERY IMPORTANT !!!!!!!!! Because the tests are run async, the database queries
        // might race condition themselves. Because of that, write all db queries __after__
        // creating the bot. Bot creation makes a lock that prevents other tests from starting,
        // before this one finishes
        let mut bot = MockBot::new(
            MockMessageText::new().text(keyboards::REMOVE_PHRASE_BUTTON),
            handler_tree(),
        );

        // Create an isolated environment with db queries
        db::full_user_redeletion(MockUser::ID as i64, None);
        db::create_phrase(
            MockUser::ID as i64,
            "🤗".to_string(),
            "hug".to_string(),
            "(me) hugged (reply)".to_string(),
        )
        .unwrap();
        let all_phrases = db::get_user_phrases(MockUser::ID as i64).unwrap();

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::Start).await;

        // And then dispatch
        bot.dispatch_and_check_last_text_and_state(
            &text::delete_phrase(&all_phrases),
            State::WhatPhraseToDelete {
                phrases: all_phrases,
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_deleted_phrase() {
        let mut bot = MockBot::new(MockMessageText::new().text("not a number"), handler_tree());

        db::full_user_redeletion(MockUser::ID as i64, None);
        db::create_phrase(
            MockUser::ID as i64,
            "🤗".to_string(),
            "hug".to_string(),
            "(me) hugged (reply)".to_string(),
        )
        .unwrap();
        let all_phrases = db::get_user_phrases(MockUser::ID as i64).unwrap();

        // Trying to send not a number
        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::WhatPhraseToDelete {
            phrases: all_phrases.clone(),
        })
        .await;

        bot.dispatch_and_check_last_text(text::PLEASE_SEND_NUMBER)
            .await;

        // Trying to send not a text message
        bot.update(MockMessageDocument::new());
        bot.dispatch_and_check_last_text(text::PLEASE_SEND_TEXT)
            .await;

        // Trying to send the wrong number
        bot.update(MockMessageText::new().text("100"));
        bot.dispatch_and_check_last_text(text::NO_SUCH_PHRASE).await;

        // Sending the correct response
        bot.update(MockMessageText::new().text("1"));
        bot.dispatch_and_check_last_text_and_state(text::MENU, State::Start)
            .await;

        let new_all_phrases = db::get_user_phrases(MockUser::ID as i64).unwrap();
        let responses = bot.get_responses();
        assert_eq!(
            responses.sent_messages.first().unwrap().text(),
            Some(text::DELETED_PHRASE)
        );

        assert_eq!(all_phrases.len() - 1, new_all_phrases.len());
    }

    #[tokio::test]
    async fn test_add_phrase() {
        let mut bot = MockBot::new(
            MockMessageText::new().text(keyboards::ADD_PHRASE_BUTTON),
            handler_tree(),
        );

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::Start).await;

        bot.dispatch_and_check_last_text_and_state(
            &text::what_is_new_phrase_emoji(),
            State::WhatIsNewPhraseEmoji,
        )
        .await;
    }

    #[tokio::test]
    async fn test_what_is_new_phrase_text() {
        let mut bot = MockBot::new(MockMessageText::new().text("🤗🤗🤗🤗"), handler_tree());

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::WhatIsNewPhraseEmoji).await;

        bot.dispatch_and_check_last_text(text::NO_MORE_CHARACTERS)
            .await;

        bot.update(MockMessageText::new().text("🤗"));
        bot.dispatch_and_check_last_text_and_state(
            &text::what_is_new_phrase_text("🤗"),
            State::WhatIsNewPhraseText {
                emoji: "🤗".to_string(),
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_what_is_new_phrase_bot_text() {
        let mut bot = MockBot::new(MockMessageText::new().text("hug"), handler_tree());

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::WhatIsNewPhraseText {
            emoji: "🤗".to_string(),
        })
        .await;

        bot.dispatch_and_check_last_text_and_state(
            &text::what_is_new_phrase_bot_text("🤗", "hug"),
            State::WhatIsNewPhraseBotText {
                emoji: "🤗".to_string(),
                text: "hug".to_string(),
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_added_phrase() {
        let mut bot = MockBot::new(
            MockMessageText::new().text("(me) hugged (reply)"),
            handler_tree(),
        );
        db::full_user_redeletion(MockUser::ID as i64, None);

        bot.dependencies(deps![get_bot_storage().await]);
        bot.set_state(State::WhatIsNewPhraseBotText {
            emoji: "🤗".to_string(),
            text: "hug".to_string(),
        })
        .await;

        bot.dispatch_and_check_last_text_and_state(text::MENU, State::Start)
            .await;

        let responses = bot.get_responses();
        assert_eq!(
            responses.sent_messages.first().unwrap().text(),
            Some(text::added_phrase("🤗", "hug", "(me) hugged (reply)")).as_deref()
        );
        // It is better to make the tests regarding the database __before__ the bot goes out of
        // scope, otherwise nasty race conditions can happen
        assert_eq!(
            db::get_user_phrases(MockUser::ID as i64)
                .unwrap()
                .first()
                .unwrap()
                .text,
            "hug".to_string()
        );
        drop(bot); // This is here not to ensure the bot is dropped, but to ensure that it is
                   // __not__ dropped before.
    }
}
