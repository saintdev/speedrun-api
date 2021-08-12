use std::fmt::Display;

use crate::api::regions::RegionId;

use super::Region;

impl<'a> From<Region<'a>> for RegionId<'a> {
    fn from(value: Region<'a>) -> Self {
        value.id
    }
}

impl Display for Region<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
