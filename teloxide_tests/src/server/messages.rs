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

    pub fn edit_message<T>(&mut self, message_id: i32, field: &str, value: T) -> Option<Message>
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
            // Only the inline keyboard can be inside of a message
            Some(ReplyMarkup::InlineKeyboard(reply_markup)) => {
                self.edit_message(message_id, "reply_markup", reply_markup)
            }
            _ => self.get_message(message_id),
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dataset::*;
    use serial_test::serial;
    use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

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
    fn test_edit_messages() {
        let mut messages = Messages::default();
        messages.add_message(
            message_common::MockMessageText::new()
                .text("123")
                .id(1)
                .build(),
        );
        messages.edit_message(1, "text", "1234");
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
    fn test_delete_messages() {
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
