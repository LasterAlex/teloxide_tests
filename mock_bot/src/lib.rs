use std::{
    env,
    fmt::Debug,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
};

use dataset::{IntoUpdate, MockMe};
use telegram_test_server::{Responses, SERVER_PORT};
use teloxide::{
    dispatching::{
        dialogue::{GetChatId, Storage},
        UpdateHandler,
    },
    prelude::*,
    types::Me,
};

#[cfg(test)]
mod tests;

static DISPATCHING_LOCK: Mutex<()> = Mutex::new(());
// Otherwise the fake server will error because of a taken port

pub struct MockBot {
    bot: Bot, // The bot with a fake server url
    update: Update,
    handler_tree: UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>>, // The thing that dptree::entry() returns
    me: Mutex<Me>, // Mutex is here to not worry about mut references, its easier for the user without them
    dependencies: Mutex<DependencyMap>, // If you have something like a state, you should add the storage here
    responses: Mutex<Option<Responses>>, // Caught responses from the server
}

impl MockBot {
    const CURRENT_UPDATE_ID: AtomicI32 = AtomicI32::new(0); // So that every update is different

    pub fn new<T>(
        update: T, // This 'T' is just anything that can be turned into an Update, like a
        // MockMessageText or MockCallbackQuery
        handler_tree: UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>>,
    ) -> Self
    where
        T: IntoUpdate, // And that code just "proves" that it can be turned into an update
    {
        env::set_var(
            // So that teloxide bot doesn't complain
            "TELOXIDE_TOKEN",
            "1234567890:QWERTYUIOPASDFGHJKLZXCVBNMQWERTYUIO",
        );
        let update_id = Self::CURRENT_UPDATE_ID.fetch_add(1, Ordering::Relaxed);

        let bot = Bot::from_env().set_api_url(
            reqwest::Url::parse(&format!(
                "http://localhost:{}",
                SERVER_PORT.lock().unwrap().to_string()
            ))
            .unwrap(),
        );
        Self {
            bot,
            me: Mutex::new(MockMe::new().build()),
            update: update.into_update(update_id),
            handler_tree,
            responses: Mutex::new(None),
            dependencies: Mutex::new(DependencyMap::new()),
        }
    }

    pub fn dependencies(&self, deps: DependencyMap) {
        *self.dependencies.lock().unwrap() = deps;
    }

    pub fn me(&self, me: MockMe) {
        *self.me.lock().unwrap() = me.build();
    }

    pub async fn dispatch(&self) {
        let mut deps = self.dependencies.lock().unwrap();
        deps.insert(self.bot.clone()); // Insert the nessesary for dispatchment dependencies
        deps.insert(self.me.lock().unwrap().clone());
        deps.insert(self.update.clone());

        let lock = DISPATCHING_LOCK.lock(); // Lock all the other threads out
        let handler = tokio::spawn(telegram_test_server::main()); // This starts the server in the background

        let result = self.handler_tree.dispatch(deps.clone()).await; // This is the part that actually calls the handler
        *self.responses.lock().unwrap() =
            Some(telegram_test_server::RESPONSES.lock().unwrap().clone()); // Get the responses
                                                                           // while the lock is still active
        handler.abort(); // Stop the server

        drop(lock); // And free the lock so that the next test can use it

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

    pub async fn get_state<T, S>(&self, storage: Arc<T>) -> S
    where
        T: Storage<S, Error: Debug>,
    {
        storage
            .get_dialogue(self.update.chat_id().expect("No chat id"))
            .await
            .expect("Error getting dialogue")
            .expect("State is None")
    }
}
