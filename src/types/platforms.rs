use serde::Deserialize;

use crate::api::platforms::PlatformId;

use super::Link;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Platform<'a> {
    pub id: PlatformId<'a>,
    pub name: String,
    pub released: i64,
    pub links: Vec<Link>,
}
