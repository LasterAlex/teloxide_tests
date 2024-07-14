# Teloxide tests examples
To run them, firstly create a .env file like .example.env

### hello_world

A simple bot that sends Hello World! and introduces to the bot testing.

### calculator_bot

A little harder bot that shows the sintactic sugar for testing, and how to work with persistent state storage (using redis, but changing to InMemStorage is possible and easy).

### file_download_bot

Bot that shows how to download files from the server.

### album_bot

Bot that tests the album sending.

### phrase_bot

The biggest bot, a bot that adds reactions, similar to some other chat bots. Not particularly made to show some features, more like battle testing the crate and showing, how i will use this crate.

The tests are in the same files as handlers.

To run it you need to set up diesel for database.

1. You need to install and start postgres on your machine, here is [ubuntu install](https://www.digitalocean.com/community/tutorials/how-to-install-postgresql-on-ubuntu-20-04-quickstart)
2. `cargo install diesel_cli --no-default-features --features postgres`
3. Add `~/.cargo/bin` to `PATH` (or just run ~/.cargo/bin/diesel by itself)
4. `diesel setup --database-url postgres://postgres:mvcx90XKxasd@localhost/phrase_bot` in the phrase_bot directory
5. `cargo run` or `cargo test`!
