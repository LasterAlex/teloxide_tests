# Contributing

First of all, thanks! I really want to make this crate as good as possible, as i think that testing the bot is very important and helps a lot with debugging, and if you share my thoughts, and want to contribute, im very thankful!

### Here are the rules for writing code i consider important for PRs:

1. Add code comments and docstrings. I take an example from teloxide source, and they have documented everything in their code very nicely.
2. Add tests for the source code. Not for every single function, but if you add a dataset item, or a new endpoint, add a test for it. (endpoint tests are in the teloxide_tests/src/tests.rs)
3. The teloxide bot testing for the users of this crate should be very intuitive and easy. That is the reason i made so that the tests can be run without serial_test crate, it adds unnecessary boilerplate.
4. The bot should handle the test failiure with grace. After all, the tests are made to fail, so the error messages and panics should be clear. Mutex poison errors because of a one failed test are not good at all, as well as server errors. If one test fails, no others should.
5. Users have to have many options for testing their bot. For that very reason i save bot requests to the fake server, as well as making some fields in the MockBot public.
6. Write the code that is similar to the one that already exists. Not identical, but similar. Also, i hate boilerplate, as you could've seen by proc macros and regular macros. If you know, how to avoid boilerplate, please do.
7. Be VERY careful when modifying the existing MockBot code. I am very very sorry if you come across stupid race condition bugs, they have caused way too much pain, and i do not want the same happening to you.

These aren't super strict rules (unless you are modifying MockBot code), and you can step away a little from them, just write good working code!

### And here are the rules for issues:

Just write a clear description of what is the problem. A bug, a reasonable feature request, a documentation issue, etc. are all valid problems.

# What to contribute

The main two things that need to be done: add all of dataset items and add all of the endpoints. It doesn't matter, what and in what order, just write what you consider important. Or you can just look at the TODO field in the README.md

You can also try cleaning up the code, writing more tests, adding more comments, more examples, more syntactic sugar, whatever your heart desires and will be useful for that crate!

Thanks once again, i am very grateful for any contributions that improve this crate!
