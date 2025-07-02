use teloxide::{
    dispatching::dialogue::GetChatId,
    types::{
        CallbackQueryId, ChatId, FileId, MediaGroupId, MessageEntity, MessageId, PollId, UpdateId,
        UpdateKind, UserId,
    },
};
use update::MockUpdatePoll;

use crate::{dataset::*, proc_macros::Changeable};

#[derive(Changeable)]
struct Test {
    pub field1: String,
    pub field2: ChatId,
    pub field3: Option<String>,
    pub field4: Option<i32>,
}

#[test]
fn test_changeable() {
    let test = Test {
        field1: "123".to_string(),
        field2: ChatId(456),
        field3: None,
        field4: None,
    };
    let test = test.field1("789").field2(1234).field3("456").field4(123);

    assert_eq!(test.field1, "789");
    assert_eq!(test.field2, ChatId(1234));
    assert_eq!(test.field3, Some("456".to_string()));
    assert_eq!(test.field4, Some(123));
}

//
//
//

#[test]
fn test_user() {
    let user = MockUser::new()
        .first_name("Test")
        .last_name("User")
        .id(1234)
        .username("test_user");

    let user_object = user.build();
    assert_eq!(user_object.first_name, "Test");
    assert_eq!(user_object.last_name, Some("User".to_string()));
    assert_eq!(user_object.id, UserId(1234));
    assert_eq!(user_object.username, Some("test_user".to_string()));
}

#[test]
fn test_location() {
    let location = MockLocation::new().latitude(0.0).longitude(1.0);
    let location_object = location.build();
    assert_eq!(location_object.latitude, 0.0);
    assert_eq!(location_object.longitude, 1.0);
}

//
//
//

#[test]
fn test_public_group_chat() {
    let chat = MockGroupChat::new().title("Test").id(-1234);
    let chat_photo = MockChatPhoto::new().build();
    let chat_full_info = MockChatFullInfoGroup::new()
        .title("Test2")
        .id(-1234)
        .photo(chat_photo.clone());

    let chat_object = chat.build();
    assert_eq!(chat_object.title(), Some("Test"));
    assert_eq!(chat_object.id, ChatId(-1234));

    let chat_full_info_object = chat_full_info.build();
    assert_eq!(chat_full_info_object.title(), Some("Test2"));
    assert_eq!(chat_full_info_object.id, ChatId(-1234));
    assert_eq!(chat_full_info_object.photo, Some(chat_photo));
}

#[test]
fn test_supergroup_chat() {
    let chat = MockSupergroupChat::new().id(-1234);
    let chat_full_info = MockChatFullInfoSupergroup::new()
        .join_by_request(true)
        .id(-1234);

    let chat_object = chat.build();
    assert_eq!(chat_object.id, ChatId(-1234));

    let chat_full_info_object = chat_full_info.build();
    assert_eq!(chat_full_info_object.id, ChatId(-1234));
    assert_eq!(chat_full_info_object.join_by_request(), true);
}

#[test]
fn test_channel_chat() {
    let chat = MockChannelChat::new().username("test_channel").id(-1234);
    let chat_full_info = MockChatFullInfoChannel::new()
        .username("test_channel")
        .linked_chat_id(-12345)
        .id(-1234);

    let chat_object = chat.build();
    assert_eq!(chat_object.id, ChatId(-1234));
    assert_eq!(chat_object.username(), Some("test_channel"));

    let chat_full_info_object = chat_full_info.build();
    assert_eq!(chat_full_info_object.id, ChatId(-1234));
    assert_eq!(chat_full_info_object.username(), Some("test_channel"));
    assert_eq!(chat_full_info_object.linked_chat_id(), Some(-12345));
}

#[test]
fn test_private_group_chat() {
    let chat = MockPrivateChat::new().first_name("Test").id(1234);

    let chat_full_info = MockChatFullInfoPrivate::new()
        .first_name("Test")
        .id(1234)
        .bio("Test bio");

    let chat_object = chat.build();
    assert_eq!(chat_object.first_name(), Some("Test"));
    assert_eq!(chat_object.id, ChatId(1234));

    let chat_full_info_object = chat_full_info.build();
    assert_eq!(chat_full_info_object.first_name(), Some("Test"));
    assert_eq!(chat_full_info_object.id, ChatId(1234));
    assert_eq!(chat_full_info_object.bio(), Some("Test bio"));
}

//
//
//

#[test]
fn test_message_common_text() {
    let simple_message = MockMessageText::new().text("simple");
    let simple_message_object = simple_message.build(); // This is now teloxide::types::Message

    assert_eq!(simple_message_object.text(), Some("simple"));
    assert_eq!(
        simple_message_object.from.unwrap().first_name,
        MockUser::FIRST_NAME
    );
    assert_eq!(simple_message_object.chat.id, ChatId(MockUser::ID as i64)); // Some sane default values
                                                                            // User id because it is a private chat

    let message = MockMessageText::new()
        .text("text")
        .id(123) // If you want - you can change everything by just calling it as a method
        .from(MockUser::new().first_name("Test").build()) // Sub categories need to be built in separately
        .chat(MockGroupChat::new().id(-123).build())
        .is_automatic_forward(true)
        .entities(vec![MessageEntity::bold(0, 3)]);

    let message_object = message.build();

    assert_eq!(message_object.text(), Some("text"));
    assert_eq!(message_object.id, MessageId(123));
    assert_eq!(message_object.from.clone().unwrap().first_name, "Test");
    assert_eq!(message_object.chat.id, ChatId(-123));
    assert_eq!(message_object.is_automatic_forward(), true);
    assert_eq!(
        message_object.entities(),
        Some(vec![MessageEntity::bold(0, 3)]).as_deref()
    );
}

#[test]
fn test_into_update() {
    let message = MockMessageText::new().text("text");

    let update = message.into_update(&AtomicI32::new(42))[0].clone();

    assert_eq!(update.id, UpdateId(42));
    assert_eq!(update.chat_id(), Some(ChatId(MockUser::ID as i64)));
}

#[test]
fn test_message_common_animation() {
    let message = MockMessageAnimation::new()
        .caption("caption")
        .caption_entities(vec![MessageEntity::bold(0, 3)]);

    let message_object = message.build();
    assert_eq!(message_object.caption(), Some("caption"));
    assert_eq!(
        message_object.caption_entities(),
        Some(vec![MessageEntity::bold(0, 3)]).as_deref()
    );
}

#[test]
fn test_message_common_audio() {
    let message = MockMessageAudio::new()
        .caption("caption")
        .caption_entities(vec![MessageEntity::bold(0, 3)])
        .media_group_id("123");

    let message_object = message.build();
    assert_eq!(message_object.caption(), Some("caption"));
    assert_eq!(
        message_object.caption_entities(),
        Some(vec![MessageEntity::bold(0, 3)]).as_deref()
    );
    assert_eq!(
        message_object.media_group_id(),
        Some(&MediaGroupId("123".to_string()))
    );
}

#[test]
fn test_message_common_contact() {
    let message = MockMessageContact::new()
        .last_name("last_name")
        .vcard("vcard");

    let message_object = message.build();
    assert_eq!(
        message_object.contact().unwrap().phone_number,
        MockMessageContact::PHONE_NUMBER
    );
    assert_eq!(
        message_object.contact().unwrap().first_name,
        MockUser::FIRST_NAME
    );
    assert_eq!(
        message_object.contact().unwrap().last_name,
        Some("last_name".to_string())
    );
    assert_eq!(
        message_object.contact().unwrap().vcard,
        Some("vcard".to_string())
    );
}

#[test]
fn test_message_common_document() {
    let message = MockMessageDocument::new()
        .caption("caption")
        .caption_entities(vec![MessageEntity::bold(0, 3)]);

    let message_object = message.build();
    assert_eq!(message_object.caption(), Some("caption"));
    assert_eq!(
        message_object.caption_entities(),
        Some(vec![MessageEntity::bold(0, 3)]).as_deref()
    );
}

#[test]
fn test_message_common_game() {
    let message = MockMessageGame::new();

    let message_object = message.build();
    assert_eq!(message_object.game().unwrap().title, MockMessageGame::TITLE);
    assert_eq!(
        message_object.game().unwrap().description,
        MockMessageGame::DESCRIPTION
    );
}

#[test]
fn test_message_common_venue() {
    let message = MockMessageVenue::new();

    let message_object = message.build();
    assert_eq!(
        message_object.venue().unwrap().title,
        MockMessageVenue::TITLE
    );
    assert_eq!(
        message_object.venue().unwrap().address,
        MockMessageVenue::ADDRESS
    );
}

#[test]
fn test_message_common_location() {
    let message = MockMessageLocation::new();

    let message_object = message.build();
    assert_eq!(
        message_object.location().unwrap().latitude,
        MockMessageLocation::LATITUDE
    );
    assert_eq!(
        message_object.location().unwrap().longitude,
        MockMessageLocation::LONGITUDE
    );
}

#[test]
fn test_message_common_photo() {
    let message = MockMessagePhoto::new();

    let message_object = message.build();
    assert_eq!(
        message_object.photo().unwrap()[0].width,
        MockPhotoSize::WIDTH
    );
    assert_eq!(
        message_object.photo().unwrap()[0].height,
        MockPhotoSize::HEIGHT
    );
}

#[test]
fn test_message_common_poll() {
    let message = MockMessagePoll::new();

    let message_object = message.build();
    assert_eq!(
        message_object.poll().unwrap().question,
        MockMessagePoll::QUESTION
    );
    assert_eq!(
        message_object.poll().unwrap().poll_type,
        MockMessagePoll::POLL_TYPE
    );
}

#[test]
fn test_message_common_sticker() {
    let message = MockMessageSticker::new();

    let message_object = message.build();
    assert_eq!(
        message_object.sticker().unwrap().file.id,
        FileId(MockMessageSticker::FILE_ID.to_string())
    );
}

#[test]
fn test_message_common_video() {
    let message = MockMessageVideo::new();

    let message_object = message.build();
    assert_eq!(message_object.video().unwrap().width, MockVideo::WIDTH);
    assert_eq!(message_object.video().unwrap().height, MockVideo::HEIGHT);
}

#[test]
fn test_message_common_video_note() {
    let message = MockMessageVideoNote::new();

    let message_object = message.build();
    assert_eq!(
        message_object.video_note().unwrap().duration,
        MockMessageVideoNote::DURATION
    );
    assert_eq!(
        message_object.video_note().unwrap().length,
        MockMessageVideoNote::LENGTH
    );
}

#[test]
fn test_message_common_voice() {
    let message = MockMessageVoice::new();

    let message_object = message.build();
    assert_eq!(
        message_object.voice().unwrap().duration,
        MockMessageVoice::DURATION
    );
}

#[test]
fn test_message_common_migration_to_chat() {
    let message = MockMessageMigrationToChat::new();

    let message_object = message.build();
    assert_eq!(
        message_object.migrate_to_chat_id().unwrap(),
        &ChatId(MockMessageMigrationToChat::MIGRATE_TO_CHAT_ID)
    );
}

#[test]
fn test_message_common_migration_from_chat() {
    let message = MockMessageMigrationFromChat::new();

    let message_object = message.build();
    assert_eq!(
        message_object.migrate_from_chat_id().unwrap(),
        &ChatId(MockMessageMigrationFromChat::MIGRATE_FROM_CHAT_ID)
    );
}

//
//
//

#[test]
fn test_callback_query() {
    let query = MockCallbackQuery::new();
    let query_object = query.build();
    assert_eq!(
        query_object.id,
        CallbackQueryId(MockCallbackQuery::ID.into())
    );
    assert_eq!(query_object.from.first_name, MockUser::FIRST_NAME);
}

//
//
//

#[test]
fn test_update_poll() {
    let update = MockUpdatePoll::new().poll_id("123");

    let update_object = update.into_update(&AtomicI32::new(1))[0].clone();

    if let UpdateKind::Poll(poll) = update_object.kind {
        assert_eq!(poll.question, MockMessagePoll::QUESTION);
        assert_eq!(poll.poll_type, MockMessagePoll::POLL_TYPE);
        assert_eq!(poll.id, PollId("123".to_owned()));
    } else {
        unreachable!()
    }
}
