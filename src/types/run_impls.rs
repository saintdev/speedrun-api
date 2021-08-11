use crate::api::{categories::CategoryId, games::GameId, runs::RunId};

use super::Run;

impl<'a> From<Run<'a>> for RunId<'a> {
    fn from(value: Run<'a>) -> Self {
        value.id
    }
}

impl<'a> From<Run<'a>> for GameId<'a> {
    fn from(value: Run<'a>) -> Self {
        value.game
    }
}

impl<'a> From<Run<'a>> for CategoryId<'a> {
    fn from(value: Run<'a>) -> Self {
        value.category
    }
}
