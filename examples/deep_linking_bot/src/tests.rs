use crate::{add_deep_link, handler_tree::handler_tree, text, State};
use teloxide::{dispatching::dialogue::InMemStorage, dptree::deps};
use teloxide_tests::{MockBot, MockMessagePhoto, MockMessageText};

#[tokio::test]
async fn test_start() {
    // Just a regular start
    let mock_message = MockMessageText::new().text("/start");
    let bot = MockBot::new(mock_message.clone(), handler_tree());

    bot.dependencies(deps![InMemStorage::<State>::new()]);
    let me = bot.me.lock().unwrap().clone(); // Yeah, we can access the default 'me' like that

    bot.dispatch_and_check_last_text_and_state(
        &add_deep_link(text::START, me, mock_message.chat.id),
        State::Start,
    )
    .await;
}

#[tokio::test]
async fn test_with_deep_link() {
    // Because https://t.me/some_bot?start=987654321 is the same as sending "/start 987654321", 
    // we can simulate it with this
    let mock_message = MockMessageText::new().text("/start 987654321");
    let bot = MockBot::new(mock_message, handler_tree());

    bot.dependencies(deps![InMemStorage::<State>::new()]);

    bot.dispatch_and_check_last_text_and_state(
        text::SEND_YOUR_MESSAGE,
        State::WriteToSomeone { id: 987654321 },
    )
    .await;
}

#[tokio::test]
async fn test_send_message() {
    // The text we want to send to a 987654321 user
    let mock_message = MockMessageText::new().text("I love you!");
    let bot = MockBot::new(mock_message.clone(), handler_tree());

    let me = bot.me.lock().unwrap().clone();
    bot.dependencies(deps![InMemStorage::<State>::new()]);
    bot.set_state(State::WriteToSomeone { id: 987654321 }).await;

    // Just checking that the state returned to normal
    bot.dispatch_and_check_state(State::Start).await;

    let responses = bot.get_responses();

    // This is the message that was sent to 987654321. It is always first
    let sent_message = responses.sent_messages[0].clone();
    // And this is the message that was sent to the default user
    let response_message = responses.sent_messages[1].clone();

    assert_eq!(
        sent_message.text().unwrap(),
        text::YOU_HAVE_A_NEW_MESSAGE.replace("{message}", "I love you!")
    );  // Just checking that the text and sender are correct
    assert_eq!(sent_message.chat.id.0, 987654321);

    assert_eq!(
        response_message.text().unwrap(),
        add_deep_link(text::MESSAGE_SENT, me, mock_message.chat.id)
    );
    assert_eq!(response_message.chat.id, mock_message.chat.id);
}

#[tokio::test]
async fn test_wrong_link() {
    let mock_message = MockMessageText::new().text("/start not_id");
    let bot = MockBot::new(mock_message, handler_tree());
    bot.dependencies(deps![InMemStorage::<State>::new()]);

    bot.dispatch_and_check_last_text(text::WRONG_LINK).await;
}

#[tokio::test]
async fn test_not_a_text() {
    let mock_message = MockMessagePhoto::new();
    let bot = MockBot::new(mock_message, handler_tree());
    bot.dependencies(deps![InMemStorage::<State>::new()]);

    bot.set_state(State::WriteToSomeone { id: 987654321 }).await;

    bot.dispatch_and_check_last_text(text::SEND_TEXT).await;
}
