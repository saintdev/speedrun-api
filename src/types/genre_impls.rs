use crate::api::genres::GenreId;

use super::Genre;

impl<'a> From<Genre<'a>> for GenreId<'a> {
    fn from(value: Genre<'a>) -> Self {
        value.id
    }
}
