use std::{
    env,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use dataset::message_common::MockMessageText;
use dptree::deps;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Me, UpdateKind},
};

pub fn get_bot_id() -> i64 {
    // Every token starts with a bot id
    let token = env::var("TELOXIDE_TOKEN").unwrap();
    let parts: Vec<&str> = token.split(':').collect();
    parts[0].parse::<i64>().unwrap()
}

pub fn make_bot_string() -> String {
    format!(
        r#"{{"id":{bot_id},"is_bot":true,"first_name":"Test","last_name":"Bot","username":"test_bot","language_code":"en","can_join_groups":false,"can_read_all_group_messages":false,"supports_inline_queries":true}}"#,
        bot_id = get_bot_id()
    )
}

#[tokio::main]
async fn main() {
    env::set_var(
        "TELOXIDE_TOKEN",
        "1234567890:QWERTYUIOPASDFGHJKLZXCVBNMQWERTYUIO",
    );

    pretty_env_logger::init();

    let bot = Bot::from_env().set_api_url(reqwest::Url::parse("http://localhost:8080").unwrap());
    let messages_total = Arc::new(AtomicU64::new(0));

    let handler = dptree::entry().branch(Update::filter_message().endpoint(
        |bot: Bot, messages_total: Arc<AtomicU64>, msg: Message| async move {
            let previous = messages_total.fetch_add(1, Ordering::Relaxed);
            bot.send_message(
                msg.chat.id,
                format!("I received {previous} messages in total."),
            )
            .reply_markup(InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::callback("123", "123"),
            ]]))
            .await
            .unwrap();
            respond(())
        },
    ));

    let update = Update {
        id: 1,
        kind: UpdateKind::Message(MockMessageText::new("hello").build()),
    };
    let me = serde_json::from_str::<Me>(&make_bot_string()).unwrap();
    handler
        .dispatch(deps![
            messages_total.clone(),
            me.clone(),
            bot.clone(),
            update.clone()
        ])
        .await;
}
