use dptree::case;
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateFilterExt, UpdateHandler},
    prelude::*,
};

use crate::{handlers::*, StartCommand, State};

pub fn handler_tree() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(
            Update::filter_message()
                .filter_command::<StartCommand>()
                .branch(case![StartCommand::Start(start)].endpoint(start)),
        )
        .branch(
            Update::filter_message()
                .branch(case![State::WriteToSomeone { id }].endpoint(send_message)),
        )
}
