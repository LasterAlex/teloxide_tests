# Teloxide tests

I am trying to make tests for teloxide by bot mocking.

The mocking shouldn't be very difficult. Complex - yes, but it isn't what was holding back that issue. The problem is an easy way to make test objects and check the request data. That's why im prioritizing it now.

## Todo

- [ ] Add dataset of all needed structs
    - [x] Add dataset of chats
    - [ ] Add dataset of messages
    - [ ] Add dataset of media kinds
    - [ ] Add structs without a category
- [ ] Try to think of a good way to compare the behaviour of the bot
- [ ] Add the bot mocking
- [ ] Make it into a library
