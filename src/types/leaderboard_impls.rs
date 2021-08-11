use crate::api::{categories::CategoryId, games::GameId};

use super::Leaderboard;

impl<'a> From<Leaderboard<'a>> for CategoryId<'a> {
    fn from(value: Leaderboard<'a>) -> Self {
        value.category
    }
}

impl<'a> From<Leaderboard<'a>> for GameId<'a> {
    fn from(value: Leaderboard<'a>) -> Self {
        value.game
    }
}
