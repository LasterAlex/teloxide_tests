use std::sync::atomic::{AtomicI32, Ordering};

use teloxide::types::*;

use super::{MockMessageText, MockUser};
use crate::proc_macros::Changeable;

#[derive(Changeable, Clone)]
pub struct MockCallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
    make_message_inaccessible: bool,
}

impl MockCallbackQuery {
    pub const ID: &'static str = "id";
    pub const CHAT_INSTANCE: &'static str = "chat_instance";

    /// Creates a new easily changable callback query builder
    ///
    /// # Examples
    /// ```
    /// let callback_query = teloxide_tests::MockCallbackQuery::new()
    ///     .id("id")
    ///     .build();
    /// assert_eq!(callback_query.id, "id");
    /// ```
    ///
    pub fn new() -> Self {
        Self {
            id: Self::ID.to_string(),
            from: MockUser::new().build(),
            message: Some(
                MockMessageText::new()
                    .text("This is the callback message")
                    .build(),
            ),
            inline_message_id: None,
            chat_instance: Self::CHAT_INSTANCE.to_string(),
            data: None,
            game_short_name: None,
            make_message_inaccessible: false,
        }
    }

    /// Converts the message from MaybeInaccessibleMessage::Regular to MaybeInaccessibleMessage::Inaccessible in the final build
    ///
    /// # Example
    /// ```rust
    /// use teloxide_tests::{MockCallbackQuery, MockMessageText};
    ///
    /// let message_in_query = MockMessageText::new().build();
    /// let callback_query = MockCallbackQuery::new()
    ///     .message(message_in_query.clone())
    ///     .make_message_inaccessible()
    ///     .build();
    ///
    /// match callback_query.message.unwrap() {
    ///     teloxide::types::MaybeInaccessibleMessage::Inaccessible(msg) => assert_eq!(msg.message_id, message_in_query.id),
    ///     teloxide::types::MaybeInaccessibleMessage::Regular(msg) => panic!("Message should be inaccessible"),
    /// }
    /// ```
    pub fn make_message_inaccessible(mut self) -> Self {
        self.make_message_inaccessible = true;
        self
    }

    /// Builds the callback query
    ///
    /// # Example
    /// ```
    /// let mock_callback_query = teloxide_tests::MockCallbackQuery::new();
    /// let callback_query = mock_callback_query.build();
    /// assert_eq!(callback_query.id, teloxide_tests::MockCallbackQuery::ID);  // ID is a default value
    /// ```
    ///
    pub fn build(self) -> CallbackQuery {
        CallbackQuery {
            id: self.id,
            from: self.from,
            message: self.message.map(|message| {
                if !self.make_message_inaccessible {
                    MaybeInaccessibleMessage::Regular(message)
                } else {
                    MaybeInaccessibleMessage::Inaccessible(InaccessibleMessage {
                        chat: message.chat,
                        message_id: message.id,
                    })
                }
            }),
            inline_message_id: self.inline_message_id,
            chat_instance: self.chat_instance,
            data: self.data,
            game_short_name: self.game_short_name,
        }
    }
}

impl crate::dataset::IntoUpdate for MockCallbackQuery {
    /// Converts the MockCallbackQuery into an updates vector
    ///
    /// # Example
    /// ```
    /// use teloxide_tests::IntoUpdate;
    /// use teloxide::types::{UpdateId, UpdateKind::CallbackQuery};
    /// use std::sync::atomic::AtomicI32;
    ///
    /// let mock_callback_query = teloxide_tests::MockCallbackQuery::new();
    /// let update = mock_callback_query.clone().into_update(&AtomicI32::new(42))[0].clone();
    ///
    /// assert_eq!(update.id, UpdateId(42));
    /// assert_eq!(update.kind, CallbackQuery(mock_callback_query.build()));
    /// ```
    ///
    fn into_update(self, id: &AtomicI32) -> Vec<Update> {
        vec![Update {
            id: UpdateId(id.fetch_add(1, Ordering::Relaxed) as u32),
            kind: UpdateKind::CallbackQuery(self.build()),
        }]
    }
}

// Add more queries here like ShippingQuery, PreCheckoutQuery etc.
