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
    dispatching::UpdateHandler,
    prelude::*,
    types::{Me, UpdateKind},
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

pub async fn handler(
    bot: Bot,
    messages_total: Arc<AtomicU64>,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    bot.send_message(
        msg.chat.id,
        format!(
            "I received {} messages in total.",
            messages_total.fetch_add(1, Ordering::Relaxed)
        ),
    )
    .await?;
    Ok(())
}

pub fn get_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dptree::entry().branch(Update::filter_message().endpoint(handler))
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

    let update = Update {
        id: 1,
        kind: UpdateKind::Message(MockMessageText::new("hello").build()),
    };
    let me = serde_json::from_str::<Me>(&make_bot_string()).unwrap();
    get_schema()
        .dispatch(deps![
            messages_total.clone(),
            me.clone(),
            bot.clone(),
            update.clone()
        ])
        .await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use telegram_test_server::RESPONSES;

    #[test]
    fn test_bot() {
        env::set_var(
            "TELOXIDE_TOKEN",
            "1234567890:QWERTYUIOPASDFGHJKLZXCVBNMQWERTYUIO",
        );
        let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
        runtime.spawn(async move {
            telegram_test_server::main().unwrap();
        });
        runtime.spawn(async move {
            let bot =
                Bot::from_env().set_api_url(reqwest::Url::parse("http://localhost:8080").unwrap());
            let me = serde_json::from_str::<Me>(&make_bot_string()).unwrap();
            let update = Update {
                id: 1,
                kind: UpdateKind::Message(MockMessageText::new("hello").build()),
            };

            let messages_total = Arc::new(AtomicU64::new(0));
            get_schema()
                .dispatch(deps![me, bot, update, messages_total])
                .await;
            let last_response = RESPONSES.lock().unwrap().sent_messages.pop().unwrap();
            assert_eq!(
                last_response.0.text(),
                Some("I received 0 messages in total.")
            );
        });
    }
}
