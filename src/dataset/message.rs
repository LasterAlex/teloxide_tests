use crate::dataset::{chat::MockPrivateChat, MockUser};
use chrono::{DateTime, Utc};
use mime::Mime;
use proc_macros::Changeable;
use teloxide::types::*;

use super::{MockLocation, MockPhotoSize, DEFAULT_USER_ID};

pub const DEFAULT_MESSAGE_ID: i32 = 1;
pub const DEFAULT_IS_TOPIC_MESSAGE: bool = false;
pub const DEFAULT_IS_AUTOMATIC_FORWARD: bool = false;
pub const DEFAULT_HAS_PROTECTED_CONTENT: bool = false;
pub const DEFAULT_HAS_MEDIA_SPOILER: bool = false;
pub const DEFAULT_POLL_ID: &str = "12345";
pub const DEFAULT_IS_ANONYMOUS: bool = true;
pub const DEFAULT_IS_CLOSED: bool = true;

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
            $pub fn new_message($($field:$type,)*) -> Self{
                Self {  // To not repeat this over and over again
                    id: MessageId(DEFAULT_MESSAGE_ID),
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
            $pub fn new_message_common($($field:$type,)*) -> Self {
                 $name::new_message(
                     Some(MockUser::new().build()),
                     Some(MockPrivateChat::new().build()),
                     None,
                     None,
                     None,
                     None,
                     None,
                     DEFAULT_IS_TOPIC_MESSAGE,
                     DEFAULT_IS_AUTOMATIC_FORWARD,
                     DEFAULT_HAS_PROTECTED_CONTENT,
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
    pub fn new(
        width: u32,
        height: u32,
        duration: u32,
        file_id: &str,
        file_unique_id: &str,
        file_size: u32,
    ) -> Self {
        Self::new_message_common(
            None,
            vec![],
            DEFAULT_HAS_MEDIA_SPOILER,
            width,
            height,
            duration,
            None,
            None,
            None,
            file_id.to_string(),
            file_unique_id.to_string(),
            file_size,
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
    pub fn new(duration: u32, file_id: &str, file_unique_id: &str, file_size: u32) -> Self {
        Self::new_message_common(
            None,
            vec![],
            None,
            duration,
            None,
            None,
            None,
            None,
            None,
            file_id.to_string(),
            file_unique_id.to_string(),
            file_size,
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
    pub fn new(phone_number: &str, first_name: &str) -> Self {
        Self::new_message_common(
            phone_number.to_string(),
            first_name.to_string(),
            None,
            Some(UserId(DEFAULT_USER_ID)),
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
    pub fn new(file_id: &str, file_unique_id: &str, file_size: u32) -> Self {
        Self::new_message_common(
            None,
            vec![],
            None,
            None,
            None,
            None,
            file_id.to_string(),
            file_unique_id.to_string(),
            file_size,
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
    pub fn new(title: &str, description: &str, photo: Vec<MockPhotoSize>) -> Self {
        Self::new_message_common(
            title.to_string(),
            description.to_string(),
            photo.iter().map(|ps| ps.clone().build()).collect(),
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
    pub fn new(location: MockLocation, title: &str, address: &str) -> Self {
        Self::new_message_common(
            location.build(),
            title.to_string(),
            address.to_string(),
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
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self::new_message_common(latitude, longitude, None, None, None, None)
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
    pub fn new(photo: Vec<MockPhotoSize>) -> Self {
        Self::new_message_common(
            None,
            vec![],
            None,
            false,
            photo.iter().map(|ps| ps.clone().build()).collect(),
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
    pub fn new(
        question: &str,
        options: Vec<PollOption>,
        total_voter_count: i32,
        poll_type: PollType,
        allows_multiple_answers: bool,
    ) -> Self {
        Self::new_message_common(
            DEFAULT_POLL_ID.to_string(),
            question.to_string(),
            options,
            DEFAULT_IS_CLOSED,
            total_voter_count,
            DEFAULT_IS_ANONYMOUS,
            poll_type,
            allows_multiple_answers,
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
    pub fn new(
        width: u16,
        height: u16,
        kind: StickerKind,
        format: StickerFormat,
        file_id: String,
        file_unique_id: String,
        file_size: u32,
    ) -> Self {
        Self::new_message_common(
            width,
            height,
            kind,
            format,
            None,
            None,
            None,
            file_id,
            file_unique_id,
            file_size,
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
