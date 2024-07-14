use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub const PROFILE_BUTTON: &str = "Profile";
pub const ADD_PHRASE_BUTTON: &str = "Add a phrase";
pub const REMOVE_PHRASE_BUTTON: &str = "Remove a phrase";
pub const CHANGE_NICKNAME_BUTTON: &str = "Change nickname";

pub fn menu_keyboard() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![KeyboardButton::new(PROFILE_BUTTON)],
        vec![
            KeyboardButton::new(ADD_PHRASE_BUTTON),
            KeyboardButton::new(REMOVE_PHRASE_BUTTON),
        ],
        vec![KeyboardButton::new(CHANGE_NICKNAME_BUTTON)],
    ])
    .persistent()
}
