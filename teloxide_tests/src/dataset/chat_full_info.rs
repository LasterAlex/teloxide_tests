use chrono::{DateTime, Utc};
use teloxide::types::*;

use super::MockUser;
use crate::proc_macros::Changeable;

macro_rules! ChatFullInfo {
    (
        #[derive($($derive:meta),*)]
        $pub:vis struct $name:ident {
            $($fpub:vis $field:ident : $type:ty,)*
        }
    ) => {
        #[derive($($derive),*)]
        $pub struct $name {  // This is basically a template
            pub id: ChatId,
            pub photo: Option<ChatPhoto>,
            pub pinned_message: Option<Box<Message>>,
            pub message_auto_delete_time: Option<Seconds>,
            pub has_hidden_members: bool,
            pub has_aggressive_anti_spam_enabled: bool,
            pub accent_color_id: Option<u8>,
            pub background_custom_emoji_id: Option<String>,
            pub profile_accent_color_id: Option<u8>,
            pub profile_background_custom_emoji_id: Option<String>,
            pub emoji_status_custom_emoji_id: Option<String>,
            pub emoji_status_expiration_date: Option<DateTime<Utc>>,
            pub has_visible_history: bool,
            pub max_reaction_count: u8,
            $($fpub $field : $type,)*
        }
        impl $name {
            pub const ID: i64 = -12345678;  // Make them into a constant cuz why not
            pub const HAS_HIDDEN_MEMBERS: bool = false;
            pub const AGGRESSIVE_ANTI_SPAM_ENABLED: bool = false;
            pub const HAS_VISIBLE_HISTORY: bool = true;
            pub const MAX_REACTION_COUNT: u8 = 100;

            pub(crate) fn new_chat_full_info($($field:$type,)*) -> Self{
                Self {  // To not repeat this over and over again
                    id: ChatId(Self::ID),
                    photo: None,
                    pinned_message: None,
                    message_auto_delete_time: None,
                    has_hidden_members: Self::HAS_HIDDEN_MEMBERS,
                    has_aggressive_anti_spam_enabled: Self::AGGRESSIVE_ANTI_SPAM_ENABLED,
                    accent_color_id: None,
                    background_custom_emoji_id: None,
                    profile_accent_color_id: None,
                    profile_background_custom_emoji_id: None,
                    emoji_status_custom_emoji_id: None,
                    emoji_status_expiration_date: None,
                    has_visible_history: Self::HAS_VISIBLE_HISTORY,
                    max_reaction_count: Self::MAX_REACTION_COUNT,
                    $($field,)*
                }
            }

            pub(crate) fn build_chat_full_info(self, chat_full_info_kind: ChatFullInfoKind) -> ChatFullInfo {
                ChatFullInfo {
                    id: self.id,
                    photo: self.photo,
                    pinned_message: self.pinned_message,
                    message_auto_delete_time: self.message_auto_delete_time,
                    has_hidden_members: self.has_hidden_members,
                    has_aggressive_anti_spam_enabled: self.has_aggressive_anti_spam_enabled,
                    accent_color_id: self.accent_color_id,
                    background_custom_emoji_id: self
                        .background_custom_emoji_id
                        .map(Into::into),
                    profile_accent_color_id: self.profile_accent_color_id,
                    profile_background_custom_emoji_id: self
                        .profile_background_custom_emoji_id
                        .map(Into::into),
                    emoji_status_custom_emoji_id: self
                        .emoji_status_custom_emoji_id
                        .map(Into::into),
                    emoji_status_expiration_date: self.emoji_status_expiration_date,
                    has_visible_history: self.has_visible_history,
                    max_reaction_count: self.max_reaction_count,
                    kind: chat_full_info_kind,
                }
            }
        }
    }
}

macro_rules! ChatFullInfoPublic {  // A specialization of Chat!, again, to not repeat myself
    (
        #[derive($($derive:meta),*)]
        $pub:vis struct $name:ident {
            $($fpub:vis $field:ident : $type:ty,)*
        }
    ) => {
        ChatFullInfo! {
            #[derive($($derive),*)]
            $pub struct $name {
                pub title: Option<String>,
                pub description: Option<String>,
                pub invite_link: Option<String>,
                pub has_protected_content: bool,
                pub available_reactions: Option<Vec<ReactionType>>,
                $($fpub $field : $type,)*
            }
        }
        impl $name {
            pub const HAS_PROTECTED_CONTENT: bool = false;

            pub(crate) fn new_chat_full_info_public($($field:$type,)*) -> Self {
                 $name::new_chat_full_info(
                     None,
                     None,
                     None,
                     Self::HAS_PROTECTED_CONTENT,
                     None,
                     $($field,)*
                 )
            }

            pub(crate) fn build_chat_full_info_public(self, chat_full_info_public_kind: ChatFullInfoPublicKind) -> ChatFullInfo {
                self.clone().build_chat_full_info(ChatFullInfoKind::Public(Box::new(ChatFullInfoPublic {
                    title: self.title,
                    kind: chat_full_info_public_kind,
                    description: self.description,
                    invite_link: self.invite_link,
                    has_protected_content: self.has_protected_content,
                    available_reactions: self.available_reactions,
                })))
            }
        }
    }
}

ChatFullInfoPublic! {
    #[derive(Changeable, Clone)]
    pub struct MockChatFullInfoGroup {
        pub permissions: Option<ChatPermissions>,
    }
}

impl MockChatFullInfoGroup {
    /// Creates a new easily changable group chat full info builder
    ///
    /// Example:
    /// ```
    /// let chat = teloxide_tests::MockChatFullInfoGroup::new()
    ///     .id(-1234)
    ///     .build();
    /// assert_eq!(chat.id.0, -1234);
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_chat_full_info_public(None)
    }

    /// Builds the group chat full info
    ///
    /// Example:
    /// ```
    /// let mock_chat = teloxide_tests::MockChatFullInfoGroup::new();
    /// let chat = mock_chat.build();
    /// assert_eq!(chat.id.0, teloxide_tests::MockChatFullInfoGroup::ID);  // ID is a default value
    /// ```
    ///
    pub fn build(self) -> ChatFullInfo {
        self.clone()
            .build_chat_full_info_public(ChatFullInfoPublicKind::Group(ChatFullInfoPublicGroup {
                permissions: self.permissions,
            }))
    }
}

ChatFullInfoPublic! {
    #[derive(Changeable, Clone)]
    pub struct MockChatFullInfoChannel {
        pub username: Option<String>,
        pub linked_chat_id: Option<i64>,
        pub can_send_paid_media: bool,
    }
}

impl MockChatFullInfoChannel {
    pub const CAN_SEND_PAID_MEDIA: bool = false;
    /// Creates a new easily changable channel chat full info builder
    ///
    /// Example:
    /// ```
    /// let chat = teloxide_tests::MockChatFullInfoChannel::new()
    ///     .id(-1234)
    ///     .username("test_channel")
    ///     .build();
    /// assert_eq!(chat.id.0, -1234);
    /// assert_eq!(chat.username(), Some("test_channel"));
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_chat_full_info_public(None, None, Self::CAN_SEND_PAID_MEDIA)
    }

    /// Builds the channel chat full info
    ///
    /// Example:
    /// ```
    /// let mock_chat = teloxide_tests::MockChatFullInfoChannel::new();
    /// let chat = mock_chat.build();
    /// assert_eq!(chat.id.0, teloxide_tests::MockChatFullInfoChannel::ID);  // ID is a default value
    /// assert_eq!(chat.username(), None);
    /// ```
    ///
    pub fn build(self) -> ChatFullInfo {
        self.clone()
            .build_chat_full_info_public(ChatFullInfoPublicKind::Channel(
                ChatFullInfoPublicChannel {
                    username: self.username,
                    linked_chat_id: self.linked_chat_id,
                    can_send_paid_media: self.can_send_paid_media,
                },
            ))
    }
}

ChatFullInfoPublic! {
    #[derive(Changeable, Clone)]
    pub struct MockChatFullInfoSupergroup {
        pub username: Option<String>,
        pub active_usernames: Option<Vec<String>>,
        pub is_forum: bool,
        pub sticker_set_name: Option<String>,
        pub can_set_sticker_set: bool,
        pub custom_emoji_sticker_set_name: Option<String>,
        pub permissions: Option<ChatPermissions>,
        pub slow_mode_delay: Option<Seconds>,
        pub unrestrict_boost_count: Option<u16>,
        pub linked_chat_id: Option<i64>,
        pub location: Option<ChatLocation>,
        pub join_to_send_messages: bool,
        pub join_by_request: bool,
    }
}

impl MockChatFullInfoSupergroup {
    pub const IS_FORUM: bool = false;
    pub const CAN_SET_STICKER_SET: bool = false;
    pub const JOIN_TO_SEND_MESSAGES: bool = false;
    pub const JOIN_BY_REQUEST: bool = false;

    /// Creates a new easily changable supergroup chat full info builder
    ///
    /// Example:
    /// ```
    /// let chat = teloxide_tests::MockChatFullInfoSupergroup::new()
    ///     .id(-1234)
    ///     .build();
    /// assert_eq!(chat.id.0, -1234);
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_chat_full_info_public(
            None,
            None,
            Self::IS_FORUM,
            None,
            Self::CAN_SET_STICKER_SET,
            None,
            None,
            None,
            None,
            None,
            None,
            Self::JOIN_TO_SEND_MESSAGES,
            Self::JOIN_BY_REQUEST,
        )
    }

    /// Builds the supergroup chat
    ///
    /// Example:
    /// ```
    /// let mock_chat = teloxide_tests::MockChatFullInfoSupergroup::new();
    /// let chat = mock_chat.build();
    /// assert_eq!(chat.id.0, teloxide_tests::MockChatFullInfoSupergroup::ID);  // ID is a default value
    /// ```
    ///
    pub fn build(self) -> ChatFullInfo {
        self.clone()
            .build_chat_full_info_public(ChatFullInfoPublicKind::Supergroup(
                ChatFullInfoPublicSupergroup {
                    username: self.username,
                    active_usernames: self.active_usernames,
                    is_forum: self.is_forum,
                    sticker_set_name: self.sticker_set_name,
                    can_set_sticker_set: self.can_set_sticker_set,
                    permissions: self.permissions,
                    slow_mode_delay: self.slow_mode_delay,
                    linked_chat_id: self.linked_chat_id,
                    location: self.location,
                    join_to_send_messages: self.join_to_send_messages,
                    join_by_request: self.join_by_request,
                    custom_emoji_sticker_set_name: self.custom_emoji_sticker_set_name,
                    unrestrict_boost_count: self.unrestrict_boost_count,
                },
            ))
    }
}

ChatFullInfo! {
    #[derive(Changeable, Clone)]
    pub struct MockChatFullInfoPrivate {
        pub username: Option<String>,
        pub first_name: Option<String>,
        pub last_name: Option<String>,
        pub bio: Option<String>,
        pub has_private_forwards: bool,
        pub has_restricted_voice_and_video_messages: bool,
        pub personal_chat: Option<Box<Chat>>,
        pub birthdate: Option<Birthdate>,
        pub business_intro: Option<BusinessIntro>,
        pub business_location: Option<BusinessLocation>,
        pub business_opening_hours: Option<BusinessOpeningHours>,
    }
}

impl MockChatFullInfoPrivate {
    pub const HAS_PRIVATE_FORWARDS: bool = false;
    pub const HAS_RESTRICTED_VOICE_AND_VIDEO_MESSAGES: bool = false;

    /// Creates a new easily changable private chat full info builder
    ///
    /// Example:
    /// ```
    /// let chat = teloxide_tests::MockChatFullInfoPrivate::new()
    ///     .id(-1234)
    ///     .build();
    /// assert_eq!(chat.id.0, -1234);
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_chat_full_info(
            None,
            None,
            None,
            None,
            Self::HAS_PRIVATE_FORWARDS,
            Self::HAS_RESTRICTED_VOICE_AND_VIDEO_MESSAGES,
            None,
            None,
            None,
            None,
            None,
        )
        .id(MockUser::ID as i64)
    }

    /// Builds the private chat full info
    ///
    /// Example:
    /// ```
    /// let mock_chat = teloxide_tests::MockChatFullInfoPrivate::new();
    /// let chat = mock_chat.build();
    /// assert_eq!(chat.id.0 as u64, teloxide_tests::MockUser::ID);  // Private chats have the id of users
    /// ```
    ///
    pub fn build(self) -> ChatFullInfo {
        self.clone()
            .build_chat_full_info(ChatFullInfoKind::Private(Box::new(ChatFullInfoPrivate {
                username: self.username,
                first_name: self.first_name,
                last_name: self.last_name,
                bio: self.bio,
                has_private_forwards: self.has_private_forwards,
                has_restricted_voice_and_video_messages: self
                    .has_restricted_voice_and_video_messages,
                birthdate: self.birthdate,
                business_intro: self.business_intro,
                business_location: self.business_location,
                business_opening_hours: self.business_opening_hours,
                personal_chat: self.personal_chat,
            })))
    }
}
