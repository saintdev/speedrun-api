use serde::Deserialize;

use crate::api::publishers::PublisherId;

use super::Link;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Publisher<'a> {
    pub id: PublisherId<'a>,
    pub name: String,
    pub links: Vec<Link>,
}
