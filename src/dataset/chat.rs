use proc_macros::Changeable;
use teloxide::types::{
    Chat, ChatId, ChatKind, ChatLocation, ChatPermissions, ChatPrivate, ChatPublic, Message,
    PublicChatChannel, PublicChatGroup, PublicChatKind, PublicChatSupergroup, True,
};

use super::MockChatPhoto;

pub const DEFAULT_CHAT_ID: i64 = -12345678;
pub const DEFAULT_AGGRESSIVE_ANTI_SPAM_ENABLED: bool = false;
pub const DEFAULT_HAS_HIDDEN_MEMBERS: bool = false;
pub const DEFAULT_IS_FORUM: bool = false;

//
//
//

#[derive(Changeable)]
pub struct MockGroupChat {
    pub id: ChatId,
    pub title: Option<String>,
    pub permissions: Option<ChatPermissions>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub has_protected_content: Option<True>,
    pub photo: Option<MockChatPhoto>,
    pub pinned_message: Option<Box<Message>>,
    pub message_auto_delete_time: Option<u32>,
    pub has_hidden_members: bool,
    pub has_aggressive_anti_spam_enabled: bool,
}

impl MockGroupChat {
    pub fn new() -> Self {
        Self {
            id: ChatId(DEFAULT_CHAT_ID),
            title: None,
            permissions: None,
            description: None,
            invite_link: None,
            has_protected_content: None,
            photo: None,
            pinned_message: None,
            message_auto_delete_time: None,
            has_hidden_members: DEFAULT_HAS_HIDDEN_MEMBERS,
            has_aggressive_anti_spam_enabled: DEFAULT_AGGRESSIVE_ANTI_SPAM_ENABLED,
        }
    }

    pub fn to_object(self) -> Chat {
        Chat {
            id: self.id,
            kind: ChatKind::Public(ChatPublic {
                title: self.title,
                kind: PublicChatKind::Group(PublicChatGroup {
                    permissions: self.permissions,
                }),
                description: self.description,
                invite_link: self.invite_link,
                has_protected_content: self.has_protected_content,
            }),
            photo: if self.photo.is_some() {
                Some(self.photo.unwrap().to_object())
            } else {
                None
            },
            pinned_message: self.pinned_message,
            message_auto_delete_time: self.message_auto_delete_time,
            has_hidden_members: self.has_hidden_members,
            has_aggressive_anti_spam_enabled: self.has_aggressive_anti_spam_enabled,
        }
    }
}

//
//
//

#[derive(Changeable)]
pub struct MockSupergroupChat {
    pub id: ChatId,
    pub title: Option<String>,
    pub username: Option<String>,
    pub active_usernames: Option<Vec<String>>,
    pub is_forum: bool,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
    pub permissions: Option<ChatPermissions>,
    pub slow_mode_delay: Option<u32>,
    pub linked_chat_id: Option<i64>,
    pub location: Option<ChatLocation>,
    pub join_to_send_messages: Option<True>,
    pub join_by_request: Option<True>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub has_protected_content: Option<True>,
    pub photo: Option<MockChatPhoto>,
    pub pinned_message: Option<Box<Message>>,
    pub message_auto_delete_time: Option<u32>,
    pub has_hidden_members: bool,
    pub has_aggressive_anti_spam_enabled: bool,
}

impl MockSupergroupChat {
    pub fn new() -> Self {
        Self {
            id: ChatId(DEFAULT_CHAT_ID),
            title: None,
            username: None,
            active_usernames: None,
            is_forum: DEFAULT_IS_FORUM,
            sticker_set_name: None,
            can_set_sticker_set: None,
            permissions: None,
            slow_mode_delay: None,
            linked_chat_id: None,
            location: None,
            join_to_send_messages: None,
            join_by_request: None,
            description: None,
            invite_link: None,
            has_protected_content: None,
            photo: None,
            pinned_message: None,
            message_auto_delete_time: None,
            has_hidden_members: DEFAULT_HAS_HIDDEN_MEMBERS,
            has_aggressive_anti_spam_enabled: DEFAULT_AGGRESSIVE_ANTI_SPAM_ENABLED,
        }
    }

    pub fn to_object(self) -> Chat {
        Chat {
            id: self.id,
            kind: ChatKind::Public(ChatPublic {
                title: self.title,
                kind: PublicChatKind::Supergroup(PublicChatSupergroup {
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
                }),
                description: self.description,
                invite_link: self.invite_link,
                has_protected_content: self.has_protected_content,
            }),
            photo: if self.photo.is_some() {
                Some(self.photo.unwrap().to_object())
            } else {
                None
            },
            pinned_message: self.pinned_message,
            message_auto_delete_time: self.message_auto_delete_time,
            has_hidden_members: self.has_hidden_members,
            has_aggressive_anti_spam_enabled: self.has_aggressive_anti_spam_enabled,
        }
    }
}

//
//
//

#[derive(Changeable)]
pub struct MockChannelChat {
    pub id: ChatId,
    pub title: Option<String>,
    pub linked_chat_id: Option<i64>,
    pub username: Option<String>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub has_protected_content: Option<True>,
    pub photo: Option<MockChatPhoto>,
    pub pinned_message: Option<Box<Message>>,
    pub message_auto_delete_time: Option<u32>,
    pub has_hidden_members: bool,
    pub has_aggressive_anti_spam_enabled: bool,
}

impl MockChannelChat {
    pub fn new() -> Self {
        Self {
            id: ChatId(DEFAULT_CHAT_ID),
            title: None,
            linked_chat_id: None,
            username: None,
            description: None,
            invite_link: None,
            has_protected_content: None,
            photo: None,
            pinned_message: None,
            message_auto_delete_time: None,
            has_hidden_members: DEFAULT_HAS_HIDDEN_MEMBERS,
            has_aggressive_anti_spam_enabled: DEFAULT_AGGRESSIVE_ANTI_SPAM_ENABLED,
        }
    }

    pub fn to_object(self) -> Chat {
        Chat {
            id: self.id,
            kind: ChatKind::Public(ChatPublic {
                title: self.title,
                kind: PublicChatKind::Channel(PublicChatChannel {
                    linked_chat_id: self.linked_chat_id,
                    username: self.username,
                }),
                description: self.description,
                invite_link: self.invite_link,
                has_protected_content: self.has_protected_content,
            }),
            photo: if self.photo.is_some() {
                Some(self.photo.unwrap().to_object())
            } else {
                None
            },
            pinned_message: self.pinned_message,
            message_auto_delete_time: self.message_auto_delete_time,
            has_hidden_members: self.has_hidden_members,
            has_aggressive_anti_spam_enabled: self.has_aggressive_anti_spam_enabled,
        }
    }
}

//
//
//

#[derive(Changeable)]
pub struct MockPrivateChat {
    pub id: ChatId,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub has_private_forwards: Option<True>,
    pub has_restricted_voice_and_video_messages: Option<True>,
    pub emoji_status_custom_emoji_id: Option<String>,
    pub photo: Option<MockChatPhoto>,
    pub pinned_message: Option<Box<Message>>,
    pub message_auto_delete_time: Option<u32>,
}

impl MockPrivateChat {
    pub fn new() -> Self {
        Self {
            id: ChatId(DEFAULT_CHAT_ID),
            username: None,
            first_name: None,
            last_name: None,
            bio: None,
            has_private_forwards: None,
            has_restricted_voice_and_video_messages: None,
            emoji_status_custom_emoji_id: None,
            photo: None,
            pinned_message: None,
            message_auto_delete_time: None,
        }
    }

    pub fn to_object(self) -> Chat {
        Chat {
            id: self.id,
            kind: ChatKind::Private(ChatPrivate {
                username: self.username,
                first_name: self.first_name,
                last_name: self.last_name,
                bio: self.bio,
                has_private_forwards: self.has_private_forwards,
                has_restricted_voice_and_video_messages: self
                    .has_restricted_voice_and_video_messages,
                emoji_status_custom_emoji_id: self.emoji_status_custom_emoji_id,
            }),
            photo: if self.photo.is_some() {
                Some(self.photo.unwrap().to_object())
            } else {
                None
            },
            pinned_message: self.pinned_message,
            message_auto_delete_time: self.message_auto_delete_time,
            has_hidden_members: false,
            has_aggressive_anti_spam_enabled: false,
        }
    }
}
