use crate::api::regions::RegionId;

use super::Region;

impl<'a> From<Region<'a>> for RegionId<'a> {
    fn from(value: Region<'a>) -> Self {
        value.id
    }
}
