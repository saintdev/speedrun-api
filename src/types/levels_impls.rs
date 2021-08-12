use std::fmt::Display;

use crate::api::levels::LevelId;

use super::Level;

impl<'a> From<Level<'a>> for LevelId<'a> {
    fn from(value: Level<'a>) -> Self {
        value.id
    }
}

impl Display for Level<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
