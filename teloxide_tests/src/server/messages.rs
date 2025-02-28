use std::collections::HashSet;

use serde::Serialize;
use teloxide::types::{Message, ReplyMarkup};

#[derive(Default)]
pub struct Messages {
    pub messages: Vec<Message>,
    last_message_id: i32,
}

impl Messages {
    pub fn max_message_id(&self) -> i32 {
        self.last_message_id
    }

    pub fn edit_message(&mut self, message: Message) -> Option<Message> {
        self.messages.iter().find(|m| m.id == message.id)?; // Find the message (return None if not found)

        self.messages.retain(|m| m.id != message.id); // Remove the old message
        self.messages.push(message.clone()); // Add the new message
        Some(message) // Profit!
    }

    pub fn edit_message_field<T>(
        &mut self,
        message_id: i32,
        field: &str,
        value: T,
    ) -> Option<Message>
    where
        T: Serialize,
    {
        let message = self.messages.iter().find(|m| m.id.0 == message_id)?; // Find the message
                                                                            // (return None if not found)

        let mut json = serde_json::to_value(message).ok()?; // Convert the message to JSON
        json[field] = serde_json::to_value(value).ok()?; // Edit the field
        let new_message: Message = serde_json::from_value(json).ok()?; // Convert back to Message

        self.messages.retain(|m| m.id.0 != message_id); // Remove the old message
        self.messages.push(new_message.clone()); // Add the new message
        Some(new_message) // Profit!
    }

    pub fn edit_message_reply_markup(
        &mut self,
        message_id: i32,
        reply_markup: Option<ReplyMarkup>,
    ) -> Option<Message> {
        match reply_markup {
            None => {
                // Telegram deletes reply markup when `editMessageText` is called without any.
                self.edit_message_field(message_id, "reply_markup", None::<()>)
            }
            // Only the inline keyboard can be inside of a message
            Some(ReplyMarkup::InlineKeyboard(reply_markup)) => {
                self.edit_message_field(message_id, "reply_markup", reply_markup)
            }
            _ => unreachable!("Only InlineKeyboard is allowed"),
        }
    }

    pub fn add_message(&mut self, message: Message) -> Message {
        self.messages.push(message.clone());
        self.last_message_id += 1;
        message
    }

    pub fn get_message(&self, message_id: i32) -> Option<Message> {
        self.messages.iter().find(|m| m.id.0 == message_id).cloned()
    }

    pub fn delete_message(&mut self, message_id: i32) -> Option<Message> {
        let message = self
            .messages
            .iter()
            .find(|m| m.id.0 == message_id)
            .cloned()?;
        self.messages.retain(|m| m.id.0 != message_id);
        Some(message)
    }

    pub fn delete_messages(&mut self, message_ids: &[i32]) -> Vec<Message> {
        let message_ids: HashSet<i32> = message_ids.iter().cloned().collect();
        let deleted = self
            .messages
            .iter()
            .filter(|m| message_ids.contains(&m.id.0))
            .cloned()
            .collect();
        self.messages.retain(|m| !message_ids.contains(&m.id.0));
        deleted
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use serial_test::serial;
    use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, MessageId};

    use super::*;
    use crate::dataset::*;

    #[test]
    #[serial]
    fn test_add_messages() {
        let mut messages = Messages::default();
        messages.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(1)
                .build(),
        );
        messages.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(2)
                .build(),
        );
        messages.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(3)
                .build(),
        );
        assert_eq!(messages.max_message_id(), 3);
    }

    #[test]
    #[serial]
    fn test_edit_message() {
        let mut messages = Messages::default();
        let date = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        messages.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(1)
                .build(),
        );
        messages.edit_message(
            message_common::MockMessageText::new()
                .text("321")
                .edit_date(date)
                .id(1)
                .build(),
        );
        let message = messages.get_message(1).unwrap();
        assert_eq!(message.text().unwrap(), "321");
        assert_eq!(message.edit_date().unwrap(), &date);
    }

    #[test]
    #[serial]
    fn test_edit_message_field() {
        let mut messages = Messages::default();
        messages.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(1)
                .build(),
        );
        messages.edit_message_field(1, "text", "1234");
        assert_eq!(messages.get_message(1).unwrap().text().unwrap(), "1234");
    }

    #[test]
    #[serial]
    fn test_get_messages() {
        let mut messages = Messages::default();
        messages.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(1)
                .build(),
        );
        assert_eq!(messages.get_message(1).unwrap().text().unwrap(), "123");
    }

    #[test]
    #[serial]
    fn test_delete_message() {
        let mut messages = Messages::default();
        messages.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(1)
                .build(),
        );
        messages.delete_message(1);
        assert_eq!(messages.get_message(1), None);
    }

    #[test]
    #[serial]
    fn test_delete_messages() {
        let mut messages = Messages::default();
        for id in 1..=5 {
            messages.add_message(
                message_common::MockMessageText::new()
                    .text(format!("Message {}", id))
                    .id(id)
                    .build(),
            );
        }

        let deleted = messages.delete_messages(&[2, 3]);

        assert_eq!(deleted.len(), 2);
        assert_eq!(deleted[0].id, MessageId(2));
        assert_eq!(deleted[1].id, MessageId(3));

        assert!(messages.get_message(1).is_some());
        assert_eq!(messages.get_message(2), None);
        assert_eq!(messages.get_message(3), None);
        assert!(messages.get_message(4).is_some());
        assert!(messages.get_message(5).is_some());
    }

    #[test]
    #[serial]
    fn test_edit_message_reply_markup() {
        let mut messages = Messages::default();
        messages.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(1)
                .build(),
        );
        messages.edit_message_reply_markup(
            1,
            Some(ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup::new(
                vec![vec![InlineKeyboardButton::callback("123", "123")]],
            ))),
        );
        assert_eq!(
            messages
                .get_message(1)
                .unwrap()
                .reply_markup()
                .unwrap()
                .inline_keyboard[0][0]
                .text,
            "123"
        );
    }
}
