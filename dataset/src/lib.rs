use mime::Mime;
use proc_macros::Changeable;
use teloxide::types::{ChatPhoto, FileMeta, Location, PhotoSize, User, UserId, Video};
pub mod chat;

pub mod message_common;
pub mod message;
pub mod queries;
#[cfg(test)]
mod tests;

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
    pub const ID: u64 = 12345678;
    pub const IS_BOT: bool = false;
    pub const FIRST_NAME: &'static str = "First";
    pub const ADDED_TO_ATTACHMENT_MENU: bool = false;
    pub const IS_PREMIUM: bool = false;

    pub fn new() -> Self {
        Self {
            id: UserId(Self::ID),
            is_bot: Self::IS_BOT,
            first_name: Self::FIRST_NAME.to_string(),
            last_name: None,
            username: None,
            language_code: None,
            added_to_attachment_menu: Self::ADDED_TO_ATTACHMENT_MENU,
            is_premium: Self::IS_PREMIUM,
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
    pub const SMALL_FILE_ID: &'static str = "small_file_id";
    pub const SMALL_FILE_UNIQUE_ID: &'static str = "small_file_unique_id";
    pub const BIG_FILE_ID: &'static str = "big_file_id";
    pub const BIG_FILE_UNIQUE_ID: &'static str = "big_file_unique_id";

    pub fn new() -> Self {
        Self {
            small_file_id: Self::SMALL_FILE_ID.to_string(),
            small_file_unique_id: Self::SMALL_FILE_UNIQUE_ID.to_string(),
            big_file_id: Self::BIG_FILE_ID.to_string(),
            big_file_unique_id: Self::BIG_FILE_UNIQUE_ID.to_string(),
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
    pub const LATITUDE: f64 = 50.693416;
    pub const LONGITUDE: f64 = 30.624605;

    pub fn new() -> Self {
        Self {
            latitude: Self::LATITUDE,
            longitude: Self::LONGITUDE,
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
    pub const WIDTH: u32 = 90;
    pub const HEIGHT: u32 = 51;
    pub const FILE_ID: &'static str = "AgADBAADFak0G88YZAf8OAug7bHyS9x2ZxkABHVfpJywcloRAAGAAQABAg";
    pub const UNIQUE_FILE_ID: &'static str = "file_unique_id";
    pub const FILE_SIZE: u32 = 1101;

    pub fn new() -> Self {
        Self {
            width: Self::WIDTH,
            height: Self::HEIGHT,
            file_id: Self::FILE_ID.to_string(),
            file_unique_id: Self::UNIQUE_FILE_ID.to_string(),
            file_size: Self::FILE_SIZE,
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

pub struct MockVideo {
    pub width: u32,
    pub height: u32,
    pub duration: u32,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<Mime>,
    // FileMeta
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: u32,
}

impl MockVideo {
    pub const WIDTH: u32 = 640;
    pub const HEIGHT: u32 = 480;
    pub const DURATION: u32 = 52;
    pub const FILE_ID: &'static str = "BAADAgpAADdawy_JxS72kRvV3cortAg";
    pub const UNIQUE_FILE_ID: &'static str = "unique_file_id";
    pub const FILE_SIZE: u32 = 10099782;

    pub fn new() -> Self {
        Self {
            width: Self::WIDTH,
            height: Self::HEIGHT,
            duration: Self::DURATION,
            thumb: None,
            file_name: None,
            mime_type: None,
            file_id: Self::FILE_ID.to_string(),
            file_unique_id: Self::UNIQUE_FILE_ID.to_string(),
            file_size: Self::FILE_SIZE,
        }
    }

    pub fn build(self) -> Video {
        Video {
            width: self.width,
            height: self.height,
            duration: self.duration,
            thumb: self.thumb,
            file_name: self.file_name,
            mime_type: self.mime_type,
            file: FileMeta {
                id: self.file_id,
                unique_id: self.file_unique_id,
                size: self.file_size,
            },
        }
    }
}

