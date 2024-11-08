//! Mock bot that sends requests to the fake server
use gag::Gag;
use serde_json::Value;
use std::{
    mem::discriminant,
    panic,
    sync::{atomic::AtomicI32, Arc, Mutex, MutexGuard, PoisonError},
};
use teloxide::{
    dispatching::dialogue::ErasedStorage,
    dptree::di::DependencySupplier,
    types::{File, FileMeta, MaybeInaccessibleMessage, MessageId, MessageKind},
};
use teloxide::{dptree::deps, types::UpdateKind};

use crate::server::{self, Responses, MESSAGES};
use crate::{
    dataset::{IntoUpdate, MockMe},
    server::ServerManager,
};
use lazy_static::lazy_static;
use teloxide::{
    dispatching::{
        dialogue::{GetChatId, InMemStorage, Storage},
        UpdateHandler,
    },
    prelude::*,
    types::Me,
};

lazy_static! {
    static ref BOT_LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
}

static GET_POTENTIAL_STORAGE_LOCK: Mutex<()> = Mutex::new(());

fn find_file(value: Value) -> Option<FileMeta> {
    // Recursively searches for file meta
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
    if let (Some(id), Some(unique_id)) = (file_id, file_unique_id) {
        return Some(FileMeta {
            id,
            unique_id,
            size: file_size.unwrap_or(0),
        });
    }
    None
}

fn find_chat_id(value: Value) -> Option<i64> {
    // Recursively searches for chat id
    if let Value::Object(map) = value {
        for (k, v) in map {
            if k == "chat" {
                return v["id"].as_i64();
            } else if let Some(found) = find_chat_id(v) {
                return Some(found);
            }
        }
    }
    None
}

fn add_message(message: &mut Message, state: Arc<Mutex<State>>) {
    let max_id = MESSAGES.max_message_id();
    if message.id.0 <= max_id || MESSAGES.get_message(message.id.0).is_some() {
        message.id = MessageId(max_id + 1);
    }
    if let Some(file_meta) = find_file(serde_json::to_value(&message).unwrap()) {
        let file = File {
            meta: file_meta,
            path: "some_path.txt".to_string(), // This doesn't really matter
        };
        state.lock().unwrap().files.push(file);
    }
    if let MessageKind::Common(ref mut message_kind) = message.kind {
        if let Some(ref mut reply_message) = message_kind.reply_to_message {
            add_message(reply_message, state);
        }
    }
    MESSAGES.add_message(message.clone());
}

#[derive(Default)]
pub struct State {
    pub files: Vec<File>,
    pub responses: Responses,
}

impl State {
    pub fn reset(&mut self) {
        self.responses = Responses::default();
    }
}

/// A mocked bot that sends requests to the fake server
/// Please check the `new` function docs and [github examples](https://github.com/LasterAlex/teloxide_tests/tree/master/examples) for more information.
pub struct MockBot {
    /// The bot with a fake server url
    pub bot: Bot,
    /// The thing that dptree::entry() returns
    pub handler_tree: UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>>,
    /// Updates to send as user
    pub updates: Vec<Update>,
    /// Bot parameters are here
    pub me: Me,
    /// If you have something like a state, you should add the storage here using .dependencies()
    pub dependencies: DependencyMap,

    current_update_id: AtomicI32,
    stack_size: usize,
    state: Arc<Mutex<State>>,
    _bot_lock: MutexGuard<'static, ()>,
}

impl MockBot {
    const DEFAULT_STACK_SIZE: usize = 8 * 1024 * 1024;

    /// Creates a new MockBot, using something that can be turned into Updates, and a handler tree.
    /// You can't create a new bot while you have another bot in scope. Otherwise you will have a
    /// lot of race conditions. If you still somehow manage to create two bots at the same time
    /// (idk how),
    /// please look into [this crate for serial testing](https://crates.io/crates/serial_test)
    ///
    /// The `update` is just any Mock type, like `MockMessageText` or `MockCallbackQuery` or
    /// `vec![MockMessagePhoto]` if you want! All updates will be sent consecutively and asynchronously.
    /// The `handler_tree` is the same as in `dptree::entry()`, you will need to make your handler
    /// tree into a separate function, like this:
    /// ```no_run
    /// use teloxide::dispatching::UpdateHandler;
    /// fn handler_tree() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    ///     teloxide::dptree::entry() /* your handlers go here */
    /// }
    /// ```
    ///
    /// # Full example
    ///
    /// ```no_run
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
    ///     let mut bot = MockBot::new(MockMessageText::new().text("Hi!"), handler_tree());
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
        // MockMessageText or MockCallbackQuery, or a vec[MockMessagePhoto] if you want!
        handler_tree: UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>>,
    ) -> Self
    where
        T: IntoUpdate, // And that code just "proves" that it can be turned into an update
    {
        let _ = pretty_env_logger::try_init();

        let token = "1234567890:QWERTYUIOPASDFGHJKLZXCVBNMQWERTYUIO";
        let bot = Bot::new(token);
        let current_update_id = AtomicI32::new(42);
        let state = Arc::new(Mutex::new(State::default()));

        // If the lock is poisoned, we don't care, some other bot panicked and can't do anything
        let lock = BOT_LOCK.lock().unwrap_or_else(PoisonError::into_inner);

        Self {
            bot,
            me: MockMe::new().build(),
            updates: update.into_update(&current_update_id),
            handler_tree,
            dependencies: DependencyMap::new(),
            _bot_lock: lock,
            current_update_id,
            stack_size: Self::DEFAULT_STACK_SIZE,
            state,
        }
    }

    /// Sets the dependencies of the dptree. The same as deps![] in bot dispatching.
    /// Just like in this teloxide example: <https://github.com/teloxide/teloxide/blob/master/crates/teloxide/examples/dialogue.rs>
    /// You can use it to add dependencies to your handler tree.
    /// For more examples - look into `get_state` method documentation
    pub fn dependencies(&mut self, deps: DependencyMap) {
        self.dependencies = deps;
    }

    /// Sets the bot parameters, like supports_inline_queries, first_name, etc.
    pub fn me(&mut self, me: MockMe) {
        self.me = me.build();
    }

    /// Sets the updates. Useful for reusing the same mocked bot instance in different tests
    /// Reminder: You can pass in vec![MockMessagePhoto] or something else!
    pub fn update<T: IntoUpdate>(&mut self, update: T) {
        self.updates = update.into_update(&self.current_update_id);
    }

    fn collect_handles(&self, handles: &mut Vec<std::thread::JoinHandle<()>>, bot: Bot) {
        let self_deps = self.dependencies.clone();
        for mut update in self.updates.clone() {
            match update.kind.clone() {
                UpdateKind::Message(mut message) => {
                    // Add the message to the list of messages, so the bot can interact with it
                    add_message(&mut message, self.state.clone());
                    update.kind = UpdateKind::Message(message.clone());
                }
                UpdateKind::CallbackQuery(mut callback) => {
                    if let Some(MaybeInaccessibleMessage::Regular(ref mut message)) =
                        callback.message
                    {
                        add_message(message, self.state.clone());
                    }
                    update.kind = UpdateKind::CallbackQuery(callback.clone());
                }
                _ => {}
            }

            let mut deps = deps![
                bot.clone(),
                self.me.clone(),
                update.clone() // This actually makes an update go through the dptree
            ];

            deps.insert_container(self_deps.clone()); // These are nessessary for the dispatch

            // This, too, will need to be redone in the ideal world, but it just waits until the server is up
            let handler_tree = self.handler_tree.clone();

            let stack_size = self.stack_size;

            // To fix the stack overflow, a new thread with a new runtime is needed
            let builder = std::thread::Builder::new().stack_size(self.stack_size);
            handles.push(
                builder
                    .spawn(move || {
                        let runtime = tokio::runtime::Builder::new_multi_thread()
                            .thread_stack_size(stack_size) // Not needed, but just in case
                            .enable_all()
                            .build()
                            .unwrap();

                        runtime.block_on(async move {
                            let result = handler_tree.dispatch(deps.clone()).await;
                            if let ControlFlow::Break(result) = result {
                                // If it returned `ControlFlow::Break`, everything is fine, but we need to check, if the
                                // handler didn't error out
                                assert!(result.is_ok(), "Error in handler: {:?}", result);
                            } else {
                                log::error!("Update didn't get handled!");
                                panic!("Unhandled update!");
                            }
                        })
                    })
                    .unwrap(),
            );
        }
    }

    /// Actually dispatches the bot, calling the update through the handler tree.
    /// All the requests made through the bot will be stored in `responses`, and can be retrieved
    /// with `get_responses`. All the responses are unique to that dispatch, and will be erased for
    /// every new dispatch.
    pub async fn dispatch(&mut self) {
        self.state.lock().unwrap().reset();

        let server = ServerManager::start(self.me.clone(), self.state.clone())
            .await
            .unwrap();

        let api_url = reqwest::Url::parse(&format!("http://127.0.0.1:{}", server.port)).unwrap();
        let bot = self.bot.clone().set_api_url(api_url);

        // Gets all of the updates to send
        let mut handles = vec![];
        self.collect_handles(&mut handles, bot.clone());

        for handle in handles {
            // Waits until every update has been sent
            match handle.join() {
                Ok(_) => {}
                Err(_) => {
                    server.stop().await.unwrap();
                    panic!("Something went wrong and the bot panicked!");
                }
            };
        }

        server.stop().await.unwrap();
    }

    /// Returns the responses stored in `responses`
    /// Panics if no dispatching was done.
    /// Should be treated as a variable, because it kinda is
    pub fn get_responses(&self) -> server::Responses {
        self.state.lock().unwrap().responses.clone()
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
        let dependencies = Arc::new(self.dependencies.clone());
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

        let dependencies = Arc::new(self.dependencies.clone());
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
    /// The only supported storages are `InMemStorage` and `ErasedStorage`,
    /// using raw storages without `.erase()` is not supported.
    ///
    /// For example on how to make `ErasedStorage` from `RedisStorage` or `SqliteStorage` go to [this teloxide example](https://github.com/teloxide/teloxide/blob/master/crates/teloxide/examples/db_remember.rs#L41)
    ///
    /// # Example
    /// ```no_run
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
    ///     let mut bot = MockBot::new(MockMessageText::new().text("Hi!"), handler_tree());
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
        let first_update = self.updates.first().expect("No updates were detected!");
        let chat_id = match first_update.chat_id() {
            Some(chat_id) => chat_id,
            None => match find_chat_id(serde_json::to_value(first_update).unwrap()) {
                Some(id) => ChatId(id),
                None => {
                    log::error!("No chat id was detected in the update! Did you send an update without a chat identifier? Like MockCallbackQuery without an attached message?");
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
            log::error!("No storage was detected! Did you add it to bot.dependencies(deps![get_bot_storage().await]); ?");
            panic!("No storage was detected!");
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
        let first_update = self.updates.first().expect("No updates were detected!");
        let chat_id = match first_update.chat_id() {
            Some(chat_id) => chat_id,
            None => match find_chat_id(serde_json::to_value(first_update).unwrap()) {
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
            log::error!("No storage was detected! Did you add it to bot.dependencies(deps![get_bot_storage().await]); ?");
            panic!("No storage was detected!");
        }
    }

    //
    // Syntactic sugar
    //

    /// Dispatches and checks the last sent message text or caption. Pass in an empty string if you
    /// want the text or caption to be None
    pub async fn dispatch_and_check_last_text(&mut self, text_or_caption: &str) {
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
    pub async fn dispatch_and_check_last_text_and_state<S>(
        &mut self,
        text_or_caption: &str,
        state: S,
    ) where
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
        &mut self,
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
    pub async fn dispatch_and_check_state<S>(&mut self, state: S)
    where
        S: Send + 'static + Clone + std::fmt::Debug + PartialEq,
    {
        self.dispatch().await;
        let got_state: S = self.get_state().await;
        assert_eq!(got_state, state, "States are not equal!");
    }

    /// Just checks the state discriminant after dispathing the update, like `dispatch_and_check_last_text_and_state_discriminant`
    pub async fn dispatch_and_check_state_discriminant<S>(&mut self, state: S)
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
