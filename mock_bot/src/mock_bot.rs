use gag::Gag;
use serde_json::Value;
use std::{
    env,
    mem::discriminant,
    panic,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex, MutexGuard,
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

static GET_POTENTIAL_STORAGE_LOCK: Mutex<()> = Mutex::new(());
static BOT_LOCK: Mutex<()> = Mutex::new(());
static UPDATE_LOCK: Mutex<()> = Mutex::new(());

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

fn find_chat_id(value: Value) -> Option<i64> {
    if let Value::Object(map) = value {
        for (k, v) in map {
            if k == "chat" {
                return Some(v["id"].as_i64()?);
            } else if k == "from" {
                return Some(v["id"].as_i64()?);
            } else if let Some(found) = find_chat_id(v) {
                return Some(found);
            }
        }
    }
    None
}

fn add_message(message: &mut Message) {
    let max_id = MESSAGES.max_message_id();
    if message.id.0 <= max_id || MESSAGES.get_message(message.id.0).is_some() {
        message.id = MessageId(max_id + 1);
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

/// A mocked bot that sends requests to the fake server
/// Please check the `new` function docs and github examples for more information.
/// The github examples will have more information than this doc.
#[allow(dead_code)]
pub struct MockBot {
    /// The bot with a fake server url
    pub bot: Bot,
    /// The thing that dptree::entry() returns
    pub handler_tree: UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>>,
    /// Mutex is here to not worry about mut references, its easier for the user without them
    pub update: Mutex<Update>,
    /// Bot parameters are here
    pub me: Mutex<Me>,
    /// If you have something like a state, you should add the storage here using .dependencies()
    pub dependencies: Mutex<DependencyMap>,
    /// Caught responses from the server
    pub responses: Mutex<Option<Responses>>,
    bot_lock: MutexGuard<'static, ()>,
    update_lock: Mutex<Option<MutexGuard<'static, ()>>>, // I am sorry.
}

impl MockBot {
    const CURRENT_UPDATE_ID: AtomicI32 = AtomicI32::new(0); // So that every update is different
    const PORT: Mutex<u16> = Mutex::new(6504);

    /// Creates a new MockBot, using something that can be turned into an Update, and a handler tree.
    /// You can't create a new bot while you have another bot in scope. Otherwise you will have a
    /// lot of race conditions. If you still somehow manage to create two bots at the same time
    /// (idk how),
    /// please look into [this crate for serial testing](https://crates.io/crates/serial_test)
    ///
    /// The `update` is just any Mock type, like `MockMessageText` or `MockCallbackQuery`.
    /// The `handler_tree` is the same as in `dptree::entry()`, you will need to make your handler
    /// tree into a separate function, like this:
    /// ```
    /// use teloxide::dispatching::UpdateHandler;
    /// fn handler_tree() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    ///     teloxide::dptree::entry() /* your handlers go here */
    /// }
    /// ```
    ///
    /// # Full example
    ///
    /// ```
    /// use teloxide::dispatching::UpdateHandler;
    /// use teloxide::types::Update;
    /// use teloxide_tests::{MockBot, MockMessageText};
    /// use teloxide::dispatching::dialogue::GetChatId;
    /// use teloxide::prelude::*;
    ///
    /// fn handler_tree() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    ///     teloxide::dptree::entry().endpoint(|update: Update, bot: Bot| async move {
    ///         bot.send_message(update.chat_id().unwrap(), "Hello!").await?;
    ///         Ok(())
    ///     })
    /// }
    ///
    /// #[tokio::main]  // Change for tokio::test in your implementation
    /// async fn main() {
    ///     let bot = MockBot::new(MockMessageText::new().text("Hi!"), handler_tree());
    ///     bot.dispatch().await;
    ///     let responses = bot.get_responses();
    ///     let message = responses
    ///         .sent_messages
    ///         .last()
    ///         .expect("No sent messages were detected!");
    ///     assert_eq!(message.text(), Some("Hello!"));
    /// }
    /// ```
    ///
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
                Self::PORT.lock().unwrap().clone()
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
            bot_lock: BOT_LOCK.lock().unwrap(), // This makes a lock that forbids the creation of
            // other bots until this one goes out of scope. That way there will be no race
            // conditions!
            update_lock: Mutex::new(Some(UPDATE_LOCK.lock().unwrap())),
        }
    }

    /// Sets the dependencies of the dptree. The same as deps![] in bot dispatching.
    /// Just like in this teloxide example: <https://github.com/teloxide/teloxide/blob/master/crates/teloxide/examples/dialogue.rs>
    /// You can use it to add dependencies to your handler tree.
    /// For more examples - look into `get_state` method documentation
    pub fn dependencies(&self, deps: DependencyMap) {
        *self.dependencies.lock().unwrap() = deps;
    }

    /// Sets the bot parameters, like supports_inline_queries, first_name, etc.
    pub fn me(&self, me: MockMe) {
        *self.me.lock().unwrap() = me.build();
    }

    /// Sets the update. Useful for reusing the same mocked bot instance in different tests
    /// Also, you can't set an update if it wasn't dispatched yet, this is to aviod a race condition
    pub fn update<T: IntoUpdate>(&self, update: T) {
        *self.update_lock.lock().unwrap() = Some(UPDATE_LOCK.lock().unwrap());
        *self.update.lock().unwrap() =
            update.into_update(Self::CURRENT_UPDATE_ID.fetch_add(1, Ordering::Relaxed));
    }

    /// Actually dispatches the bot, calling the update through the handler tree.
    /// All the requests made through the bot will be stored in `responses`, and can be retrieved
    /// with `get_responses`. All the responces are unique to that dispatch, and will be erased for
    /// every new dispatch.
    pub async fn dispatch(&self) {
        let mut update_lock = self.update.lock().unwrap().clone();
        let self_deps = self.dependencies.lock().unwrap().clone();

        let runtime = tokio::runtime::Handle::current();
        // If the user presses ctrl-c, the server will be shut down
        let _ = ctrlc::set_handler(move || {
            let client = reqwest::Client::new();
            runtime
                .block_on(
                    client
                        .post(format!(
                            "http://127.0.0.1:{}/stop/false",
                            Self::PORT.lock().unwrap().clone()
                        ))
                        .send(),
                )
                .unwrap();
            std::process::exit(1);
        });

        match update_lock.kind.clone() {
            UpdateKind::Message(mut message) => {
                // Add the message to the list of messages, so the bot can interact with it
                add_message(&mut message);
                update_lock.kind = UpdateKind::Message(message.clone());
            }
            UpdateKind::CallbackQuery(mut callback) => {
                if let Some(ref mut message) = callback.message {
                    add_message(message);
                }
                update_lock.kind = UpdateKind::CallbackQuery(callback.clone());
            }
            _ => {}
        }

        let mut deps = deps![
            self.bot.clone(),
            self.me.lock().unwrap().clone(),
            update_lock.clone() // This actually makes an update go through the dptree
        ];

        deps.insert_container(self_deps); // These are nessessary for the dispatch

        // In the future, this will need to be redone nicely, but right now it works.
        // It prevents a race condition for different bot instances to try to use the same server
        // (like in docstring)
        let mut left_tries = 200;
        while reqwest::get(format!(
            "http://127.0.0.1:{}/ping",
            Self::PORT.lock().unwrap().clone()
        ))
        .await
        .is_ok()
        {
            left_tries -= 1;
            if left_tries == 0 {
                panic!(
                    "Failed to unbind the server on the port {}!",
                    Self::PORT.lock().unwrap().clone()
                );
            }
        }

        let server = tokio::spawn(telegram_test_server::main(Self::PORT)); // This starts the server in the background

        // This, too, will need to be redone in the ideal world, but it just waits until the server is up
        let mut left_tries = 200;
        while reqwest::get(format!(
            "http://127.0.0.1:{}/ping",
            Self::PORT.lock().unwrap().clone()
        ))
        .await
        .is_err()
        {
            left_tries -= 1;
            if left_tries == 0 {
                panic!(
                    "Failed to get the server on the port {}!",
                    Self::PORT.lock().unwrap().clone()
                );
            }
        }

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
        let client = reqwest::Client::new();
        client
            .post(format!(
                "http://127.0.0.1:{}/stop/false",
                Self::PORT.lock().unwrap().clone()
            ))
            .send()
            .await
            .unwrap();
        server.await.unwrap();

        *self.update_lock.lock().unwrap() = None;
    }

    /// Returns the responses stored in `responses`
    /// Panics if no dispatching was done.
    pub fn get_responses(&self) -> telegram_test_server::Responses {
        let responses = self.responses.lock().unwrap().clone();
        match responses {
            Some(responses) => responses,
            None => panic!("No responses received! Maybe you forgot to dispatch the mocked bot?"),
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
        // If not this lock, some panic messages will make it to stderr, even with gag, because
        // race condition.
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

    /// Sets the state of the dialogue, if the storage exists in dependencies
    /// Panics if no storage was found
    ///
    /// # Example
    /// ```
    /// use teloxide::dispatching::UpdateHandler;
    /// use teloxide::types::Update;
    /// use teloxide_tests::{MockBot, MockMessageText};
    /// use teloxide::dispatching::dialogue::GetChatId;
    /// use teloxide::prelude::*;
    /// use teloxide::{
    ///     dispatching::{
    ///         dialogue::{self, InMemStorage},
    ///         UpdateFilterExt,
    ///     }
    /// };
    /// use dptree::deps;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
    /// enum State {
    ///     #[default]
    ///     Start,
    ///     NotStart
    /// }
    ///
    /// type MyDialogue = Dialogue<State, InMemStorage<State>>;
    ///
    /// fn handler_tree() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    ///     dialogue::enter::<Update, InMemStorage<State>, State, _>().endpoint(|update: Update, bot: Bot, dialogue: MyDialogue| async move {
    ///         let message = bot.send_message(update.chat_id().unwrap(), "Hello!").await?;
    ///         dialogue.update(State::NotStart).await?;
    ///         Ok(())
    ///     })
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let bot = MockBot::new(MockMessageText::new().text("Hi!"), handler_tree());
    ///     bot.dependencies(deps![InMemStorage::<State>::new()]);
    ///     bot.set_state(State::Start).await;
    ///     // Yes, Start is the default state, but this just shows how it works
    ///
    ///     bot.dispatch().await;
    ///
    ///     let state: State = bot.get_state().await;
    ///     // The `: State` type annotation is nessessary! Otherwise the compiler wont't know, what to return
    ///     assert_eq!(state, State::NotStart);
    ///
    ///     let responses = bot.get_responses();
    ///     let message = responses
    ///         .sent_messages
    ///         .last()
    ///         .expect("No sent messages were detected!");
    ///     assert_eq!(message.text(), Some("Hello!"));
    /// }
    /// ```
    ///
    pub async fn set_state<S>(&self, state: S)
    where
        S: Send + 'static + Clone,
    {
        let (in_mem_storage, erased_storage) = self.get_potential_storages().await;
        let update_lock = self.update.lock().unwrap().clone();
        let chat_id = match update_lock.chat_id() {
            Some(chat_id) => chat_id,
            None => match find_chat_id(serde_json::to_value(&update_lock).unwrap()) {
                Some(id) => ChatId(id),
                None => {
                    panic!("No chat id was detected!");
                }
            },
        };
        if let Some(storage) = in_mem_storage {
            // If memory storage exists
            (*storage)
                .clone()
                .update_dialogue(chat_id, state)
                .await
                .expect("Failed to update dialogue");
        } else if let Some(storage) = erased_storage {
            // If erased storage exists
            (*storage)
                .clone()
                .update_dialogue(chat_id, state)
                .await
                .expect("Failed to update dialogue");
        } else {
            panic!("No storage was getected!");
        }
    }

    /// Gets the state of the dialogue, if the storage exists in dependencies
    /// Panics if no storage was found
    /// You need to use type annotation to get the state, please refer to the `set_state`
    /// documentation example
    pub async fn get_state<S>(&self) -> S
    where
        S: Send + 'static + Clone,
    {
        let (in_mem_storage, erased_storage) = self.get_potential_storages().await;
        let update_lock = self.update.lock().unwrap().clone();
        let chat_id = match update_lock.chat_id() {
            Some(chat_id) => chat_id,
            None => match find_chat_id(serde_json::to_value(&update_lock).unwrap()) {
                Some(id) => ChatId(id),
                None => {
                    panic!("No chat id was detected!");
                }
            },
        };
        if let Some(storage) = in_mem_storage {
            // If memory storage exists
            (*storage)
                .clone()
                .get_dialogue(chat_id)
                .await
                .expect("Error getting dialogue")
                .expect("State is None")
        } else if let Some(storage) = erased_storage {
            // If erased storage exists
            (*storage)
                .clone()
                .get_dialogue(chat_id)
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

    /// Dispatches and checks the last sent message text or caption. Pass in an empty string if you
    /// want the text or caption to be None
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

    /// Same as `dispatch_and_check_last_text`, but also checks the state. You need to derive
    /// PartialEq, Clone and Debug for the state like in `set_state` example
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

    /// Same as `dispatch_and_check_last_text`, but also checks, if the variants of the state are the same
    ///
    /// For example, `State::Start { some_field: "value" }` and `State::Start { some_field: "other value" }` are the same in this function
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

    /// Just checks the state after dispathing the update, like `dispatch_and_check_last_text_and_state`
    pub async fn dispatch_and_check_state<S>(&self, state: S)
    where
        S: Send + 'static + Clone + std::fmt::Debug + PartialEq,
    {
        self.dispatch().await;
        let got_state: S = self.get_state().await;
        assert_eq!(got_state, state, "States are not equal!");
    }

    /// Just checks the state discriminant after dispathing the update, like `dispatch_and_check_last_text_and_state_discriminant`
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
