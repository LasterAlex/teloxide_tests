use mime::Mime;
use proc_macros::Changeable;
use teloxide::types::{ChatPhoto, FileMeta, Location, Me, PhotoSize, Update, User, UserId, Video};
pub mod chat;

pub mod message;
pub mod message_common;
pub mod queries;
pub use chat::*;
pub use message_common::*;
pub use queries::*;
#[cfg(test)]
mod tests;

pub trait IntoUpdate {
    /// Converts the mocked struct into an update
    fn into_update(self, id: i32) -> Update;
}


//
//  Structs below are just misc mocked structs
//

#[derive(Changeable, Clone)]
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

    /// Creates a new easily changable user builder
    ///
    /// # Examples
    /// ```
    /// let user = dataset::MockUser::new()
    ///     .id(12345678)
    ///     .build();
    /// assert_eq!(user.id.0, 12345678);
    /// ```
    ///
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

    /// Builds the user
    ///
    /// # Examples
    /// ```
    /// let mock_user = dataset::MockUser::new();
    /// let user = mock_user.build();
    /// assert_eq!(user.id.0 as u64, dataset::MockUser::ID);  // ID is a default value
    /// ```
    ///
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

#[derive(Changeable, Clone)]
pub struct MockMe {
    pub id: UserId,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub can_join_groups: bool,
    pub can_read_all_group_messages: bool,
    pub supports_inline_queries: bool,
}

impl MockMe {
    pub const ID: u64 = 12345678;
    pub const IS_BOT: bool = true;
    pub const FIRST_NAME: &'static str = "Test";
    pub const LAST_NAME: &'static str = "Bot";
    pub const USERNAME: &'static str = "test_bot";
    pub const LANGUAGE_CODE: &'static str = "en";
    pub const CAN_JOIN_GROUPS: bool = false;
    pub const CAN_READ_ALL_GROUP_MESSAGES: bool = false;
    pub const SUPPORTS_INLINE_QUERIES: bool = false;

    /// Creates a new easily changable me builder
    ///
    /// # Examples
    /// ```
    /// let me = dataset::MockMe::new()
    ///     .first_name("Test")
    ///     .build();
    /// assert_eq!(me.first_name, "Test");
    /// ```
    ///
    pub fn new() -> Self {
        Self {
            id: UserId(Self::ID),
            is_bot: Self::IS_BOT,
            first_name: Self::FIRST_NAME.to_string(),
            last_name: Some(Self::LAST_NAME.to_string()),
            username: Some(Self::USERNAME.to_string()),
            language_code: Some(Self::LANGUAGE_CODE.to_string()),
            can_join_groups: Self::CAN_JOIN_GROUPS,
            can_read_all_group_messages: Self::CAN_READ_ALL_GROUP_MESSAGES,
            supports_inline_queries: Self::SUPPORTS_INLINE_QUERIES,
        }
    }

    /// Builds the me
    ///
    /// # Examples
    /// ```
    /// let mock_me = dataset::MockMe::new();
    /// let me = mock_me.build();
    /// assert_eq!(me.id.0 as u64, dataset::MockMe::ID);  // ID is a default value
    /// ```
    ///
    pub fn build(self) -> Me {
        let mut user = MockUser::new();

        user.id = self.id;
        user.is_bot = self.is_bot;
        user.first_name = self.first_name;
        user.last_name = self.last_name;
        user.username = self.username;
        user.language_code = self.language_code;

        Me {
            user: user.build(),
            can_join_groups: self.can_join_groups,
            can_read_all_group_messages: self.can_read_all_group_messages,
            supports_inline_queries: self.supports_inline_queries,
        }
    }
}

//
//
//

#[derive(Changeable, Clone)]
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

    /// Creates a new easily changable chat photo builder
    ///
    /// # Examples
    /// ```
    /// let chat_photo = dataset::MockChatPhoto::new()
    ///     .small_file_id("small_file_id")
    ///     .build();
    /// assert_eq!(chat_photo.small_file_id, "small_file_id");
    /// ```
    ///
    pub fn new() -> Self {
        Self {
            small_file_id: Self::SMALL_FILE_ID.to_string(),
            small_file_unique_id: Self::SMALL_FILE_UNIQUE_ID.to_string(),
            big_file_id: Self::BIG_FILE_ID.to_string(),
            big_file_unique_id: Self::BIG_FILE_UNIQUE_ID.to_string(),
        }
    }

    /// Builds the chat photo
    ///
    /// # Examples
    /// ```
    /// let mock_chat_photo = dataset::MockChatPhoto::new();
    /// let chat_photo = mock_chat_photo.build();
    /// assert_eq!(chat_photo.small_file_id, dataset::MockChatPhoto::SMALL_FILE_ID);  // SMALL_FILE_ID is a default value
    /// ```
    ///
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

    /// Creates a new easily changable location builder
    ///
    /// # Examples
    /// ```
    /// let location = dataset::MockLocation::new()
    ///     .latitude(50.693416)
    ///     .build();
    /// assert_eq!(location.latitude, 50.693416);
    /// ```
    ///
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

    /// Builds the location
    ///
    /// # Examples
    /// ```
    /// let mock_location = dataset::MockLocation::new();
    /// let location = mock_location.build();
    /// assert_eq!(location.latitude, dataset::MockLocation::LATITUDE); // LATITUDE is a default value
    /// ```
    ///
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

    /// Creates a new easily changable photo size builder
    ///
    /// # Examples
    /// ```
    /// let photo_size = dataset::MockPhotoSize::new()
    ///     .width(90)
    ///     .build();
    /// assert_eq!(photo_size.width, 90);
    /// ```
    ///
    pub fn new() -> Self {
        Self {
            width: Self::WIDTH,
            height: Self::HEIGHT,
            file_id: Self::FILE_ID.to_string(),
            file_unique_id: Self::UNIQUE_FILE_ID.to_string(),
            file_size: Self::FILE_SIZE,
        }
    }

    /// Builds the photo size
    ///
    /// # Examples
    /// ```
    /// let mock_photo_size = dataset::MockPhotoSize::new();
    /// let photo_size = mock_photo_size.build();
    /// assert_eq!(photo_size.width, dataset::MockPhotoSize::WIDTH); // WIDTH is a default value
    /// ```
    ///
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

#[derive(Changeable, Clone)]
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

    /// Creates a new easily changable video builder
    ///
    /// # Examples
    /// ```
    /// let video = dataset::MockVideo::new()
    ///     .width(640)
    ///     .build();
    /// assert_eq!(video.width, 640);
    /// ```
    ///
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

    /// Builds the video
    ///
    /// # Examples
    /// ```
    /// let mock_video = dataset::MockVideo::new();
    /// let video = mock_video.build();
    /// assert_eq!(video.width, dataset::MockVideo::WIDTH); // WIDTH is a default value
    /// ```
    ///
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
