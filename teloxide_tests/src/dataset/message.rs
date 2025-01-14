use core::sync::atomic::{AtomicI32, Ordering};

use chrono::{DateTime, Utc};
use teloxide::types::*;

use super::chat::MockPrivateChat;
use crate::{proc_macros::Changeable, MockUser};

macro_rules! Message {
    (
        #[derive($($derive:meta),*)]
        $pub:vis struct $name:ident {
            $($fpub:vis $field:ident : $type:ty,)*
        }
    ) => {
        #[derive($($derive),*)]
        $pub struct $name {  // This is basically a template
            pub id: MessageId,
            pub thread_id: Option<ThreadId>,
            pub from: Option<User>,
            pub sender_chat: Option<Chat>,
            pub date: DateTime<Utc>,
            pub chat: Chat,
            pub is_topic_message: bool,
            pub via_bot: Option<User>,
            pub sender_business_bot: Option<User>,
            $($fpub $field : $type,)*
        }
        impl $name {
            pub const ID: i32 = 1;
            pub(crate) fn new_message($($field:$type,)*) -> Self{
                Self {  // To not repeat this over and over again
                    id: MessageId($name::ID),
                    thread_id: None,
                    from: Some(MockUser::new().build()),
                    sender_chat: None,
                    date: Utc::now(),
                    chat: MockPrivateChat::new().build(),
                    is_topic_message: false,
                    via_bot: None,
                    sender_business_bot: None,
                    $($field,)*
                }
            }

            pub(crate) fn build_message(self, message_kind: MessageKind) -> Message {
                Message {
                    id: self.id,
                    thread_id: self.thread_id,
                    from: self.from,
                    sender_chat: self.sender_chat,
                    date: self.date,
                    chat: self.chat,
                    is_topic_message: self.is_topic_message,
                    via_bot: self.via_bot,
                    kind: message_kind,
                    sender_business_bot: self.sender_business_bot,
                }
            }
        }

        impl crate::dataset::IntoUpdate for $name {
            /// Converts the MockCallbackQuery into an updates vector
            ///
            /// # Example
            /// ```
            /// use teloxide_tests::IntoUpdate;
            /// use teloxide::types::{UpdateId, UpdateKind::Message};
            /// use std::sync::atomic::AtomicI32;
            ///
            /// let mock_message = teloxide_tests::MockMessageText::new();
            /// let update = mock_message.clone().into_update(&AtomicI32::new(42))[0].clone();
            ///
            /// assert_eq!(update.id, UpdateId(42));
            /// assert_eq!(update.kind, Message(mock_message.build()));
            /// ```
            ///
            fn into_update(self, id: &AtomicI32) -> Vec<Update> {
                vec![Update {
                    id: UpdateId(id.fetch_add(1, Ordering::Relaxed) as u32),
                    kind: UpdateKind::Message(self.build()),
                }]
            }
        }
    }
}

pub(crate) use Message;

// More messages like Webapp data is needed

Message! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageDice {
        pub value: u8,
        pub emoji: DiceEmoji,
    }
}

impl MockMessageDice {
    pub const VALUE: u8 = 1;
    pub const EMOJI: DiceEmoji = DiceEmoji::Dice;

    /// Creates a new easily changable message dice builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessageDice::new()
    ///     .value(2)
    ///     .build();
    /// assert_eq!(message.dice().unwrap().value, 2);
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message(Self::VALUE, Self::EMOJI)
    }

    /// Builds the message dice
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageDice::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.dice().unwrap().emoji, teloxide_tests::MockMessageDice::EMOJI);  // EMOJI is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone().build_message(MessageKind::Dice(MessageDice {
            dice: Dice {
                emoji: self.emoji,
                value: self.value,
            },
        }))
    }
}

Message! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageInvoice {
        pub title: String,
        pub description: String,
        pub start_parameter: String,
        pub currency: String,
        pub total_amount: u32,
    }
}

impl MockMessageInvoice {
    pub const TITLE: &'static str = "Title of Invoice";
    pub const DESCRIPTION: &'static str = "Description of Invoice";
    pub const START_PARAMETER: &'static str = "Start parameter of Invoice";
    pub const CURRENCY: &'static str = "XTR";
    pub const TOTAL_AMOUNT: u32 = 0;

    /// Creates a new easily changable message invoice builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessageInvoice::new()
    ///     .title("Some title")
    ///     .build();
    /// assert_eq!(message.invoice().unwrap().title, "Some title".to_owned());
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message(
            Self::TITLE.to_owned(),
            Self::DESCRIPTION.to_owned(),
            Self::START_PARAMETER.to_owned(),
            Self::CURRENCY.to_owned(),
            Self::TOTAL_AMOUNT,
        )
    }

    /// Builds the message dice
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageInvoice::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.invoice().unwrap().currency, teloxide_tests::MockMessageInvoice::CURRENCY);  // CURRENCY is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message(MessageKind::Invoice(MessageInvoice {
                invoice: Invoice {
                    title: self.title,
                    description: self.description,
                    start_parameter: self.start_parameter,
                    currency: self.currency,
                    total_amount: self.total_amount,
                },
            }))
    }
}

Message! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageNewChatMembers {
        pub new_chat_members: Vec<User>,
    }
}

impl MockMessageNewChatMembers {
    /// Creates a new easily changeable new chat member message builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessageNewChatMembers::new()
    ///     .new_chat_members(vec![teloxide_tests::MockUser::new().id(123).build()])
    ///     .build();
    /// assert_eq!(message.new_chat_members().unwrap()[0].id.0, 123);
    pub fn new() -> Self {
        Self::new_message(vec![MockUser::new().build()])
    }

    /// Builds the new chat member message
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageNewChatMembers::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.new_chat_members().unwrap().len(), 1);  // Contains a single MockUser by default
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message(MessageKind::NewChatMembers(MessageNewChatMembers {
                new_chat_members: self.new_chat_members,
            }))
    }
}
