# Teloxide tests

I am trying to make tests for teloxide using fake server and a bot with an url to that server.

Server is done, now only finshing touches will be needed.

I am going right now by the following steps:

1. Make it work
2. Make it beautiful for the user
3. Make it beautiful on the inside

What is needed until release:

- [x] Add most common endpoints (SendMessage, SendPhoto, SendDocument, SendVideo, EditMessageText, EditMessageCaption, EditMessageReplyMarkup, DeleteMessage, AnswerCallbackQuery, GetFile)
- [x] Clean up the test server code and make it easily extendable
- [x] Add some syntactic sugar for testing (e.g .dispatch_and_check_last_sent_text(), .dispatch_and_check_state(), etc.)
- [x] Export publicly only what is needed
- [x] Add a lot of different examples for referencing
- [x] Try to make a real bot with these tests, to see, where it lacks in the real usecases
- [ ] Some feedback for a sanity check

The main crate is mock_bot, everything is split up for simplicity.

## Pitfalls

Race conditions. They are, to my knoledge, the most difficult.
To avoid it, try to do the following:

- Make a new bot for every test. Reusing the bot is ok, just try to not make one bot to rule them all.
- If you REALLY want to make the one bot work, call bot.update(/_ some update _/); BEFORE any change to the database, state or anything else that can be shared accross tests. .update() locks the bot, and has to wait before this update gets dispatched or goes out of scope to change their own update
- If you are tired, look at [this crate for serial testing](https://crates.io/crates/serial_test), this basically eliminates all the pain, but it doesn't look as nice

### Some errors associated with these race conditions:

- trait `Send` is not implemented for `std::sync::MutexGuard<'static, ()>`

  This means you can't share the bot between any threads, as you should not in any circumstance.

- PoisonError(...)

  You shouldn't see it, i tried to mitigate it, but if you do, it's not the problem, it just means that something else panicked and now the bot doesn't know, what to do. Just fix whatever was causing the problem, and poison errors should be gone.

- Stupid bugs that change every time you run a test

  Once again, either use serial tests, or try to add `drop(bot);` at the end of every test, and do everything AFTER calling `MockBot::new()`, as the bot creation makes a safe lock that prevent any race conditions.

## Structure

- ./dataset has different mocked structs, that are easy to implement and use
- ./proc_macros has proc macros, cuz for some reason it has to be a separate crate
- ./telegram_test_server has a server that mimicks the real one
- ./mock_bot has a mocked version of a bot, that sends requests to the fake server. It is also the main crate.

## How to implement it?

Hopefully it is as easy as doing what happens in `./examples`

1. Import the `teloxide_tests`
2. Create a mocked bot with something that can be turned into an update, like MockMessageText or MockMessagePhoto
3. Add dependencies and/or a different bot using .dependencies(deps![]) and .me(MockedMe::new().build())
4. Dispatch it with .dispatch().await
5. Get the responces with .get_responces()
6. Do the testing with the gotten responces

**Do NOT** use raw MockBot fields like bot.updates or bot.me to mutate the bot, unless you know what you are doing. Use given abstractions, and if some feature is missing, you can mention it in the github repo (or contact me via telegram [@laster_alex](https://t.me/laster_alex))

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
