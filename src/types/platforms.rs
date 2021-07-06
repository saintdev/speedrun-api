use serde::Deserialize;

use super::Link;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Platform {
    pub id: String,
    pub name: String,
    pub released: i64,
    pub links: Vec<Link>,
}
