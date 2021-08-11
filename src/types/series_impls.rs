use crate::api::series::SeriesId;

use super::Series;

impl<'a> From<Series<'a>> for SeriesId<'a> {
    fn from(value: Series<'a>) -> Self {
        value.id
    }
}
