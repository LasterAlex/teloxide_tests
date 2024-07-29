<div align="center">
  <img src="https://github.com/user-attachments/assets/627beca8-5852-4c70-97e0-5f4fcb5e2040" width="250"/>
  <h1><code>teloxide tests</code></h1>
  <a href="https://docs.rs/teloxide_tests/">
    <img src="https://docs.rs/teloxide_tests/badge.svg">
  </a>
  <a href="https://crates.io/crates/teloxide_tests">
    <img src="https://img.shields.io/crates/v/teloxide_tests.svg">
  </a>
  <a href="https://t.me/teloxide_tests">
    <img src="https://img.shields.io/badge/support-t.me%2Fteloxide__tests-blueviolet">
  </a>

  A crate that allows you to unit test your teloxide bots easily! No internet, accounts or anything required!
</div>


[[`file_download_bot`]](https://github.com/LasterAlex/teloxide_tests/blob/master/examples/file_download_bot/src/main.rs)
```rust,ignore
#[cfg(test)]
mod tests {
    use super::*;
    use teloxide_tests::{MockBot, MockMessageDocument, MockMessageText};

    #[tokio::test]
    async fn test_not_a_document() {
        let bot = MockBot::new(MockMessageText::new().text("Hi!"), handler_tree());
        bot.dispatch_and_check_last_text("Not a document").await;
    }

    #[tokio::test]
    async fn test_download_document_and_check() {
        let bot = MockBot::new(MockMessageDocument::new(), handler_tree());
        bot.dispatch_and_check_last_text("Downloaded!").await;
    }
}
```

[[`calculator_bot`]](https://github.com/LasterAlex/teloxide_tests/blob/master/examples/calculator_bot/src/tests.rs)
```rust,ignore
#[tokio::test]
async fn test_what_is_the_first_number() {
    let bot = MockBot::new(MockCallbackQuery::new().data("add"), handler_tree());

    bot.dependencies(deps![get_bot_storage().await]);
    bot.set_state(State::WhatDoYouWant).await;

    bot.dispatch_and_check_last_text_and_state(
        text::ENTER_THE_FIRST_NUMBER,
        State::GetFirstNumber {
            operation: "add".to_owned(),
        },
    )
    .await;
}
```

You can see more useful examples at [examples/](https://github.com/LasterAlex/teloxide_tests/tree/master/examples) and the docs at [docs.rs](https://docs.rs/teloxide_tests)

## How to implement it?

Hopefully it is as easy as doing what happens in `./examples`

1. Import the `teloxide_tests`
2. Create a mocked bot with something that can be turned into an update, like MockMessageText or MockMessagePhoto
3. Add dependencies and/or a different bot using .dependencies(deps![]) and .me(MockedMe::new().build())
4. Dispatch it with .dispatch().await
5. Get the responces with .get_responces()
6. Do the testing with the gotten responces

**Do NOT** use raw MockBot fields like bot.updates or bot.me to mutate the bot, unless you know what you are doing. Use given abstractions, and if some feature is missing, you can mention it in the github repo (or contact me via telegram [@laster_alex](https://t.me/laster_alex))

## What this crate has

- Easy testing of handlers with access to raw bot requests (see [hello_world_bot](https://github.com/LasterAlex/teloxide_tests/blob/master/examples/hello_world_bot/src/main.rs))
- Support of dependencies, changes of `me` and multiple updates (see [album_bot](https://github.com/LasterAlex/teloxide_tests/blob/master/examples/album_bot/src/main.rs))
- Syntactic sugar and native support for storage, dialogue and states (see [calculator_bot](https://github.com/LasterAlex/teloxide_tests/blob/master/examples/calculator_bot/src/tests.rs))
- Fake file getting and downloading (see [file_download_bot](https://github.com/LasterAlex/teloxide_tests/blob/master/examples/file_download_bot/src/main.rs))
- Ability to be used with databases (see [phrase_bot](https://github.com/LasterAlex/teloxide_tests/blob/master/examples/phrase_bot/src/main.rs))

## Pitfalls

1. Race conditions. They are, to my knoledge, the most difficult.

2. And also when you use a method that is still not supported by this crate. Please reffer to the docs to see, what endpoints are implemented in the latest release (or look at [server/routes](https://github.com/LasterAlex/teloxide_tests/tree/master/teloxide_tests/src/server/routes) files to look at the current endpoints)

3. Maybe also the fact that the fake server actually checks the messages and files that are present, and it starts with a clean state. You can't just send a file by file_id or forward a message by an arbitrary message_id that was sent long ago, the bot wouldn't know what to do with it, so you need to separately add it by dispatching the bot with that update, so that it gets added as the user message to memory (you can change file_id and message_id in the mocked structs to anything you need).

### Some errors associated with these race conditions:

- trait `Send` is not implemented for `std::sync::MutexGuard<'static, ()>`

  This means you can't share the bot between any threads, as you should not in any circumstance.

- PoisonError(...)

  You shouldn't see it, i tried to mitigate it, but if you do, it's not the problem, it just means that something else panicked and now the bot doesn't know, what to do. Just fix whatever was causing the problem, and poison errors should be gone.

- Stupid bugs that change every time you run a test

  You can use the crate [serial_test](https://crates.io/crates/serial_test), or try to add `drop(bot);` at the end of every test, and do everything AFTER calling `MockBot::new()`, as the bot creation makes a safe lock that prevent any race conditions.

## Contributing

Please see [CONTRIBUTING.md](https://github.com/LasterAlex/teloxide_tests/blob/master/CONTRIBUTING.md)

## Todo

- [x] Add dataset
  - [x] Add dataset of chats
  - [x] Add dataset of common messages
  - [ ] Add dataset of queries (low priority)
  - [ ] Add dataset of messages (low priority)
  - [ ] Add structs without a category (low priority)
- [x] Add fake server
  - [x] Add most common endpoints
  - [ ] Add all common messages (low priority)
  - [ ] Add inline queries (low priority)
  - [ ] Add all queries (low priority)
  - [ ] Add all messages (super low priority)
  - [ ] Add everything else (may never be done)
- [x] Make mocked bot that sends requests to fake server
- [x] Add tests to that bot
- [x] Make it into a library
- [x] Publish it when it is ready

## Special thanks to

The teloxide team! They made an absolutely incredible library with amazing internal documentation, which helped me a lot during development! It is an amazing project, and i'm happy i'm able to add to it something useful!
