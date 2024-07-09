use proc_macros::Changeable;
use teloxide::types::{ChatPhoto, FileMeta, Location, PhotoSize, User, UserId};
pub mod chat;

pub mod message;
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

    pub fn build(self) -> User {
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

    pub fn build(self) -> ChatPhoto {
        ChatPhoto {
            small_file_id: self.small_file_id,
            small_file_unique_id: self.small_file_unique_id,
            big_file_id: self.big_file_id,
            big_file_unique_id: self.big_file_unique_id,
        }
    }
}

#[derive(Changeable, Clone)]
pub struct MockLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<u32>,
    pub heading: Option<u16>,
    pub proximity_alert_radius: Option<u32>,
}

impl MockLocation {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
        }
    }

    pub fn build(self) -> Location {
        Location {
            longitude: self.longitude,
            latitude: self.latitude,
            horizontal_accuracy: self.horizontal_accuracy,
            live_period: self.live_period,
            heading: self.heading,
            proximity_alert_radius: self.proximity_alert_radius,
        }
    }
}

#[derive(Changeable, Clone)]
pub struct MockPhotoSize {
    pub width: u32,
    pub height: u32,
    // FileMeta
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: u32,
}

impl MockPhotoSize {
    pub fn new(
        width: u32,
        height: u32,
        file_id: &str,
        file_unique_id: &str,
        file_size: u32,
    ) -> Self {
        Self {
            width,
            height,
            file_id: file_id.to_string(),
            file_unique_id: file_unique_id.to_string(),
            file_size,
        }
    }

    pub fn build(self) -> PhotoSize {
        PhotoSize {
            file: FileMeta {
                id: self.file_id,
                unique_id: self.file_unique_id,
                size: self.file_size,
            },
            width: self.width,
            height: self.height,
        }
    }
}
