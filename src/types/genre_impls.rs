use std::fmt::Display;

use crate::api::genres::GenreId;

use super::Genre;

impl<'a> From<Genre<'a>> for GenreId<'a> {
    fn from(value: Genre<'a>) -> Self {
        value.id
    }
}

impl Display for Genre<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
