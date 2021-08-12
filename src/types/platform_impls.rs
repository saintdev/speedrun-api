use std::fmt::Display;

use crate::api::platforms::PlatformId;

use super::Platform;

impl<'a> From<Platform<'a>> for PlatformId<'a> {
    fn from(value: Platform<'a>) -> Self {
        value.id
    }
}

impl Display for Platform<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
