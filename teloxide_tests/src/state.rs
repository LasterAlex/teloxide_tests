use teloxide::{
    prelude::*,
    types::{File, MessageId, MessageKind},
};

use crate::{server::messages::Messages, utils::find_file, MockMessageText, Responses};

#[derive(Default)]
pub(crate) struct State {
    pub files: Vec<File>,
    pub responses: Responses,
    pub messages: Messages,
}

impl State {
    pub fn reset(&mut self) {
        self.responses = Responses::default();
    }

    pub(crate) fn add_message(&mut self, message: &mut Message) {
        let max_id = self.messages.max_message_id();
        let maybe_message = self.messages.get_message(message.id.0);

        // If message exists in the database, and it isn't a default,
        // let it be, the user knows best
        if maybe_message.is_some() && message.id != MessageId(MockMessageText::ID) {
            log::debug!(
                "Not inserting message with id {}, this id exists in the database.",
                message.id
            );
            return;
        }

        if message.id.0 <= max_id || maybe_message.is_some() {
            message.id = MessageId(max_id + 1);
        }

        if let Some(file_meta) = find_file(serde_json::to_value(&message).unwrap()) {
            let file = File {
                meta: file_meta,
                path: "some_path.txt".to_string(), // This doesn't really matter
            };
            self.files.push(file);
        }
        if let MessageKind::Common(ref mut message_kind) = message.kind {
            if let Some(ref mut reply_message) = message_kind.reply_to_message {
                self.add_message(reply_message);
            }
        }
        log::debug!("Inserted message with {}.", message.id);
        self.messages.add_message(message.clone());
    }

    pub(crate) fn edit_message(&mut self, message: &mut Message) {
        let old_message = self.messages.get_message(message.id.0);

        if old_message.is_none() {
            log::error!(
                "Not editing message with id {}, this id does not exist in the database.",
                message.id
            );
            return;
        }

        if let Some(file_meta) = find_file(serde_json::to_value(&message).unwrap()) {
            if self
                .files
                .iter()
                .all(|f| f.meta.unique_id != file_meta.unique_id)
            {
                let file = File {
                    meta: file_meta,
                    path: "some_path.txt".to_string(), // This doesn't really matter
                };
                self.files.push(file);
            }
        }
        log::debug!("Edited message with {}.", message.id);
        self.messages.edit_message(message.clone());
    }
}
