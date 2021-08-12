use std::fmt::Display;

use crate::api::categories::CategoryId;

use super::Category;

impl<'a> From<Category<'a>> for CategoryId<'a> {
    fn from(value: Category<'a>) -> Self {
        value.id
    }
}

impl Display for Category<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
