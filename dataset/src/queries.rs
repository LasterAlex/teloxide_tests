use proc_macros::Changeable;
use teloxide::types::*;

use crate::MockMessageText;

use super::MockUser;

#[derive(Changeable, Clone)]
pub struct MockCallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

impl MockCallbackQuery {
    pub const ID: &'static str = "id";
    pub const CHAT_INSTANCE: &'static str = "chat_instance";

    /// Creates a new easily changable callback query builder
    ///
    /// # Examples
    /// ```
    /// let callback_query = dataset::MockCallbackQuery::new()
    ///     .id("id")
    ///     .build();
    /// assert_eq!(callback_query.id, "id");
    /// ```
    ///
    pub fn new() -> Self {
        Self {
            id: Self::ID.to_string(),
            from: MockUser::new().build(),
            message: Some(MockMessageText::new().text("This is the callback message").build()),
            inline_message_id: None,
            chat_instance: Self::CHAT_INSTANCE.to_string(),
            data: None,
            game_short_name: None,
        }
    }

    /// Builds the callback query
    ///
    /// # Example
    /// ```
    /// let mock_callback_query = dataset::MockCallbackQuery::new();
    /// let callback_query = mock_callback_query.build();
    /// assert_eq!(callback_query.id, dataset::MockCallbackQuery::ID);  // ID is a default value
    /// ```
    ///
    pub fn build(self) -> CallbackQuery {
        CallbackQuery {
            id: self.id,
            from: self.from,
            message: self.message,
            inline_message_id: self.inline_message_id,
            chat_instance: self.chat_instance,
            data: self.data,
            game_short_name: self.game_short_name,
        }
    }
}

impl crate::IntoUpdate for MockCallbackQuery {
    /// Converts the MockCallbackQuery into an update
    ///
    /// # Example
    /// ```
    /// use dataset::IntoUpdate;
    /// let mock_callback_query = dataset::MockCallbackQuery::new();
    /// let update = mock_callback_query.clone().into_update(1);
    /// assert_eq!(update.id, 1);
    /// assert_eq!(update.kind, teloxide::types::UpdateKind::CallbackQuery(
    ///     mock_callback_query.build())
    /// );
    /// ```
    ///
    fn into_update(self, id: i32) -> Update {
        Update {
            id,
            kind: UpdateKind::CallbackQuery(self.build()),
        }
    }
}

// Add more queries here like ShippingQuery, PreCheckoutQuery etc.
