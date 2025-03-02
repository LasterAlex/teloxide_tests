use teloxide::{
    dispatching::dialogue::GetChatId,
    macros::BotCommands,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

use crate::{text, HandlerResult, MyDialogue, State};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum StartCommand {
    #[command()]
    Start,
}

/*
    Just some simple example handlers to test
*/

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let keyboard = InlineKeyboardMarkup::new([[
        InlineKeyboardButton::callback("Add", "add"),
        InlineKeyboardButton::callback("Subtract", "subtract"),
    ]]);
    bot.send_message(msg.chat.id, text::WHAT_DO_YOU_WANT)
        .reply_markup(keyboard)
        .await?;
    dialogue.update(State::WhatDoYouWant).await?;
    Ok(())
}

pub async fn what_is_the_first_number(
    bot: Bot,
    dialogue: MyDialogue,
    call: CallbackQuery,
) -> HandlerResult {
    let chat_id = call.clone().chat_id().unwrap();
    bot.edit_message_reply_markup(chat_id, call.regular_message().unwrap().id)
        .await?;
    bot.send_message(chat_id, text::ENTER_THE_FIRST_NUMBER)
        .await?;
    dialogue
        .update(State::GetFirstNumber {
            operation: call.data.unwrap(),
        })
        .await?;
    Ok(())
}

pub async fn what_is_the_second_number(
    bot: Bot,
    dialogue: MyDialogue,
    message: Message,
    state_data: String,
) -> HandlerResult {
    let message_text = match message.text() {
        // Just extracting the text from the message
        Some(text) => text,
        None => {
            bot.send_message(message.chat.id, text::PLEASE_SEND_TEXT)
                .await?;
            return Ok(());
        }
    };
    let first_number = match message_text.parse::<i32>() {
        // And then parsing it
        Ok(number) => number,
        Err(_) => {
            bot.send_message(message.chat.id, text::PLEASE_ENTER_A_NUMBER)
                .await?;
            return Ok(());
        }
    };
    bot.send_message(message.chat.id, text::ENTER_THE_SECOND_NUMBER)
        .await?;
    dialogue
        .update(State::GetSecondNumber {
            first_number,
            operation: state_data,
        })
        .await?;
    Ok(())
}

pub async fn get_result(
    bot: Bot,
    dialogue: MyDialogue,
    message: Message,
    state_data: (i32, String),
) -> HandlerResult {
    let message_text = match message.text() {
        // Who cares about DRY anyway
        Some(text) => text,
        None => {
            bot.send_message(message.chat.id, text::PLEASE_SEND_TEXT)
                .await?;
            return Ok(());
        }
    };
    let second_number = match message_text.parse::<i32>() {
        Ok(number) => number,
        Err(_) => {
            bot.send_message(message.chat.id, text::PLEASE_ENTER_A_NUMBER)
                .await?;
            return Ok(());
        }
    };

    let (first_number, operation) = state_data;
    let result = match operation.as_str() {
        "add" => first_number + second_number,
        "subtract" => first_number - second_number,
        _ => unreachable!(),
    };

    bot.send_message(
        message.chat.id,
        text::YOUR_RESULT.to_owned() + result.to_string().as_str(),
    )
    .await?;
    dialogue.update(State::default()).await?;
    Ok(())
}
