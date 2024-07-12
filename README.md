# Teloxide tests

I am trying to make tests for teloxide using fake server and a bot with an url to that server.

Server is done, now only finshing touches will be needed.

I am going right now by the following steps:

1) Make it work
2) Make it beautiful for the user
3) Make it beautiful on the inside

What is needed until release:

- [x] Add most common endpoints (SendMessage, SendPhoto, SendDocument, SendVideo, EditMessageText, EditMessageCaption, EditMessageReplyMarkup, DeleteMessage, AnswerCallbackQuery, GetFile)
- [x] Clean up the test server code and make it easily extendable
- [x] Add some syntactic sugar for testing (e.g .dispatch_and_check_last_sent_text(), .dispatch_and_check_state(), etc.)
- [ ] Export publicly only what is needed
- [ ] Add a lot of different examples for referencing
- [ ] Try to make a real bot with these tests, to see, where it lacks in the real usecases
- [ ] Some feedback for a sanity check

## Structure

- ./dataset has different mocked structs, that are easy to implement and use
- ./proc_macros has proc macros, cuz for some reason it has to be a separate crate
- ./telegram_test_server has a server that mimicks the real one
- ./mock_bot has a mocked version of a bot, that sends requests to the fake server

## Where are the examples of a mocked bot?

`./mock_bot/src/tests.rs`

## How to implement it?

Hopefully it is as easy as doing what happens in `./mock_bot/src/tests.rs`

1) Import the dataset
2) Create a mocked bot with something that can be turned into an update, like MockMessageText or MockMessagePhoto
3) Add dependencies and/or a different bot using .dependencies(deps![]) and .me(MockedMe::new().build())
4) Dispatch it with .dispatch().await
5) Get the responces with .get_responces()
6) Do the testing with the gotten responces

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
