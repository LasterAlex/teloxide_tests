# Teloxide tests

A crate that allows you to unit test your teloxide bots easily! No internet, accounts or anything required!

You can see the examples at [examples/](https://github.com/LasterAlex/teloxide_tests/tree/master/examples), while this crate isn't out, you can see the docs by going to `teloxide_tests/` and running `cargo doc --no-deps --open`

What is needed until release:

- [x] Add most common endpoints (SendMessage, SendPhoto, SendDocument, SendVideo, EditMessageText, EditMessageCaption, EditMessageReplyMarkup, DeleteMessage, AnswerCallbackQuery, GetFile)
- [x] Clean up the test server code and make it easily extendable
- [x] Add some syntactic sugar for testing (e.g .dispatch_and_check_last_sent_text(), .dispatch_and_check_state(), etc.)
- [x] Export publicly only what is needed
- [x] Add a lot of different examples for referencing
- [x] Try to make a real bot with these tests, to see, where it lacks in the real usecases
- [ ] Some feedback for a sanity check

## Pitfalls

Race conditions. They are, to my knoledge, the most difficult.

### Some errors associated with these race conditions:

- trait `Send` is not implemented for `std::sync::MutexGuard<'static, ()>`

  This means you can't share the bot between any threads, as you should not in any circumstance.

- PoisonError(...)

  You shouldn't see it, i tried to mitigate it, but if you do, it's not the problem, it just means that something else panicked and now the bot doesn't know, what to do. Just fix whatever was causing the problem, and poison errors should be gone.

- Stupid bugs that change every time you run a test

  You can use the crate [serial_test](https://crates.io/crates/serial_test), or try to add `drop(bot);` at the end of every test, and do everything AFTER calling `MockBot::new()`, as the bot creation makes a safe lock that prevent any race conditions.

## How to implement it?

Hopefully it is as easy as doing what happens in `./examples`

1. Import the `teloxide_tests`
2. Create a mocked bot with something that can be turned into an update, like MockMessageText or MockMessagePhoto
3. Add dependencies and/or a different bot using .dependencies(deps![]) and .me(MockedMe::new().build())
4. Dispatch it with .dispatch().await
5. Get the responces with .get_responces()
6. Do the testing with the gotten responces

**Do NOT** use raw MockBot fields like bot.updates or bot.me to mutate the bot, unless you know what you are doing. Use given abstractions, and if some feature is missing, you can mention it in the github repo (or contact me via telegram [@laster_alex](https://t.me/laster_alex))

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
- [ ] Publish it when it is ready
