use crate::handlers::*;
use dptree::case;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateFilterExt, UpdateHandler};
use teloxide::prelude::*;

use crate::{StartCommand, State};

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
