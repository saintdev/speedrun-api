use std::fmt::Display;

use crate::api::developers::DeveloperId;

use super::Developer;

impl<'a> From<Developer<'a>> for DeveloperId<'a> {
    fn from(value: Developer<'a>) -> Self {
        value.id
    }
}

impl Display for Developer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
