use crate::db::models;

pub const NO_SUCH_PHRASE: &str = "There is no such phrase!";

pub const PLEASE_SEND_NUMBER: &str = "Please send a number!";

pub const DELETED_PHRASE: &str = "Your phrase has been deleted!";

pub const CHANGED_NICKNAME: &str = "Your nickname has been changed!

New nickname: ";
pub const START: &str = "Hello there, welcome to the phrase bot!

What do you want to do?";

pub const MENU: &str = "What do you want to do?";
pub const SORRY_BOT_UPDATED: &str =
    "Sorry, bot updated and we lost where you were. Please try again.";
pub const PLEASE_SEND_TEXT: &str = "Please send text!";
pub const NO_MORE_CHARACTERS: &str = "Your message must not be more than 3 characters!";

pub const CHANGE_NICKNAME: &str = "Send me new nickname!

If you want to return, send /cancel";

pub const CANCELED: &str = "Canceled.";

pub fn delete_phrase(all_phrases: &[models::Phrase]) -> String {
    format!(
        "These are your phrases:

{}

Send me a number of a phrase you want to delete!

If you want to return, press /cancel",
        list_all_phrases(all_phrases)
    )
}

pub fn phrase_progress(emoji: Option<&str>, text: Option<&str>, bot_text: Option<&str>) -> String {
    format!(
        "😀Emoji: {}
📝Text: {}
🗂Bot text: {}",
        emoji.unwrap_or("Not set🚫"),
        text.unwrap_or("Not set🚫"),
        bot_text.unwrap_or("Not set🚫")
    )
}

pub fn what_is_new_phrase_emoji() -> String {
    format!(
        "Send an emoji for your phrase💬:

To cancel at any time, send /cancel

{}",
        phrase_progress(None, None, None)
    )
}

pub fn what_is_new_phrase_text(emoji: &str) -> String {
    format!(
        "Now send a text that will trigger a phrase:

{}",
        phrase_progress(Some(emoji), None, None)
    )
}

pub fn what_is_new_phrase_bot_text(emoji: &str, text: &str) -> String {
    format!(
        "And finally, send a text that the bot will send. To mention yourself, add `(me)` in the text, and to mention someone you replied to, add `(reply)`.

Example: (me) hugged (reply) 🤗

{}",
        phrase_progress(Some(emoji), Some(text), None)
    )
}

pub fn added_phrase(emoji: &str, text: &str, bot_text: &str) -> String {
    format!(
        "A new phrase was added!

{}",
        phrase_progress(Some(emoji), Some(text), Some(bot_text))
    )
}

pub fn make_link(name: String, id: u64) -> String {
    format!("<a href=\"tg://user?id={}\">{}</a>", id, name)
}

pub fn make_phrase_string(phrase: &models::Phrase) -> String {
    format!("{} - {} | {}", phrase.text, phrase.emoji, phrase.bot_text)
}

pub fn list_all_phrases(phrases: &[models::Phrase]) -> String {
    phrases
        .iter()
        .map(make_phrase_string)
        .enumerate()
        .map(|(i, phrase)| format!("{}. {}", i + 1, phrase))
        .collect::<Vec<String>>()
        .join("\n\n")
}

pub fn profile(nickname: Option<String>, phrases: &[models::Phrase]) -> String {
    format!(
        "Your nickname📜: {}

Your phrases: 

{}",
        nickname.unwrap_or("Not set🚫".to_string()),
        list_all_phrases(phrases)
    )
}
