use teloxide::types::*;

use super::MockUser;
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
            $($fpub $field : $type,)*
        }
        impl $name {
            pub const ID: i64 = -12345678;  // Make them into a constant cuz why not

            pub(crate) fn new_chat($($field:$type,)*) -> Self {
                Self {  // To not repeat this over and over again
                    id: ChatId(Self::ID),
                    $($field,)*
                }
            }

            pub(crate) fn build_chat(self, chat_kind: ChatKind) -> Chat {
                Chat {
                    id: self.id,
                    kind: chat_kind,
                }
            }
        }
    }
}

macro_rules! ChatPublic {  // A specialization of Chat!, again, to not repeat myself
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
                $($fpub $field : $type,)*
            }
        }
        impl $name {
            pub(crate) fn new_chat_public($($field:$type,)*) -> Self {
                 $name::new_chat(
                     None,
                     $($field,)*
                 )
            }

            pub(crate) fn build_chat_public(self, chat_public_kind: PublicChatKind) -> Chat {
                self.clone().build_chat(ChatKind::Public(ChatPublic {
                    title: self.title,
                    kind: chat_public_kind,
                }))
            }
        }
    }
}

ChatPublic! {
    #[derive(Changeable, Clone)]
    pub struct MockGroupChat { }
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
        Self::new_chat_public()
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
        self.clone().build_chat_public(PublicChatKind::Group)
    }
}

ChatPublic! {
    #[derive(Changeable, Clone)]
    pub struct MockChannelChat {
        pub username: Option<String>,
    }
}

impl MockChannelChat {
    /// Creates a new easily changable channel builder
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
        Self::new_chat_public(None)
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
            .build_chat_public(PublicChatKind::Channel(PublicChatChannel {
                username: self.username,
            }))
    }
}

ChatPublic! {
    #[derive(Changeable, Clone)]
    pub struct MockSupergroupChat {
        pub username: Option<String>,
        pub is_forum: bool,
    }
}

impl MockSupergroupChat {
    pub const IS_FORUM: bool = false;

    /// Creates a new easily changable supergroup chat full info builder
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
        Self::new_chat_public(None, Self::IS_FORUM)
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
            .build_chat_public(PublicChatKind::Supergroup(PublicChatSupergroup {
                username: self.username,
                is_forum: self.is_forum,
            }))
    }
}

Chat! {
    #[derive(Changeable, Clone)]
    pub struct MockPrivateChat {
        pub username: Option<String>,
        pub first_name: Option<String>,
        pub last_name: Option<String>,
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
        Self::new_chat(None, None, None).id(MockUser::ID as i64)
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
        }))
    }
}
