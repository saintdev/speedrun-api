use std::fmt::Display;

use crate::api::gametypes::GameTypeId;

use super::GameType;

impl<'a> From<GameType<'a>> for GameTypeId<'a> {
    fn from(value: GameType<'a>) -> Self {
        value.id
    }
}

impl Display for GameType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
