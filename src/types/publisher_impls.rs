use crate::api::publishers::PublisherId;

use super::Publisher;

impl<'a> From<Publisher<'a>> for PublisherId<'a> {
    fn from(value: Publisher<'a>) -> Self {
        value.id
    }
}
