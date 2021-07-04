use crate::types::Link;
use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    pub id: String,
    pub name: String,
    pub weblink: String,
    pub rules: Option<String>,
    pub links: Vec<Link>,
}
