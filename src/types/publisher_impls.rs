use std::fmt::Display;

use crate::api::publishers::PublisherId;

use super::Publisher;

impl<'a> From<Publisher<'a>> for PublisherId<'a> {
    fn from(value: Publisher<'a>) -> Self {
        value.id
    }
}

impl Display for Publisher<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}