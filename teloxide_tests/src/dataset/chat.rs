use teloxide::types::{
    Chat, ChatId, ChatKind, ChatLocation, ChatPermissions, ChatPhoto, ChatPrivate, ChatPublic,
    Message, PublicChatChannel, PublicChatGroup, PublicChatKind, PublicChatSupergroup, True, *,
};

use super::{MockChatFullInfo, MockUser};
use crate::proc_macros::Changeable;

macro_rules! Chat {
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
            pub available_reactions: Option<Vec<ReactionType>>,
            pub pinned_message: Option<Box<Message>>,
            pub message_auto_delete_time: Option<Seconds>,
            pub has_hidden_members: bool,
            pub has_aggressive_anti_spam_enabled: bool,
            pub chat_full_info: ChatFullInfo,
            $($fpub $field : $type,)*
        }
        impl $name {
            pub const ID: i64 = -12345678;  // Make them into a constant cuz why not
            pub const HAS_HIDDEN_MEMBERS: bool = false;
            pub const AGGRESSIVE_ANTI_SPAM_ENABLED: bool = false;

            pub(crate) fn new_chat($($field:$type,)*) -> Self{
                Self {  // To not repeat this over and over again
                    id: ChatId(Self::ID),
                    photo: None,
                    available_reactions: None,
                    pinned_message: None,
                    message_auto_delete_time: None,
                    has_hidden_members: Self::HAS_HIDDEN_MEMBERS,
                    has_aggressive_anti_spam_enabled: Self::AGGRESSIVE_ANTI_SPAM_ENABLED,
                    chat_full_info: MockChatFullInfo::new().build(),
                    $($field,)*
                }
            }

            pub(crate) fn build_chat(self, chat_kind: ChatKind) -> Chat {
                Chat {
                    id: self.id,
                    kind: chat_kind,
                    available_reactions: self.available_reactions,
                    photo: self.photo,
                    pinned_message: self.pinned_message,
                    message_auto_delete_time: self.message_auto_delete_time,
                    has_hidden_members: self.has_hidden_members,
                    has_aggressive_anti_spam_enabled: self.has_aggressive_anti_spam_enabled,
                    chat_full_info: self.chat_full_info,
                }
            }
        }
    }
}

macro_rules! PublicChat {  // A specialization of Chat!, again, to not repeat myself
    (
        #[derive($($derive:meta),*)]
        $pub:vis struct $name:ident {
            $($fpub:vis $field:ident : $type:ty,)*
        }
    ) => {
        Chat! {
            #[derive($($derive),*)]
            $pub struct $name {
                pub title: Option<String>,
                pub description: Option<String>,
                pub invite_link: Option<String>,
                pub has_protected_content: Option<True>,
                $($fpub $field : $type,)*
            }
        }
        impl $name {
            pub(crate) fn new_public_chat($($field:$type,)*) -> Self {
                 $name::new_chat(
                     None,
                     None,
                     None,
                     None,
                     $($field,)*
                 )
            }

            pub(crate) fn build_public_chat(self, public_chat_kind: PublicChatKind) -> Chat {
                self.clone().build_chat(ChatKind::Public(Box::new(ChatPublic {
                    title: self.title,
                    kind: public_chat_kind,
                    description: self.description,
                    invite_link: self.invite_link,
                    has_protected_content: self.has_protected_content,
                })))
            }
        }
    }
}

PublicChat! {
    #[derive(Changeable, Clone)]
    pub struct MockGroupChat {
        pub permissions: Option<ChatPermissions>,
    }
}

impl MockGroupChat {
    /// Creates a new easily changable group chat builder
    ///
    /// Example:
    /// ```
    /// let chat = teloxide_tests::MockGroupChat::new()
    ///     .id(-1234)
    ///     .build();
    /// assert_eq!(chat.id.0, -1234);
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_public_chat(None)
    }

    /// Builds the group chat
    ///
    /// Example:
    /// ```
    /// let mock_chat = teloxide_tests::MockGroupChat::new();
    /// let chat = mock_chat.build();
    /// assert_eq!(chat.id.0, teloxide_tests::MockGroupChat::ID);  // ID is a default value
    /// ```
    ///
    pub fn build(self) -> Chat {
        self.clone()
            .build_public_chat(PublicChatKind::Group(PublicChatGroup {
                permissions: self.permissions,
            }))
    }
}

PublicChat! {
    #[derive(Changeable, Clone)]
    pub struct MockChannelChat {
        pub linked_chat_id: Option<i64>,
        pub username: Option<String>,
    }
}

impl MockChannelChat {
    /// Creates a new easily changable channel chat builder
    ///
    /// Example:
    /// ```
    /// let chat = teloxide_tests::MockChannelChat::new()
    ///     .id(-1234)
    ///     .username("test_channel")
    ///     .build();
    /// assert_eq!(chat.id.0, -1234);
    /// assert_eq!(chat.username(), Some("test_channel"));
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_public_chat(None, None)
    }

    /// Builds the channel chat
    ///
    /// Example:
    /// ```
    /// let mock_chat = teloxide_tests::MockChannelChat::new();
    /// let chat = mock_chat.build();
    /// assert_eq!(chat.id.0, teloxide_tests::MockChannelChat::ID);  // ID is a default value
    /// assert_eq!(chat.username(), None);
    /// ```
    ///
    pub fn build(self) -> Chat {
        self.clone()
            .build_public_chat(PublicChatKind::Channel(PublicChatChannel {
                linked_chat_id: self.linked_chat_id,
                username: self.username,
            }))
    }
}

PublicChat! {
    #[derive(Changeable, Clone)]
    pub struct MockSupergroupChat {
        pub username: Option<String>,
        pub active_usernames: Option<Vec<String>>,
        pub is_forum: bool,
        pub sticker_set_name: Option<String>,
        pub can_set_sticker_set: Option<bool>,
        pub permissions: Option<ChatPermissions>,
        pub slow_mode_delay: Option<Seconds>,
        pub linked_chat_id: Option<i64>,
        pub location: Option<ChatLocation>,
        pub join_to_send_messages: Option<True>,
        pub join_by_request: Option<True>,
        pub custom_emoji_sticker_set_name: Option<String>,
        pub unrestrict_boost_count: Option<u16>,
    }
}

impl MockSupergroupChat {
    pub const IS_FORUM: bool = false;

    /// Creates a new easily changable supergroup chat builder
    ///
    /// Example:
    /// ```
    /// let chat = teloxide_tests::MockSupergroupChat::new()
    ///     .id(-1234)
    ///     .build();
    /// assert_eq!(chat.id.0, -1234);
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_public_chat(
            None,
            None,
            Self::IS_FORUM,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    /// Builds the supergroup chat
    ///
    /// Example:
    /// ```
    /// let mock_chat = teloxide_tests::MockSupergroupChat::new();
    /// let chat = mock_chat.build();
    /// assert_eq!(chat.id.0, teloxide_tests::MockSupergroupChat::ID);  // ID is a default value
    /// ```
    ///
    pub fn build(self) -> Chat {
        self.clone()
            .build_public_chat(PublicChatKind::Supergroup(PublicChatSupergroup {
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
            }))
    }
}

Chat! {
    #[derive(Changeable, Clone)]
    pub struct MockPrivateChat {
        pub username: Option<String>,
        pub first_name: Option<String>,
        pub last_name: Option<String>,
        pub bio: Option<String>,
        pub has_private_forwards: Option<True>,
        pub has_restricted_voice_and_video_messages: Option<True>,
        pub birthdate: Option<Birthdate>,
        pub business_intro: Option<BusinessIntro>,
        pub business_location: Option<BusinessLocation>,
        pub business_opening_hours: Option<BusinessOpeningHours>,
        pub personal_chat: Option<Box<Chat>>,
    }
}

impl MockPrivateChat {
    /// Creates a new easily changable private chat builder
    ///
    /// Example:
    /// ```
    /// let chat = teloxide_tests::MockPrivateChat::new()
    ///     .id(-1234)
    ///     .build();
    /// assert_eq!(chat.id.0, -1234);
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_chat(
            None, None, None, None, None, None, None, None, None, None, None,
        )
        .id(MockUser::ID as i64)
    }

    /// Builds the private chat
    ///
    /// Example:
    /// ```
    /// let mock_chat = teloxide_tests::MockPrivateChat::new();
    /// let chat = mock_chat.build();
    /// assert_eq!(chat.id.0 as u64, teloxide_tests::MockUser::ID);  // Private chats have the id of users
    /// ```
    ///
    pub fn build(self) -> Chat {
        self.clone().build_chat(ChatKind::Private(ChatPrivate {
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            bio: self.bio,
            has_private_forwards: self.has_private_forwards,
            has_restricted_voice_and_video_messages: self.has_restricted_voice_and_video_messages,
            birthdate: self.birthdate,
            business_intro: self.business_intro,
            business_location: self.business_location,
            business_opening_hours: self.business_opening_hours,
            personal_chat: self.personal_chat,
        }))
    }
}
