use crate::api::levels::LevelId;

use super::Level;

impl<'a> From<Level<'a>> for LevelId<'a> {
    fn from(value: Level<'a>) -> Self {
        value.id
    }
}
