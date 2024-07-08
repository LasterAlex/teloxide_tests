use proc_macros::Changeable;
use teloxide::types::{ChatId, UserId};

use super::{TestPublicGroupChat, TestUser};

#[derive(Changeable)]
struct Test {
    field1: String,
    field2: ChatId,
    field3: Option<String>,
    field4: Option<i32>,
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

#[test]
fn test_user() {
    let user = TestUser::new()
        .first_name("Test")
        .last_name("User")
        .id(1234)
        .username("test_user");

    let user_object = user.to_object();
    assert_eq!(user_object.first_name, "Test");
    assert_eq!(user_object.last_name, Some("User".to_string()));
    assert_eq!(user_object.id, UserId(1234));
    assert_eq!(user_object.username, Some("test_user".to_string()));
}

#[test]
fn test_public_group_chat() {
    let chat = TestPublicGroupChat::new().title("Test").id(1234);

    let chat_object = chat.to_object();
    assert_eq!(chat_object.title(), Some("Test"));
    assert_eq!(chat_object.id, ChatId(1234));
}

#[test]
fn test_private_group_chat() {
    let chat = TestPublicGroupChat::new()
        .title("Test")
        .id(1234)
        .description("Test description");

    let chat_object = chat.to_object();
    assert_eq!(chat_object.title(), Some("Test"));
    assert_eq!(chat_object.id, ChatId(1234));
    assert_eq!(chat_object.description(), Some("Test description"));
}
