use crate::api::games::GameId;

use super::Game;

impl<'a> From<Game<'a>> for GameId<'a> {
    fn from(value: Game<'a>) -> Self {
        value.id
    }
}
