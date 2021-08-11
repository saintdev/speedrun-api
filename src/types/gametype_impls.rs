use crate::api::gametypes::GameTypeId;

use super::GameType;

impl<'a> From<GameType<'a>> for GameTypeId<'a> {
    fn from(value: GameType<'a>) -> Self {
        value.id
    }
}
