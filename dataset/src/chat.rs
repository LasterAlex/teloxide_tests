use proc_macros::Changeable;
use teloxide::types::{
    Chat, ChatId, ChatKind, ChatLocation, ChatPermissions, ChatPhoto, ChatPrivate, ChatPublic,
    Message, PublicChatChannel, PublicChatGroup, PublicChatKind, PublicChatSupergroup, True,
};

use super::MockUser;

macro_rules! Chat {
    (#[derive($($derive:meta),*)] $pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty,)* }) => {
        #[derive($($derive),*)]
        $pub struct $name {  // This is basically a template
            pub id: ChatId,
            pub photo: Option<ChatPhoto>,
            pub pinned_message: Option<Box<Message>>,
            pub message_auto_delete_time: Option<u32>,
            pub has_hidden_members: bool,
            pub has_aggressive_anti_spam_enabled: bool,
            $($fpub $field : $type,)*
        }
        impl $name {
            pub const ID: i64 = -12345678;  // Make them into a constant cuz why not
            pub const HAS_HIDDEN_MEMBERS: bool = false;
            pub const AGGRESSIVE_ANTI_SPAM_ENABLED: bool = false;

            $pub fn new_chat($($field:$type,)*) -> Self{
                Self {  // To not repeat this over and over again
                    id: ChatId(Self::ID),
                    photo: None,
                    pinned_message: None,
                    message_auto_delete_time: None,
                    has_hidden_members: Self::HAS_HIDDEN_MEMBERS,
                    has_aggressive_anti_spam_enabled: Self::AGGRESSIVE_ANTI_SPAM_ENABLED,
                    $($field,)*
                }
            }

            $pub fn build_chat(self, chat_kind: ChatKind) -> Chat {
                Chat {
                    id: self.id,
                    kind: chat_kind,
                    photo: self.photo,
                    pinned_message: self.pinned_message,
                    message_auto_delete_time: self.message_auto_delete_time,
                    has_hidden_members: self.has_hidden_members,
                    has_aggressive_anti_spam_enabled: self.has_aggressive_anti_spam_enabled,
                }
            }
        }
    }
}

macro_rules! PublicChat {  // A specialization of Chat!, again, to not repeat myself
    (#[derive($($derive:meta),*)] $pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty,)* }) => {
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
            $pub fn new_public_chat($($field:$type,)*) -> Self {
                 $name::new_chat(
                     None,
                     None,
                     None,
                     None,
                     $($field,)*
                 )
            }

            $pub fn build_public_chat(self, public_chat_kind: PublicChatKind) -> Chat {
                self.clone().build_chat(ChatKind::Public(ChatPublic {
                    title: self.title,
                    kind: public_chat_kind,
                    description: self.description,
                    invite_link: self.invite_link,
                    has_protected_content: self.has_protected_content,
                }))
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
    pub fn new() -> Self {
        Self::new_public_chat(None)
    }

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
    pub fn new() -> Self {
        Self::new_public_chat(None, None)
    }

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
        pub slow_mode_delay: Option<u32>,
        pub linked_chat_id: Option<i64>,
        pub location: Option<ChatLocation>,
        pub join_to_send_messages: Option<True>,
        pub join_by_request: Option<True>,
    }
}

impl MockSupergroupChat {
    pub const IS_FORUM: bool = false;

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
        )
    }

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
        pub emoji_status_custom_emoji_id: Option<String>,
    }
}

impl MockPrivateChat {
    pub fn new() -> Self {
        Self::new_chat(None, None, None, None, None, None, None).id(MockUser::ID as i64)
    }

    pub fn build(self) -> Chat {
        self.clone().build_chat(ChatKind::Private(ChatPrivate {
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            bio: self.bio,
            has_private_forwards: self.has_private_forwards,
            has_restricted_voice_and_video_messages: self.has_restricted_voice_and_video_messages,
            emoji_status_custom_emoji_id: self.emoji_status_custom_emoji_id,
        }))
    }
}
