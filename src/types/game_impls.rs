use std::fmt::Display;

use crate::api::games::GameId;

use super::Game;

impl<'a> From<Game<'a>> for GameId<'a> {
    fn from(value: Game<'a>) -> Self {
        value.id
    }
}

impl Display for Game<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.names.international)
    }
}
