use crate::api::categories::CategoryId;

use super::Category;

impl<'a> From<Category<'a>> for CategoryId<'a> {
    fn from(value: Category<'a>) -> Self {
        value.id
    }
}
