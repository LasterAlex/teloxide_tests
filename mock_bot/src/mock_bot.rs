use gag::Gag;
use serde_json::Value;
use std::{
    env,
    mem::discriminant,
    panic,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
};
use teloxide::{
    dispatching::dialogue::ErasedStorage,
    dptree::di::DependencySupplier,
    types::{File, FileMeta, MessageId},
};
use teloxide::{dptree::deps, types::UpdateKind};

use dataset::{IntoUpdate, MockMe};
use telegram_test_server::{Responses, FILES, MESSAGES};
use teloxide::{
    dispatching::{
        dialogue::{GetChatId, InMemStorage, Storage},
        UpdateHandler,
    },
    prelude::*,
    types::Me,
};

static DISPATCHING_LOCK: Mutex<()> = Mutex::new(());
static GET_POTENTIAL_STORAGE_LOCK: Mutex<()> = Mutex::new(());
// Otherwise the fake server will error because of a taken port

fn find_file(value: Value) -> Option<FileMeta> {
    let mut file_id = None;
    let mut file_unique_id = None;
    let mut file_size = None;
    if let Value::Object(map) = value {
        for (k, v) in map {
            if k == "file_id" {
                file_id = Some(v.as_str().unwrap().to_string());
            } else if k == "file_unique_id" {
                file_unique_id = Some(v.as_str().unwrap().to_string());
            } else if k == "file_size" {
                file_size = Some(v.as_u64().unwrap() as u32);
            } else if let Some(found) = find_file(v) {
                return Some(found);
            }
        }
    }
    if file_id.is_some() && file_unique_id.is_some() {
        return Some(FileMeta {
            id: file_id.unwrap(),
            unique_id: file_unique_id.unwrap(),
            size: file_size.unwrap_or(0),
        });
    }
    None
}

pub struct MockBot {
    pub bot: Bot, // The bot with a fake server url
    pub handler_tree: UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>>, // The thing that dptree::entry() returns
    pub update: Mutex<Update>,
    pub me: Mutex<Me>, // Mutex is here to not worry about mut references, its easier for the user without them
    pub dependencies: Mutex<DependencyMap>, // If you have something like a state, you should add the storage here
    pub responses: Mutex<Option<Responses>>, // Caught responses from the server
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
                }
                if let Some(file_meta) = find_file(serde_json::to_value(&message).unwrap()) {
                    let file = File {
                        meta: file_meta,
                        path: "some_path.txt".to_string(), // This doesn't really matter
                    };
                    FILES.lock().unwrap().push(file);
                }
                MESSAGES.add_message(message.clone());
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
        let print_gag = Gag::stderr().unwrap(); // Otherwise the panic will be printed
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

    //
    // Syntactic sugar
    //

    pub async fn dispatch_and_check_last_text(&self, text_or_caption: &str) {
        self.dispatch().await;
        let responses = self.get_responses();
        let message = responses
            .sent_messages
            .last()
            .expect("No sent messages were detected!");
        if let Some(text) = message.text() {
            assert_eq!(text, text_or_caption, "Texts are not equal!");
        } else if let Some(caption) = message.caption() {
            assert_eq!(caption, text_or_caption, "Captions are not equal!");
        } else if !text_or_caption.is_empty() {
            panic!("Message has no text or caption!");
        }
    }

    pub async fn dispatch_and_check_last_text_and_state<S>(&self, text_or_caption: &str, state: S)
    where
        S: Send + 'static + Clone + std::fmt::Debug + PartialEq,
    {
        self.dispatch().await;
        let responses = self.get_responses();
        let message = responses
            .sent_messages
            .last()
            .expect("No sent messages were detected!");
        if let Some(text) = message.text() {
            assert_eq!(text, text_or_caption, "Texts are not equal!");
        } else if let Some(caption) = message.caption() {
            assert_eq!(caption, text_or_caption, "Captions are not equal!");
        } else if !text_or_caption.is_empty() {
            panic!("Message has no text or caption!");
        }

        let got_state: S = self.get_state().await;
        assert_eq!(got_state, state, "States are not equal!");
    }

    pub async fn dispatch_and_check_last_text_and_state_discriminant<S>(
        &self,
        text_or_caption: &str,
        state: S,
    ) where
        S: Send + 'static + Clone,
    {
        self.dispatch().await;
        let responses = self.get_responses();
        let message = responses
            .sent_messages
            .last()
            .expect("No sent messages were detected!");
        if let Some(text) = message.text() {
            assert_eq!(text, text_or_caption, "Texts are not equal!");
        } else if let Some(caption) = message.caption() {
            assert_eq!(caption, text_or_caption, "Captions are not equal!");
        } else if !text_or_caption.is_empty() {
            panic!("Message has no text or caption!");
        }

        let got_state: S = self.get_state().await;
        assert_eq!(
            discriminant(&got_state),
            discriminant(&state),
            "State variants are not equal!"
        );
    }

    pub async fn dispatch_and_check_state<S>(&self, state: S)
    where
        S: Send + 'static + Clone + std::fmt::Debug + PartialEq,
    {
        self.dispatch().await;
        let got_state: S = self.get_state().await;
        assert_eq!(got_state, state, "States are not equal!");
    }

    pub async fn dispatch_and_check_state_discriminant<S>(&self, state: S)
    where
        S: Send + 'static + Clone,
    {
        self.dispatch().await;
        let got_state: S = self.get_state().await;
        assert_eq!(
            discriminant(&got_state),
            discriminant(&state),
            "State variants are not equal!"
        );
    }
}
