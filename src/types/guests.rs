use crate::types::Link;
use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guest {
    pub name: String,
    pub links: Vec<Link>,
}
