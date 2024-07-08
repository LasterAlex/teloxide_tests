use proc_macros::Changeable;
use teloxide::types::{ChatPhoto, User, UserId};
pub mod chat;

#[cfg(test)]
mod tests;

pub const DEFAULT_USER_ID: u64 = 12345678;
pub const DEFAULT_FIRST_NAME: &str = "First";
pub const DEFAULT_IS_BOT: bool = false;
pub const DEFAULT_ADDED_TO_ATTACHMENT_MENU: bool = false;
pub const DEFAULT_IS_PREMIUM: bool = false;

//
//
//

#[derive(Changeable)]
pub struct MockUser {
    pub id: UserId,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub added_to_attachment_menu: bool,
    pub is_premium: bool,
}

impl MockUser {
    pub fn new() -> Self {
        Self {
            id: UserId(DEFAULT_USER_ID),
            is_bot: DEFAULT_IS_BOT,
            first_name: DEFAULT_FIRST_NAME.to_string(),
            last_name: None,
            username: None,
            language_code: None,
            added_to_attachment_menu: DEFAULT_ADDED_TO_ATTACHMENT_MENU,
            is_premium: DEFAULT_IS_PREMIUM,
        }
    }

    pub fn to_object(self) -> User {
        User {
            id: self.id,
            is_bot: self.is_bot,
            first_name: self.first_name,
            last_name: self.last_name,
            username: self.username,
            language_code: self.language_code,
            added_to_attachment_menu: self.added_to_attachment_menu,
            is_premium: self.is_premium,
        }
    }
}

//
//
//

#[derive(Changeable)]
pub struct MockChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String,
}

impl MockChatPhoto {
    pub fn new() -> Self {
        Self {
            small_file_id: "small_file_id".to_string(),
            small_file_unique_id: "small_file_unique_id".to_string(),
            big_file_id: "big_file_id".to_string(),
            big_file_unique_id: "big_file_unique_id".to_string(),
        }
    }

    pub fn to_object(self) -> ChatPhoto {
        ChatPhoto {
            small_file_id: self.small_file_id,
            small_file_unique_id: self.small_file_unique_id,
            big_file_id: self.big_file_id,
            big_file_unique_id: self.big_file_unique_id,
        }
    }
}
