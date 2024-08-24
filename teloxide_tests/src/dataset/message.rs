use super::chat::MockPrivateChat;
use crate::proc_macros::Changeable;
use chrono::{DateTime, Utc};
use core::sync::atomic::{AtomicI32, Ordering};
use teloxide::types::*;
use crate::MockUser;

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
                }
            }
        }

        impl crate::dataset::IntoUpdate for $name {
            /// Converts the MockCallbackQuery into an updates vector
            ///
            /// # Example
            /// ```
            /// use teloxide_tests::IntoUpdate;
            /// let mock_message = teloxide_tests::MockMessageText::new();
            /// let update = mock_message.clone().into_update(1.into())[0].clone();
            /// assert_eq!(update.id, teloxide::types::UpdateId(1));
            /// assert_eq!(update.kind, teloxide::types::UpdateKind::Message(
            ///     mock_message.build())
            /// );
            /// ```
            ///
            fn into_update(self, id: AtomicI32) -> Vec<Update> {
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
