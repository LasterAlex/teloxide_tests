use crate::dataset::{chat::MockPrivateChat, MockUser};
use chrono::{DateTime, Utc};
use mime::Mime;
use proc_macros::Changeable;
use teloxide::types::*;

use super::{MockLocation, MockPhotoSize, MockVideo};

macro_rules! Message {
    (#[derive($($derive:meta),*)] $pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty,)* }) => {
        #[derive($($derive),*)]
        $pub struct $name {  // This is basically a template
            pub id: MessageId,
            pub thread_id: Option<i32>,
            pub date: DateTime<Utc>,
            pub chat: Chat,
            pub via_bot: Option<User>,
            $($fpub $field : $type,)*
        }
        impl $name {
            pub const ID: i32 = 1;
            $pub fn new_message($($field:$type,)*) -> Self{
                Self {  // To not repeat this over and over again
                    id: MessageId($name::ID),
                    thread_id: None,
                    date: Utc::now(),
                    chat: MockPrivateChat::new().build(),
                    via_bot: None,
                    $($field,)*
                }
            }

            $pub fn build_message(self, message_kind: MessageKind) -> Message {
                Message {
                    id: self.id,
                    thread_id: self.thread_id,
                    date: self.date,
                    chat: self.chat,
                    via_bot: self.via_bot,
                    kind: message_kind,
                }
            }
        }
    }
}

macro_rules! MessageCommon {
    (#[derive($($derive:meta),*)] $pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty,)* }) => {
        Message! {  // DRY is dangerous.
            #[derive($($derive),*)]
            $pub struct $name {
                pub from: Option<User>,
                pub sender_chat: Option<Chat>,
                pub author_signature: Option<String>,
                pub forward: Option<Forward>,
                pub reply_to_message: Option<Box<Message>>,
                pub edit_date: Option<DateTime<Utc>>,
                pub reply_markup: Option<InlineKeyboardMarkup>,
                pub is_topic_message: bool,
                pub is_automatic_forward: bool,
                pub has_protected_content: bool,
                $($fpub $field : $type,)*
            }
        }
        impl $name {
            pub const IS_TOPIC_MESSAGE: bool = false;
            pub const IS_AUTOMATIC_FORWARD: bool = false;
            pub const HAS_PROTECTED_CONTENT: bool = false;

            $pub fn new_message_common($($field:$type,)*) -> Self {
                 $name::new_message(
                     Some(MockUser::new().build()),
                     Some(MockPrivateChat::new().build()),
                     None,
                     None,
                     None,
                     None,
                     None,
                     $name::IS_TOPIC_MESSAGE,
                     $name::IS_AUTOMATIC_FORWARD,
                     $name::HAS_PROTECTED_CONTENT,
                     $($field,)*
                 )
            }

            $pub fn build_message_common(self, media_kind: MediaKind) -> Message {
                self.clone().build_message(MessageKind::Common(MessageCommon {
                    from: self.from,
                    sender_chat: self.sender_chat,
                    author_signature: self.author_signature,
                    forward: self.forward,
                    reply_to_message: self.reply_to_message,
                    edit_date: self.edit_date,
                    reply_markup: self.reply_markup,
                    media_kind,
                    is_topic_message: self.is_topic_message,
                    is_automatic_forward: self.is_automatic_forward,
                    has_protected_content: self.has_protected_content,
                }))
            }
        }
    }

}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageText {
        pub text: String,
        pub entities: Vec<MessageEntity>,
    }
}

impl MockMessageText {
    pub fn new(text: &str) -> Self {
        Self::new_message_common(text.to_string(), vec![])
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Text(MediaText {
                text: self.text,
                entities: self.entities,
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageAnimation {
        pub caption: Option<String>,
        pub caption_entities: Vec<MessageEntity>,
        pub has_media_spoiler: bool,
        // Animation
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
}

impl MockMessageAnimation {
    pub const HAS_MEDIA_SPOILER: bool = false;
    pub const WIDTH: u32 = 50;
    pub const HEIGHT: u32 = 50;
    pub const DURATION: u32 = 50;
    pub const FILE_ID: &'static str = "file_id";
    pub const UNIQUE_FILE_ID: &'static str = "file_unique_id";
    pub const FILE_SIZE: u32 = 50;

    pub fn new() -> Self {
        Self::new_message_common(
            None,
            vec![],
            Self::HAS_MEDIA_SPOILER,
            Self::WIDTH,
            Self::HEIGHT,
            Self::DURATION,
            None,
            None,
            None,
            Self::FILE_ID.to_string(),
            Self::UNIQUE_FILE_ID.to_string(),
            Self::FILE_SIZE,
        )
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Animation(MediaAnimation {
                caption: self.caption,
                caption_entities: self.caption_entities,
                has_media_spoiler: self.has_media_spoiler,
                animation: Animation {
                    file: FileMeta {
                        id: self.file_id,
                        unique_id: self.file_unique_id,
                        size: self.file_size,
                    },
                    width: self.width,
                    height: self.height,
                    duration: self.duration,
                    thumb: self.thumb,
                    file_name: self.file_name,
                    mime_type: self.mime_type,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageAudio {
        pub caption: Option<String>,
        pub caption_entities: Vec<MessageEntity>,
        pub media_group_id: Option<String>,
        // Audio
        pub duration: u32,
        pub performer: Option<String>,
        pub title: Option<String>,
        pub thumb: Option<PhotoSize>,
        pub file_name: Option<String>,
        pub mime_type: Option<Mime>,
        // FileMeta
        pub file_id: String,
        pub file_unique_id: String,
        pub file_size: u32,
    }
}

impl MockMessageAudio {
    pub const DURATION: u32 = 236;
    pub const FILE_ID: &'static str = "CQADAgADbQEAAsnrIUpNoRRNsH7_hAI";
    pub const UNIQUE_FILE_ID: &'static str = "file_unique_id";
    pub const FILE_SIZE: u32 = 9507774;

    pub fn new() -> Self {
        Self::new_message_common(
            None,
            vec![],
            None,
            Self::DURATION,
            None,
            None,
            None,
            None,
            None,
            Self::FILE_ID.to_string(),
            Self::UNIQUE_FILE_ID.to_string(),
            Self::FILE_SIZE,
        )
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Audio(MediaAudio {
                caption: self.caption,
                caption_entities: self.caption_entities,
                media_group_id: self.media_group_id,
                audio: Audio {
                    file: FileMeta {
                        id: self.file_id,
                        unique_id: self.file_unique_id,
                        size: self.file_size,
                    },
                    duration: self.duration,
                    performer: self.performer,
                    title: self.title,
                    thumb: self.thumb,
                    file_name: self.file_name,
                    mime_type: self.mime_type,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageContact {
        pub phone_number: String,
        pub first_name: String,
        pub last_name: Option<String>,
        pub user_id: Option<UserId>,
        pub vcard: Option<String>,
    }
}

impl MockMessageContact {
    pub const PHONE_NUMBER: &'static str = "+123456789";
    pub const FIRST_NAME: &'static str = "First";

    pub fn new() -> Self {
        Self::new_message_common(
            Self::PHONE_NUMBER.to_string(),
            Self::FIRST_NAME.to_string(),
            None,
            None,
            None,
        )
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Contact(MediaContact {
                contact: Contact {
                    phone_number: self.phone_number,
                    first_name: self.first_name,
                    last_name: self.last_name,
                    user_id: self.user_id,
                    vcard: self.vcard,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageDocument {
        pub caption: Option<String>,
        pub caption_entities: Vec<MessageEntity>,
        pub media_group_id: Option<String>,
        // Document
        pub thumb: Option<PhotoSize>,
        pub file_name: Option<String>,
        pub mime_type: Option<Mime>,
        // FileMeta
        pub file_id: String,
        pub file_unique_id: String,
        pub file_size: u32,
    }
}

impl MockMessageDocument {
    pub const FILE_ID: &'static str = "BQADAgADpgADy_JxS66XQTBRHFleAg";
    pub const UNIQUE_FILE_ID: &'static str = "file_unique_id";
    pub const FILE_SIZE: u32 = 21331;

    pub fn new() -> Self {
        Self::new_message_common(
            None,
            vec![],
            None,
            None,
            None,
            None,
            Self::FILE_ID.to_string(),
            Self::UNIQUE_FILE_ID.to_string(),
            Self::FILE_SIZE,
        )
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Document(MediaDocument {
                caption: self.caption,
                caption_entities: self.caption_entities,
                media_group_id: self.media_group_id,
                document: Document {
                    file: FileMeta {
                        id: self.file_id,
                        unique_id: self.file_unique_id,
                        size: self.file_size,
                    },
                    thumb: self.thumb,
                    file_name: self.file_name,
                    mime_type: self.mime_type,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageGame {
        pub title: String,
        pub description: String,
        pub photo: Vec<PhotoSize>,
        pub text: Option<String>,
        pub text_entities: Option<Vec<MessageEntity>>,
        pub animation: Option<Animation>,
    }
}

impl MockMessageGame {
    pub const TITLE: &'static str = "Title";
    pub const DESCRIPTION: &'static str = "Description";

    pub fn new() -> Self {
        Self::new_message_common(
            Self::TITLE.to_string(),
            Self::DESCRIPTION.to_string(),
            vec![MockPhotoSize::new().build()],
            None,
            None,
            None,
        )
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Game(MediaGame {
                game: Game {
                    title: self.title,
                    description: self.description,
                    photo: self.photo,
                    text: self.text,
                    text_entities: self.text_entities,
                    animation: self.animation,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageVenue {
        pub location: Location,
        pub title: String,
        pub address: String,
        pub foursquare_id: Option<String>,
        pub foursquare_type: Option<String>,
        pub google_place_id: Option<String>,
        pub google_place_type: Option<String>,
    }
}

impl MockMessageVenue {
    pub const TITLE: &'static str = "Title";
    pub const ADDRESS: &'static str = "Address";

    pub fn new() -> Self {
        Self::new_message_common(
            MockLocation::new().build(),
            Self::TITLE.to_string(),
            Self::ADDRESS.to_string(),
            None,
            None,
            None,
            None,
        )
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Venue(MediaVenue {
                venue: Venue {
                    location: self.location,
                    title: self.title,
                    address: self.address,
                    foursquare_id: self.foursquare_id,
                    foursquare_type: self.foursquare_type,
                    google_place_id: self.google_place_id,
                    google_place_type: self.google_place_type,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageLocation {
        pub latitude: f64,
        pub longitude: f64,
        pub horizontal_accuracy: Option<f64>,
        pub live_period: Option<u32>,
        pub heading: Option<u16>,
        pub proximity_alert_radius: Option<u32>,
    }
}

impl MockMessageLocation {
    pub const LATITUDE: f64 = 50.0;
    pub const LONGITUDE: f64 = 30.0;

    pub fn new() -> Self {
        Self::new_message_common(Self::LATITUDE, Self::LONGITUDE, None, None, None, None)
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Location(MediaLocation {
                location: Location {
                    longitude: self.longitude,
                    latitude: self.latitude,
                    horizontal_accuracy: self.horizontal_accuracy,
                    live_period: self.live_period,
                    heading: self.heading,
                    proximity_alert_radius: self.proximity_alert_radius,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessagePhoto {
        pub caption: Option<String>,
        pub caption_entities: Vec<MessageEntity>,
        pub media_group_id: Option<String>,
        pub has_media_spoiler: bool,
        pub photo: Vec<PhotoSize>,
    }
}

impl MockMessagePhoto {
    pub const HAS_MEDIA_SPOILER: bool = false;

    pub fn new() -> Self {
        Self::new_message_common(
            None,
            vec![],
            None,
            Self::HAS_MEDIA_SPOILER,
            vec![MockPhotoSize::new().build()],
        )
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Photo(MediaPhoto {
                caption: self.caption,
                caption_entities: self.caption_entities,
                media_group_id: self.media_group_id,
                has_media_spoiler: self.has_media_spoiler,
                photo: self.photo,
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessagePoll {
        pub poll_id: String,
        pub question: String,
        pub options: Vec<PollOption>,
        pub is_closed: bool,
        pub total_voter_count: i32,
        pub is_anonymous: bool,
        pub poll_type: PollType,
        pub allows_multiple_answers: bool,
        pub correct_option_id: Option<u8>,
        pub explanation: Option<String>,
        pub explanation_entities: Option<Vec<MessageEntity>>,
        pub open_period: Option<u16>,
        pub close_date: Option<DateTime<Utc>>,
    }
}

impl MockMessagePoll {
    pub const POLL_ID: &'static str = "12345";
    pub const QUESTION: &'static str = "Question";
    pub const IS_CLOSED: bool = true;
    pub const IS_ANONYMOUS: bool = true;
    pub const TOTAL_VOTER_COUNT: i32 = 50;
    pub const POLL_TYPE: PollType = PollType::Regular;
    pub const ALLOW_MULTIPLE_ANSWERS: bool = true;

    pub fn new() -> Self {
        Self::new_message_common(
            Self::POLL_ID.to_string(),
            Self::QUESTION.to_string(),
            vec![],
            Self::IS_CLOSED,
            Self::TOTAL_VOTER_COUNT,
            Self::IS_ANONYMOUS,
            Self::POLL_TYPE,
            Self::ALLOW_MULTIPLE_ANSWERS,
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Poll(MediaPoll {
                poll: Poll {
                    id: self.poll_id,
                    question: self.question,
                    options: self.options,
                    is_closed: self.is_closed,
                    total_voter_count: self.total_voter_count,
                    is_anonymous: self.is_anonymous,
                    poll_type: self.poll_type,
                    allows_multiple_answers: self.allows_multiple_answers,
                    correct_option_id: self.correct_option_id,
                    explanation: self.explanation,
                    explanation_entities: self.explanation_entities,
                    open_period: self.open_period,
                    close_date: self.close_date,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageSticker {
        pub width: u16,
        pub height: u16,
        pub kind: StickerKind,
        pub format: StickerFormat,
        pub thumb: Option<PhotoSize>,
        pub emoji: Option<String>,
        pub set_name: Option<String>,
        // File meta
        pub file_id: String,
        pub file_unique_id: String,
        pub file_size: u32,
    }
}

impl MockMessageSticker {
    pub const WIDTH: u16 = 512;
    pub const HEIGHT: u16 = 512;
    pub const KIND: StickerKind = StickerKind::Regular {
        premium_animation: None,
    };
    pub const FORMAT: StickerFormat = StickerFormat::Raster;
    pub const FILE_ID: &'static str = "AAbbCCddEEffGGhh1234567890";
    pub const FILE_UNIQUE_ID: &'static str = "file_unique_id";
    pub const FILE_SIZE: u32 = 12345;

    pub fn new() -> Self {
        Self::new_message_common(
            Self::WIDTH,
            Self::HEIGHT,
            Self::KIND,
            Self::FORMAT,
            None,
            None,
            None,
            Self::FILE_ID.to_string(),
            Self::FILE_UNIQUE_ID.to_string(),
            Self::FILE_SIZE,
        )
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Sticker(MediaSticker {
                sticker: Sticker {
                    file: FileMeta {
                        id: self.file_id,
                        unique_id: self.file_unique_id,
                        size: self.file_size,
                    },
                    width: self.width,
                    height: self.height,
                    kind: self.kind,
                    format: self.format,
                    thumb: self.thumb,
                    emoji: self.emoji,
                    set_name: self.set_name,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageVideo {
        pub caption: Option<String>,
        pub caption_entities: Vec<MessageEntity>,
        pub media_group_id: Option<String>,
        pub has_media_spoiler: bool,
        pub video: Video,
    }
}

impl MockMessageVideo {
    pub const HAS_MEDIA_SPOILER: bool = false;

    pub fn new() -> Self {
        Self::new_message_common(
            None,
            vec![],
            None,
            Self::HAS_MEDIA_SPOILER,
            MockVideo::new().build(),
        )
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Video(MediaVideo {
                caption: self.caption,
                caption_entities: self.caption_entities,
                media_group_id: self.media_group_id,
                has_media_spoiler: self.has_media_spoiler,
                video: self.video,
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageVideoNote {
        length: u32,
        duration: u32,
        thumb: Option<PhotoSize>,
        // File meta
        file_id: String,
        file_unique_id: String,
        file_size: u32,
    }
}

impl MockMessageVideoNote {
    pub const LENGTH: u32 = 50;
    pub const DURATION: u32 = 50;
    pub const FILE_ID: &'static str = "file_id";
    pub const FILE_UNIQUE_ID: &'static str = "file_unique_id";
    pub const FILE_SIZE: u32 = 50;

    pub fn new() -> Self {
        Self::new_message_common(
            Self::LENGTH,
            Self::DURATION,
            None,
            Self::FILE_ID.to_string(),
            Self::FILE_UNIQUE_ID.to_string(),
            Self::FILE_SIZE,
        )
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::VideoNote(MediaVideoNote {
                video_note: VideoNote {
                    file: FileMeta {
                        id: self.file_id,
                        unique_id: self.file_unique_id,
                        size: self.file_size,
                    },
                    length: self.length,
                    duration: self.duration,
                    thumb: self.thumb,
                },
            }))
    }
}
