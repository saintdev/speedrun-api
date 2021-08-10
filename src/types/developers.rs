use serde::Deserialize;

use crate::api::developers::DeveloperId;

use super::Link;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Developer<'a> {
    pub id: DeveloperId<'a>,
    pub name: String,
    pub links: Vec<Link>,
}
