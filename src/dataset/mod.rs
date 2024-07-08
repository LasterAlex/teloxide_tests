use proc_macros::Changeable;
use teloxide::types::{
    Chat, ChatId, ChatKind, ChatPermissions, ChatPhoto, ChatPrivate, ChatPublic, Message,
    PublicChatGroup, PublicChatKind, True, User, UserId,
};
#[cfg(test)]
mod tests;

const DEFAULT_USER_ID: u64 = 12345678;
const DEFAULT_FIRST_NAME: &str = "First";
const DEFAULT_IS_BOT: bool = false;
const DEFAULT_ADDED_TO_ATTACHMENT_MENU: bool = false;
const DEFAULT_IS_PREMIUM: bool = false;

const DEFAULT_CHAT_ID: i64 = -12345678;
const DEFAULT_AGGRESSIVE_ANTI_SPAM_ENABLED: bool = false;
const DEFAULT_HAS_HIDDEN_MEMBERS: bool = false;

#[derive(Changeable)]
pub struct TestUser {
    pub id: UserId,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub added_to_attachment_menu: bool,
    pub is_premium: bool,
}

impl TestUser {
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

#[derive(Changeable)]
pub struct TestPublicGroupChat {
    pub id: ChatId,
    pub title: Option<String>,
    pub permissions: Option<ChatPermissions>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub has_protected_content: Option<True>,
    pub photo: Option<ChatPhoto>,
    pub pinned_message: Option<Box<Message>>,
    pub message_auto_delete_time: Option<u32>,
    pub has_hidden_members: bool,
    pub has_aggressive_anti_spam_enabled: bool,
}

impl TestPublicGroupChat {
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
            photo: self.photo,
            pinned_message: self.pinned_message,
            message_auto_delete_time: self.message_auto_delete_time,
            has_hidden_members: self.has_hidden_members,
            has_aggressive_anti_spam_enabled: self.has_aggressive_anti_spam_enabled,
        }
    }
}

#[derive(Changeable)]
pub struct TestPrivateChat {
    pub id: ChatId,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub has_private_forwards: Option<True>,
    pub has_restricted_voice_and_video_messages: Option<True>,
    pub emoji_status_custom_emoji_id: Option<String>,
    pub photo: Option<ChatPhoto>,
    pub pinned_message: Option<Box<Message>>,
    pub message_auto_delete_time: Option<u32>,
}

impl TestPrivateChat {
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
            photo: self.photo,
            pinned_message: self.pinned_message,
            message_auto_delete_time: self.message_auto_delete_time,
            has_hidden_members: false,
            has_aggressive_anti_spam_enabled: false,
        }
    }
}
