use std::{
    env, panic,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
};
use gag::Gag;
use teloxide::{
    dispatching::dialogue::ErasedStorage, dptree::di::DependencySupplier, types::MessageId,
};
use teloxide::{dptree::deps, types::UpdateKind};

use dataset::{IntoUpdate, MockMe};
use telegram_test_server::{Responses, MESSAGES};
use teloxide::{
    dispatching::{
        dialogue::{GetChatId, InMemStorage, Storage},
        UpdateHandler,
    },
    prelude::*,
    types::Me,
};

#[cfg(test)]
mod tests;

static DISPATCHING_LOCK: Mutex<()> = Mutex::new(());
static GET_POTENTIAL_STORAGE_LOCK: Mutex<()> = Mutex::new(());
// Otherwise the fake server will error because of a taken port

pub struct MockBot {
    bot: Bot, // The bot with a fake server url
    handler_tree: UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>>, // The thing that dptree::entry() returns
    update: Mutex<Update>,
    me: Mutex<Me>, // Mutex is here to not worry about mut references, its easier for the user without them
    dependencies: Mutex<DependencyMap>, // If you have something like a state, you should add the storage here
    responses: Mutex<Option<Responses>>, // Caught responses from the server
}

impl MockBot {
    const CURRENT_UPDATE_ID: AtomicI32 = AtomicI32::new(0); // So that every update is different
    const PORT: Mutex<u16> = Mutex::new(6504);

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
                Self::PORT.lock().unwrap().to_string()
            ))
            .unwrap(),
        );
        Self {
            bot,
            me: Mutex::new(MockMe::new().build()),
            update: Mutex::new(update.into_update(update_id)),
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

    pub fn update<T: IntoUpdate>(&self, update: T) {
        *self.update.lock().unwrap() =
            update.into_update(Self::CURRENT_UPDATE_ID.fetch_add(1, Ordering::Relaxed));
    }

    pub async fn dispatch(&self) {
        let lock = DISPATCHING_LOCK.lock(); // Lock all the other threads out
        
        let mut deps = self.dependencies.lock().unwrap();

        let mut update_lock = self.update.lock().unwrap();

        match update_lock.kind.clone() {
            UpdateKind::Message(mut message) => {
                // Add the message to the list of messages, so the bot can interact with it
                let max_id = MESSAGES.max_message_id();
                if message.id.0 <= max_id || MESSAGES.get_message(message.id.0).is_some() {
                    message.id = MessageId(max_id + 1);
                    update_lock.kind = UpdateKind::Message(message.clone());
                    MESSAGES.add_message(message.clone());
                }
            }
            _ => {}
        }

        deps.insert_container(deps![
            self.bot.clone(),
            self.me.lock().unwrap().clone(),
            update_lock.clone() // This actually makes an update go through the dptree
        ]); // These are nessessary for the dispatch

        tokio::spawn(telegram_test_server::main(Self::PORT)); // This starts the server in the background

        let result = self.handler_tree.dispatch(deps.clone()).await; // This is the part that actually calls the handler
        *self.responses.lock().unwrap() =
            Some(telegram_test_server::RESPONSES.lock().unwrap().clone()); // Get the responses
                                                                           // while the lock is still active

        if let ControlFlow::Break(result) = result {
            // If it returned `ControlFlow::Break`, everything is fine, but we need to check, if the
            // handler didn't error out
            assert!(result.is_ok(), "Error in handler: {:?}", result);
        } else {
            panic!("Unhandled update!");
        }
        drop(lock); // And free the lock so that the next test can use it
    }

    pub fn get_responses(&self) -> telegram_test_server::Responses {
        let responses = self.responses.lock().unwrap().clone();
        match responses {
            Some(responses) => responses,
            None => panic!("No responses received! Maybe you forgot to dismatch the mocked bot?"),
        }
    }

    async fn get_potential_storages<S>(
        &self,
    ) -> (
        Option<Arc<Arc<InMemStorage<S>>>>,
        Option<Arc<Arc<ErasedStorage<S>>>>,
    )
    where
        S: Send + 'static + Clone,
    {
        let get_potential_storage_lock = GET_POTENTIAL_STORAGE_LOCK.lock();
        // If not this lock, some panic messages will make it to stderr, even with gag
        let default_panic = panic::take_hook();
        let in_mem_storage: Option<Arc<Arc<InMemStorage<S>>>>;
        let erased_storage: Option<Arc<Arc<ErasedStorage<S>>>>;
        // No trace storage cuz who uses it
        let dependencies = Arc::new(self.dependencies.lock().unwrap().clone());
        // Get dependencies into Arc cuz otherwise it complaints about &self being moved

        panic::set_hook(Box::new(|_| {
            // Do nothing to ignore the panic
        }));
        let print_gag = Gag::stderr().unwrap();  // Otherwise the panic will be printed
        in_mem_storage = std::thread::spawn(move || {
            // Try to convert one of dptrees fields into an InMemStorage
            dependencies.get()
        })
        .join()
        .ok();

        let dependencies = Arc::new(self.dependencies.lock().unwrap().clone());
        // Dependencies were moved to a prev. thread, so create a new one
        erased_storage = std::thread::spawn(move || {
            // The same for ErasedStorage
            dependencies.get()
        })
        .join()
        .ok();


        panic::set_hook(default_panic); // Restore the default panic hook
        drop(print_gag);
        drop(get_potential_storage_lock);
        (in_mem_storage, erased_storage)
    }

    pub async fn set_state<S>(&self, state: S)
    where
        S: Send + 'static + Clone,
    {
        let (in_mem_storage, erased_storage) = self.get_potential_storages().await;
        if let Some(storage) = in_mem_storage {
            // If memory storage exists
            (*storage)
                .clone()
                .update_dialogue(
                    self.update.lock().unwrap().chat_id().expect("No chat id"),
                    state,
                )
                .await
                .expect("Failed to update dialogue");
        } else if let Some(storage) = erased_storage {
            // If erased storage exists
            (*storage)
                .clone()
                .update_dialogue(
                    self.update.lock().unwrap().chat_id().expect("No chat id"),
                    state,
                )
                .await
                .expect("Failed to update dialogue");
        } else {
            panic!("No storage was getected!");
        }
    }

    pub async fn get_state<S>(&self) -> S
    where
        S: Send + 'static + Clone,
    {
        let (in_mem_storage, erased_storage) = self.get_potential_storages().await;
        if let Some(storage) = in_mem_storage {
            // If memory storage exists
            (*storage)
                .clone()
                .get_dialogue(self.update.lock().unwrap().chat_id().expect("No chat id"))
                .await
                .expect("Error getting dialogue")
                .expect("State is None")
        } else if let Some(storage) = erased_storage {
            // If erased storage exists
            (*storage)
                .clone()
                .get_dialogue(self.update.lock().unwrap().chat_id().expect("No chat id"))
                .await
                .expect("Error getting dialogue")
                .expect("State is None")
        } else {
            panic!("No storage was getected!");
        }
    }
}
