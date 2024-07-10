use std::{
    env,
    sync::{
        atomic::{AtomicI32, Ordering},
        Mutex,
    },
};

use dataset::IntoUpdate;
use telegram_test_server::Responses;
use teloxide::{dispatching::UpdateHandler, prelude::*, types::Me};

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
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    bot.send_message(msg.chat.id, msg.text().unwrap()).await?;
    Ok(())
}

pub fn get_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dptree::entry().branch(Update::filter_message().endpoint(handler))
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    pretty_env_logger::init();

    let bot = Bot::from_env();

    Dispatcher::builder(bot, get_schema())
        .build()
        .dispatch()
        .await;
}

pub static DISPATCHING_LOCK: Mutex<()> = Mutex::new(());

pub struct MockBot {
    bot: Bot,
    update: Update,
    handler_tree: UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>>,
    responses: Mutex<Option<Responses>>,
    dependencies: Mutex<DependencyMap>,
}

impl MockBot {
    const CURRENT_UPDATE_ID: AtomicI32 = AtomicI32::new(0);

    pub fn new<T>(
        update: T,
        handler_tree: UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>>,
    ) -> Self
    where
        T: IntoUpdate,
    {
        env::set_var(
            "TELOXIDE_TOKEN",
            "1234567890:QWERTYUIOPASDFGHJKLZXCVBNMQWERTYUIO",
        );
        let update_id = Self::CURRENT_UPDATE_ID.fetch_add(1, Ordering::Relaxed);
        let bot = Bot::from_env()
            .set_api_url(reqwest::Url::parse(&format!("http://localhost:8080")).unwrap());
        Self {
            bot,
            update: update.into_update(update_id),
            handler_tree,
            responses: Mutex::new(None),
            dependencies: Mutex::new(DependencyMap::new()),
        }
    }

    pub fn dependencies(&self, deps: DependencyMap) {
        *self.dependencies.lock().unwrap() = deps;
    }

    pub async fn dispatch(&self) {
        let mut deps = self.dependencies.lock().unwrap();
        deps.insert(self.bot.clone());
        let me = serde_json::from_str::<Me>(&make_bot_string()).unwrap();
        deps.insert(me);
        deps.insert(self.update.clone());

        let lock = DISPATCHING_LOCK.lock();
        let handler = tokio::spawn(telegram_test_server::main());

        let result = self.handler_tree.dispatch(deps.clone()).await;
        *self.responses.lock().unwrap() =
            Some(telegram_test_server::RESPONSES.lock().unwrap().clone());
        handler.abort();

        drop(lock);
        if let ControlFlow::Break(result) = result {
            // If it returned `ControlFlow::Break`, everything is fine, but we need to check, if the
            // handler didn't error out
            assert!(result.is_ok(), "Error in handler: {:?}", result);
        } else {
            panic!("Unhandled update!");
        }
    }

    pub fn get_responses(&self) -> telegram_test_server::Responses {
        let responses = self.responses.lock().unwrap().clone();
        match responses {
            Some(responses) => responses,
            None => panic!("No responses received! Maybe you forgot to dismatch the mocked bot?"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dataset::message_common::MockMessageText;

    #[tokio::test]
    async fn test_echo_hello() {
        let bot = MockBot::new(MockMessageText::new("hello"), get_schema());

        bot.dispatch().await;

        let last_response = bot.get_responses().sent_messages.pop().unwrap();

        assert_eq!(last_response.0.text(), Some("hello"));
    }

    #[tokio::test]
    async fn test_echo_hi() {
        let bot = MockBot::new(MockMessageText::new("hi"), get_schema());

        bot.dispatch().await;

        let last_response = bot.get_responses().sent_messages.pop().unwrap();

        assert_eq!(
            last_response.0.text(),
            Some("hi")
        );
    }
}
