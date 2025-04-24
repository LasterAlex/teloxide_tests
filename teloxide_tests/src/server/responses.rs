use teloxide::types::{Message, MessageId};

use super::routes::{
    answer_callback_query::*, ban_chat_member::*, copy_message::*, delete_message::*,
    edit_message_caption::*, edit_message_reply_markup::*, edit_message_text::*,
    forward_message::*, pin_chat_message::*, restrict_chat_member::*, send_animation::*,
    send_audio::*, send_chat_action::*, send_contact::*, send_dice::*, send_document::*,
    send_invoice::*, send_location::*, send_media_group::*, send_message::*, send_photo::*,
    send_poll::*, send_sticker::*, send_venue::*, send_video::*, send_video_note::*, send_voice::*,
    set_message_reaction::*, set_my_commands::*, unban_chat_member::*, unpin_all_chat_messages::*,
    unpin_chat_message::*,
};

#[derive(Clone, Debug)]
pub struct SentMessageText {
    // For better syntax, this is a struct, not a tuple
    pub message: Message,
    pub bot_request: SendMessageTextBody,
}

#[derive(Clone, Debug)]
pub struct SentMessagePhoto {
    pub message: Message,
    pub bot_request: SendMessagePhotoBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageVideo {
    pub message: Message,
    pub bot_request: SendMessageVideoBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageAudio {
    pub message: Message,
    pub bot_request: SendMessageAudioBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageVoice {
    pub message: Message,
    pub bot_request: SendMessageVoiceBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageVideoNote {
    pub message: Message,
    pub bot_request: SendMessageVideoNoteBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageDocument {
    pub message: Message,
    pub bot_request: SendMessageDocumentBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageAnimation {
    pub message: Message,
    pub bot_request: SendMessageAnimationBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageLocation {
    pub message: Message,
    pub bot_request: SendMessageLocationBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageVenue {
    pub message: Message,
    pub bot_request: SendMessageVenueBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageContact {
    pub message: Message,
    pub bot_request: SendMessageContactBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageDice {
    pub message: Message,
    pub bot_request: SendMessageDiceBody,
}

#[derive(Clone, Debug)]
pub struct SentMessagePoll {
    pub message: Message,
    pub bot_request: SendMessagePollBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageSticker {
    pub message: Message,
    pub bot_request: SendMessageStickerBody,
}

#[derive(Clone, Debug)]
pub struct SentMediaGroup {
    pub messages: Vec<Message>,
    pub bot_request: SendMediaGroupBody,
}

#[derive(Clone, Debug)]
pub struct SentMessageInvoice {
    pub message: Message,
    pub bot_request: SendMessageInvoiceBody,
}

#[derive(Clone, Debug)]
pub struct EditedMessageText {
    pub message: Message,
    pub bot_request: EditMessageTextBody,
}

#[derive(Clone, Debug)]
pub struct EditedMessageCaption {
    pub message: Message,
    pub bot_request: EditMessageCaptionBody,
}

#[derive(Clone, Debug)]
pub struct DeletedMessage {
    pub message: Message,
    pub bot_request: DeleteMessageBody,
}

#[derive(Clone, Debug)]
pub struct EditedMessageReplyMarkup {
    pub message: Message,
    pub bot_request: EditMessageReplyMarkupBody,
}

#[derive(Clone, Debug)]
pub struct ForwardedMessage {
    pub message: Message,
    pub bot_request: ForwardMessageBody,
}

#[derive(Clone, Debug)]
pub struct CopiedMessage {
    pub message_id: MessageId,
    pub bot_request: CopyMessageBody,
}

#[derive(Clone, Debug, Default)]
pub struct Responses {
    /// All of the sent messages, including text, photo, audio, etc.
    /// Be warned, editing or deleting messages do not affect this list!
    pub sent_messages: Vec<Message>,

    /// This has only messages that are text messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_text: Vec<SentMessageText>,

    /// This has only messages that are photo messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_photo: Vec<SentMessagePhoto>,

    /// This has only messages that are video messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_video: Vec<SentMessageVideo>,

    /// This has only messages that are audio messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_audio: Vec<SentMessageAudio>,

    /// This has only messages that are voice messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_voice: Vec<SentMessageVoice>,

    /// This has only messages that are video note messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_video_note: Vec<SentMessageVideoNote>,

    /// This has only messages that are document messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_document: Vec<SentMessageDocument>,

    /// This has only messages that are animation messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_animation: Vec<SentMessageAnimation>,

    /// This has only messages that are location messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_location: Vec<SentMessageLocation>,

    /// This has only messages that are venue messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_venue: Vec<SentMessageVenue>,

    /// This has only messages that are contact messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_contact: Vec<SentMessageContact>,

    /// This has only messages that are dice messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_dice: Vec<SentMessageDice>,

    /// This has only messages that are poll messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_poll: Vec<SentMessagePoll>,

    /// This has only messages that are stickers, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_sticker: Vec<SentMessageSticker>,

    /// This has only messages that are media group messages, sent by the bot.
    /// The `.messages` field has the sent by bot messages, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_media_group: Vec<SentMediaGroup>,

    /// This has only messages that are invoice messages, sent by the bot.
    /// The `.message` field has the sent by bot message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub sent_messages_invoice: Vec<SentMessageInvoice>,

    /// This has only edited by the bot text messages.
    /// The `.message` field has the new edited message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub edited_messages_text: Vec<EditedMessageText>,

    /// This has only edited by the bot caption messages.
    /// The `.message` field has the new edited message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub edited_messages_caption: Vec<EditedMessageCaption>,

    /// This has only messages whos reply markup was edited by the bot.
    /// The `.message` field has the new edited message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub edited_messages_reply_markup: Vec<EditedMessageReplyMarkup>,

    /// This has only messages which were deleted by the bot.
    /// The `.message` field has the deleted message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub deleted_messages: Vec<DeletedMessage>,

    /// This has only the requests that were sent to the fake server to forward messages.
    /// The `.message` field has the forwarded message, and `.bot_request`
    /// has the request that was sent to the fake server
    pub forwarded_messages: Vec<ForwardedMessage>,

    /// This has only the requests that were sent to the fake server to copy messages.
    /// The `.message_id` field has the copied message id, and `.bot_request`
    /// has the request that was sent to the fake server
    pub copied_messages: Vec<CopiedMessage>,

    /// This has only the requests that were sent to the fake server to answer callback queries.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub answered_callback_queries: Vec<AnswerCallbackQueryBody>,

    /// This has only the requests that were sent to the fake server to pin messages.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub pinned_chat_messages: Vec<PinChatMessageBody>,

    /// This has only the requests that were sent to the fake server to unpin messages.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub unpinned_chat_messages: Vec<UnpinChatMessageBody>,

    /// This has only the requests that were sent to the fake server to unpin all messages.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub unpinned_all_chat_messages: Vec<UnpinAllChatMessagesBody>,

    /// This has only the requests that were sent to the fake server to ban chat members.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub banned_chat_members: Vec<BanChatMemberBody>,

    /// This has only the requests that were sent to the fake server to unban chat members.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub unbanned_chat_members: Vec<UnbanChatMemberBody>,

    /// This has only the requests that were sent to the fake server to restrict chat members.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub restricted_chat_members: Vec<RestrictChatMemberBody>,

    /// This has only the requests that were sent to the fake server to send chat actions.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub sent_chat_actions: Vec<SendChatActionBody>,

    /// This has only the requests that were sent to the fake server to set message reactions.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub set_message_reaction: Vec<SetMessageReactionBody>,

    /// This has only the requests that were sent to the fake server to set message reactions.
    /// Telegram doesn't return anything, because there isn't anything to return, so there is no
    /// `.message` field.
    pub set_my_commands: Vec<SetMyCommandsBody>,
}
