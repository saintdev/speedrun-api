use std::fmt::Display;

use crate::api::series::SeriesId;

use super::Series;

impl<'a> From<Series<'a>> for SeriesId<'a> {
    fn from(value: Series<'a>) -> Self {
        value.id
    }
}

impl Display for Series<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.names.international)
    }
}
