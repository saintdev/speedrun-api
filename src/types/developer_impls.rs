use crate::api::developers::DeveloperId;

use super::Developer;

impl<'a> From<Developer<'a>> for DeveloperId<'a> {
    fn from(value: Developer<'a>) -> Self {
        value.id
    }
}
