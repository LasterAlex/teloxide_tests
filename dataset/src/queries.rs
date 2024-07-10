use proc_macros::Changeable;
use teloxide::types::*;

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
    pub fn new() -> Self {
        Self {
            id: Self::ID.to_string(),
            from: MockUser::new().build(),
            message: None,
            inline_message_id: None,
            chat_instance: Self::CHAT_INSTANCE.to_string(),
            data: None,
            game_short_name: None,
        }
    }

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
    fn into_update(self, id: i32) -> Update {
        Update {
            id,
            kind: UpdateKind::CallbackQuery(self.build()),
        }
    }
}

// Add more queries here like ShippingQuery, PreCheckoutQuery etc.
