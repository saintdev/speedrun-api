use crate::api::platforms::PlatformId;

use super::Platform;

impl<'a> From<Platform<'a>> for PlatformId<'a> {
    fn from(value: Platform<'a>) -> Self {
        value.id
    }
}
