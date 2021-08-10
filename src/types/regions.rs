use serde::Deserialize;

use crate::api::regions::RegionId;

use super::Link;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Region<'a> {
    pub id: RegionId<'a>,
    pub name: String,
    pub links: Vec<Link>,
}
